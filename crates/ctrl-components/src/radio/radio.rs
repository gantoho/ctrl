use dioxus::prelude::*;

/// Radio 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct RadioProps {
    /// 是否选中
    #[props(default = false)]
    pub checked: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 单选值
    #[props(default = "".to_string())]
    pub value: String,

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
    pub onchange: Option<EventHandler<String>>,
}

/// Radio 单选框组件
#[allow(non_snake_case)]
pub fn Radio(props: RadioProps) -> Element {
    let mut label_classes = vec!["ctrl-radio".to_string()];
    let mut circle_classes = vec!["ctrl-radio__circle".to_string()];

    if props.checked {
        circle_classes.push("ctrl-radio__circle--checked".into());
    }
    if props.disabled {
        label_classes.push("ctrl-radio--disabled".into());
        circle_classes.push("ctrl-radio__circle--disabled".into());
    }

    if !props.class.is_empty() {
        label_classes.push(props.class.clone());
    }

    let label_class = label_classes.join(" ");
    let circle_class = circle_classes.join(" ");

    let onchange = props.onchange.clone();
    let value = props.value.clone();

    rsx! {
        label {
            class: "{label_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            div {
                class: "{circle_class}",
                onclick: move |evt| {
                    evt.stop_propagation();
                    if !props.disabled {
                        if let Some(ref handler) = onchange {
                            handler.call(value.clone());
                        }
                    }
                },
            }
            if !props.label.is_empty() {
                span { "{props.label}" }
            }
        }
    }
}
