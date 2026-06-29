use dioxus::prelude::*;
use ctrl_core::types::{Size, Variant};

/// 构建按钮 class 列表
fn build_button_class(variant: Variant, size: Size, disabled: bool, block: bool) -> String {
    let mut classes = vec!["ctrl-button".to_string()];

    match variant {
        Variant::Primary => classes.push("ctrl-button--primary".into()),
        Variant::Secondary => classes.push("ctrl-button--secondary".into()),
        Variant::Outline => classes.push("ctrl-button--outline".into()),
        Variant::Ghost => classes.push("ctrl-button--ghost".into()),
        _ => {}
    }

    match size {
        Size::Sm => classes.push("ctrl-button--sm".into()),
        Size::Md => classes.push("ctrl-button--md".into()),
        Size::Lg => classes.push("ctrl-button--lg".into()),
        _ => {}
    }

    if disabled {
        classes.push("ctrl-button--disabled".into());
    }

    if block {
        classes.push("ctrl-button--block".into());
    }

    classes.join(" ")
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
#[allow(non_snake_case)]
pub fn Button(props: ButtonProps) -> Element {
    const CSS: &str = include_str!("../../assets/button.css");
    let btn_class = build_button_class(
        props.variant,
        props.size,
        props.disabled,
        props.block,
    );

    let user_class = if props.class.is_empty() {
        btn_class.clone()
    } else {
        format!("{} {}", btn_class, props.class)
    };

    let onclick = props.onclick.clone();

    rsx! {
        style { {CSS} }
        button {
            class: "{user_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            r#type: "{props.r#type}",
            disabled: props.disabled,
            onclick: move |evt| {
                if let Some(ref handler) = onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}
