use dioxus::prelude::*;

/// Empty 空状态组件属性
#[derive(Props, PartialEq, Clone)]
pub struct EmptyProps {
    /// 描述文字
    #[props(default = "暂无数据".to_string())]
    pub description: String,

    /// 自定义图片 URL
    #[props(default = "".to_string())]
    pub image: String,

    /// 尺寸：small / default / large
    #[props(default = "default".to_string())]
    pub size: String,

    /// 自定义图片/图标尺寸
    #[props(default = "".to_string())]
    pub image_size: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 底部操作区
    pub children: Element,
}

/// 默认 SVG 空状态图标
const DEFAULT_ICON: &str = r#"<svg width="100%" height="100%" viewBox="0 0 80 80" fill="none" xmlns="http://www.w3.org/2000/svg"><rect x="16" y="12" width="48" height="56" rx="8" stroke="currentColor" stroke-width="2" fill="none"/><line x1="28" y1="28" x2="52" y2="28" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="28" y1="38" x2="52" y2="38" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="28" y1="48" x2="42" y2="48" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>"#;

/// Empty 空状态组件
#[allow(non_snake_case)]
pub fn Empty(props: EmptyProps) -> Element {
    const CSS: &str = include_str!("../../assets/empty.css");

    let mut classes = vec!["ctrl-empty".to_string()];
    if props.size != "default" {
        classes.push(format!("ctrl-empty--{}", props.size));
    }
    if !props.class.is_empty() {
        classes.push(props.class);
    }
    let wrapper_class = classes.join(" ");

    let img_size = if props.image_size.is_empty() {
        match props.size.as_str() {
            "small" => "48px",
            "large" => "120px",
            _ => "80px",
        }
    } else {
        &props.image_size
    };

    rsx! {
        style { {CSS} }
        div { class: "{wrapper_class}", style: "{props.style}",
            div { class: "ctrl-empty__image", style: "width:{img_size}; height:{img_size};",
                if !props.image.is_empty() {
                    img { class: "ctrl-empty__img", src: "{props.image}", alt: "空状态" }
                } else {
                    div { dangerous_inner_html: DEFAULT_ICON }
                }
            }
            p { class: "ctrl-empty__description", "{props.description}" }
            div { class: "ctrl-empty__footer", {props.children} }
        }
    }
}
