use dioxus::prelude::*;

/// 浮动按钮位置
#[derive(PartialEq, Clone, Copy)]
pub enum FloatButtonPosition {
    BottomRight,
    BottomLeft,
    TopRight,
    TopLeft,
}

/// FloatButton 浮动按钮组件属性
#[derive(Props, PartialEq, Clone)]
pub struct FloatButtonProps {
    /// 位置
    #[props(default = FloatButtonPosition::BottomRight)]
    pub position: FloatButtonPosition,

    /// 距边缘偏移（px）
    #[props(default = 24)]
    pub offset: u32,

    /// 按钮文案（不设置则只显示图标）
    pub label: Option<String>,

    /// 图标 SVG（不设置则使用默认 + 号图标）
    pub icon: Option<String>,

    /// 点击回调
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// 是否显示小红点徽标
    #[props(default = false)]
    pub dot: bool,

    /// 徽标数字
    pub badge: Option<u32>,

    /// 提示文字
    pub tooltip: Option<String>,

    /// 按钮形状：circle / square
    #[props(default = "circle".to_string())]
    pub shape: String,

    /// 按钮类型：default / primary
    #[props(default = "default".to_string())]
    pub btn_type: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// FloatButtonGroup 浮动按钮组属性
#[derive(Props, PartialEq, Clone)]
pub struct FloatButtonGroupProps {
    /// 位置
    #[props(default = FloatButtonPosition::BottomRight)]
    pub position: FloatButtonPosition,

    /// 距边缘偏移（px）
    #[props(default = 24)]
    pub offset: u32,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    pub children: Element,
}

const DEFAULT_ICON: &str = r#"<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 5v14M5 12h14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>"#;

/// FloatButton 浮动按钮组件
#[allow(non_snake_case)]
pub fn FloatButton(props: FloatButtonProps) -> Element {
    const CSS: &str = include_str!("../../assets/float_button.css");

    let mut classes = vec!["ctrl-float-btn".to_string()];

    // 形状
    if props.shape == "square" {
        classes.push("ctrl-float-btn--square".to_string());
    }

    // 类型
    if props.btn_type == "primary" {
        classes.push("ctrl-float-btn--primary".to_string());
    }

    // 位置通过行内 style 控制
    let pos_style = match props.position {
        FloatButtonPosition::BottomRight => format!("bottom:{}px;right:{}px;", props.offset, props.offset),
        FloatButtonPosition::BottomLeft => format!("bottom:{}px;left:{}px;", props.offset, props.offset),
        FloatButtonPosition::TopRight => format!("top:{}px;right:{}px;", props.offset, props.offset),
        FloatButtonPosition::TopLeft => format!("top:{}px;left:{}px;", props.offset, props.offset),
    };

    let inline_style = format!("{} {}", pos_style, props.style);

    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let class_str = classes.join(" ");

    let icon_html = props.icon.as_deref().unwrap_or(DEFAULT_ICON);

    let has_badge = props.dot || props.badge.is_some();

    rsx! {
        style { {CSS} }
        div {
            class: "{class_str}",
            style: "{inline_style}",
            title: props.tooltip.clone().unwrap_or_default(),
            onclick: move |evt| {
                if let Some(ref cb) = props.onclick {
                    cb.call(evt);
                }
            },
            span {
                class: "ctrl-float-btn__icon",
                dangerous_inner_html: icon_html,
            }
            if let Some(ref label) = props.label {
                span { class: "ctrl-float-btn__label", "{label}" }
            }
            if has_badge {
                span {
                    class: if props.dot { "ctrl-float-btn__badge ctrl-float-btn__badge--dot" } else { "ctrl-float-btn__badge" },
                    if let Some(n) = props.badge {
                        if n > 99 { "99+" } else { "{n}" }
                    }
                }
            }
        }
    }
}

/// FloatButtonGroup 浮动按钮组组件
#[allow(non_snake_case)]
pub fn FloatButtonGroup(props: FloatButtonGroupProps) -> Element {
    let pos_style = match props.position {
        FloatButtonPosition::BottomRight => format!("bottom:{}px;right:{}px;", props.offset, props.offset),
        FloatButtonPosition::BottomLeft => format!("bottom:{}px;left:{}px;", props.offset, props.offset),
        FloatButtonPosition::TopRight => format!("top:{}px;right:{}px;", props.offset, props.offset),
        FloatButtonPosition::TopLeft => format!("top:{}px;left:{}px;", props.offset, props.offset),
    };

    let mut classes = vec!["ctrl-float-btn-group".to_string()];
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }

    rsx! {
        div {
            class: classes.join(" "),
            style: "{pos_style} {props.style}",
            {props.children}
        }
    }
}
