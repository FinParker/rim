/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-07 20:05:58
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-05-12 09:51:30
 * @FilePath: \rim\src\editor\view.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
mod buffer;
use buffer::Buffer;
use std::collections::VecDeque;

use crate::editor::terminal::{Position, Size, Terminal};
use std::cmp::min;
use std::io::Error;

// const NAME: &str = env!("CARGO_PKG_NAME");
// const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const INFO_SECTION_SIZE: usize = 5;

pub struct View {
    key_events_info: VecDeque<String>,
    buffer: Buffer,
    needs_redraw_buffer: bool,
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
    pub fn load_file(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load_file(filename) {
            self.buffer = buffer;
            self.log_event("INFO", &format!("{filename:?} opened."));
        }
    }

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
                // 没有信息时显示空行标志
                Terminal::print("")?;
            }
            Terminal::move_cursor_to(Position {
                x: 0,
                y: min(cur_row + 1, INFO_SECTION_SIZE - 1),
            })?;
        }
        Ok(())
    }

    fn render_buffer(&mut self) -> Result<(), Error> {
        let Size { height, width } = self.size;
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

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

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
