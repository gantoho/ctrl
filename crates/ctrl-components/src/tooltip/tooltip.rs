use dioxus::prelude::*;

/// Tooltip 气泡提示组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
    /// 提示文字内容
    #[props(default = "".to_string())]
    pub content: String,

    /// 弹出位置："top" / "bottom" / "left" / "right"
    #[props(default = "top".to_string())]
    pub placement: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（触发元素）
    pub children: Element,
}

/// Tooltip 气泡提示组件（纯 CSS 实现，无需 JavaScript 定位）
#[allow(non_snake_case)]
pub fn Tooltip(props: TooltipProps) -> Element {
    let mut visible = use_signal(|| false);

    let placement_class = format!("ctrl-tooltip__popper--{}", props.placement);
    let wrapper_class = if props.class.is_empty() {
        "ctrl-tooltip".to_string()
    } else {
        format!("ctrl-tooltip {}", props.class)
    };

    rsx! {
        div {
            class: "{wrapper_class}",
            onmouseenter: move |_| visible.set(true),
            onmouseleave: move |_| visible.set(false),
            {props.children}
            if visible() {
                div {
                    class: "ctrl-tooltip__popper {placement_class}",
                    "{props.content}"
                }
            }
        }
    }
}
