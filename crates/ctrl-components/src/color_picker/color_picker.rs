use dioxus::prelude::*;

/// 预设色板
const PRESET_COLORS: &[&str] = &[
    "#EF4444", "#F97316", "#F59E0B", "#EAB308", "#84CC16",
    "#10B981", "#14B8A6", "#06B6D4", "#3B82F6", "#6366F1",
    "#8B5CF6", "#A855F7", "#D946EF", "#EC4899", "#F43F5E",
    "#787B7D", "#374151", "#111827", "#FFFFFF", "#000000",
];

#[derive(Props, PartialEq, Clone)]
pub struct ColorPickerProps {
    /// 当前颜色值
    #[props(default = "#3FC99E".to_string())]
    pub value: String,

    /// 颜色变化回调
    pub onchange: Option<EventHandler<String>>,

    /// 是否显示 hex 输入框
    #[props(default = true)]
    pub show_text: bool,

    /// 占位文字
    #[props(default = "选择颜色".to_string())]
    pub placeholder: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

#[allow(non_snake_case)]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    const CSS: &str = include_str!("../../assets/color_picker.css");
    let mut value = use_signal(|| props.value.clone());
    let onchange = props.onchange.clone();

    let class = if props.class.is_empty() {
        "ctrl-color-picker".to_string()
    } else {
        format!("ctrl-color-picker {}", props.class)
    };

    let trigger_style = format!(
        "background: {v}; --ctrl-color-picker-value: {v}; {s}",
        v = value,
        s = props.style,
    );

    rsx! {
        style { {CSS} }
        div {
            class: "{class}",
            div { class: "ctrl-color-picker__trigger", style: "{trigger_style}",
                input {
                    r#type: "color",
                    value: "{value}",
                    disabled: props.disabled,
                    oninput: move |evt: FormEvent| {
                        let v = evt.value().to_string();
                        value.set(v.clone());
                        if let Some(ref handler) = onchange {
                            handler.call(v);
                        }
                    },
                }
            }
            if props.show_text {
                input {
                    class: "ctrl-color-picker__input",
                    r#type: "text",
                    value: "{value}",
                    disabled: props.disabled,
                    placeholder: "{props.placeholder}",
                    oninput: move |evt: FormEvent| {
                        let v = evt.value().to_string();
                        value.set(v.clone());
                        if let Some(ref handler) = onchange {
                            handler.call(v);
                        }
                    },
                }
            }

            div { class: "ctrl-color-picker__presets",
                for color in PRESET_COLORS {
                    div {
                        key: "{color}",
                        class: "ctrl-color-picker__preset",
                        style: "background: {color};",
                        onclick: move |_| {
                            value.set(color.to_string());
                            if let Some(ref handler) = onchange {
                                handler.call(color.to_string());
                            }
                        },
                    }
                }
            }
        }
    }
}
