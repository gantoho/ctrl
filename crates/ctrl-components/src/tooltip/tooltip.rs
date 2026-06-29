use dioxus::prelude::*;
use ctrl_core::types::Placement;

/// Tooltip 气泡提示组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
    /// 提示文字内容
    #[props(default = "".to_string())]
    pub content: String,

    /// 弹出位置
    #[props(default = Placement::Top)]
    pub placement: Placement,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（触发元素）
    pub children: Element,
}

/// Tooltip 气泡提示组件（纯 CSS 实现，无需 JavaScript 定位）
#[allow(non_snake_case)]
pub fn Tooltip(props: TooltipProps) -> Element {
    const CSS: &str = include_str!("../../assets/tooltip.css");
    let mut visible = use_signal(|| false);

    let placement_class = format!("ctrl-tooltip__popper--{}", props.placement);
    let wrapper_class = if props.class.is_empty() {
        "ctrl-tooltip".to_string()
    } else {
        format!("ctrl-tooltip {}", props.class)
    };

    rsx! {
        style { {CSS} }
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
