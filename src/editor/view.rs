/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-07 20:05:58
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-05-12 09:51:30
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
use buffer::Buffer;
use std::collections::VecDeque;

use crate::editor::terminal::{Position, Size, Terminal};
use std::cmp::min;
use std::io::Error;

/// 信息区域高度（固定行数）
pub const INFO_SECTION_SIZE: usize = 5;

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
    /// 缓冲区重绘标志
    needs_redraw_buffer: bool,
    /// 当前终端尺寸
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            key_events_info: VecDeque::default(),
            buffer: Buffer::default(),
            needs_redraw_buffer: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    /// 处理终端尺寸变化
    ///
    /// # 参数
    /// - `to`: 新的终端尺寸
    ///
    /// 会触发重绘并记录尺寸变化事件
    pub fn resize(&mut self, to: Size) {
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

    /// 渲染信息区域
    ///
    /// 在终端顶部显示事件日志队列
    fn render_info(&mut self) -> Result<(), Error> {
        let Size { height: _, width } = self.size;
        for cur_row in 0..INFO_SECTION_SIZE {
            Terminal::clear_line()?;
            if let Some(info) = self.key_events_info.get(cur_row) {
                let display_info = if info.len() > width {
                    format!("{}...", &info[..width.saturating_sub(3)])
                } else {
                    info.clone()
                };
                Terminal::print(&display_info)?;
            } else {
                Terminal::print("")?;
            }
            Terminal::move_cursor_to(Position {
                x: 0,
                y: min(cur_row + 1, INFO_SECTION_SIZE - 1),
            })?;
        }
        Ok(())
    }

    /// 渲染文本缓冲区
    ///
    /// 在信息区域下方显示文件内容
    fn render_buffer(&mut self) -> Result<(), Error> {
        let Size { height, width: _ } = self.size;
        Terminal::move_cursor_to(Position {
            x: 0,
            y: INFO_SECTION_SIZE,
        })?;
        for cur_row in INFO_SECTION_SIZE..height {
            let buffer_index = cur_row - INFO_SECTION_SIZE;
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(buffer_index) {
                Terminal::print(line)?;
            } else {
                Self::draw_empty_row()?;
            }
            Terminal::move_cursor_to(Position {
                x: 0,
                y: min(cur_row + 1, height - 1),
            })?;
        }
        self.needs_redraw_buffer = false;
        Ok(())
    }

    /// 主渲染入口
    ///
    /// 根据终端尺寸决定渲染策略：
    /// - 足够大：渲染信息区域+缓冲区
    /// - 太小：显示警告信息
    pub fn render(&mut self) -> Result<(), Error> {
        let Size { height, width: _ } = self.size;
        Terminal::move_cursor_to(Position { x: 0, y: 0 })?;

        if height > INFO_SECTION_SIZE {
            self.render_info()?;
            if self.needs_redraw_buffer {
                self.render_buffer()?;
            }
        } else {
            self.draw_size_warning()?;
        }
        Ok(())
    }

    /// 绘制空行指示符
    ///
    /// 在缓冲区末尾显示 `~` 符号表示空行
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    /// 绘制终端过小警告
    fn draw_size_warning(&self) -> Result<(), Error> {
        const WARNING_MSG: &str = "终端尺寸过小，建议调整窗口大小";
        for row in 0..self.size.height {
            Terminal::clear_line()?;
            if row == 0 {
                Terminal::print(WARNING_MSG)?;
            }
            Terminal::move_cursor_to(Position {
                x: 0,
                y: min(row + 1, self.size.height - 1),
            })?;
        }
        Ok(())
    }

    /// 记录事件到信息区域
    ///
    /// # 参数
    /// - `tag`: 事件分类标签（如 "INFO"）
    /// - `info`: 事件详细信息
    ///
    /// 事件会显示在终端顶部的信息区域
    pub fn log_event(&mut self, tag: &str, info: &str) {
        let str = format!("[{:<4}] {}", tag, info);
        if INFO_SECTION_SIZE == 0 {
            return;
        }
        if self.key_events_info.len() >= INFO_SECTION_SIZE {
            self.key_events_info.pop_front();
        }
        self.key_events_info.push_back(str);
    }
}
