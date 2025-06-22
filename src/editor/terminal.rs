/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-03 20:49:45
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-06-22 12:08:39
 * @FilePath: \rim\src\terminal.rs
 * @Description: 终端操作抽象层
 */
//! 终端交互模块
//!
//! 提供跨平台的终端操作抽象，包括：
//! - 原始模式切换
//! - 光标控制
//! - 屏幕清理
//! - 尺寸获取
//!
//! 使用 `crossterm` 库实现跨平台支持

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};

/// 终端尺寸表示
#[derive(Default, Copy, Clone, Debug)]
pub struct Size {
    /// 终端高度（行数）
    pub height: usize,
    /// 终端宽度（列数）
    pub width: usize,
}

/// 光标位置坐标
///
/// Position = where we are on the screen
#[derive(Copy, Clone)]
pub struct Position {
    /// 水平位置（列索引，0-based）
    pub x: usize,
    /// 垂直位置（行索引，0-based）
    pub y: usize,
}

/// 终端操作抽象
///
/// 所有方法均为静态方法，提供全局终端状态管理
pub struct Terminal {}

impl Terminal {
    /// 初始化终端
    ///
    /// 执行以下操作：
    /// 1. 启用raw mode
    /// 2. 进入备用Screen, 防止应用输出污染终端历史
    /// 3. 清屏
    /// 4. 移动光标到原点 (0, 0)
    ///
    /// # 错误
    /// 返回 `std::io::Error` 如果底层终端操作失败
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    /// 恢复终端原始状态
    ///
    /// 执行以下操作：
    /// 1. 退出备用Screen, 回到主Screen
    /// 2. 显示光标
    /// 3. 刷新输出缓存
    /// 4. 禁用原始模式
    ///
    /// # 错误
    /// 返回 `std::io::Error` 如果底层终端操作失败
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::show_cursor()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    /// 进入备用屏幕
    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    /// 退出备用屏幕
    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    /// 在当前位置打印字符串
    ///
    /// # 参数
    /// - `str`: 要打印的字符串内容
    ///
    /// # 注意
    /// 需要配合 [`Terminal::execute`] 方法刷新输出
    ///
    /// # 示例
    /// ```ignore
    /// Terminal::print("Hello world!")?;
    /// Terminal::execute()?;
    /// ```
    pub fn print(str: &str) -> Result<(), Error> {
        Self::queue_command(Print(str))?;
        Ok(())
    }

    /// 清空整个屏幕
    ///
    /// # 注意
    /// 需要配合 [`Terminal::execute`] 方法刷新输出
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    /// 清空当前行
    ///
    /// # 注意
    /// 需要配合 [`Terminal::execute`] 方法刷新输出
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// 移动光标到指定位置
    ///
    /// # 参数
    /// - `position`: 目标位置坐标
    ///
    /// # 注意
    /// 坐标超出终端尺寸会导致未定义行为
    ///
    /// # 示例
    /// ```ignore
    /// Terminal::move_cursor_to(Position { x: 5, y: 10 })?;
    /// ```
    pub fn move_cursor_to(postion: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(postion.x as u16, postion.y as u16))?;
        Ok(())
    }

    /// 移动光标到指定行
    ///
    /// # 参数
    /// - `row`: 目标行
    pub fn move_cursor_to_row(row: usize) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(0, row as u16))?;
        Ok(())
    }

    /// 隐藏光标
    ///
    /// 常用于全屏应用避免光标闪烁干扰
    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    /// 显示光标
    ///
    /// 恢复光标显示状态
    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    /// 获取当前终端尺寸
    ///
    /// # 返回
    /// [`Size`] 结构体包含当前终端的行数和列数
    ///
    /// # 错误
    /// 返回 `std::io::Error` 如果获取尺寸失败
    ///
    /// # 示例
    /// ```ignore
    /// let size = Terminal::size()?;
    /// println!("终端尺寸: {}x{}", size.width, size.height);
    /// ```
    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        #[allow(clippy::as_conversions)]
        let width = width as usize;
        #[allow(clippy::as_conversions)]
        let height = height as usize;
        Ok(Size { height, width })
    }

    /// 刷新输出缓冲区
    ///
    /// 执行所有排队中的终端命令
    ///
    /// # 注意
    /// 所有队列操作（如打印、移动光标等）都需要调用此方法才能生效
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    /// 内部：排队终端命令
    fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
