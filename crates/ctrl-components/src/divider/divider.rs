use dioxus::prelude::*;
use ctrl_core::types::Direction;

/// Divider 分割线组件属性
#[derive(Props, PartialEq, Clone)]
pub struct DividerProps {
    /// 分割线方向
    #[props(default = Direction::Horizontal)]
    pub direction: Direction,

    /// 分割线中间的文字（空字符串则为纯分割线）
    #[props(default = "".to_string())]
    pub content: String,

    /// 是否为虚线
    #[props(default = false)]
    pub dashed: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// Divider 分割线组件
#[allow(non_snake_case)]
pub fn Divider(props: DividerProps) -> Element {
    const CSS: &str = include_str!("../../assets/divider.css");
    let has_text = !props.content.is_empty();
    let is_vertical = props.direction == Direction::Vertical;

    let mut classes = vec!["ctrl-divider".to_string()];

    if has_text {
        classes.push("ctrl-divider--with-text".into());
    } else if !is_vertical {
        classes.push("ctrl-divider--plain".into());
    }

    if is_vertical {
        classes.push("ctrl-divider--vertical".into());
    }

    if props.dashed {
        classes.push("ctrl-divider--dashed".into());
    }

    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }

    let class_str = classes.join(" ");

    rsx! {
        style { {CSS} }
        div {
            class: "{class_str}",
            "{props.content}"
        }
    }
}
