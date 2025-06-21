/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-04-30 21:21:13
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-06-21 10:23:47
 * @FilePath: \rim\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::as_conversions,
    clippy::integer_division
)]

//! RIM 文本编辑器主模块
//!
//! 提供编辑器核心功能入口点，负责初始化并运行编辑器实例

mod editor;
use editor::Editor;

/// 编辑器主入口函数
///
/// 初始化默认编辑器实例并启动主事件循环
///
/// # 示例
/// ```no_run
/// fn main() {
///     rim::main();
/// }
/// ```
///
/// # 错误
/// 返回 `std::io::Error` 如果终端初始化失败
fn main() {
    let mut editor = Editor::default();
    editor.run();
    // Editor::default().run();
}
