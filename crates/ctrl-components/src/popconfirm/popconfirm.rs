use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use std::rc::Rc;
use std::cell::RefCell;
use ctrl_core::types::Placement;
use crate::overlay::{self, OverlayClosures};

static POPCONFIRM_ID: AtomicU16 = AtomicU16::new(1);

/// Popconfirm 气泡确认框组件属性
#[derive(Props, PartialEq, Clone)]
pub struct PopconfirmProps {
    /// 确认框标题
    #[props(default = "确认操作".to_string())]
    pub title: String,

    /// 确认框描述
    #[props(default = "".to_string())]
    pub description: String,

    /// 确认按钮文字
    #[props(default = "确定".to_string())]
    pub confirm_text: String,

    /// 取消按钮文字
    #[props(default = "取消".to_string())]
    pub cancel_text: String,

    /// 弹出位置
    #[props(default = Placement::Top)]
    pub placement: Placement,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 确认回调
    #[props(default = None)]
    pub onconfirm: Option<EventHandler<()>>,

    /// 取消回调
    #[props(default = None)]
    pub oncancel: Option<EventHandler<()>>,

    /// 触发元素
    pub children: Element,
}

/// Popconfirm 气泡确认框组件
///
/// 点击触发元素弹出确认卡片，用户点击「确定」或「取消」后关闭。
/// 常用于危险操作前的二次确认。
#[allow(non_snake_case)]
pub fn Popconfirm(props: PopconfirmProps) -> Element {
    const CSS: &str = include_str!("../../assets/popconfirm.css");
    let mut visible = use_signal(|| false);
    let id = POPCONFIRM_ID.fetch_add(1, Ordering::Relaxed);
    let trigger_id = format!("ctrl-popconfirm-trigger-{}", id);
    let card_id = format!("ctrl-popconfirm-card-{}", id);

    let wrapper_class = if props.class.is_empty() {
        "ctrl-popconfirm".to_string()
    } else {
        format!("ctrl-popconfirm {}", props.class)
    };

    // ── fixed 弹层定位：按 placement 计算坐标 + 滚动跟随（不受 overflow:hidden 裁切）──
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
    use_drop(move || {
        listeners.read().borrow_mut().cleanup();
    });

    let do_confirm = move |_| {
        visible.set(false);
        if let Some(ref cb) = props.onconfirm {
            cb.call(());
        }
    };

    let do_cancel = move |_| {
        visible.set(false);
        if let Some(ref cb) = props.oncancel {
            cb.call(());
        }
    };

    rsx! {
        style { {CSS} }
        div { class: "{wrapper_class}",
            div {
                id: "{trigger_id}",
                class: "ctrl-popconfirm__trigger",
                onclick: move |_| { visible.set(!visible()); },
                {props.children}
            }
            div {
                id: "{card_id}",
                class: "ctrl-popconfirm__card",
                div { class: "ctrl-popconfirm__title",
                    span { class: "ctrl-popconfirm__icon", "!" }
                    "{props.title}"
                }
                if !props.description.is_empty() {
                    div { class: "ctrl-popconfirm__description", "{props.description}" }
                }
                div { class: "ctrl-popconfirm__actions",
                    button {
                        class: "ctrl-popconfirm__btn",
                        onclick: do_cancel,
                        "{props.cancel_text}"
                    }
                    button {
                        class: "ctrl-popconfirm__btn ctrl-popconfirm__btn--confirm",
                        onclick: do_confirm,
                        "{props.confirm_text}"
                    }
                }
            }
        }
    }
}
