use dioxus::prelude::*;

/// 菜单项
#[derive(Clone, PartialEq)]
pub struct ContextMenuItem {
    /// 标签文本
    pub label: String,
    /// 图标（emoji 或短文本）
    pub icon: String,
    /// 是否分隔线
    pub divider: bool,
    /// 是否禁用
    pub disabled: bool,
    /// 点击回调 ID
    pub id: String,
}

impl ContextMenuItem {
    pub fn new(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self { label: label.into(), icon: String::new(), divider: false, disabled: false, id: id.into() }
    }
    pub fn icon(mut self, icon: impl Into<String>) -> Self { self.icon = icon.into(); self }
    pub fn divider(mut self) -> Self { self.divider = true; self }
    pub fn disabled(mut self) -> Self { self.disabled = true; self }
}

/// ContextMenu 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ContextMenuProps {
    /// 菜单项
    pub items: Vec<ContextMenuItem>,
    /// 菜单点击回调（id 标识哪一项被点击）
    pub onselect: EventHandler<String>,
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
    /// 触发区域内容
    pub children: Element,
}

/// ContextMenu 右键菜单组件
///
/// 在 children 区域内右键弹出自定义操作菜单，点击菜单项或点击其他区域关闭。
#[allow(non_snake_case)]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    const CSS: &str = include_str!("../../assets/context_menu.css");

    // None = 关闭，Some((page_x, page_y)) = 打开
    let mut active = use_signal(|| None::<(f64, f64)>);

    let container_class = if props.class.is_empty() {
        "ctrl-context-menu".to_string()
    } else {
        format!("ctrl-context-menu {}", props.class)
    };

    let pos = active();
    let menu_style = if let Some((px, py)) = pos {
        let win_w = web_sys::window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|w| w.as_f64())
            .unwrap_or(1920.0);
        let win_h = web_sys::window()
            .and_then(|w| w.inner_height().ok())
            .and_then(|h| h.as_f64())
            .unwrap_or(1080.0);
        let menu_w = 180.0;
        let x = if px + menu_w > win_w { px - menu_w } else { px };
        let y = if py + 300.0 > win_h { py - 300.0 } else { py };
        format!("display:block; left:{x:.0}px; top:{y:.0}px;")
    } else {
        "display:none;".to_string()
    };

    rsx! {
        style { {CSS} }
        // 遮罩层：点击任意位置关闭
        if pos.is_some() {{
            let mut a = active;
            rsx! {
                div {
                    class: "ctrl-context-menu__backdrop",
                    onclick: move |e: MouseEvent| {
                        e.prevent_default();
                        e.stop_propagation();
                        a.set(None);
                    },
                    oncontextmenu: move |e: MouseEvent| {
                        e.prevent_default();
                        e.stop_propagation();
                        a.set(None);
                    },
                }
            }
        }}
        div {
            class: "{container_class}",
            style: "{props.style}",
            oncontextmenu: move |e: MouseEvent| {
                e.prevent_default();
                e.stop_propagation();
                let cc = e.data().client_coordinates();
                active.set(Some((cc.x, cc.y)));
            },
            {props.children}
        }
        // 菜单面板（fixed 定位，同级渲染）
        {
            let items = props.items.clone();
            let onselect = props.onselect;
            let panel_class = if pos.is_some() { "ctrl-context-menu__panel is-open" } else { "ctrl-context-menu__panel" };
            rsx! {
                div {
                    class: "{panel_class}",
                    style: "{menu_style}",
                    onclick: move |e: MouseEvent| {
                        e.stop_propagation();
                    },
                    for (i, item) in items.iter().enumerate() {
                        {
                            if item.divider {
                                rsx! { div { key: "div-{i}", class: "ctrl-context-menu__divider" } }
                            } else {
                                let id = item.id.clone();
                                let disabled = item.disabled;
                                let onselect = onselect.clone();
                                let mut active_close = active;
                                rsx! {
                                    button {
                                        key: "btn-{i}",
                                        class: if disabled { "ctrl-context-menu__item is-disabled" } else { "ctrl-context-menu__item" },
                                        disabled: disabled,
                                        onclick: move |_| {
                                            if !disabled {
                                                onselect.call(id.clone());
                                                active_close.set(None);
                                            }
                                        },
                                        if !item.icon.is_empty() {
                                            span { class: "ctrl-context-menu__icon", "{item.icon}" }
                                        }
                                        span { class: "ctrl-context-menu__label", "{item.label}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
