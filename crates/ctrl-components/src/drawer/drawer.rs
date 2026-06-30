use dioxus::prelude::*;
use ctrl_core::types::Placement;

// ═══════════════════════════════════════════════════════════
// DrawerConfig — 命令式 API 配置
// ═══════════════════════════════════════════════════════════

/// Drawer 命令式 API 配置
#[derive(Clone)]
pub struct DrawerConfig {
    /// 标题
    pub title: String,
    /// 内容
    pub content: Element,
    /// 显示位置
    pub placement: Placement,
    /// 宽度/高度
    pub size: String,
    /// 是否显示关闭按钮
    pub show_close: bool,
    /// 关闭回调
    pub onclose: Option<EventHandler<()>>,
}

impl Default for DrawerConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            content: rsx! {},
            placement: Placement::Right,
            size: "380px".to_string(),
            show_close: true,
            onclose: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════
// DrawerAPI — 通过 use_drawer() 获取
// ═══════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct DrawerAPI {
    pub(crate) visible: Signal<bool>,
    pub(crate) config: Signal<DrawerConfig>,
}

/// 获取 Drawer 命令式 API
pub fn use_drawer() -> DrawerAPI {
    use_context::<DrawerAPI>()
}

impl DrawerAPI {
    /// 打开抽屉
    pub fn open(&mut self, config: DrawerConfig) {
        self.config.set(config);
        self.visible.set(true);
    }

    /// 关闭抽屉
    pub fn close(&mut self) {
        self.visible.set(false);
    }
}

// ═══════════════════════════════════════════════════════════
// DrawerProvider
// ═══════════════════════════════════════════════════════════

/// DrawerProvider 属性
#[derive(Props, PartialEq, Clone)]
pub struct DrawerProviderProps {
    pub children: Element,
}

/// 抽屉命令式 Provider，包裹在应用根节点
///
/// 通过 use_drawer() 获取 API 触发抽屉：
///
/// ```rust
/// let mut drawer = use_drawer();
/// drawer.open(DrawerConfig {
///     title: "详情".into(),
///     content: rsx! { p { "这里是抽屉内容" } },
///     placement: Placement::Right,
///     ..Default::default()
/// });
/// ```
#[allow(non_snake_case)]
pub fn DrawerProvider(props: DrawerProviderProps) -> Element {
    let visible = use_signal(|| false);
    let config = use_signal(DrawerConfig::default);

    let api = DrawerAPI {
        visible: visible.clone(),
        config: config.clone(),
    };
    use_context_provider(|| api);

    let vis = visible();
    let cfg = config.read().clone();

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
        Drawer {
            visible: vis,
            title: cfg.title,
            placement: cfg.placement,
            size: cfg.size,
            show_close: cfg.show_close,
            onclose: Some(EventHandler::new(move |_| close())),
            {cfg.content}
        }
    }
}

// ═══════════════════════════════════════════════════════════
// DrawerProps — 声明式 API
// ═══════════════════════════════════════════════════════════
#[derive(Props, PartialEq, Clone)]
pub struct DrawerProps {
    /// 是否打开
    #[props(default = false)]
    pub visible: bool,

    /// 标题
    #[props(default = "".to_string())]
    pub title: String,

    /// 显示位置
    #[props(default = Placement::Right)]
    pub placement: Placement,

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
    let placement = props.placement;
    let dim = props.size.clone();

    let panel_style = match placement {
        Placement::Left | Placement::Right => format!("width:{}", dim),
        Placement::Top | Placement::Bottom => format!("height:{}", dim),
        _ => format!("width:{}", dim),
    };

    let placement_str = placement.to_string();
    let panel_class = if props.class.is_empty() {
        format!("ctrl-drawer ctrl-drawer--{} ctrl-drawer--open", placement_str)
    } else {
        format!("ctrl-drawer ctrl-drawer--{} ctrl-drawer--open {}", placement_str, props.class)
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
            if let Some(footer) = props.footer.clone() {
                div {
                    class: "ctrl-drawer__footer",
                    {footer}
                }
            }
        }
    }
}
