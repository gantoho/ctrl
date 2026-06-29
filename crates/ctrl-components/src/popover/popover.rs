use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

static POPOVER_ID: AtomicU16 = AtomicU16::new(1);

/// Popover 气泡卡片组件属性
#[derive(Props, PartialEq, Clone)]
pub struct PopoverProps {
    /// 弹出位置：top / bottom / left / right
    #[props(default = "top".to_string())]
    pub placement: String,

    /// 标题
    #[props(default = "".to_string())]
    pub title: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 触发元素
    pub children: Element,

    /// 气泡内容
    pub content: Element,
}

/// 计算最佳弹出位置（自动翻转）
fn resolve_placement(
    preferred: &str,
    trigger_rect: &web_sys::DomRect,
    card_width: f64,
    card_height: f64,
    vw: f64,
    vh: f64,
) -> String {
    let gap = 8.0;

    let space_top = trigger_rect.top();
    let space_bottom = vh - trigger_rect.bottom();
    let space_left = trigger_rect.left();
    let space_right = vw - trigger_rect.right();

    match preferred {
        "top" => {
            if space_top < card_height + gap && space_bottom >= card_height + gap {
                "bottom".to_string()
            } else {
                "top".to_string()
            }
        }
        "bottom" => {
            if space_bottom < card_height + gap && space_top >= card_height + gap {
                "top".to_string()
            } else {
                "bottom".to_string()
            }
        }
        "left" => {
            if space_left < card_width + gap && space_right >= card_width + gap {
                "right".to_string()
            } else {
                "left".to_string()
            }
        }
        "right" => {
            if space_right < card_width + gap && space_left >= card_width + gap {
                "left".to_string()
            } else {
                "right".to_string()
            }
        }
        _ => preferred.to_string(),
    }
}

/// 根据 placement 计算 fixed 坐标
fn calc_position(
    placement: &str,
    trigger_rect: &web_sys::DomRect,
    card_width: f64,
    card_height: f64,
    vw: f64,
    vh: f64,
) -> (f64, f64) {
    let gap = 8.0;
    let t = trigger_rect;

    match placement {
        "top" => {
            let top = (t.top() - card_height - gap).max(0.0);
            let left = (t.left() + t.width() / 2.0 - card_width / 2.0)
                .max(gap)
                .min(vw - card_width - gap);
            (top, left)
        }
        "bottom" => {
            let top = (t.bottom() + gap).min(vh - card_height - gap);
            let left = (t.left() + t.width() / 2.0 - card_width / 2.0)
                .max(gap)
                .min(vw - card_width - gap);
            (top, left)
        }
        "left" => {
            let top = (t.top() + t.height() / 2.0 - card_height / 2.0)
                .max(gap)
                .min(vh - card_height - gap);
            let left = (t.left() - card_width - gap).max(0.0);
            (top, left)
        }
        "right" => {
            let top = (t.top() + t.height() / 2.0 - card_height / 2.0)
                .max(gap)
                .min(vh - card_height - gap);
            let left = (t.right() + gap).min(vw - card_width - gap);
            (top, left)
        }
        _ => {
            let top = (t.bottom() + gap).min(vh - card_height - gap);
            let left = (t.left() + t.width() / 2.0 - card_width / 2.0)
                .max(gap)
                .min(vw - card_width - gap);
            (top, left)
        }
    }
}

/// 核心定位函数：测量 trigger 和 card 的尺寸，计算最佳位置并设置 inline style
fn position_card(trigger_id: &str, card_id: &str, preferred_placement: &str) {
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
    let card = match doc.get_element_by_id(card_id) {
        Some(el) => el,
        None => return,
    };

    let trigger_rect = trigger.get_bounding_client_rect();
    let card_rect = card.get_bounding_client_rect();

    let vw = window.inner_width().unwrap().as_f64().unwrap_or(1024.0);
    let vh = window.inner_height().unwrap().as_f64().unwrap_or(768.0);

    let card_w = if card_rect.width() > 0.0 {
        card_rect.width()
    } else {
        card.client_width() as f64
    };
    let card_h = if card_rect.height() > 0.0 {
        card_rect.height()
    } else {
        card.client_height() as f64
    };

    let placement = resolve_placement(preferred_placement, &trigger_rect, card_w, card_h, vw, vh);
    let (top, left) = calc_position(&placement, &trigger_rect, card_w, card_h, vw, vh);

    let _ = card.set_attribute(
        "style",
        &format!(
            "position: fixed; top: {}px; left: {}px; z-index: 1001;",
            top.max(0.0),
            left.max(0.0)
        ),
    );
}

/// Popover 气泡卡片组件
#[allow(non_snake_case)]
pub fn Popover(props: PopoverProps) -> Element {
    const CSS: &str = include_str!("../../assets/popover.css");
    let mut visible = use_signal(|| false);

    let id = POPOVER_ID.fetch_add(1, Ordering::Relaxed);
    let trigger_id = format!("ctrl-popover-trigger-{}", id);
    let card_id = format!("ctrl-popover-card-{}", id);

    let wrapper_class = if props.class.is_empty() {
        "ctrl-popover".to_string()
    } else {
        format!("ctrl-popover {}", props.class)
    };

    // ── 当 visible 变为 true 时，计算 fixed 定位 ──
    {
        let trigger_id = trigger_id.clone();
        let card_id = card_id.clone();
        let placement = props.placement.clone();
        use_effect(use_reactive((&visible,), move |(visible,)| {
            if !visible() {
                return;
            }
            let trigger_id = trigger_id.clone();
            let card_id = card_id.clone();
            let placement = placement.clone();
            wasm_bindgen_futures::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(1).await;
                position_card(&trigger_id, &card_id, &placement);
            });
        }));
    }

    // ── 事件监听器（resize/scroll 定位 + 外部点击关闭），组件卸载时自动清理 ──
    {
        struct PopoverListeners {
            resize: Closure<dyn FnMut()>,
            scroll: Closure<dyn FnMut()>,
            click: Closure<dyn FnMut(web_sys::MouseEvent)>,
        }

        let listeners: Signal<Rc<RefCell<Option<PopoverListeners>>>> = use_signal(|| Rc::new(RefCell::new(None)));

        // 注册监听器（只执行一次）
        {
            let closures = listeners.clone();
            let tid = trigger_id.clone();
            let cid = card_id.clone();
            let placement = props.placement.clone();
            let vs_rx = visible.clone();
            let mut vs_for_click = visible.clone();

            let mut done = use_signal(|| false);
            use_effect(move || {
                if done() { return; }
                done.set(true);

                // resize → 重新定位
                let resize_cb = Closure::wrap(Box::new({
                    let tid = tid.clone();
                    let cid = cid.clone();
                    let placement = placement.clone();
                    let v = vs_rx.clone();
                    move || {
                        if v.try_read().is_ok_and(|r| *r) {
                            position_card(&tid, &cid, &placement);
                        }
                    }
                }) as Box<dyn FnMut()>);

                // scroll → 关闭
                let scroll_cb = Closure::wrap(Box::new({
                    let mut v = vs_rx.clone();
                    move || {
                        if let Ok(mut w) = v.try_write() { *w = false; }
                    }
                }) as Box<dyn FnMut()>);

                // click → 外部关闭
                let click_cb = Closure::wrap(Box::new({
                    let tid = tid.clone();
                    let cid = cid.clone();
                    move |event: web_sys::MouseEvent| {
                        let Some(target) = event.target() else { return };
                        let Some(el) = target.dyn_ref::<web_sys::Element>() else { return };
                        let doc = match web_sys::window().and_then(|w| w.document()) {
                            Some(d) => d,
                            None => return,
                        };
                        if let (Some(trigger), Some(card)) = (
                            doc.get_element_by_id(&tid),
                            doc.get_element_by_id(&cid),
                        ) {
                            if !trigger.contains(Some(el)) && !card.contains(Some(el)) {
                                if let Ok(mut w) = vs_for_click.try_write() { *w = false; }
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                if let Some(window) = web_sys::window() {
                    let _ = window.add_event_listener_with_callback("resize", resize_cb.as_ref().unchecked_ref());
                    let _ = window.add_event_listener_with_callback("scroll", scroll_cb.as_ref().unchecked_ref());
                }
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    let _ = doc.add_event_listener_with_callback("click", click_cb.as_ref().unchecked_ref());
                }

                *closures().borrow_mut() = Some(PopoverListeners { resize: resize_cb, scroll: scroll_cb, click: click_cb });
            });
        }

        // 组件卸载时清理
        use_drop(move || {
            if let Some(b) = listeners().borrow_mut().take() {
                if let Some(window) = web_sys::window() {
                    let _ = window.remove_event_listener_with_callback("resize", b.resize.as_ref().unchecked_ref());
                    let _ = window.remove_event_listener_with_callback("scroll", b.scroll.as_ref().unchecked_ref());
                }
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    let _ = doc.remove_event_listener_with_callback("click", b.click.as_ref().unchecked_ref());
                }
            }
        });
    }

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            // 触发区
            div {
                id: "{trigger_id}",
                class: "ctrl-popover__trigger",
                onclick: move |e| {
                    e.stop_propagation();
                    visible.set(!visible());
                },
                {props.children}
            }
            if visible() {
                div {
                    id: "{card_id}",
                    class: "ctrl-popover__card",
                    if !props.title.is_empty() {
                        div {
                            class: "ctrl-popover__title",
                            "{props.title}"
                        }
                    }
                    div {
                        class: "ctrl-popover__content",
                        {props.content}
                    }
                }
            }
        }
    }
}