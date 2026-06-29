//! 弹层组件通用工具
//!
//! 核心思路（效仿 Element Plus）：
//! 1. position 由 CSS 处理（`position: absolute; top: calc(100% + 4px)`），浏览器原生滚动
//! 2. 仅 JS 控制 visibility 切换
//! 3. mousedown/mouseup 模式的 click-outside 检测
//! 4. 统一的 Closure 生命周期管理

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

fn write_style(el: &web_sys::Element, css: &str) {
    let _ = el.set_attribute("style", css);
}

/// 在 use_effect 中控制 overlay 的 visibility 切换
pub fn use_visibility_effect(overlay_id: &str, open: Signal<bool>) {
    let oid = overlay_id.to_string();
    use_effect(move || {
        let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
        let Some(el) = doc.get_element_by_id(&oid) else { return };
        if open() {
            write_style(&el, "visibility:visible;");
        } else {
            write_style(&el, "visibility:hidden;");
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
