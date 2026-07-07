use dioxus::prelude::*;
use ctrl_core::types::Size;

/// 构建 Kbd class 列表
fn build_kbd_class(size: Size, class: &str) -> String {
    let mut classes = vec!["ctrl-kbd".to_string()];
    match size {
        Size::Sm => classes.push("ctrl-kbd--sm".into()),
        Size::Lg => classes.push("ctrl-kbd--lg".into()),
        _ => {}
    }
    if !class.is_empty() {
        classes.push(class.to_string());
    }
    classes.join(" ")
}

/// Kbd 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct KbdProps {
    /// 尺寸（默认 md）
    #[props(default = Size::default())]
    pub size: Size,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 按键内容
    pub children: Element,
}

/// Kbd 键盘按键组件
#[allow(non_snake_case)]
pub fn Kbd(props: KbdProps) -> Element {
    const CSS: &str = include_str!("../../assets/kbd.css");
    let kbd_class = build_kbd_class(props.size, &props.class);

    rsx! {
        style { {CSS} }
        kbd {
            class: "{kbd_class}",
            style: "{props.style}",
            {props.children}
        }
    }
}
