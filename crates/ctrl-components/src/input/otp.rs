use dioxus::prelude::*;
use ctrl_core::types::Size;

/// InputOtp 一次性验证码输入组件属性
#[derive(Props, PartialEq, Clone)]
pub struct InputOtpProps {
    /// 验证码长度（位数，默认 6 位）
    #[props(default = 6)]
    pub length: usize,

    /// 当前值（字符串，长度等于 length 时完成）
    #[props(default = "".to_string())]
    pub value: String,

    /// 尺寸（默认 md）
    #[props(default = Size::Md)]
    pub size: Size,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否显示错误状态
    #[props(default = false)]
    pub error: bool,

    /// 是否使用密码模式（显示为圆点）
    #[props(default = false)]
    pub mask: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 值变化事件（每次输入变化时触发）
    pub onchange: Option<EventHandler<String>>,

    /// 完成事件（全部输入完成后触发）
    pub oncomplete: Option<EventHandler<String>>,
}

/// InputOtp 验证码输入组件
///
/// 渲染为多个独立的单字符输入框，适合短信验证码、PIN 码等场景。
/// 支持键盘操作（Backspace 回退）。
///
/// # 示例
///
/// ```rust
/// let mut code = use_signal(|| String::new());
/// InputOtp {
///     value: code(),
///     onchange: move |v| code.set(v),
/// }
/// ```
#[allow(non_snake_case)]
pub fn InputOtp(props: InputOtpProps) -> Element {
    let otp_class = {
        let mut c = format!("ctrl-otp ctrl-otp--{}", props.size);
        if props.disabled {
            c.push_str(" ctrl-otp--disabled");
        }
        if props.error {
            c.push_str(" ctrl-otp--error");
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    let chars: Vec<char> = props.value.chars().collect();

    // 渲染各输入格
    let cells: Vec<Element> = (0..props.length)
        .map(|idx| {
            let ch = chars.get(idx).copied();
            let cell_class = {
                let mut c = String::from("ctrl-otp__cell");
                if props.error {
                    c.push_str(" ctrl-otp__cell--error");
                }
                if ch.is_some() {
                    c.push_str(" ctrl-otp__cell--filled");
                }
                c
            };

            let display_char = if props.mask && ch.is_some() {
                "●".to_string()
            } else {
                ch.map(|c| c.to_string()).unwrap_or_default()
            };

            rsx! {
                input {
                    key: "{idx}",
                    class: "{cell_class}",
                    r#type: "text",
                    inputmode: "numeric",
                    maxlength: "1",
                    value: "{display_char}",
                    disabled: props.disabled,
                    readonly: true,
                    autocomplete: "off",
                    onkeydown: {
                        let onchange = props.onchange.clone();
                        let oncomplete = props.oncomplete.clone();
                        let value = props.value.clone();
                        let length = props.length;
                        move |evt: KeyboardEvent| {
                            match evt.key() {
                                Key::Character(ch) => {
                                    let first_ch = ch.chars().next();
                                    if first_ch.map_or(false, |c| c.is_alphanumeric()) {
                                        if let Some(ref handler) = onchange {
                                            let mut new_val = value.clone();
                                            if new_val.len() < length {
                                                new_val.push(first_ch.unwrap());
                                                handler.call(new_val.clone());
                                                if new_val.len() == length {
                                                    if let Some(ref cb) = oncomplete {
                                                        cb.call(new_val);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Key::Backspace | Key::Delete => {
                                    if let Some(ref handler) = onchange {
                                        let mut new_val = value.clone();
                                        new_val.pop();
                                        handler.call(new_val);
                                    }
                                }
                                _ => {}
                            }
                        }
                    },
                }
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        style { {".ctrl-otp{display:inline-flex;gap:8px;align-items:center;}"}
            {".ctrl-otp__cell{width:48px;height:48px;border:1px solid var(--ctrl-border,#d9d9d9);border-radius:var(--ctrl-radius-sm,4px);text-align:center;font-size:var(--ctrl-font-size-lg,16px);font-weight:600;color:var(--ctrl-text,#333);outline:none;box-sizing:border-box;transition:border-color var(--ctrl-transition,.2s),box-shadow var(--ctrl-transition,.2s);cursor:default;caret-color:transparent;}"}
            {".ctrl-otp__cell:focus{border-color:var(--ctrl-primary,#4a90d9);box-shadow:0 0 0 1px var(--ctrl-primary,#4a90d9);}"}
            {".ctrl-otp__cell--filled{border-color:var(--ctrl-primary,#4a90d9);}"}
            {".ctrl-otp__cell--error{border-color:var(--ctrl-danger,#e74c3c);}"}
            {".ctrl-otp--disabled .ctrl-otp__cell{background:var(--ctrl-bg-disabled,#f5f5f5);color:var(--ctrl-text-disabled,#bfbfbf);cursor:not-allowed;}"}
            {".ctrl-otp--sm .ctrl-otp__cell{width:40px;height:40px;font-size:var(--ctrl-font-size-md,14px);}"}
            {".ctrl-otp--lg .ctrl-otp__cell{width:56px;height:56px;font-size:20px;}"}
        }
        div {
            class: "{otp_class}",
            style: "{props.style}",
            for cell in cells.into_iter() {
                {cell}
            }
        }
    }
}
