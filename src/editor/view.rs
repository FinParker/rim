/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-07 20:05:58
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-05-09 12:08:45
 * @FilePath: \rim\src\editor\view.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
mod buffer;
use buffer::Buffer;

use crate::editor::terminal::{Position, Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::editor::INFO_SECTION_SIZE;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render(&self, key_events_info: &[String; INFO_SECTION_SIZE]) -> Result<(), Error> {
        Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        let Size { height, width } = Terminal::size()?;

        if height > 0 {
            for cur_row in 0..height - 1 {
                Terminal::clear_line()?;
                match cur_row {
                    // 前n行显示按键信息
                    y if y < INFO_SECTION_SIZE => {
                        let info = key_events_info[cur_row].clone();
                        let display_info = if info.len() > width {
                            format!("{}...", &info[..width.saturating_sub(3)])
                        } else {
                            info.clone()
                        };
                        Terminal::print(&display_info)?;
                    }
                    // 欢迎信息行
                    #[allow(clippy::integer_division)]
                    y if y == height / 2 => {
                        Self::draw_welcome_msg_row()?;
                    }
                    y if y >= INFO_SECTION_SIZE => {
                        if let Some(line) = self.buffer.lines.get(y - INFO_SECTION_SIZE) {
                            Terminal::print(line)?;
                        } else {
                            Self::draw_empty_row()?;
                        }
                    }
                    // 其他普通行
                    _ => {
                        Self::draw_empty_row()?;
                    }
                }
                Terminal::print("\r\n")?;
            }
            Terminal::print("~")?;
        }

        Ok(())
    }

    fn draw_welcome_msg_row() -> Result<(), Error> {
        let mut welcome_msg = format!("{NAME} editor -- version {VERSION}");
        let Size { height: _, width } = Terminal::size()?;
        let msg_len = welcome_msg.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(msg_len)) / 2;
        let spaces = " ".repeat(padding);
        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width);
        Terminal::print(&welcome_msg)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
