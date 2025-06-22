use crate::editor::terminal::Position;
/// 光标位置状态
/// location = where we are in the text
#[derive(Copy, Clone)]
pub struct Location {
    /// 水平位置（列索引）
    pub x: usize,
    /// 垂直位置（行索引）
    pub y: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl From<Location> for Position {
    fn from(location: Location) -> Self {
        Self {
            x: location.x,
            y: location.y,
        }
    }
}

impl Location {
    pub const fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}
