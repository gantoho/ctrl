use std::fmt;

/// 组件语义变体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variant::Primary => write!(f, "primary"),
            Variant::Secondary => write!(f, "secondary"),
            Variant::Outline => write!(f, "outline"),
            Variant::Ghost => write!(f, "ghost"),
        }
    }
}