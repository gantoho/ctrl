use std::fmt;

/// 过渡效果枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Effect {
    #[default]
    Slide,
    Fade,
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Effect::Slide => write!(f, "slide"),
            Effect::Fade => write!(f, "fade"),
        }
    }
}