use dioxus::prelude::*;

/// AspectRatio 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct AspectRatioProps {
    /// 宽高比，直接作为 CSS `aspect-ratio` 值（如 "16 / 9"、"1 / 1"、"4 / 3"）
    #[props(default = "16 / 9".to_string())]
    pub ratio: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素
    pub children: Element,
}

/// AspectRatio 宽高比容器组件
#[allow(non_snake_case)]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    const CSS: &str = include_str!("../../assets/aspect_ratio.css");
    let aspect_class = if props.class.is_empty() {
        "ctrl-aspect-ratio".to_string()
    } else {
        format!("ctrl-aspect-ratio {}", props.class)
    };
    let aspect_style = format!("aspect-ratio: {ratio}; {extra}", ratio = props.ratio, extra = props.style);

    rsx! {
        style { {CSS} }
        div {
            class: "{aspect_class}",
            style: "{aspect_style}",
            {props.children}
        }
    }
}
