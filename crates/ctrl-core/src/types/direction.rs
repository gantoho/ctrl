use std::fmt;

/// 方向/排列枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Horizontal => write!(f, "horizontal"),
            Direction::Vertical => write!(f, "vertical"),
        }
    }
}