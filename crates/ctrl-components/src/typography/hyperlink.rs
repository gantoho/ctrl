use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct HyperLinkProps {
    /// 链接地址
    #[props(default = "".to_string())]
    pub href: String,

    /// 打开方式，如 "_blank"
    #[props(default = "".to_string())]
    pub target: String,

    /// 是否显示下划线
    #[props(default = true)]
    pub underline: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 点击事件
    pub onclick: Option<EventHandler<()>>,

    pub children: Element,
}

#[allow(non_snake_case)]
pub fn HyperLink(props: HyperLinkProps) -> Element {
    let mut classes = vec!["ctrl-hyperlink".to_string()];

    if !props.underline {
        classes.push("ctrl-hyperlink--no-underline".to_string());
    }
    if props.disabled {
        classes.push("ctrl-hyperlink--disabled".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class);
    }

    let class = classes.join(" ");
    let onclick = props.onclick.clone();

    if props.disabled {
        rsx! {
            span { class: "{class}", style: "{props.style}", {props.children} }
        }
    } else {
        rsx! {
            a {
                class: "{class}",
                style: "{props.style}",
                href: "{props.href}",
                target: if props.target.is_empty() { None } else { Some(props.target.as_str()) },
                onclick: move |evt| {
                    if let Some(ref handler) = onclick {
                        handler.call(());
                    }
                    if props.href.is_empty() {
                        evt.prevent_default();
                    }
                },
                {props.children}
            }
        }
    }
}
