use crate::editor::terminal::Position;

/// 光标位置状态
/// location = where we are in the text
#[derive(Copy, Clone, Default)]
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
