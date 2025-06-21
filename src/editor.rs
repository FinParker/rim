/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-01 08:52:36
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-06-21 20:12:41
 * @FilePath: \rim\src\editor.rs
 * @Description: 编辑器核心模块 - 主事件循环和状态管理
 */
//! 编辑器核心引擎
//!
//! ## 设计架构
//! 1. 事件驱动模型：REPL (Read-Evaluate-Print Loop)
//! 2. 状态机管理：处理用户交互和视图更新
//! 3. 命令解析：处理命令行参数和键盘事件
//!
//! ## 关键组件
//! - `Editor`: 主控制器，协调各子系统
//! - `Location`: 光标位置状态
//! - 事件处理器：将原始事件转换为编辑器操作

mod terminal;
mod view;

use core::cmp::{max, min};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::{Position, Size, Terminal};
use view::View;

use crate::editor::view::INFO_SECTION_SIZE;

/// 光标位置状态
///
/// ## 设计选择
/// 独立于视图坐标系统，简化位置管理
#[derive(Copy, Clone)]
struct Location {
    /// 水平位置（列索引）
    x: usize,
    /// 垂直位置（行索引）
    y: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            x: 0,
            y: INFO_SECTION_SIZE,
        }
    }
}

/// 编辑器主控制器
///
/// ## 职责划分
/// 1. 管理应用程序生命周期
/// 2. 处理用户输入事件
/// 3. 协调视图更新
/// 4. 维护光标位置状态
pub struct Editor {
    /// 退出标志，控制主循环终止
    should_quit: bool,
    /// 当前光标位置状态
    location: Location,
    /// 视图控制器实例
    view: View,
}

impl Editor {
    /// Editor构造器
    ///
    /// ## 初始化流程
    /// 1. 添加自定义panic hook
    /// 2. Terminal初始化
    /// 3. 构建view
    /// 4. 解析命令行参数,加载文件
    ///
    /// ## 设计思想
    /// 把所有无法处理且需要panic的错误都移动到了new中
    /// 对于其他所有情况,我们应当容忍错误,不要让程序崩溃,这里选择设置run禁止向上传播错误
    pub fn new() -> Result<Self, Error> {
        println!("This is Editor::new()");
        let cur_hook = take_hook();
        // 使用move将所有权转移到闭包中,防止cur_hook在new后被drop
        set_hook(Box::new(move |panic_info| {
            println!("Custom panic hook");
            let _ = Terminal::terminate();
            cur_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(filename) = args.get(1) {
            view.load_file(filename);
        } else {
            view.log_event("INFO", "No file opened.");
        }
        Ok(Self {
            should_quit: false,
            location: Location::default(),
            view,
        })
    }
    /// 启动编辑器主循环
    ///
    /// ## 执行序列
    /// 1. 启动REPL事件循环
    pub fn run(&mut self) {
        self.repl();
    }

    /// 主事件循环 (REPL模式)
    ///
    /// ## 核心流程
    /// 1. 刷新屏幕
    /// 2. 读取输入事件
    /// 3. 处理事件
    /// 4. 循环直到退出标志置位
    ///
    /// ## 错误处理
    /// 1. debug模式下, 单次read event失败, 直接panic, 程序退出
    /// 2. release模式下, 容忍单次read event失败, 程序不退出, 忽略错误, 继续循环, 尝试刷新屏幕(保持监听,如果事件到了, 仍正常处理)
    fn repl(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => {
                    self.evaluate_event(&event);
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        log::error!("Read event error (ignored): {}", err); // 添加日志记录
                    }
                }
            }
        }
    }

    /// 移动光标位置
    ///
    /// ## 边界处理策略
    /// 使用saturating运算避免越界，确保位置始终有效
    fn move_point(&mut self, key_code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();

        match key_code {
            KeyCode::Up => y = max(INFO_SECTION_SIZE, y.saturating_sub(1)),
            KeyCode::Down => y = min(height.saturating_sub(1), y.saturating_add(1)),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = min(width.saturating_sub(1), x.saturating_add(1)),
            KeyCode::PageUp => y = INFO_SECTION_SIZE,
            KeyCode::PageDown => y = height.saturating_sub(1),
            KeyCode::Home => x = 0,
            KeyCode::End => x = width.saturating_sub(1),
            _ => (),
        }

        self.location = Location { x, y };
    }

    /// 事件评估与路由
    ///
    /// ## 处理策略
    /// 1. 记录所有按键事件
    /// 2. 特殊组合键触发状态变更
    /// 3. 导航键更新光标位置
    /// 4. 窗口尺寸变化通知视图
    fn evaluate_event(&mut self, event: &Event) {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                // 记录所有按键事件用于调试
                self.view.log_event(
                    "KEY",
                    &format!("Key {code:?} Pressed, modifiers: {modifiers:?}"),
                );

                match (*code, *modifiers) {
                    // Ctrl-Q 退出组合键
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        self.should_quit = true;
                    }
                    // 导航键处理
                    (
                        KeyCode::Up
                        | KeyCode::Down
                        | KeyCode::Left
                        | KeyCode::Right
                        | KeyCode::PageDown
                        | KeyCode::PageUp
                        | KeyCode::End
                        | KeyCode::Home,
                        _,
                    ) => {
                        self.move_point(*code);
                    }
                    _ => {}
                }
            }
            // 窗口尺寸变化事件
            Event::Resize(width, height) => {
                #[allow(clippy::as_conversions)]
                self.view.resize(Size {
                    height: *height as usize,
                    width: *width as usize,
                });
            }
            _ => {}
        }
    }

    /// 刷新屏幕内容
    ///
    /// ## 双状态渲染
    /// 1. 正常状态：更新视图内容
    /// 2. 退出状态：显示告别信息
    fn refresh_screen(&mut self) {
        Terminal::hide_cursor();

        if self.should_quit {
            // 退出状态渲染
            Terminal::clear_screen();
            Terminal::move_cursor_to(Position { x: 0, y: 0 });
            Terminal::print("Goodbye. <rim> user.\r\n");
        } else {
            // 正常状态渲染
            self.view.render();
            Terminal::move_cursor_to(Position {
                x: self.location.x,
                y: self.location.y,
            });
        }

        Terminal::show_cursor();
        Terminal::execute();
    }
}
