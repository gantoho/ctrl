use std::fmt;

/// 弹出位置枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Placement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    BottomStart,
    BottomEnd,
}

impl fmt::Display for Placement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Placement::Top => write!(f, "top"),
            Placement::Bottom => write!(f, "bottom"),
            Placement::Left => write!(f, "left"),
            Placement::Right => write!(f, "right"),
            Placement::TopStart => write!(f, "top-start"),
            Placement::TopEnd => write!(f, "top-end"),
            Placement::BottomStart => write!(f, "bottom-start"),
            Placement::BottomEnd => write!(f, "bottom-end"),
        }
    }
}