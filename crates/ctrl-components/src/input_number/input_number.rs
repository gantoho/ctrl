use dioxus::prelude::*;
use ctrl_core::types::Size;

/// InputNumber 数字输入框组件属性
#[derive(Props, PartialEq, Clone)]
pub struct InputNumberProps {
    /// 当前值
    #[props(default = 0)]
    pub value: i64,

    /// 最小值
    pub min: Option<i64>,

    /// 最大值
    pub max: Option<i64>,

    /// 步长
    #[props(default = 1)]
    pub step: i64,

    /// 尺寸
    #[props(default = Size::Md)]
    pub size: Size,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 占位文字
    #[props(default = "".to_string())]
    pub placeholder: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 值变化回调
    pub onchange: Option<EventHandler<i64>>,
}

/// InputNumber 数字输入框组件
///
/// 通过点击 +/- 按钮或直接输入来调整数值。
///
/// # 示例
///
/// ```rust
/// let mut value = use_signal(|| 0);
/// InputNumber {
///     value: value(),
///     min: Some(0),
///     max: Some(100),
///     onchange: move |v| value.set(v),
/// }
/// ```
#[allow(non_snake_case)]
pub fn InputNumber(props: InputNumberProps) -> Element {
    const CSS: &str = include_str!("../../assets/input-number.css");
    let mut value = use_signal(|| props.value);
    let mut input_value = use_signal(|| props.value.to_string());

    let wrapper_class = {
        let mut c = format!("ctrl-input-number ctrl-input-number--{}", props.size);
        if props.disabled {
            c.push_str(" ctrl-input-number--disabled");
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    let clamp = move |v: i64| -> i64 {
        let mut v = v;
        if let Some(min) = props.min {
            v = v.max(min);
        }
        if let Some(max) = props.max {
            v = v.min(max);
        }
        v
    };

    let commit_value = {
        let onchange = props.onchange.clone();
        move |new_val: i64| {
            let clamped = clamp(new_val);
            value.set(clamped);
            input_value.set(clamped.to_string());
            if let Some(ref cb) = onchange {
                cb.call(clamped);
            }
        }
    };

    let on_decrease = {
        let mut commit = commit_value.clone();
        move |_| {
            if !props.disabled {
                commit(value() - props.step);
            }
        }
    };

    let on_increase = {
        let mut commit = commit_value.clone();
        move |_| {
            if !props.disabled {
                commit(value() + props.step);
            }
        }
    };

    let on_input = {
        move |evt: FormEvent| {
            let raw = evt.value();
            // 允许空字符串和负号前缀
            if raw.is_empty() || raw == "-" {
                input_value.set(raw);
                return;
            }
            // 解析数字
            if let Ok(parsed) = raw.parse::<i64>() {
                let clamped = clamp(parsed);
                value.set(clamped);
                input_value.set(raw.clone());
                if let Some(ref cb) = props.onchange {
                    cb.call(clamped);
                }
            }
            // 非法输入则忽略，保持旧值
        }
    };

    let on_blur = move |_| {
        // 失焦时修正显示值
        input_value.set(value().to_string());
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            style: "{props.style}",
            button {
                class: "ctrl-input-number__btn",
                disabled: props.disabled,
                onclick: on_decrease,
                "−"
            }
            input {
                class: "ctrl-input-number__input",
                r#type: "text",
                inputmode: "numeric",
                value: "{input_value}",
                disabled: props.disabled,
                placeholder: "{props.placeholder}",
                oninput: on_input,
                onblur: on_blur,
            }
            button {
                class: "ctrl-input-number__btn",
                disabled: props.disabled,
                onclick: on_increase,
                "+"
            }
        }
    }
}