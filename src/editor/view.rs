/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-07 20:05:58
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-06-23 14:13:05
 * @FilePath: \rim\src\editor\view.rs
 * @Description: 编辑器视图组件
 */
//! 编辑器视图模块
//!
//! 负责管理编辑器界面渲染，包括：
//! - 信息区域（事件日志）
//! - 文本缓冲区显示
//! - 尺寸适应
//!
//! 使用双缓冲区策略优化渲染性能

mod buffer;
mod line;
mod location;
use super::editorcommand::{Direction, EditorCommand};
use buffer::Buffer;
use line::Line;
use location::Location;
use std::collections::VecDeque;

use crate::editor::terminal::{Position, Size, Terminal};
use std::cmp::min;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 信息区域高度（固定行数）
pub const INFO_SECTION_SIZE: usize = 5;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Normal,
    Insert,
    Visual,
    Command,
}

/// 编辑器视图管理器
///
/// 包含两个主要区域：
/// 1. 顶部信息区域（显示事件日志）
/// 2. 主文本缓冲区显示区域
pub struct View {
    /// 事件日志队列（FIFO）
    key_events_info: VecDeque<String>,
    /// 文本缓冲区实例
    buffer: Buffer,
    /// 当前终端尺寸
    size: Size,
    /// 当前位置,确定渲染边界
    location: Location,
    /// 字素位置
    /// `screen`的`buffer`区左上角和原始数据左上角的偏移
    scroll_offset: Location,
    /// 当前模式
    mode: ViewMode,
    /// 缓冲区重绘标志
    needs_redraw_buffer: bool,
    /// 是否记录`KeyRelease`和`KeyRepeat`
    only_log_key_press: bool,
}

impl Default for View {
    fn default() -> Self {
        Self {
            key_events_info: VecDeque::default(),
            buffer: Buffer::default(),
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
            mode: ViewMode::Normal,
            needs_redraw_buffer: true,
            only_log_key_press: true,
        }
    }
}

impl View {
    pub fn get_mode(&self) -> ViewMode {
        self.mode
    }
    /// 加载文件到缓冲区
    ///
    /// # 参数
    /// - `filename`: 文件路径
    ///
    /// 成功加载后会记录打开事件
    pub fn load_file(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load_file(filename) {
            self.buffer = buffer;
            self.log_event("INFO", &format!("{filename:?} opened."));
        }
    }

    /// 处理事件命令
    ///
    /// # 参数
    /// - `command`: 时间命令
    ///
    /// 根据不同命令执行不同路径
    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Help => {
                self.help();
            }
            EditorCommand::Move(direction) => {
                self.move_text_location(direction);
            }
            EditorCommand::Resize(size) => {
                self.resize(size);
            }
            EditorCommand::OtherKeyCommand(string) => {
                if !self.only_log_key_press {
                    self.handle_other_key_command(&string);
                }
            }
            EditorCommand::OtherEvent(string) => {
                self.handle_other_event(&string);
            }
            EditorCommand::Quit => {}
            _ => {}
        }
    }

    /// 主渲染入口
    ///
    /// 根据终端尺寸决定渲染策略：
    /// - 足够大：渲染信息区域+缓冲区
    /// - 太小：显示警告信息
    pub fn render(&mut self) {
        let Size { height, width: _ } = self.size;
        let _ = Terminal::move_cursor_to(Position { x: 0, y: 0 });

        if height > INFO_SECTION_SIZE {
            self.render_info();
            if self.needs_redraw_buffer {
                if self.buffer.is_empty() {
                    self.render_welcome_buffer();
                } else {
                    self.render_buffer();
                }
            }
        } else {
            self.draw_size_warning();
        }
    }

    pub fn get_cursor_position(&self) -> Position {
        let Location { x, y } = self.location;
        let Location { x: x_off, y: y_off } = self.scroll_offset;
        Position {
            x: x.saturating_sub(x_off),
            y: y.saturating_sub(y_off) + INFO_SECTION_SIZE,
        }
    }

    /// 渲染信息区域
    ///
    /// 在终端顶部显示事件日志队列
    fn render_info(&mut self) {
        let Size { height: _, width } = self.size;
        for row in 0..INFO_SECTION_SIZE {
            let _ = Terminal::move_cursor_to_row(row);
            let _ = Terminal::clear_line();
            if let Some(info) = self.key_events_info.get(row) {
                let display_info = if info.len() > width {
                    format!("{}", &info)
                    //format!("{}...", &info[..width.saturating_sub(3)])
                } else {
                    info.clone()
                };
                let _ = Terminal::print(&display_info);
            } else {
                let _ = Terminal::print("");
            }
        }
    }

    /// 渲染文本缓冲区
    ///
    /// 在信息区域下方显示文件内容
    fn render_buffer(&mut self) {
        let Size { height, width } = self.size;
        let top_row = self.scroll_offset.y;

        // let mut log_entries = Vec::new();

        for row in INFO_SECTION_SIZE..height {
            let _ = Terminal::move_cursor_to_row(row);
            let _ = Terminal::clear_line();
            let buffer_index = row - INFO_SECTION_SIZE;
            if let Some(line) = self.buffer.lines.get(buffer_index.saturating_add(top_row)) {
                let start = self.scroll_offset.x;
                let end = self.scroll_offset.x.saturating_add(width);
                let info = &line.get_display_string(start..end);
                // log_entries.push((start, end, info.clone()));
                let _ = Terminal::print(info);
            } else {
                Self::draw_empty_row();
            }
        }
        // for (start, end, info) in log_entries {
        //     self.log_event("TEST", &format!("{start} - {end}"));
        //     self.log_event("TEST", &info);
        // }
        self.needs_redraw_buffer = false;
    }

    fn render_welcome_buffer(&mut self) {
        let Size { height, width: _ } = self.size;
        for row in INFO_SECTION_SIZE..height {
            let _ = Terminal::move_cursor_to_row(row);
            let _ = Terminal::clear_line();
            let buffer_index = row - INFO_SECTION_SIZE;
            #[allow(clippy::integer_division)]
            let start_index = (height - INFO_SECTION_SIZE) / 3;
            if buffer_index == start_index {
                self.draw_welcome_msg();
            } else if buffer_index == start_index + 2 {
                self.draw_help_msg();
            } else {
                Self::draw_empty_row();
            }
        }
        self.needs_redraw_buffer = false;
    }

    /// 绘制空行指示符
    ///
    /// 在缓冲区末尾显示 `~` 符号表示空行
    fn draw_empty_row() {
        let _ = Terminal::print("~");
    }

    /// 绘制欢迎指示符
    ///
    fn draw_welcome_msg(&self) {
        let mut welcome_msg = format!("{NAME} editor -- version {VERSION}");
        let width = self.size.width;
        let len = welcome_msg.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width);
        let _ = Terminal::print(&welcome_msg);
    }

    /// 绘制帮助指示符
    ///
    fn draw_help_msg(&self) {
        let mut help_msg = "Press <Ctrl+h> for help; Press <Ctrl+q> to exit".to_string();
        let width = self.size.width;
        let len = help_msg.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        help_msg = format!("~{spaces}{help_msg}");
        help_msg.truncate(width);
        let _ = Terminal::print(&help_msg);
    }

    /// 绘制终端过小警告
    fn draw_size_warning(&self) {
        const WARNING_MSG: &str = "终端尺寸过小，建议调整窗口大小";
        for row in 0..self.size.height {
            let _ = Terminal::clear_line();
            if row == 0 {
                let _ = Terminal::print(WARNING_MSG);
            }
            let _ = Terminal::move_cursor_to(Position {
                x: 0,
                y: min(row + 1, self.size.height - 1),
            });
        }
    }

    /// 记录事件到信息区域
    ///
    /// # 参数
    /// - `tag`: 事件分类标签（如 "INFO"）
    /// - `info`: 事件详细信息
    ///
    /// 事件会显示在终端顶部的信息区域
    pub fn log_event(&mut self, tag: &str, info: &str) {
        let str = format!("[{tag:<4}] {info}");
        if INFO_SECTION_SIZE == 0 {
            return;
        }
        if self.key_events_info.len() >= INFO_SECTION_SIZE {
            self.key_events_info.pop_front();
        }
        self.key_events_info.push_back(str);
    }

    /// 处理帮助命令
    ///
    /// 会在`INFO`区打印`help`信息
    fn help(&mut self) {
        let info = "Press <Ctrl+q> to quit the editor";
        self.log_event("HELP", info);
    }

    /// 处理终端尺寸变化命令
    ///
    /// # 参数
    /// - `to`: 新的终端尺寸
    ///
    /// 会触发重绘并记录尺寸变化事件
    fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw_buffer = true;
        self.log_event(
            "INFO",
            &format!(
                "Change window size to {{ height: {} }}, {{ width:  {} }}",
                self.size.height, self.size.width
            ),
        );
    }

    /// 处理屏幕移动
    /// TODO
    ///
    /// # 参数
    /// - `direction`: 移动方式
    ///
    fn move_text_location(&mut self, direction: Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, .. } = self.size;
        let buffer_height = height - INFO_SECTION_SIZE;
        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            }
            Direction::Down => {
                y = y.saturating_add(1);
            }
            Direction::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    x = self.buffer.lines.get(y).map_or(0, Line::byte_len);
                }
            }
            Direction::Right => {
                let width = self.buffer.lines.get(y).map_or(0, Line::byte_len);
                if x < width {
                    x += 1;
                } else {
                    y = y.saturating_add(1);
                    x = 0;
                }
            }
            Direction::PageUp => {
                y = y.saturating_sub(buffer_height - 1);
            }
            Direction::PageDown => {
                y = y.saturating_add(buffer_height - 1);
            }
            Direction::Home => {
                x = 0;
            }
            Direction::End => {
                x = self.buffer.lines.get(y).map_or(0, Line::byte_len);
            }
        }
        // 限制Location {x, y} 不会超出一行的长度,不会超出文档的长度
        x = self
            .buffer
            .lines
            .get(y)
            .map_or(0, |line| min(line.byte_len(), x));
        y = min(y, self.buffer.lines.len());
        self.location = Location { x, y };
        self.scroll_location_into_view();
        self.log_event("MOVE", &format!("{direction:?}"));
    }

    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;
        // 如果text location在screen外, 需要滚动
        // 水平滚动
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        // 垂直滚动
        let buffer_height = height - INFO_SECTION_SIZE;
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(buffer_height) {
            self.scroll_offset.y = y.saturating_sub(buffer_height).saturating_add(1);
            offset_changed = true;
        }

        self.needs_redraw_buffer = offset_changed;
    }

    fn handle_other_key_command(&mut self, str: &str) {
        self.log_event("KEY", str);
    }

    fn handle_other_event(&mut self, str: &str) {
        self.log_event("OTH", str);
    }
}
