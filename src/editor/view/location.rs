/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-06-22 15:20:34
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-06-23 16:06:06
 * @FilePath: \rim\src\editor\view\location.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use crate::editor::terminal::Position;
use std::fmt;

/// 光标位置状态
/// location = where we are in the text
#[derive(Copy, Clone, Default, Debug)]
pub struct Location {
    /// 水平位置（列索引）
    pub x: usize,
    /// 垂直位置（行索引）
    pub y: usize,
}

impl From<Location> for Position {
    fn from(location: Location) -> Self {
        Self {
            x: location.x,
            y: location.y,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "row {} col {}", self.y + 1, self.x + 1)
    }
}
