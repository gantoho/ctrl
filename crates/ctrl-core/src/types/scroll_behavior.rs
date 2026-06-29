use std::fmt;

/// 滚动行为枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ScrollBehavior {
    #[default]
    Smooth,
    Auto,
}

impl fmt::Display for ScrollBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScrollBehavior::Smooth => write!(f, "smooth"),
            ScrollBehavior::Auto => write!(f, "auto"),
        }
    }
}