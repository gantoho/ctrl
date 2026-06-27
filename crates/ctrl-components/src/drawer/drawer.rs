use dioxus::prelude::*;

/// Drawer 抽屉组件属性
#[derive(Props, PartialEq, Clone)]
pub struct DrawerProps {
    /// 是否打开
    #[props(default = false)]
    pub visible: bool,

    /// 标题
    #[props(default = "".to_string())]
    pub title: String,

    /// 显示位置：left / right / top / bottom
    #[props(default = "right".to_string())]
    pub placement: String,

    /// 宽度或高度（根据 placement）
    #[props(default = "380px".to_string())]
    pub size: String,

    /// 是否显示关闭按钮
    #[props(default = true)]
    pub show_close: bool,

    /// 关闭事件
    pub onclose: Option<EventHandler<()>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 底部操作区
    pub footer: Option<Element>,

    /// 子元素（抽屉内容）
    pub children: Element,
}

/// Drawer 抽屉组件
#[allow(non_snake_case)]
pub fn Drawer(props: DrawerProps) -> Element {
    const CSS: &str = include_str!("../../assets/drawer.css");
    if !props.visible {
        return rsx! {};
    }

    let onclose = props.onclose.clone();
    let placement = props.placement.clone();
    let dim = props.size.clone();

    let panel_style = match placement.as_str() {
        "left" | "right" => format!("width:{}", dim),
        "top" | "bottom" => format!("height:{}", dim),
        _ => format!("width:{}", dim),
    };

    let panel_class = if props.class.is_empty() {
        format!("ctrl-drawer ctrl-drawer--{} ctrl-drawer--open", placement)
    } else {
        format!("ctrl-drawer ctrl-drawer--{} ctrl-drawer--open {}", placement, props.class)
    };

    rsx! {
        style { {CSS} }
        // 遮罩
        div {
            class: "ctrl-drawer__overlay",
            onclick: move |_| {
                if let Some(ref handler) = onclose {
                    handler.call(());
                }
            },
        }
        // 面板
        div {
            class: "{panel_class}",
            style: "{panel_style}",
            // 头部
            if !props.title.is_empty() || props.show_close {
                div {
                    class: "ctrl-drawer__header",
                    h3 { class: "ctrl-drawer__title", "{props.title}" }
                    if props.show_close {
                        button {
                            class: "ctrl-drawer__close",
                            onclick: move |_| {
                                if let Some(ref handler) = onclose {
                                    handler.call(());
                                }
                            },
                            "✕"
                        }
                    }
                }
            }
            // 内容
            div {
                class: "ctrl-drawer__body",
                {props.children}
            }
            // 底部
            if props.footer.is_some() {
                div {
                    class: "ctrl-drawer__footer",
                    {props.footer.unwrap()}
                }
            }
        }
    }
}
