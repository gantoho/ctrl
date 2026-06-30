use dioxus::prelude::*;

// ═══════════════════════════════════════════════════════════
// DialogConfig — 命令式 API 配置
// ═══════════════════════════════════════════════════════════

/// Dialog 命令式 API 配置
#[derive(Clone)]
pub struct DialogConfig {
    /// 标题
    pub title: String,
    /// 内容
    pub content: Element,
    /// 宽度
    pub width: String,
    /// 是否显示关闭按钮
    pub show_close: bool,
    /// 点击遮罩是否关闭
    pub mask_closable: bool,
    /// 确认回调
    pub on_confirm: Option<EventHandler<()>>,
    /// 关闭回调
    pub onclose: Option<EventHandler<()>>,
    /// 确认按钮文字
    pub confirm_text: String,
    /// 取消按钮文字
    pub cancel_text: String,
}

impl Default for DialogConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            content: rsx! {},
            width: "480px".to_string(),
            show_close: true,
            mask_closable: true,
            on_confirm: None,
            onclose: None,
            confirm_text: "确定".to_string(),
            cancel_text: "取消".to_string(),
        }
    }
}

// ═══════════════════════════════════════════════════════════
// DialogAPI — 通过 use_dialog() 获取
// ═══════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct DialogAPI {
    pub(crate) visible: Signal<bool>,
    pub(crate) config: Signal<DialogConfig>,
}

/// 获取 Dialog 命令式 API
pub fn use_dialog() -> DialogAPI {
    use_context::<DialogAPI>()
}

impl DialogAPI {
    /// 打开对话框
    pub fn open(&mut self, config: DialogConfig) {
        self.config.set(config);
        self.visible.set(true);
    }

    /// 关闭对话框
    pub fn close(&mut self) {
        self.visible.set(false);
    }
}

// ═══════════════════════════════════════════════════════════
// DialogProvider
// ═══════════════════════════════════════════════════════════

/// DialogProvider 属性
#[derive(Props, PartialEq, Clone)]
pub struct DialogProviderProps {
    pub children: Element,
}

/// 对话框命令式 Provider，包裹在应用根节点
///
/// 通过 use_dialog() 获取 API 触发对话框：
///
/// ```rust
/// let mut dialog = use_dialog();
/// dialog.open(DialogConfig {
///     title: "确认删除".into(),
///     content: rsx! { p { "确定要删除此项目吗？" } },
///     confirm_text: "确定".into(),
///     cancel_text: "取消".into(),
///     on_confirm: Some(EventHandler::new(|_| { /* 删除逻辑 */ })),
///     ..Default::default()
/// });
/// ```
#[allow(non_snake_case)]
pub fn DialogProvider(props: DialogProviderProps) -> Element {
    let visible = use_signal(|| false);
    let config = use_signal(DialogConfig::default);

    let api = DialogAPI {
        visible: visible.clone(),
        config: config.clone(),
    };
    use_context_provider(|| api);

    let vis = visible();
    let cfg = config.read().clone();

    let footer = {
        let confirm = cfg.on_confirm.clone();
        let mut close_api = api.clone();
        rsx! {
            button {
                class: "ctrl-dialog__btn ctrl-dialog__btn--cancel",
                onclick: move |_| { close_api.close(); },
                "{cfg.cancel_text}"
            }
            button {
                class: "ctrl-dialog__btn ctrl-dialog__btn--confirm",
                onclick: move |_| {
                    if let Some(ref handler) = confirm {
                        handler.call(());
                    }
                    close_api.close();
                },
                "{cfg.confirm_text}"
            }
        }
    };

    let mut close = {
        let mut api = api.clone();
        let onclose = cfg.onclose.clone();
        move || {
            if let Some(ref handler) = onclose {
                handler.call(());
            }
            api.close();
        }
    };

    rsx! {
        {props.children}
        Dialog {
            visible: vis,
            title: cfg.title,
            width: cfg.width,
            show_close: cfg.show_close,
            mask_closable: cfg.mask_closable,
            onclose: Some(EventHandler::new(move |_| close())),
            footer: Some(footer),
            {cfg.content}
        }
    }
}

// ═══════════════════════════════════════════════════════════
// DialogProps — 声明式 API
// ═══════════════════════════════════════════════════════════
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
            class: "ctrl-dialog__overlay",
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
                if let Some(footer) = props.footer.clone() {
                    div {
                        class: "ctrl-dialog__footer",
                        {footer}
                    }
                }
            }
        }
    }
}
