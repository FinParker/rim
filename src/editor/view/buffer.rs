/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-07 21:39:44
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-05-09 11:58:49
 * @FilePath: \rim\src\editor\view\buffer.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            lines: vec!["Hello, World!".to_string()],
        }
    }
}
