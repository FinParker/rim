/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-03 20:49:45
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-05-03 22:21:36
 * @FilePath: \rim\src\terminal.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use crossterm::cursor::{position, Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Error, Write};

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}
pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn print(str: &str) -> Result<(), Error> {
        queue!(stdout(), Print(str))?;

        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(postion: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(postion.x, postion.y))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}
