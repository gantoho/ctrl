use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use std::rc::Rc;
use std::cell::RefCell;
use ctrl_core::types::Size;
use crate::overlay::{self, OverlayClosures};
use wasm_bindgen::JsCast;

static SELECT_ID: AtomicU16 = AtomicU16::new(1);

/// Select 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    #[props(default = Size::default())]
    pub size: Size,
    #[props(default = "请选择".to_string())]
    pub placeholder: String,
    #[props(default = "".to_string())]
    pub value: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = "".to_string())]
    pub style: String,
    pub onchange: Option<EventHandler<String>>,
    #[props(default = Vec::new())]
    pub options: Vec<(String, String, bool)>,
}

/// Select 下拉选择组件
#[allow(non_snake_case)]
pub fn Select(props: SelectProps) -> Element {
    const CSS: &str = include_str!("../../assets/select.css");
    let mut open = use_signal(|| false);
    let id = SELECT_ID.fetch_add(1, Ordering::Relaxed);
    let container_id = format!("ctrl-select-{}", id);
    let trigger_id = format!("ctrl-select-trigger-{}", id);
    let dropdown_id = format!("ctrl-select-dropdown-{}", id);

    let selected_label = props.options.iter()
        .find(|(v, _, _)| v == &props.value)
        .map(|(_, l, _)| l.clone())
        .unwrap_or(props.placeholder.clone());

    let mut trigger_classes = vec!["ctrl-select__trigger".to_string(),
        format!("ctrl-select__trigger--{}", match props.size {
            Size::Sm => "sm", Size::Md => "md", Size::Lg => "lg", _ => "md",
        })];
    if props.disabled { trigger_classes.push("ctrl-select__trigger--disabled".into()); }
    let trigger_class = trigger_classes.join(" ");

    let mut container_class = "ctrl-select".to_string();
    if !props.class.is_empty() { container_class = format!("{} {}", container_class, props.class); }

    let arrow_class = if open() { "ctrl-select__arrow ctrl-select__arrow--open" } else { "ctrl-select__arrow" };

    let onchange = props.onchange.clone();
    let render_options = props.options.clone();
    let selected_val = props.value.clone();
    let disabled = props.disabled;

    // ── fixed 弹层：不受 overflow:hidden 裁切 + document capture scroll 跟随 + 匹配 trigger 宽度 ──
    overlay::use_fixed_panel_effect(&dropdown_id, &trigger_id, open.clone(), 4.0, true);

    // ── z-index：打开时提升容器层级，防止被下方兄弟元素遮挡 ──
    {
        let cid = container_id.clone();
        let o = open.clone();
        use_effect(move || {
            let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
            let Some(el) = doc.get_element_by_id(&cid) else { return };
            let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() else { return };
            if o() {
                let _ = html_el.style().set_property("z-index", "100");
            } else {
                let _ = html_el.style().set_property("z-index", "auto");
            }
        });
    }

    // ── 事件监听（mousedown/mouseup click-outside）──
    let listeners = use_signal(|| Rc::new(RefCell::new(OverlayClosures::new())));
    {
        let store = listeners.clone();
        let tid = trigger_id.clone();
        let did = dropdown_id.clone();
        let o = open.clone();
        use_effect(move || {
            if o() {
                overlay::setup_click_outside(&store.read(), &tid, &did, o.clone());
            } else {
                store.read().borrow_mut().cleanup();
            }
        });
    }
    use_drop(move || {
        listeners.read().borrow_mut().cleanup();
    });

    rsx! {
        style { {CSS} }
        div {
            id: "{container_id}",
            class: "{container_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            div {
                id: "{trigger_id}",
                class: "{trigger_class}",
                onclick: move |_| { if !disabled { open.set(!open()); } },
                span {
                    class: "ctrl-select__text",
                    if props.value.is_empty() {
                        span { class: "ctrl-select__placeholder", "{selected_label}" }
                    } else { "{selected_label}" }
                }
                svg {
                    class: "{arrow_class}", width: "12", height: "12", view_box: "0 0 12 12", fill: "none",
                    path {
                        d: "M3 4.5l3 3 3-3", stroke: "var(--ctrl-text-secondary)",
                        stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
                    }
                }
            }
            div {
                id: "{dropdown_id}",
                class: "ctrl-select__dropdown",
                { render_options.into_iter().map(move |(v, l, d)| {
                    let is_selected = v == selected_val;
                    let mut opt_classes = vec!["ctrl-select__option".to_string()];
                    if is_selected { opt_classes.push("ctrl-select__option--selected".into()); }
                    if d { opt_classes.push("ctrl-select__option--disabled".into()); }
                    let opt_class = opt_classes.join(" ");
                    let vc = v.clone();
                    let oc = onchange.clone();
                    let mut open_clone = open.clone();
                    rsx! { div { key: "{v}", class: "{opt_class}",
                        onclick: move |_| {
                            if !d {
                                if let Some(ref handler) = oc { handler.call(vc.clone()); }
                                open_clone.set(false);
                            }
                        }, "{l}"
                    }}
                })}
            }
        }
    }
}
