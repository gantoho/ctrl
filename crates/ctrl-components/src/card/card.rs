use dioxus::prelude::*;

/// Card 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    /// 卡片标题
    #[props(default = "".to_string())]
    pub title: String,

    /// 是否显示边框
    #[props(default = true)]
    pub bordered: bool,

    /// 是否带阴影
    #[props(default = false)]
    pub shadow: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 头部插槽
    pub header: Option<Element>,

    /// 子元素（卡片内容）
    pub children: Element,
}

/// Card 卡片组件
#[allow(non_snake_case)]
pub fn Card(props: CardProps) -> Element {
    let mut classes = vec!["ctrl-card".to_string()];

    if props.bordered {
        classes.push("ctrl-card--bordered".into());
    }
    if props.shadow {
        classes.push("ctrl-card--shadow".into());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }

    let card_class = classes.join(" ");

    rsx! {
        div {
            class: "{card_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },

            // 头部
            if !props.title.is_empty() || props.header.is_some() {
                div {
                    class: "ctrl-card__header",
                    if props.header.is_some() {
                        {props.header.unwrap()}
                    } else {
                        "{props.title}"
                    }
                }
            }

            // 内容
            div {
                class: "ctrl-card__body",
                {props.children}
            }
        }
    }
}
