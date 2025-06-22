/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-07 21:39:44
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-06-22 15:46:31
 * @FilePath: \rim\src\editor\view\buffer.rs
 * @Description: 文本缓冲区实现
 */
//! 文本缓冲区模块
//!
//! 提供文件加载和文本行存储功能
//!
//! 主要结构：
//! - [`Buffer`]: 存储文本行并提供基本操作

use super::line::Line;
use std::{fs::read_to_string, io::Error};

/// 文本缓冲区
///
/// 按行存储文本内容，支持从文件加载
#[derive(Default)]
pub struct Buffer {
    /// 文本行存储向量
    pub lines: Vec<Line>,
}

impl Buffer {
    /// 从文件加载内容到缓冲区
    ///
    /// # 参数
    /// - `filename`: 文件路径
    ///
    /// # 返回
    /// 包含文本行的 [`Buffer`] 实例
    ///
    /// # 错误
    /// 返回 `std::io::Error` 如果文件读取失败
    ///
    /// # 示例
    /// ```no_run
    /// let buffer = Buffer::load_file("example.txt")?;
    /// ```
    pub fn load_file(filename: &str) -> Result<Self, Error> {
        let file_contents = read_to_string(filename)?;
        let mut lines = Vec::new();
        for line_str in file_contents.lines() {
            lines.push(Line::from(line_str));
        }
        Ok(Self { lines })
    }

    /// 检查缓冲区是否为空
    ///
    /// # 返回
    /// `true` 如果缓冲区不包含任何文本行
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
