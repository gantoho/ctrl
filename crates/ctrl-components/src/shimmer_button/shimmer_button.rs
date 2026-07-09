use dioxus::prelude::*;

/// ShimmerButton 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ShimmerButtonProps {
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 点击事件
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 按钮内容
    pub children: Element,
}

/// ShimmerButton 微光按钮组件
///
/// 一束高光周期性扫过按钮表面，适合强调主行动按钮、付费入口等。
#[allow(non_snake_case)]
pub fn ShimmerButton(props: ShimmerButtonProps) -> Element {
    const CSS: &str = include_str!("../../assets/shimmer_button.css");

    let btn_class = if props.class.is_empty() {
        "ctrl-shimmer-button".to_string()
    } else {
        format!("ctrl-shimmer-button {}", props.class)
    };

    let onclick = props.onclick;

    rsx! {
        style { {CSS} }
        button {
            class: "{btn_class}",
            style: "{props.style}",
            r#type: "button",
            disabled: props.disabled,
            onclick: move |evt| {
                if let Some(ref handler) = onclick {
                    handler.call(evt);
                }
            },
            span { class: "ctrl-shimmer-button__shine" }
            span { class: "ctrl-shimmer-button__label", {props.children} }
        }
    }
}
