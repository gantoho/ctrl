use std::fmt;

/// 形状枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Shape {
    #[default]
    Default,
    Circle,
    Rounded,
    Square,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Default => write!(f, "default"),
            Shape::Circle => write!(f, "circle"),
            Shape::Rounded => write!(f, "rounded"),
            Shape::Square => write!(f, "square"),
        }
    }
}