use dioxus::prelude::*;
use ctrl_core::types::Size;

/// 构建输入框样式字符串
fn build_input_style(
    size: Size,
    focused: bool,
    disabled: bool,
    has_error: bool,
    custom_style: &str,
) -> String {
    let mut styles: Vec<String> = vec![
        "display: inline-flex".into(),
        "width: 100%".into(),
        "font-family: var(--ctrl-font-family)".into(),
        format!("font-size: {}", size.font_size_var()),
        "border-radius: var(--ctrl-radius-md)".into(),
        "transition: all var(--ctrl-transition)".into(),
        "outline: none".into(),
        "line-height: 1.5".into(),
        "box-sizing: border-box".into(),
        "background: var(--ctrl-bg)".into(),
        "color: var(--ctrl-text)".into(),
        format!("padding: {}", size.input_padding()),
        format!("height: {}", size.height()),
    ];

    // 边框颜色：error > focus > default
    if has_error {
        styles.push("border: 1px solid var(--ctrl-danger)".into());
        styles.push("box-shadow: 0 0 0 1px var(--ctrl-danger)".into());
    } else if focused {
        styles.push("border: 1px solid var(--ctrl-primary)".into());
        styles.push("box-shadow: 0 0 0 1px var(--ctrl-primary)".into());
    } else {
        styles.push("border: 1px solid var(--ctrl-border)".into());
    }

    if disabled {
        styles.push("opacity: 0.5".into());
        styles.push("cursor: not-allowed".into());
        styles.push("background: var(--ctrl-bg-secondary)".into());
    }

    styles.push("&::placeholder { color: var(--ctrl-text-disabled); }".into());

    if !custom_style.is_empty() {
        styles.push(custom_style.to_string());
    }

    styles.join("; ")
}

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

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 输入事件
    pub oninput: Option<EventHandler<FormEvent>>,

    /// 获得焦点事件
    pub onfocus: Option<EventHandler<FocusEvent>>,

    /// 失去焦点事件
    pub onblur: Option<EventHandler<FocusEvent>>,
}

/// Input 输入框组件
///
/// # 示例
///
/// ```rust
/// let mut value = use_signal(|| String::new());
///
/// rsx! {
///     Input {
///         value: value(),
///         placeholder: "请输入内容",
///         oninput: move |evt| value.set(evt.value()),
///     }
///     Input {
///         value: "".to_string(),
///         placeholder: "错误状态",
///         error: true,
///     }
/// }
/// ```
#[allow(non_snake_case)]
pub fn Input(props: InputProps) -> Element {
    let mut focused = use_signal(|| false);

    let style_str = build_input_style(
        props.size,
        focused(),
        props.disabled,
        props.error,
        &props.style,
    );

    let oninput = props.oninput.clone();
    let onfocus = props.onfocus.clone();
    let onblur = props.onblur.clone();

    rsx! {
        input {
            class: if props.class.is_empty() { None } else { Some(props.class.as_str()) },
            style: "{style_str}",
            r#type: "{props.r#type}",
            value: "{props.value}",
            placeholder: if props.placeholder.is_empty() { None } else { Some(props.placeholder.as_str()) },
            disabled: props.disabled,
            readonly: props.readonly,
            onfocusin: move |evt| {
                focused.set(true);
                if let Some(ref handler) = onfocus {
                    handler.call(evt);
                }
            },
            onfocusout: move |evt| {
                focused.set(false);
                if let Some(ref handler) = onblur {
                    handler.call(evt);
                }
            },
            oninput: move |evt| {
                if let Some(ref handler) = oninput {
                    handler.call(evt);
                }
            },
        }
    }
}