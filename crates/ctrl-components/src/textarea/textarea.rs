use dioxus::prelude::*;
use ctrl_core::types::Size;

/// Textarea 多行输入组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TextareaProps {
    /// 输入框尺寸（默认 md）
    #[props(default = Size::default())]
    pub size: Size,

    /// 当前值（受控组件）
    #[props(default = "".to_string())]
    pub value: String,

    /// 占位文本
    #[props(default = "".to_string())]
    pub placeholder: String,

    /// 可见行数（控制高度）
    #[props(default = 3)]
    pub rows: u32,

    /// 最大字符数（0 表示不限制）
    #[props(default = 0)]
    pub maxlength: usize,

    /// 是否显示字数统计
    #[props(default = false)]
    pub show_count: bool,

    /// 是否根据内容自动调整高度
    #[props(default = false)]
    pub auto_height: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,

    /// 是否显示错误状态
    #[props(default = false)]
    pub error: bool,

    /// resize 行为：none / both / horizontal / vertical（默认 vertical）
    #[props(default = "vertical".to_string())]
    pub resize: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 值变化事件（传递新值 String）
    pub onchange: Option<EventHandler<String>>,

    /// 获得焦点事件
    pub onfocus: Option<EventHandler<FocusEvent>>,

    /// 失去焦点事件
    pub onblur: Option<EventHandler<FocusEvent>>,
}

/// 构建 Textarea class 列表
fn build_textarea_class(
    size: Size,
    disabled: bool,
    error: bool,
    readonly: bool,
    resize: &str,
) -> String {
    let mut classes = vec!["ctrl-textarea".to_string()];

    match size {
        Size::Sm => classes.push("ctrl-textarea--sm".into()),
        Size::Md => classes.push("ctrl-textarea--md".into()),
        Size::Lg => classes.push("ctrl-textarea--lg".into()),
        _ => {}
    }

    if disabled {
        classes.push("ctrl-textarea--disabled".into());
    }
    if error {
        classes.push("ctrl-textarea--error".into());
    }
    if readonly {
        classes.push("ctrl-textarea--readonly".into());
    }

    match resize {
        "none" => classes.push("ctrl-textarea--resize-none".into()),
        "both" => classes.push("ctrl-textarea--resize-both".into()),
        "horizontal" => classes.push("ctrl-textarea--resize-horizontal".into()),
        _ => classes.push("ctrl-textarea--resize-vertical".into()),
    }

    classes.join(" ")
}

/// Textarea 多行输入组件
///
/// ### 使用示例
/// ```rust
/// let mut value = use_signal(|| String::new());
///
/// Textarea {
///     placeholder: "请输入内容",
///     rows: 4,
///     maxlength: 200,
///     show_count: true,
///     value: value(),
///     onchange: move |v: String| value.set(v),
/// }
/// ```
#[allow(non_snake_case)]
pub fn Textarea(props: TextareaProps) -> Element {
    const CSS: &str = include_str!("../../assets/textarea.css");
    let textarea_class = build_textarea_class(
        props.size,
        props.disabled,
        props.error,
        props.readonly,
        &props.resize,
    );

    let user_class = if props.class.is_empty() {
        textarea_class
    } else {
        format!("{} {}", textarea_class, props.class)
    };

    let onchange = props.onchange.clone();
    let onfocus = props.onfocus.clone();
    let onblur = props.onblur.clone();

    // 字数统计
    let count_ui = if props.show_count && props.maxlength > 0 {
        let len = props.value.chars().count();
        let over = len > props.maxlength;
        let count_class = if over {
            "ctrl-textarea__count ctrl-textarea__count--over"
        } else {
            "ctrl-textarea__count"
        };
        rsx! {
            div { class: "{count_class}", "{len} / {props.maxlength}" }
        }
    } else {
        rsx! {}
    };

    let placeholder_str = if props.placeholder.is_empty() {
        None
    } else {
        Some(props.placeholder.clone())
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{user_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            textarea {
                class: "ctrl-textarea__inner",
                rows: "{props.rows}",
                value: "{props.value}",
                placeholder: placeholder_str,
                disabled: props.disabled,
                readonly: props.readonly,
                onfocusin: move |evt| {
                    if let Some(ref handler) = onfocus {
                        handler.call(evt);
                    }
                },
                onfocusout: move |evt| {
                    if let Some(ref handler) = onblur {
                        handler.call(evt);
                    }
                },
                oninput: move |evt: FormEvent| {
                    if let Some(ref handler) = onchange {
                        handler.call(evt.value());
                    }
                },
            }
            {count_ui}
        }
    }
}
