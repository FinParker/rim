/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-06-22 15:36:55
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-06-22 20:10:13
 * @FilePath: \rim\src\editor\view\line.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use std::{cmp, ops::Range};
pub struct Line {
    string: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        Self {
            string: String::from(line_str),
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp::min(range.end, self.string.len());
        self.string.get(start..end).unwrap_or_default().to_string()
    }
    pub fn len(&self) -> usize {
        self.string.len()
    }
}
