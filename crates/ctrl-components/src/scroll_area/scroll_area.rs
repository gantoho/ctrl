use dioxus::prelude::*;

/// ScrollArea 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ScrollAreaProps {
    /// 固定高度（如 "240px"），为空则不限制
    #[props(default = "".to_string())]
    pub height: String,

    /// 最大高度（如 "320px"），为空则不限制
    #[props(default = "".to_string())]
    pub max_height: String,

    /// 是否横向滚动（默认纵向）
    #[props(default = false)]
    pub horizontal: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素
    pub children: Element,
}

/// ScrollArea 滚动区域组件（带主题化滚动条）
#[allow(non_snake_case)]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    const CSS: &str = include_str!("../../assets/scroll_area.css");
    let mut classes = vec!["ctrl-scroll-area".to_string()];
    if props.horizontal {
        classes.push("ctrl-scroll-area--horizontal".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let scroll_class = classes.join(" ");

    let mut style = String::new();
    if !props.height.is_empty() {
        style.push_str(&format!("height: {}; ", props.height));
    }
    if !props.max_height.is_empty() {
        style.push_str(&format!("max-height: {}; ", props.max_height));
    }
    style.push_str(&props.style);

    rsx! {
        style { {CSS} }
        div {
            class: "{scroll_class}",
            style: "{style}",
            {props.children}
        }
    }
}
