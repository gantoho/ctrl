use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

static DROPDOWN_ID: AtomicU16 = AtomicU16::new(1);

/// Dropdown 下拉菜单组件属性
#[derive(Props, PartialEq, Clone)]
pub struct DropdownProps {
    /// 弹出位置：bottom / bottom-start / bottom-end / top / top-start / top-end
    #[props(default = "bottom".to_string())]
    pub placement: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 触发元素（通常是 Button）
    pub trigger: Element,

    /// 菜单项（DropdownItem / DropdownDivider）
    pub children: Element,
}

/// 下拉菜单项属性
#[derive(Props, PartialEq, Clone)]
pub struct DropdownItemProps {
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 点击事件
    pub onclick: Option<EventHandler<()>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（菜单项文字）
    pub children: Element,
}

/// 下拉菜单分割线
#[derive(Props, PartialEq, Clone)]
pub struct DropdownDividerProps {}

/// 计算 fixed 定位坐标
fn calc_menu_position(
    placement: &str,
    trigger_rect: &web_sys::DomRect,
    menu_width: f64,
    menu_height: f64,
    vw: f64,
    vh: f64,
) -> (f64, f64) {
    let gap = 4.0;
    let t = trigger_rect;

    match placement {
        "bottom" => {
            let top = (t.bottom() + gap).min(vh - menu_height - gap);
            let left = (t.left() + t.width() / 2.0 - menu_width / 2.0)
                .max(gap)
                .min(vw - menu_width - gap);
            (top, left)
        }
        "bottom-start" => {
            let top = (t.bottom() + gap).min(vh - menu_height - gap);
            let left = t.left().max(gap).min(vw - menu_width - gap);
            (top, left)
        }
        "bottom-end" => {
            let top = (t.bottom() + gap).min(vh - menu_height - gap);
            let left = (t.right() - menu_width).max(gap).min(vw - menu_width - gap);
            (top, left)
        }
        "top" => {
            let top = (t.top() - menu_height - gap).max(0.0);
            let left = (t.left() + t.width() / 2.0 - menu_width / 2.0)
                .max(gap)
                .min(vw - menu_width - gap);
            (top, left)
        }
        "top-start" => {
            let top = (t.top() - menu_height - gap).max(0.0);
            let left = t.left().max(gap).min(vw - menu_width - gap);
            (top, left)
        }
        "top-end" => {
            let top = (t.top() - menu_height - gap).max(0.0);
            let left = (t.right() - menu_width).max(gap).min(vw - menu_width - gap);
            (top, left)
        }
        _ => {
            let top = (t.bottom() + gap).min(vh - menu_height - gap);
            let left = (t.left() + t.width() / 2.0 - menu_width / 2.0)
                .max(gap)
                .min(vw - menu_width - gap);
            (top, left)
        }
    }
}

/// 自动翻转 placement
fn resolve_placement(
    preferred: &str,
    trigger_rect: &web_sys::DomRect,
    _menu_width: f64,
    menu_height: f64,
    _vw: f64,
    vh: f64,
) -> String {
    let gap = 4.0;
    let space_bottom = vh - trigger_rect.bottom();
    let space_top = trigger_rect.top();

    if preferred.starts_with("bottom") {
        if space_bottom < menu_height + gap && space_top >= menu_height + gap {
            return preferred.replace("bottom", "top");
        }
    } else if preferred.starts_with("top") {
        if space_top < menu_height + gap && space_bottom >= menu_height + gap {
            return preferred.replace("top", "bottom");
        }
    }

    preferred.to_string()
}

/// 定位菜单：测量 trigger 和 menu 的尺寸，计算 fixed 坐标
fn position_menu(trigger_id: &str, menu_id: &str, preferred_placement: &str) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let doc = match window.document() {
        Some(d) => d,
        None => return,
    };
    let trigger = match doc.get_element_by_id(trigger_id) {
        Some(el) => el,
        None => return,
    };
    let menu = match doc.get_element_by_id(menu_id) {
        Some(el) => el,
        None => return,
    };

    let trigger_rect = trigger.get_bounding_client_rect();
    let menu_rect = menu.get_bounding_client_rect();

    let vw = window.inner_width().unwrap().as_f64().unwrap_or(1024.0);
    let vh = window.inner_height().unwrap().as_f64().unwrap_or(768.0);

    let menu_w = if menu_rect.width() > 0.0 {
        menu_rect.width()
    } else {
        menu.client_width() as f64
    };
    let menu_h = if menu_rect.height() > 0.0 {
        menu_rect.height()
    } else {
        menu.client_height() as f64
    };

    let placement = resolve_placement(preferred_placement, &trigger_rect, menu_w, menu_h, vw, vh);
    let (top, left) = calc_menu_position(&placement, &trigger_rect, menu_w, menu_h, vw, vh);

    let _ = menu.set_attribute(
        "style",
        &format!(
            "position: fixed; top: {}px; left: {}px; z-index: 1001; visibility: visible;",
            top.max(0.0),
            left.max(0.0)
        ),
    );
}

/// Dropdown 下拉菜单组件
#[allow(non_snake_case)]
pub fn Dropdown(props: DropdownProps) -> Element {
    const CSS: &str = include_str!("../../assets/dropdown.css");
    let mut open = use_signal(|| false);

    let id = DROPDOWN_ID.fetch_add(1, Ordering::Relaxed);
    let trigger_id = format!("ctrl-dropdown-trigger-{}", id);
    let menu_id = format!("ctrl-dropdown-menu-{}", id);

    let wrapper_class = if props.class.is_empty() {
        "ctrl-dropdown".to_string()
    } else {
        format!("ctrl-dropdown {}", props.class)
    };

    // ── 当 open 变为 true 时，计算 fixed 定位 ──
    {
        let trigger_id = trigger_id.clone();
        let menu_id = menu_id.clone();
        let placement = props.placement.clone();
        use_effect(use_reactive((&open,), move |(open,)| {
            if !open() {
                return;
            }
            let trigger_id = trigger_id.clone();
            let menu_id = menu_id.clone();
            let placement = placement.clone();
            wasm_bindgen_futures::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(1).await;
                position_menu(&trigger_id, &menu_id, &placement);
            });
        }));
    }

    // ── 窗口 resize 时重新计算位置 ──
    {
        let trigger_id = trigger_id.clone();
        let menu_id = menu_id.clone();
        let placement = props.placement.clone();
        let open_for_resize = open.clone();
        use_effect(move || {
            let trigger_id = trigger_id.clone();
            let menu_id = menu_id.clone();
            let placement = placement.clone();

            let callback = Closure::wrap(Box::new(move || {
                if open_for_resize.try_read().is_ok_and(|r| *r) {
                    position_menu(&trigger_id, &menu_id, &placement);
                }
            }) as Box<dyn FnMut()>);

            if let Some(window) = web_sys::window() {
                let _ = window
                    .add_event_listener_with_callback("resize", callback.as_ref().unchecked_ref());
            }

            callback.forget();
        });
    }

    // ── 点击外部关闭 ──
    {
        let trigger_id = trigger_id.clone();
        let menu_id = menu_id.clone();
        let mut open_close = open.clone();
        use_effect(use_reactive((&open,), move |(open,)| {
            if !open() {
                return;
            }
            let trigger_id = trigger_id.clone();
            let menu_id = menu_id.clone();
            let doc = web_sys::window()
                .and_then(|w| w.document())
                .expect("document");

            let callback = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let Some(target) = event.target() else {
                    return;
                };
                let Some(el) = target.dyn_ref::<web_sys::Element>() else {
                    return;
                };
                if let (Some(trigger), Some(menu)) = (
                    doc.get_element_by_id(&trigger_id),
                    doc.get_element_by_id(&menu_id),
                ) {
                    if !trigger.contains(Some(el)) && !menu.contains(Some(el)) {
                        if let Ok(mut w) = open_close.try_write() { *w = false; }
                    }
                }
            }) as Box<dyn FnMut(_)>);

            let _ = web_sys::window()
                .and_then(|w| w.document())
                .map(|d| d.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref()));

            callback.forget();
        }));
    }

    // ── 滚动时关闭 ──
    {
        let mut open_scroll = open.clone();
        use_effect(use_reactive((&open,), move |(open,)| {
            if !open() {
                return;
            }
            let callback = Closure::wrap(Box::new(move || {
                if let Ok(mut w) = open_scroll.try_write() { *w = false; }
            }) as Box<dyn FnMut()>);

            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback_and_bool(
                    "scroll",
                    callback.as_ref().unchecked_ref(),
                    true,
                );
            }

            callback.forget();
        }));
    }

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            // 触发区
            div {
                id: "{trigger_id}",
                class: "ctrl-dropdown__trigger",
                onclick: move |e| {
                    e.stop_propagation();
                    open.set(!open());
                },
                {props.trigger}
            }
            if open() {
                div {
                    id: "{menu_id}",
                    class: "ctrl-dropdown__menu",
                    {props.children}
                }
            }
        }
    }
}

/// DropdownItem 菜单项
#[allow(non_snake_case)]
pub fn DropdownItem(props: DropdownItemProps) -> Element {
    let onclick = props.onclick.clone();
    let disabled = props.disabled;

    let item_class = if disabled {
        "ctrl-dropdown__item ctrl-dropdown__item--disabled"
    } else {
        "ctrl-dropdown__item"
    };

    rsx! {
        button {
            class: if props.class.is_empty() { item_class.to_string() } else { format!("{} {}", item_class, props.class) },
            type: "button",
            disabled: props.disabled,
            onclick: move |_| {
                if !disabled {
                    if let Some(ref handler) = onclick {
                        handler.call(());
                    }
                }
            },
            {props.children}
        }
    }
}

/// DropdownDivider 分割线
#[allow(non_snake_case)]
pub fn DropdownDivider(_props: DropdownDividerProps) -> Element {
    rsx! {
        div {
            class: "ctrl-dropdown__divider",
        }
    }
}
