use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::atomic::{AtomicUsize, Ordering};

static AF_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// 固钉位置
#[derive(PartialEq, Clone, Copy)]
pub enum AffixPosition {
    Top,
    Bottom,
}

/// Affix 固钉组件属性
#[derive(Props, PartialEq, Clone)]
pub struct AffixProps {
    /// 距顶部/底部偏移量（px）
    #[props(default = 0.0)]
    pub offset_top: f64,

    /// 距底部偏移量（px），设置后优先于 offset_top
    #[props(default = -1.0)]
    pub offset_bottom: f64,

    /// 固定方向
    #[props(default = AffixPosition::Top)]
    pub position: AffixPosition,

    /// 指定滚动容器的 CSS 选择器（如 "#container"）
    /// 有 target：纯 CSS position:sticky（元素需放在容器内部）
    /// 无 target：JS position:fixed + 滚动监听
    #[props(default = "".to_string())]
    pub target: String,

    /// 是否禁用固钉
    #[props(default = false)]
    pub disabled: bool,

    /// 固定时的 z-index
    #[props(default = 10)]
    pub z_index: u32,

    /// 固定状态变化回调
    pub onchange: Option<EventHandler<bool>>,

    /// 子元素
    pub children: Element,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// Affix 固钉 —— 将元素固定在可视范围内
#[allow(non_snake_case)]
pub fn Affix(props: AffixProps) -> Element {
    const CSS: &str = include_str!("../../assets/affix.css");

    let af_id = use_signal(|| {
        let id = AF_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("af-{}", id)
    });

    let mut fixed = use_signal(|| false);
    let offset_bottom = props.offset_bottom;
    let position = props.position.clone();
    let target = props.target.clone();
    let disabled = props.disabled;
    let z_index = props.z_index;
    let onchange = props.onchange.clone();
    let id_str = af_id().clone();
    let placeholder_id = format!("{}-ph", id_str);

    let (use_top, offset_px) = if offset_bottom >= 0.0 {
        (false, offset_bottom)
    } else {
        match position {
            AffixPosition::Bottom => (false, props.offset_top),
            AffixPosition::Top => (true, props.offset_top),
        }
    };

    let has_target = !target.is_empty();
    let dir = if use_top { "top" } else { "bottom" };

    // viewport 模式：JS scroll 监听 + position:fixed
    if !has_target && !disabled {
        let id = id_str.clone();
        let ph_id = placeholder_id.clone();
        let cb = onchange.clone();

        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    if let Some(el) = doc.get_element_by_id(&id) {
                        let rect = el.get_bounding_client_rect();
                        let win_h = win.inner_height().unwrap().as_f64().unwrap_or(800.0);

                        let should_fix = if use_top {
                            rect.top() <= offset_px
                        } else {
                            rect.bottom() >= win_h - offset_px
                        };

                        if should_fix != fixed() {
                            if let Some(ph) = doc.get_element_by_id(&ph_id) {
                                if should_fix {
                                    let _ = ph.set_attribute("style",
                                        &format!("height: {}px; visibility: hidden;", rect.height()));
                                } else {
                                    let _ = ph.remove_attribute("style");
                                }
                            }
                            fixed.set(should_fix);
                            if let Some(ref cb) = cb {
                                cb.call(should_fix);
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut()>);

        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback_and_bool(
                "scroll", closure.as_ref().unchecked_ref(), true,
            );
        }
        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("resize",
                closure.as_ref().unchecked_ref());
        }
        closure.forget();
    }

    rsx! {
        style { {CSS} }

        // container sticky 模式：直接渲染，position:sticky 作用于子元素自身
        if has_target && !disabled {
            div {
                id: id_str,
                class: "ctrl-affix {props.class}",
                style: "position: sticky; {dir}: {offset_px}px; z-index: {z_index}; {props.style}",
                {&props.children}
            }
        }
        // disabled 或 viewport 非 fixed 状态
        else if disabled || !fixed() {
            div {
                class: "ctrl-affix {props.class}",
                style: "{props.style}",
                div { id: placeholder_id }
                div { id: id_str, {&props.children} }
            }
        }
        // viewport fixed 状态
        else {
            div {
                class: "ctrl-affix {props.class}",
                style: "{props.style}",
                div {
                    id: placeholder_id,
                    class: "ctrl-affix__placeholder",
                }
                div {
                    id: id_str,
                    class: "ctrl-affix--fixed",
                    style: "position: fixed; {dir}: {offset_px}px; left: 0; right: 0; z-index: {z_index};",
                    {&props.children}
                }
            }
        }
    }
}
