//! 弹层组件通用工具
//!
//! 核心思路：
//! 1. 面板使用 position:fixed（viewport 坐标系），不受祖先 overflow:hidden 影响
//! 2. 打开时从 trigger.getBoundingClientRect() 计算一次坐标
//! 3. document 上以 capture 模式监听 scroll，捕获任意后代滚动容器的滚动，实时更新坐标
//! 4. mousedown/mouseup 模式的 click-outside 检测
//! 5. 统一的 Closure 生命周期管理

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// 存储所有活跃的事件监听 Closure
pub struct OverlayClosures {
    pub mousedown: Option<Closure<dyn FnMut(web_sys::MouseEvent)>>,
    pub mouseup: Option<Closure<dyn FnMut(web_sys::MouseEvent)>>,
}

impl OverlayClosures {
    pub fn new() -> Self {
        Self { mousedown: None, mouseup: None }
    }

    /// 移除所有注册的监听器并释放 Closure
    pub fn cleanup(&mut self) {
        let doc = web_sys::window().and_then(|w| w.document());
        if let Some(cb) = self.mousedown.take() {
            if let Some(ref d) = doc { let _ = d.remove_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref()); }
        }
        if let Some(cb) = self.mouseup.take() {
            if let Some(ref d) = doc { let _ = d.remove_event_listener_with_callback("mouseup", cb.as_ref().unchecked_ref()); }
        }
    }
}

pub type OverlayClosuresRef = Rc<RefCell<OverlayClosures>>;

// ──────────────────────────────────────────────
// Visibility 控制
// ──────────────────────────────────────────────

/// 在 use_effect 中控制 overlay 的 visibility 切换（使用 set_property，不会覆盖其他 style 属性）
pub fn use_visibility_effect(overlay_id: &str, open: Signal<bool>) {
    let oid = overlay_id.to_string();
    use_effect(move || {
        let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
        let Some(el) = doc.get_element_by_id(&oid) else { return };
        let Some(html_el) = el.dyn_ref::<HtmlElement>() else { return };
        if open() {
            let _ = html_el.style().set_property("visibility", "visible");
        } else {
            let _ = html_el.style().set_property("visibility", "hidden");
        }
    });
}

/// position:fixed 弹层面板 — 不受祖先 overflow:hidden 影响。
/// 面板留在原始 DOM 位置，不破坏 Dioxus 事件系统。
/// 打开时计算一次坐标，滚动时通过 document capture 模式更新坐标。
/// `match_width` 为 true 时，面板宽度会匹配 trigger 宽度（适用于 Select 等）。
pub fn use_fixed_panel_effect(
    panel_id: &str,
    trigger_id: &str,
    open: Signal<bool>,
    gap_px: f64,
    match_width: bool,
) {
    let pid = panel_id.to_string();
    let tid = trigger_id.to_string();

    fn set_position(doc: &web_sys::Document, pid: &str, tid: &str, gap: f64, match_width: bool) {
        let Some(panel) = doc.get_element_by_id(pid) else { return };
        let Some(panel_el) = panel.dyn_ref::<HtmlElement>() else { return };
        let Some(trigger) = doc.get_element_by_id(tid) else { return };
        let rect = trigger.get_bounding_client_rect();
        let style = panel_el.style();
        let _ = style.set_property("left", &format!("{}px", rect.left()));
        let _ = style.set_property("top", &format!("{}px", rect.bottom() + gap));
        if match_width {
            let _ = style.set_property("width", &format!("{}px", rect.width()));
        }
    }

    // visibility + 初始定位
    {
        let pid2 = pid.clone();
        let tid2 = tid.clone();
        use_effect(move || {
            let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
            let Some(panel) = doc.get_element_by_id(&pid2) else { return };
            let Some(panel_el) = panel.dyn_ref::<HtmlElement>() else { return };
            if open() {
                set_position(&doc, &pid2, &tid2, gap_px, match_width);
                let _ = panel_el.style().set_property("visibility", "visible");
            } else {
                let _ = panel_el.style().set_property("visibility", "hidden");
            }
        });
    }

    // scroll 事件：document capture 模式，捕获任意后代滚动容器的滚动
    let mut scroll_cb: Signal<Option<Closure<dyn FnMut()>>> = use_signal(|| None);
    {
        let o = open.clone();
        let mut sc = scroll_cb.clone();
        let pid3 = pid.clone();
        let tid3 = tid.clone();
        use_effect(move || {
            if o() {
                let p = pid3.clone();
                let t = tid3.clone();
                let cb = Closure::wrap(Box::new(move || {
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        set_position(&doc, &p, &t, gap_px, match_width);
                    }
                }) as Box<dyn FnMut()>);
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    // capture=true 捕获任意后代的 scroll 事件（包括 overflow:auto/scroll 容器）
                    let _ = doc.add_event_listener_with_callback_and_bool(
                        "scroll", cb.as_ref().unchecked_ref(), true,
                    );
                }
                sc.set(Some(cb));
            } else {
                if let Some(cb) = sc.take() {
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        let _ = doc.remove_event_listener_with_callback_and_bool(
                            "scroll", cb.as_ref().unchecked_ref(), true,
                        );
                    }
                }
            }
        });
    }

    use_drop(move || {
        if let Some(cb) = scroll_cb.take() {
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                let _ = doc.remove_event_listener_with_callback_and_bool(
                    "scroll", cb.as_ref().unchecked_ref(), true,
                );
            }
        }
    });
}

// ──────────────────────────────────────────────
// Click-outside（mousedown/mouseup 模式）
// ──────────────────────────────────────────────

/// 创建 mousedown/mouseup 模式的 click-outside 监听器。
///
/// 核心逻辑（效仿 Element Plus / Popper.js）：
/// - mousedown 记录点击起始位置是否在 overlay/trigger 内部
/// - mouseup 时，只有 mousedown 和 mouseup **都**在外部才触发关闭
///   这避免了拖拽或从内部拖到外部释放的场景误关闭
pub fn setup_click_outside(
    store: &OverlayClosuresRef,
    trigger_id: &str,
    overlay_id: &str,
    mut open: Signal<bool>,
) {
    let mut guard = store.borrow_mut();
    if guard.mousedown.is_some() { return; }

    let mousedown_target_inside = Rc::new(Cell::new(false));

    {
        let tid = trigger_id.to_string();
        let oid = overlay_id.to_string();
        let inside = mousedown_target_inside.clone();
        let cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let Some(target) = event.target() else { return };
            let Some(el) = target.dyn_ref::<web_sys::Element>() else { return };
            let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
            let in_trigger = doc.get_element_by_id(&tid).map_or(false, |t| t.contains(Some(el)));
            let in_overlay = doc.get_element_by_id(&oid).map_or(false, |o| o.contains(Some(el)));
            inside.set(in_trigger || in_overlay);
        }) as Box<dyn FnMut(_)>);
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref());
        }
        guard.mousedown = Some(cb);
    }

    {
        let tid = trigger_id.to_string();
        let oid = overlay_id.to_string();
        let inside = mousedown_target_inside.clone();
        let cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if inside.get() { return; }
            let Some(target) = event.target() else { return };
            let Some(el) = target.dyn_ref::<web_sys::Element>() else { return };
            let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
            let in_trigger = doc.get_element_by_id(&tid).map_or(false, |t| t.contains(Some(el)));
            let in_overlay = doc.get_element_by_id(&oid).map_or(false, |o| o.contains(Some(el)));
            if !in_trigger && !in_overlay { open.set(false); }
        }) as Box<dyn FnMut(_)>);
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("mouseup", cb.as_ref().unchecked_ref());
        }
        guard.mouseup = Some(cb);
    }
}

/// 自定义 is_inside 判断的 mousedown/mouseup click-outside（用于 DatePicker）
pub fn setup_click_outside_custom(
    store: &OverlayClosuresRef,
    is_inside: impl Fn(&web_sys::Element) -> bool + 'static,
    mut open: Signal<bool>,
) {
    let mut guard = store.borrow_mut();
    if guard.mousedown.is_some() { return; }

    let mousedown_target_inside = Rc::new(Cell::new(false));
    let is_inside = Rc::new(is_inside);

    {
        let is_inside = is_inside.clone();
        let inside = mousedown_target_inside.clone();
        let cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let Some(target) = event.target() else { return };
            let Some(el) = target.dyn_ref::<web_sys::Element>() else { return };
            inside.set((*is_inside)(el));
        }) as Box<dyn FnMut(_)>);
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref());
        }
        guard.mousedown = Some(cb);
    }

    {
        let is_inside = is_inside.clone();
        let inside = mousedown_target_inside.clone();
        let cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if inside.get() { return; }
            let Some(target) = event.target() else { return };
            let Some(el) = target.dyn_ref::<web_sys::Element>() else { return };
            if !(*is_inside)(el) { open.set(false); }
        }) as Box<dyn FnMut(_)>);
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("mouseup", cb.as_ref().unchecked_ref());
        }
        guard.mouseup = Some(cb);
    }
}
