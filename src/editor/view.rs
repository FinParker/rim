use crate::editor::terminal::{Position, Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::editor::INFO_SECTION_SIZE;

#[derive(Default)]
pub struct View {}

impl View {
    pub fn render(key_events_info: &[String; INFO_SECTION_SIZE]) -> Result<(), Error> {
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
