use dioxus::prelude::*;
use ctrl_core::types::{Size, Shape};

/// 构建 Avatar class 列表
fn build_avatar_class(size: Size, shape: Shape, class: &str) -> String {
    let mut classes = vec!["ctrl-avatar".to_string()];

    match size {
        Size::Sm => classes.push("ctrl-avatar--sm".into()),
        Size::Md => classes.push("ctrl-avatar--md".into()),
        Size::Lg => classes.push("ctrl-avatar--lg".into()),
        _ => {}
    }

    if shape == Shape::Square {
        classes.push("ctrl-avatar--square".into());
    }

    if !class.is_empty() {
        classes.push(class.to_string());
    }

    classes.join(" ")
}

/// Avatar 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct AvatarProps {
    /// 图片地址
    #[props(default = "".to_string())]
    pub src: String,

    /// 替代文字
    #[props(default = "".to_string())]
    pub alt: String,

    /// 头像尺寸（默认 md）
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

    /// 子元素（当无 src 时作为 fallback 内容）
    pub children: Element,
}

/// Avatar 头像组件
#[allow(non_snake_case)]
pub fn Avatar(props: AvatarProps) -> Element {
    const CSS: &str = include_str!("../../assets/avatar.css");
    let avatar_class = build_avatar_class(props.size, props.shape, &props.class);
    let style = if props.style.is_empty() {
        String::new()
    } else {
        props.style.clone()
    };

    if !props.src.is_empty() {
        rsx! {
            div {
                class: "{avatar_class}",
                style: "{style}",
                img {
                    class: "ctrl-avatar__img",
                    src: "{props.src}",
                    alt: "{props.alt}",
                }
            }
        }
    } else {
        rsx! {
            style { {CSS} }
            div {
                class: "{avatar_class}",
                style: "{style}",
                span {
                    class: "ctrl-avatar__text",
                    {props.children}
                }
            }
        }
    }
}
