use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::atomic::{AtomicUsize, Ordering};

static AF_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Affix 固钉组件属性
#[derive(Props, PartialEq, Clone)]
pub struct AffixProps {
    /// 距离窗口顶部的偏移量（px），达到该值时固定
    #[props(default = 0.0)]
    pub offset_top: f64,

    /// 距离窗口底部的偏移量（px），达到该值时固定
    #[props(default = -1.0)]
    pub offset_bottom: f64,

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

/// Affix 固钉 —— 将元素固定在页面可视范围内
#[allow(non_snake_case)]
pub fn Affix(props: AffixProps) -> Element {
    const CSS: &str = include_str!("../../assets/affix.css");

    let af_id = use_signal(|| {
        let id = AF_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("af-{}", id)
    });

    let mut fixed = use_signal(|| false);
    let offset_top = props.offset_top;
    let offset_bottom = props.offset_bottom;
    let onchange = props.onchange.clone();
    let id_str = af_id().clone();
    let placeholder_id = format!("{}-ph", id_str);

    // 使用 use_effect 注册滚动监听，组件销毁时自动清理
    {
        let id = id_str.clone();
        let ph_id = placeholder_id.clone();
        let cb = onchange.clone();

        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    if let Some(el) = doc.get_element_by_id(&id) {
                        let rect = el.get_bounding_client_rect();
                        let should_fix = if offset_bottom >= 0.0 {
                            let win_h = win.inner_height().unwrap().as_f64().unwrap_or(800.0);
                            rect.bottom() >= win_h - offset_bottom
                        } else {
                            rect.top() <= offset_top
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

        // 使用 document capture scroll 捕获任意后代滚动容器的滚动事件
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback_and_bool(
                "scroll",
                closure.as_ref().unchecked_ref(),
                true, // capture
            );
        }
        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("resize",
                closure.as_ref().unchecked_ref());
        }
        closure.forget();
    }

    let fixed_style = if fixed() {
        if offset_bottom >= 0.0 {
            format!("position: fixed; bottom: {}px; left: 0; right: 0; z-index: 10;", offset_bottom)
        } else {
            format!("position: fixed; top: {}px; left: 0; right: 0; z-index: 10;", offset_top)
        }
    } else {
        String::new()
    };

    rsx! {
        style { {CSS} }
        div {
            class: "ctrl-affix {props.class}",
            style: "{props.style}",
            div {
                id: placeholder_id,
                class: if fixed() { "ctrl-affix__placeholder" } else { "" },
            }
            div {
                id: id_str,
                class: if fixed() { "ctrl-affix--fixed" } else { "" },
                style: fixed_style,
                {&props.children}
            }
        }
    }
}
