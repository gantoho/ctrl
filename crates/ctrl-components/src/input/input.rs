use dioxus::prelude::*;
use ctrl_core::types::Size;

/// Input 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct InputProps {
    /// 输入框尺寸（默认 md）
    #[props(default = Size::default())]
    pub size: Size,

    /// 当前值（受控组件）
    #[props(default = "".to_string())]
    pub value: String,

    /// 占位文本
    #[props(default = "".to_string())]
    pub placeholder: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,

    /// 是否显示错误状态
    #[props(default = false)]
    pub error: bool,

    /// 原生 input type
    #[props(default = "text".to_string())]
    pub r#type: String,

    /// 前缀内容（渲染在输入框左侧）
    pub prefix: Option<Element>,

    /// 后缀内容（渲染在输入框右侧）
    pub suffix: Option<Element>,

    /// 是否显示清除按钮（value 非空时）
    #[props(default = false)]
    pub allow_clear: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 输入事件
    pub oninput: Option<EventHandler<String>>,

    /// 清除事件（点击清除按钮时触发）
    pub onclear: Option<EventHandler<()>>,

    /// 获得焦点事件
    pub onfocus: Option<EventHandler<FocusEvent>>,

    /// 失去焦点事件
    pub onblur: Option<EventHandler<FocusEvent>>,
}

/// 构建输入框 class 列表
fn build_input_class(size: Size, disabled: bool, error: bool, readonly: bool) -> String {
    let mut classes = vec!["ctrl-input".to_string()];

    match size {
        Size::Sm => classes.push("ctrl-input--sm".into()),
        Size::Md => classes.push("ctrl-input--md".into()),
        Size::Lg => classes.push("ctrl-input--lg".into()),
        _ => {}
    }

    if disabled {
        classes.push("ctrl-input--disabled".into());
    }
    if error {
        classes.push("ctrl-input--error".into());
    }
    if readonly {
        classes.push("ctrl-input--readonly".into());
    }

    classes.join(" ")
}

/// Input 输入框组件
#[allow(non_snake_case)]
pub fn Input(props: InputProps) -> Element {
    const CSS: &str = include_str!("../../assets/input.css");
    let input_class = build_input_class(
        props.size,
        props.disabled,
        props.error,
        props.readonly,
    );

    let user_class = if props.class.is_empty() {
        input_class
    } else {
        format!("{} {}", input_class, props.class)
    };

    let oninput = props.oninput.clone();
    let onfocus = props.onfocus.clone();
    let onblur = props.onblur.clone();

    let input_el = {
        rsx! {
            input {
                class: "{user_class}",
                style: if !props.style.is_empty() { props.style.as_str() } else { "" },
                r#type: "{props.r#type}",
                value: "{props.value}",
                placeholder: if props.placeholder.is_empty() { None } else { Some(props.placeholder.as_str()) },
                disabled: props.disabled,
                readonly: props.readonly,
                onfocusin: move |evt: FocusEvent| {
                    if let Some(ref handler) = onfocus {
                        handler.call(evt);
                    }
                },
                onfocusout: move |evt: FocusEvent| {
                    if let Some(ref handler) = onblur {
                        handler.call(evt);
                    }
                },
                oninput: move |evt: FormEvent| {
                    if let Some(ref handler) = oninput {
                        handler.call(evt.value());
                    }
                },
            }
        }
    };

    // 是否需要 wrapper（有 prefix / suffix / allow_clear 时启用）
    let has_prefix = props.prefix.is_some();
    let has_suffix = props.suffix.is_some();
    let has_clear = props.allow_clear && !props.value.is_empty();

    if has_prefix || has_suffix || has_clear {
        let clear_btn = if has_clear {
            let onclear = props.onclear.clone();
            let oninput_c = props.oninput.clone();
            rsx! {
                span {
                    class: "ctrl-input__suffix-btn",
                    title: "清除",
                    onclick: move |_| {
                        if let Some(ref handler) = onclear {
                            handler.call(());
                        }
                        if let Some(ref handler) = oninput_c {
                            handler.call(String::new());
                        }
                    },
                    // × 清除图标
                    "✕"
                }
            }
        } else {
            rsx! {}
        };

        rsx! {
            style { {CSS} }
            span {
                class: "ctrl-input__wrapper",
                style: if !props.style.is_empty() { props.style.as_str() } else { "" },
                if let Some(ref px) = props.prefix {
                    span { class: "ctrl-input__prefix", {px.clone()} }
                }
                {input_el}
                if has_clear {
                    {clear_btn}
                } else if let Some(ref sx) = props.suffix {
                    span { class: "ctrl-input__suffix", {sx.clone()} }
                }
            }
        }
    } else {
        rsx! {
            style { {CSS} }
            {input_el}
        }
    }
}
