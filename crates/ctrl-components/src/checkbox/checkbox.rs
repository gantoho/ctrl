use dioxus::prelude::*;

/// Checkbox 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct CheckboxProps {
    /// 是否选中
    #[props(default = false)]
    pub checked: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 半选状态
    #[props(default = false)]
    pub indeterminate: bool,

    /// 标签文本
    #[props(default = "".to_string())]
    pub label: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 状态变化事件
    pub onchange: Option<EventHandler<bool>>,
}

/// Checkbox 复选框组件
#[allow(non_snake_case)]
pub fn Checkbox(props: CheckboxProps) -> Element {
    const CSS: &str = include_str!("../../assets/checkbox.css");
    let mut label_classes = vec!["ctrl-checkbox".to_string()];
    let mut box_classes = vec!["ctrl-checkbox__box".to_string()];

    if props.checked || props.indeterminate {
        box_classes.push("ctrl-checkbox__box--checked".into());
    }
    if props.disabled {
        label_classes.push("ctrl-checkbox--disabled".into());
        box_classes.push("ctrl-checkbox__box--disabled".into());
    }

    if !props.class.is_empty() {
        label_classes.push(props.class.clone());
    }

    let label_class = label_classes.join(" ");
    let box_class = box_classes.join(" ");

    let onchange = props.onchange.clone();
    let checked = props.checked;
    let disabled = props.disabled;

    let icon_color = if props.disabled { "var(--ctrl-primary)" } else { "var(--ctrl-text-on-primary)" };

    let check_icon = if props.indeterminate {
        rsx! {
            svg {
                view_box: "0 0 16 16",
                width: "12",
                height: "12",
                fill: "{icon_color}",
                rect { x: "3", y: "7", width: "10", height: "2", rx: "1" }
            }
        }
    } else if checked {
        rsx! {
            svg {
                view_box: "0 0 16 16",
                width: "12",
                height: "12",
                fill: "none",
                stroke: "{icon_color}",
                stroke_width: "2",
                path { d: "M3 8l3.5 3.5L13 5" }
            }
        }
    } else {
        rsx! { }
    };

    rsx! {
        style { {CSS} }
        label {
            class: "{label_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            div {
                class: "{box_class}",
                onclick: move |evt| {
                    evt.stop_propagation();
                    if !disabled {
                        if let Some(ref handler) = onchange {
                            handler.call(!checked);
                        }
                    }
                },
                {check_icon}
            }
            if !props.label.is_empty() {
                span { "{props.label}" }
            }
        }
    }
}
