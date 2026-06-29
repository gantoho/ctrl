use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use std::rc::Rc;
use std::cell::RefCell;
use ctrl_core::types::Placement;
use crate::overlay::{self, OverlayClosures};

static DROPDOWN_ID: AtomicU16 = AtomicU16::new(1);

/// Context: 传递 open signal 给 DropdownItem，使其点击后自动关闭下拉菜单
#[derive(Clone)]
struct DropdownOpen(Signal<bool>);

#[derive(Props, PartialEq, Clone)]
pub struct DropdownProps {
    #[props(default = Placement::Bottom)]
    pub placement: Placement,
    #[props(default = "".to_string())]
    pub class: String,
    pub trigger: Element,
    pub children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct DropdownItemProps {
    #[props(default = false)]
    pub disabled: bool,
    pub onclick: Option<EventHandler<()>>,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct DropdownDividerProps {}

#[allow(non_snake_case)]
pub fn Dropdown(props: DropdownProps) -> Element {
    const CSS: &str = include_str!("../../assets/dropdown.css");
    let mut open = use_signal(|| false);
    let id = DROPDOWN_ID.fetch_add(1, Ordering::Relaxed);
    let trigger_id = format!("ctrl-dropdown-trigger-{}", id);
    let menu_id = format!("ctrl-dropdown-menu-{}", id);

    // ── 通过 context 向子组件传递 open signal ──
    use_context_provider(|| DropdownOpen(open.clone()));

    let wrapper_class = if props.class.is_empty() { "ctrl-dropdown".to_string() } else { format!("ctrl-dropdown {}", props.class) };

    // ── Visibility（position 由 CSS 处理）──
    overlay::use_visibility_effect(&menu_id, open.clone());

    // ── 事件监听（mousedown/mouseup click-outside）──
    let listeners = use_signal(|| Rc::new(RefCell::new(OverlayClosures::new())));
    {
        let store = listeners.clone();
        let tid = trigger_id.clone();
        let mid = menu_id.clone();
        let o = open.clone();
        use_effect(move || {
            if o() {
                overlay::setup_click_outside(&store.read(), &tid, &mid, o.clone());
            } else {
                store.read().borrow_mut().cleanup();
            }
        });
    }
    use_drop(move || { listeners.read().borrow_mut().cleanup(); });

    rsx! {
        style { {CSS} }
        div { class: "{wrapper_class}",
            div {
                id: "{trigger_id}",
                class: "ctrl-dropdown__trigger",
                onclick: move |_| { open.set(!open()); },
                {props.trigger}
            }
            div {
                id: "{menu_id}",
                class: "ctrl-dropdown__menu",
                {props.children}
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn DropdownItem(props: DropdownItemProps) -> Element {
    let onclick = props.onclick.clone();
    let disabled = props.disabled;
    let mut dropdown_open = use_context::<DropdownOpen>();
    let item_class = if disabled { "ctrl-dropdown__item ctrl-dropdown__item--disabled" } else { "ctrl-dropdown__item" };
    rsx! {
        button {
            class: if props.class.is_empty() { item_class.to_string() } else { format!("{} {}", item_class, props.class) },
            type: "button", disabled: props.disabled,
            onclick: move |_| {
                if !disabled {
                    if let Some(ref handler) = onclick { handler.call(()); }
                    dropdown_open.0.set(false);
                }
            },
            {props.children}
        }
    }
}

#[allow(non_snake_case)]
pub fn DropdownDivider(_props: DropdownDividerProps) -> Element {
    rsx! { div { class: "ctrl-dropdown__divider" } }
}
