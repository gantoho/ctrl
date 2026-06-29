use std::fmt;

/// 表单布局枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Layout {
    #[default]
    Vertical,
    Horizontal,
    Inline,
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Layout::Vertical => write!(f, "vertical"),
            Layout::Horizontal => write!(f, "horizontal"),
            Layout::Inline => write!(f, "inline"),
        }
    }
}