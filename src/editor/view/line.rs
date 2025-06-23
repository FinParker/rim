/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-06-22 15:36:55
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-06-23 09:53:41
 * @FilePath: \rim\src\editor\view\line.rs
 * @Description: 行处理, 支持字素切分
 */
use std::{cmp, ops::Range};

use unicode_segmentation::UnicodeSegmentation;
pub struct Line {
    string: String,
    graphmes: Vec<String>,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        Self {
            string: String::from(line_str),
            graphmes: UnicodeSegmentation::graphemes(line_str, true)
                .map(|s| s.to_string())
                .collect(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.string
    }

    pub fn to_string(&self) -> String {
        self.string.clone()
    }

    pub fn graphemes(&self) -> &Vec<String> {
        &self.graphmes
    }

    pub fn get_graphemes(&self, range: Range<usize>) -> String {
        if self.is_empty() {
            return String::new();
        }
        let start = range.start;
        let end = cmp::min(range.end, self.grapheme_len());
        if start >= self.grapheme_len() {
            return String::new();
        }
        self.graphemes()[start..end].concat()
    }
    pub fn byte_len(&self) -> usize {
        self.string.len()
    }

    pub fn grapheme_len(&self) -> usize {
        self.graphmes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}
