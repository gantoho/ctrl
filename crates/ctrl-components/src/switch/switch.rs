use dioxus::prelude::*;
use ctrl_core::types::Size;

/// 构建开关 class 列表
fn build_switch_class(size: Size, checked: bool, disabled: bool) -> String {
    let mut classes = vec!["ctrl-switch".to_string()];

    match size {
        Size::Sm => classes.push("ctrl-switch--sm".into()),
        Size::Md => classes.push("ctrl-switch--md".into()),
        Size::Lg => classes.push("ctrl-switch--lg".into()),
        _ => {}
    }

    if checked {
        classes.push("ctrl-switch--checked".into());
    } else {
        classes.push("ctrl-switch--unchecked".into());
    }

    if disabled {
        classes.push("ctrl-switch--disabled".into());
    }

    classes.join(" ")
}

/// Switch 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SwitchProps {
    /// 是否选中
    #[props(default = false)]
    pub checked: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 尺寸
    #[props(default = Size::default())]
    pub size: Size,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 状态变化事件
    pub onchange: Option<EventHandler<bool>>,
}

/// Switch 开关组件
#[allow(non_snake_case)]
pub fn Switch(props: SwitchProps) -> Element {
    const CSS: &str = include_str!("../../assets/switch.css");
    let switch_class = build_switch_class(props.size, props.checked, props.disabled);

    let user_class = if props.class.is_empty() {
        switch_class
    } else {
        format!("{} {}", switch_class, props.class)
    };

    let onchange = props.onchange.clone();
    let checked = props.checked;
    let disabled = props.disabled;

    rsx! {
        style { {CSS} }
        div {
            class: "{user_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            role: "switch",
            aria_checked: if checked { "true" } else { "false" },
            onclick: move |_| {
                if !disabled {
                    if let Some(ref handler) = onchange {
                        handler.call(!checked);
                    }
                }
            },
            span { class: "ctrl-switch__thumb" }
        }
    }
}
