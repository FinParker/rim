/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-01 08:52:36
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-05-09 11:56:49
 * @FilePath: \rim\src\editor.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-01 08:52:36
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-05-08 09:26:01
 * @FilePath: \rim\src\editor.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

mod terminal;
mod view;

use core::cmp::{max, min};
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};

use std::io::Error;
use terminal::{Position, Size, Terminal};
use view::View;
pub const INFO_SECTION_SIZE: usize = 5;

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location, // cursor's position
    key_events_info: [String; INFO_SECTION_SIZE],
    current_info_line: usize,
    view: View,
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
            let info =
                format!("[INFO] <KEY> {code:?} Pressed, Modifiers: {modifiers:?} State: {state:?}");
            if self.current_info_line + 1 < INFO_SECTION_SIZE {
                self.key_events_info[self.current_info_line] = info;
                self.current_info_line += 1;
            } else if INFO_SECTION_SIZE > 0 {
                for cur_row in 0..INFO_SECTION_SIZE - 1 {
                    self.key_events_info[cur_row] = self.key_events_info[cur_row + 1].clone();
                }
                self.key_events_info[INFO_SECTION_SIZE - 1] = info;
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
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
            Terminal::print("Goodbye. <rim> user.\r\n")?;
            Terminal::execute()?;
        } else {
            self.view.render(&self.key_events_info)?; // 原来是Editor::draw_rows()方法
            Terminal::move_cursor_to(Position {
                x: self.location.x,
                y: self.location.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}
