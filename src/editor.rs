/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-01 08:52:36
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-05-07 19:46:06
 * @FilePath: \rim\src\editor.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
mod terminal;
use core::cmp::{max, min};
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::io::Error;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const INFO_SECTION_SIZE: usize = 5;

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    key_events_info: [String; INFO_SECTION_SIZE as usize],
    current_info_line: usize,
    location: Location, // cursor's position
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.location.x = 0;
        self.location.y = INFO_SECTION_SIZE;
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                y = max(INFO_SECTION_SIZE, y.saturating_sub(1));
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = INFO_SECTION_SIZE;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state,
        }) = event
        {
            let info = format!("[INFO] Code: {code:?} Modifiers: {modifiers:?} State: {state:?}");
            if self.current_info_line + 1 < INFO_SECTION_SIZE as usize {
                self.key_events_info[self.current_info_line] = info;
                self.current_info_line += 1;
            } else if INFO_SECTION_SIZE > 0 {
                for cur_row in 0..INFO_SECTION_SIZE as usize - 1 {
                    self.key_events_info[cur_row] = self.key_events_info[cur_row + 1].clone();
                }
                self.key_events_info[INFO_SECTION_SIZE as usize - 1] = info;
            }
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position {
            x: 0,
            y: INFO_SECTION_SIZE,
        })?; // 光标定位到信息区下方
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
            Terminal::execute()?;
        } else {
            self.draw_rows()?;
            Terminal::move_cursor_to(Position {
                x: self.location.x,
                y: self.location.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows(&self) -> Result<(), Error> {
        Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        let Size { height, width } = Terminal::size()?;
        let terminal_width = width as usize;

        for cur_row in 0..height - 1 {
            Terminal::clear_line()?;

            match cur_row {
                // 前n行显示按键信息
                y if y < INFO_SECTION_SIZE => {
                    let info = &self.key_events_info[cur_row as usize];
                    let display_info = if info.len() > terminal_width {
                        format!("{}...", &info[..terminal_width.saturating_sub(3)])
                    } else {
                        info.clone()
                    };
                    Terminal::print(&display_info)?;
                }
                // 欢迎信息行
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
        Ok(())
    }

    fn draw_welcome_msg_row() -> Result<(), Error> {
        let mut welcome_msg = format!("{NAME} editor -- version {VERSION}");
        let Size { height: _, width } = Terminal::size()?;
        let msg_len = welcome_msg.len();
        let width = width as usize;
        let padding = (width.saturating_sub(msg_len)) / 2;
        let spaces = " ".repeat(padding - 1);
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
