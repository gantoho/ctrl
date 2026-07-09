use dioxus::prelude::*;
use ctrl_core::types::{Shape, Size};
use crate::avatar::Avatar;

/// 头像组子项
#[derive(Clone, PartialEq, Default)]
pub struct AvatarGroupItem {
    /// 图片地址
    pub src: String,
    /// 替代文字
    pub alt: String,
    /// 无图片时的 fallback 文本（如姓名首字母）
    pub fallback: String,
    /// fallback 背景色（可选）
    pub color: String,
}

impl AvatarGroupItem {
    /// 图片头像
    pub fn image(src: impl Into<String>, alt: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            alt: alt.into(),
            ..Default::default()
        }
    }

    /// 文本头像
    pub fn text(fallback: impl Into<String>) -> Self {
        Self {
            fallback: fallback.into(),
            ..Default::default()
        }
    }

    /// 设置 fallback 背景色
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }
}

/// AvatarGroup 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct AvatarGroupProps {
    /// 头像列表
    pub items: Vec<AvatarGroupItem>,

    /// 最多显示数量，超出部分折叠为 "+N"（默认全部显示）
    #[props(default = None)]
    pub max: Option<usize>,

    /// 头像尺寸
    #[props(default = Size::default())]
    pub size: Size,

    /// 头像形状
    #[props(default = Shape::Circle)]
    pub shape: Shape,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// AvatarGroup 头像组组件
///
/// 将多个头像堆叠展示，超出 `max` 的部分折叠为 "+N"。复用 [`Avatar`] 渲染每一项。
#[allow(non_snake_case)]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element {
    const CSS: &str = include_str!("../../assets/avatar_group.css");

    let total = props.items.len();
    let shown = props.max.map(|m| m.min(total)).unwrap_or(total);
    let surplus = total - shown;

    let mut classes = vec!["ctrl-avatar-group".to_string()];
    match props.size {
        Size::Sm => classes.push("ctrl-avatar-group--sm".into()),
        Size::Lg => classes.push("ctrl-avatar-group--lg".into()),
        _ => classes.push("ctrl-avatar-group--md".into()),
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let group_class = classes.join(" ");

    rsx! {
        style { {CSS} }
        div { class: "{group_class}", style: "{props.style}",
            for (i, item) in props.items.iter().take(shown).enumerate() {
                {
                    let item_style = if item.src.is_empty() && !item.color.is_empty() {
                        format!("background: {}; color: #fff;", item.color)
                    } else {
                        String::new()
                    };
                    rsx! {
                        Avatar {
                            key: "{i}",
                            size: props.size,
                            shape: props.shape,
                            src: item.src.clone(),
                            alt: item.alt.clone(),
                            style: item_style,
                            "{item.fallback}"
                        }
                    }
                }
            }
            if surplus > 0 {
                Avatar {
                    size: props.size,
                    shape: props.shape,
                    class: "ctrl-avatar-group__more".to_string(),
                    "+{surplus}"
                }
            }
        }
    }
}
