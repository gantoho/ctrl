use dioxus::prelude::*;
use ctrl_core::types::{Size, Variant};

/// 构建按钮样式字符串
fn build_button_style(
    variant: Variant,
    size: Size,
    hovered: bool,
    disabled: bool,
    block: bool,
    custom_style: &str,
) -> String {
    let mut styles: Vec<String> = vec![
        "display: inline-flex".into(),
        "align-items: center".into(),
        "justify-content: center".into(),
        "font-family: var(--ctrl-font-family)".into(),
        format!("font-size: {}", size.font_size_var()),
        "border-radius: var(--ctrl-radius-md)".into(),
        "transition: all var(--ctrl-transition)".into(),
        "border: 1px solid transparent".into(),
        "outline: none".into(),
        "line-height: 1.5".into(),
        "gap: 6px".into(),
        "text-decoration: none".into(),
        "user-select: none".into(),
        "font-weight: 500".into(),
        "white-space: nowrap".into(),
        format!("padding: {}", size.padding()),
        format!("height: {}", size.height()),
    ];

    match variant {
        Variant::Primary => {
            let bg = if hovered {
                "var(--ctrl-primary-hover)"
            } else {
                "var(--ctrl-primary)"
            };
            styles.push(format!("background: {bg}"));
            styles.push("color: white".into());
            styles.push(format!("border-color: {bg}"));
        }
        Variant::Secondary => {
            let bg = if hovered {
                "var(--ctrl-secondary-hover)"
            } else {
                "var(--ctrl-secondary)"
            };
            styles.push(format!("background: {bg}"));
            styles.push("color: white".into());
            styles.push(format!("border-color: {bg}"));
        }
        Variant::Outline => {
            let color = if hovered {
                "var(--ctrl-primary-hover)"
            } else {
                "var(--ctrl-primary)"
            };
            styles.push("background: transparent".into());
            styles.push(format!("color: {color}"));
            styles.push(format!("border-color: {color}"));
        }
        Variant::Ghost => {
            let color = if hovered {
                "var(--ctrl-primary-hover)"
            } else {
                "var(--ctrl-primary)"
            };
            styles.push("background: transparent".into());
            styles.push(format!("color: {color}"));
            styles.push("border-color: transparent".into());
            if hovered {
                styles.push("background: var(--ctrl-primary-light)".into());
            }
        }
    }

    if disabled {
        styles.push("opacity: 0.5".into());
        styles.push("cursor: not-allowed".into());
        styles.push("pointer-events: none".into());
    } else {
        styles.push("cursor: pointer".into());
    }

    if block {
        styles.push("width: 100%".into());
    }

    if !custom_style.is_empty() {
        styles.push(custom_style.to_string());
    }

    styles.join("; ")
}

/// Button 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    /// 按钮变体（默认 primary）
    #[props(default = Variant::default())]
    pub variant: Variant,

    /// 按钮尺寸（默认 md）
    #[props(default = Size::default())]
    pub size: Size,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否加载中
    #[props(default = false)]
    pub loading: bool,

    /// 是否块级（撑满容器宽度）
    #[props(default = false)]
    pub block: bool,

    /// 自定义类名 —— 用户可自由添加 CSS 类
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式 —— 会合并到组件默认样式之后，用于覆盖
    #[props(default = "".to_string())]
    pub style: String,

    /// 按钮原生 type 属性
    #[props(default = "button".to_string())]
    pub r#type: String,

    /// 点击事件
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// 子元素
    pub children: Element,
}

/// Button 按钮组件
///
/// # 示例
///
/// ```rust
/// rsx! {
///     Button { variant: Variant::Primary, size: Size::Md, "点击我" }
///     Button { variant: Variant::Outline, disabled: true, "禁用" }
/// }
/// ```
#[allow(non_snake_case)]
pub fn Button(props: ButtonProps) -> Element {
    let mut hovered = use_signal(|| false);

    let style_str = build_button_style(
        props.variant,
        props.size,
        hovered(),
        props.disabled,
        props.block,
        &props.style,
    );

    let onclick = props.onclick.clone();

    rsx! {
        button {
            class: if props.class.is_empty() { None } else { Some(props.class.as_str()) },
            style: "{style_str}",
            r#type: "{props.r#type}",
            disabled: props.disabled,
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            onclick: move |evt| {
                if let Some(ref handler) = onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}