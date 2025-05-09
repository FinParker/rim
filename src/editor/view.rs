/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-07 20:05:58
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-05-09 14:03:02
 * @FilePath: \rim\src\editor\view.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
mod buffer;
use buffer::Buffer;
use crossterm::cursor;

use crate::editor::terminal::{Position, Size, Terminal};
use std::cmp::min;
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::editor::INFO_SECTION_SIZE;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn load_file(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load_file(filename) {
            self.buffer = buffer;
        }
    }

    fn render_info(
        &self,
        key_events_info: &[String; INFO_SECTION_SIZE],
        size: Size,
    ) -> Result<(), Error> {
        let Size { height: _, width } = size;
        for cur_row in 0..INFO_SECTION_SIZE {
            Terminal::clear_line()?;
            if cur_row < INFO_SECTION_SIZE {
                let info = key_events_info[cur_row].clone();
                let display_info = if info.len() > width {
                    format!("{}...", &info[..width.saturating_sub(3)])
                } else {
                    info
                };
                Terminal::print(&display_info)?;
            } else {
                Terminal::print("~")?;
            }
            Terminal::move_cursor_to(Position {
                x: 0,
                y: min(cur_row + 1, INFO_SECTION_SIZE - 1),
            })?;
        }
        Ok(())
    }

    fn render_buffer(&self, size: Size) -> Result<(), Error> {
        let Size { height, width: _ } = size;
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
        Ok(())
    }
    pub fn render(&self, key_events_info: &[String; INFO_SECTION_SIZE]) -> Result<(), Error> {
        let size = Terminal::size()?;
        let Size { height, width: _ } = size;
        Terminal::move_cursor_to(Position { x: 0, y: 0 })?;

        if height > INFO_SECTION_SIZE {
            self.render_info(key_events_info, size)?;
            self.render_buffer(size)?;
        } else {
            self.draw_size_warning(size)?;
        }
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_size_warning(&self, size: Size) -> Result<(), Error> {
        const WARNING_MSG: &str = "终端尺寸过小，建议调整窗口大小";
        for row in 0..size.height {
            Terminal::clear_line()?;
            if row == 0 {
                Terminal::print(WARNING_MSG)?;
            }
            Terminal::move_cursor_to(Position {
                x: 0,
                y: min(row + 1, size.height - 1),
            })?;
        }
        Ok(())
    }
}
