use std::fmt;

/// 缓动函数枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Easing {
    #[default]
    EaseOutCubic,
    EaseOutQuad,
    EaseOutQuart,
    EaseOutQuint,
    EaseOutExpo,
    EaseOutBack,
    EaseOutElastic,
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl fmt::Display for Easing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Easing::EaseOutCubic => write!(f, "easeOutCubic"),
            Easing::EaseOutQuad => write!(f, "easeOutQuad"),
            Easing::EaseOutQuart => write!(f, "easeOutQuart"),
            Easing::EaseOutQuint => write!(f, "easeOutQuint"),
            Easing::EaseOutExpo => write!(f, "easeOutExpo"),
            Easing::EaseOutBack => write!(f, "easeOutBack"),
            Easing::EaseOutElastic => write!(f, "easeOutElastic"),
            Easing::Linear => write!(f, "linear"),
            Easing::EaseIn => write!(f, "ease-in"),
            Easing::EaseOut => write!(f, "ease-out"),
            Easing::EaseInOut => write!(f, "ease-in-out"),
        }
    }
}