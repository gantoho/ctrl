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

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 底部操作区
    pub children: Element,
}

/// Empty 空状态组件
#[allow(non_snake_case)]
pub fn Empty(props: EmptyProps) -> Element {
    const CSS: &str = include_str!("../../assets/empty.css");

    let wrapper_class = if props.class.is_empty() {
        "ctrl-empty".to_string()
    } else {
        format!("ctrl-empty {}", props.class)
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            div {
                class: "ctrl-empty__image",
                if !props.image.is_empty() {
                    img {
                        class: "ctrl-empty__img",
                        src: "{props.image}",
                        alt: "空状态"
                    }
                } else {
                    // 默认空状态图标
                    "📭"
                }
            }
            p {
                class: "ctrl-empty__description",
                "{props.description}"
            }
            div {
                class: "ctrl-empty__footer",
                {props.children}
            }
        }
    }
}
