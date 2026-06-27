use dioxus::prelude::*;

/// Dialog 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct DialogProps {
    /// 是否显示
    #[props(default = false)]
    pub visible: bool,

    /// 标题
    #[props(default = "".to_string())]
    pub title: String,

    /// 宽度
    #[props(default = "480px".to_string())]
    pub width: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 是否显示关闭按钮
    #[props(default = true)]
    pub show_close: bool,

    /// 点击遮罩是否关闭
    #[props(default = true)]
    pub mask_closable: bool,

    /// 关闭事件
    pub onclose: Option<EventHandler<()>>,

    /// 底部操作区插槽
    pub footer: Option<Element>,

    /// 子元素（弹窗内容）
    pub children: Element,
}

/// Dialog 对话框组件
#[allow(non_snake_case)]
pub fn Dialog(props: DialogProps) -> Element {
    const CSS: &str = include_str!("../../assets/dialog.css");
    if !props.visible {
        return rsx! {};
    }

    let onclose = props.onclose.clone();
    let mask_closable = props.mask_closable;

    let dialog_class = if props.class.is_empty() {
        "ctrl-dialog".to_string()
    } else {
        format!("ctrl-dialog {}", props.class)
    };

    let dialog_style = format!("width: {w}; {extra}", w = props.width, extra = props.style);

    rsx! {
        style { {CSS} }
        // 遮罩层
        div {
            class: "ctrl-dialog-overlay",
            // 半透明背景
            div {
                class: "ctrl-dialog__mask",
                onclick: move |_| {
                    if mask_closable {
                        if let Some(ref handler) = onclose {
                            handler.call(());
                        }
                    }
                },
            }
            // 弹窗主体
            div {
                class: "{dialog_class}",
                style: "{dialog_style}",

                // 头部
                if !props.title.is_empty() || props.show_close {
                    div {
                        class: "ctrl-dialog__header",
                        h3 { class: "ctrl-dialog__title", "{props.title}" }
                        if props.show_close {
                            button {
                                class: "ctrl-dialog__close",
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
                    class: "ctrl-dialog__body",
                    {props.children}
                }

                // 底部
                if props.footer.is_some() {
                    div {
                        class: "ctrl-dialog__footer",
                        {props.footer.unwrap()}
                    }
                }
            }
        }
    }
}
