/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-04-30 21:21:13
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-05-01 11:03:44
 * @FilePath: \rim\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
mod editor;
use editor::Editor;
fn main() {
    let mut editor = Editor::default();
    editor.run();
    // Editor::default().run();
    println!("----------------");
    println!("Succeeds to end!");
}
