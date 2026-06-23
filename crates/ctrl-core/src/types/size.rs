use std::fmt;

/// 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    #[default]
    Md,
    Sm,
    Lg,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Size::Sm => write!(f, "sm"),
            Size::Md => write!(f, "md"),
            Size::Lg => write!(f, "lg"),
        }
    }
}

impl Size {
    /// 获取对应的高度值
    pub fn height(&self) -> &'static str {
        match self {
            Size::Sm => "28px",
            Size::Md => "36px",
            Size::Lg => "44px",
        }
    }

    /// 获取对应的内边距
    pub fn padding(&self) -> &'static str {
        match self {
            Size::Sm => "4px 12px",
            Size::Md => "8px 16px",
            Size::Lg => "12px 24px",
        }
    }

    /// 获取对应的字体大小 CSS 变量
    pub fn font_size_var(&self) -> &'static str {
        match self {
            Size::Sm => "var(--ctrl-font-size-sm)",
            Size::Md => "var(--ctrl-font-size-md)",
            Size::Lg => "var(--ctrl-font-size-lg)",
        }
    }

    /// 获取输入框内边距
    pub fn input_padding(&self) -> &'static str {
        match self {
            Size::Sm => "4px 10px",
            Size::Md => "6px 12px",
            Size::Lg => "8px 16px",
        }
    }
}