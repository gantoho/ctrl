use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use std::rc::Rc;
use std::cell::RefCell;
use ctrl_core::types::Placement;
use crate::overlay::{self, OverlayClosures};

static POPOVER_ID: AtomicU16 = AtomicU16::new(1);

/// Popover 气泡卡片组件属性
#[derive(Props, PartialEq, Clone)]
pub struct PopoverProps {
    #[props(default = Placement::default())]
    pub placement: Placement,
    #[props(default = "".to_string())]
    pub title: String,
    /// 是否使用悬停触发（默认 false，即点击触发）
    #[props(default = false)]
    pub hover: bool,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
    pub content: Element,
}

/// Popover 气泡卡片组件
#[allow(non_snake_case)]
pub fn Popover(props: PopoverProps) -> Element {
    const CSS: &str = include_str!("../../assets/popover.css");
    let mut visible = use_signal(|| false);
    let id = POPOVER_ID.fetch_add(1, Ordering::Relaxed);
    let trigger_id = format!("ctrl-popover-trigger-{}", id);
    let card_id = format!("ctrl-popover-card-{}", id);

    let wrapper_class = if props.class.is_empty() { "ctrl-popover".to_string() } else { format!("ctrl-popover {}", props.class) };

    // ── fixed 弹层：不受 overflow:hidden 裁切 + document capture scroll 跟随 ──
    overlay::use_fixed_panel_effect_with_placement(&card_id, &trigger_id, visible.clone(), 8.0, props.placement);

    // ── 事件监听（mousedown/mouseup click-outside）──
    let listeners = use_signal(|| Rc::new(RefCell::new(OverlayClosures::new())));
    {
        let store = listeners.clone();
        let tid = trigger_id.clone();
        let cid = card_id.clone();
        let v = visible.clone();
        use_effect(move || {
            if v() {
                overlay::setup_click_outside(&store.read(), &tid, &cid, v.clone());
            } else {
                store.read().borrow_mut().cleanup();
            }
        });
    }
    use_drop(move || { listeners.read().borrow_mut().cleanup(); });

    let hover = props.hover;
    rsx! {
        style { {CSS} }
        div { class: "{wrapper_class}",
            onmouseenter: move |_| { if hover { visible.set(true); } },
            onmouseleave: move |_| { if hover { visible.set(false); } },
            div {
                id: "{trigger_id}",
                class: "ctrl-popover__trigger",
                onclick: move |_| { if !hover { visible.set(!visible()); } },
                {props.children}
            }
            div {
                id: "{card_id}",
                class: "ctrl-popover__card",
                if !props.title.is_empty() {
                    div { class: "ctrl-popover__title", "{props.title}" }
                }
                div { class: "ctrl-popover__content", {props.content} }
            }
        }
    }
}
