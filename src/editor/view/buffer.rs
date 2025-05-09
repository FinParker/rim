/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-07 21:39:44
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-05-09 13:02:41
 * @FilePath: \rim\src\editor\view\buffer.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use std::{fs::read_to_string, io::Error};
#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load_file(filename: &str) -> Result<Self, Error> {
        let file_contents = read_to_string(filename)?;
        let mut lines = Vec::new();
        for line in file_contents.lines() {
            lines.push(String::from(line));
        }
        Ok(Self { lines })
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
