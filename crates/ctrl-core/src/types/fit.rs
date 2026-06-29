use std::fmt;

/// 图片填充模式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Fit {
    #[default]
    Cover,
    Contain,
    Fill,
    None,
}

impl fmt::Display for Fit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fit::Cover => write!(f, "cover"),
            Fit::Contain => write!(f, "contain"),
            Fit::Fill => write!(f, "fill"),
            Fit::None => write!(f, "none"),
        }
    }
}