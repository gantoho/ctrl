use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

/// Slider 滑块组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SliderProps {
    /// 当前值
    #[props(default = 0)]
    pub value: i32,

    /// 最小值
    #[props(default = 0)]
    pub min: i32,

    /// 最大值
    #[props(default = 100)]
    pub max: i32,

    /// 步长
    #[props(default = 1)]
    pub step: i32,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否垂直方向
    #[props(default = false)]
    pub vertical: bool,

    /// 是否显示刻度标记
    #[props(default = false)]
    pub marks: bool,

    /// 是否显示数值标签
    #[props(default = false)]
    pub show_label: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 值变化回调
    pub onchange: Option<EventHandler<i32>>,
}

/// 根据鼠标/触摸位置计算滑块值
#[allow(dead_code)]
fn calc_slider_value(
    client_pos: f64,
    rail_start: f64,
    rail_size: f64,
    min: i32,
    max: i32,
    step: i32,
    vertical: bool,
) -> i32 {
    let mut ratio = ((client_pos - rail_start) / rail_size).clamp(0.0, 1.0);
    if vertical {
        ratio = 1.0 - ratio;
    }
    let raw = min as f64 + ratio * (max - min) as f64;
    let stepped = ((raw / step as f64).round() * step as f64) as i32;
    stepped.clamp(min, max)
}

/// 直接更新 DOM 中的 track 和 handle 位置，实现即时响应
#[cfg(target_arch = "wasm32")]
fn update_dom_track(rail_id: &str, value: i32, min: i32, max: i32, vertical: bool) {
    let range = max - min;
    if range == 0 {
        return;
    }
    let pct = ((value - min) as f64 / range as f64 * 100.0).clamp(0.0, 100.0);
    let Some(window) = web_sys::window() else { return; };
    let Some(doc) = window.document() else { return; };

    if let Some(track) = doc.get_element_by_id(&format!("{}-track", rail_id)) {
        let style = if vertical {
            format!("height: {}%;", pct)
        } else {
            format!("width: {}%;", pct)
        };
        let _ = track.set_attribute("style", &style);
    }
    if let Some(handle) = doc.get_element_by_id(&format!("{}-handle", rail_id)) {
        let style = if vertical {
            format!("bottom: {}%;", pct)
        } else {
            format!("left: {}%;", pct)
        };
        let _ = handle.set_attribute("style", &style);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
fn update_dom_track(_rail_id: &str, _value: i32, _min: i32, _max: i32, _vertical: bool) {}

/// 拖拽监听器句柄
#[cfg(target_arch = "wasm32")]
struct DragListeners {
    mm: Closure<dyn FnMut(web_sys::MouseEvent)>,
    mu: Closure<dyn FnMut(web_sys::MouseEvent)>,
    tm: Closure<dyn FnMut(web_sys::TouchEvent)>,
    tu: Closure<dyn FnMut(web_sys::TouchEvent)>,
}

/// 开始拖拽的核心逻辑，绑定全局事件监听
#[cfg(target_arch = "wasm32")]
fn begin_drag(
    mut value: Signal<i32>,
    mut is_dragging: Signal<bool>,
    rail_id: Signal<String>,
    min: i32,
    max: i32,
    step: i32,
    vertical: bool,
    onchange: Option<EventHandler<i32>>,
    drag_listeners: Signal<Rc<RefCell<Option<DragListeners>>>>,
    client_x: f64,
    client_y: f64,
) {
    let Some(window) = web_sys::window() else { return; };
    let Some(doc) = window.document() else { return; };
    let rid = rail_id();

    if let Some(rail) = doc.get_element_by_id(&rid) {
        let rect = rail.get_bounding_client_rect();
        let new_val = if vertical {
            calc_slider_value(client_y, rect.top(), rect.height(), min, max, step, vertical)
        } else {
            calc_slider_value(client_x, rect.left(), rect.width(), min, max, step, vertical)
        };
        value.set(new_val);
        is_dragging.set(true);
        update_dom_track(&rid, new_val, min, max, vertical);

        if let Some(ref cb) = onchange {
            cb.call(new_val);
        }

        // 绑定全局 mouse/touch 事件
        let document: web_sys::EventTarget = doc.clone().into();
        let window_target: web_sys::EventTarget = window.into();

        let mm_closure = {
            let mut value = value;
            let rid = rail_id;
            let onchange = onchange.clone();
            Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                e.prevent_default();
                let rid = rid();
                let Some(win) = web_sys::window() else { return; };
                let Some(doc) = win.document() else { return; };
                if let Some(rail) = doc.get_element_by_id(&rid) {
                    let rect = rail.get_bounding_client_rect();
                    let new_val = if vertical {
                        calc_slider_value(e.client_y() as f64, rect.top(), rect.height(), min, max, step, vertical)
                    } else {
                        calc_slider_value(e.client_x() as f64, rect.left(), rect.width(), min, max, step, vertical)
                    };
                    value.set(new_val);
                    update_dom_track(&rid, new_val, min, max, vertical);
                    if let Some(ref cb) = onchange {
                        cb.call(new_val);
                    }
                }
            }) as Box<dyn FnMut(web_sys::MouseEvent)>)
        };

        let mu_closure = {
            let mut is_dragging = is_dragging;
            let drag_listeners = drag_listeners;
            Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
                is_dragging.set(false);
                // 清理监听器
                if let Some(listeners) = drag_listeners().borrow_mut().take() {
                    let Some(window) = web_sys::window() else { return; };
                    let Some(doc) = window.document() else { return; };
                    let document: web_sys::EventTarget = doc.into();
                    let window_target: web_sys::EventTarget = window.into();
                    let _ = document.remove_event_listener_with_callback("mousemove", listeners.mm.as_ref().unchecked_ref());
                    let _ = document.remove_event_listener_with_callback("mouseup", listeners.mu.as_ref().unchecked_ref());
                    let _ = document.remove_event_listener_with_callback("touchmove", listeners.tm.as_ref().unchecked_ref());
                    let _ = document.remove_event_listener_with_callback("touchend", listeners.tu.as_ref().unchecked_ref());
                    let _ = window_target.remove_event_listener_with_callback("mousemove", listeners.mm.as_ref().unchecked_ref());
                    let _ = window_target.remove_event_listener_with_callback("mouseup", listeners.mu.as_ref().unchecked_ref());
                    let _ = window_target.remove_event_listener_with_callback("touchmove", listeners.tm.as_ref().unchecked_ref());
                    let _ = window_target.remove_event_listener_with_callback("touchend", listeners.tu.as_ref().unchecked_ref());
                }
            }) as Box<dyn FnMut(web_sys::MouseEvent)>)
        };

        let tm_closure = {
            let mut value = value;
            let rid = rail_id;
            let onchange = onchange.clone();
            Closure::wrap(Box::new(move |e: web_sys::TouchEvent| {
                e.prevent_default();
                let rid = rid();
                if let Some(touch) = e.touches().get(0) {
                    let Some(win) = web_sys::window() else { return; };
                let Some(doc) = win.document() else { return; };
                    if let Some(rail) = doc.get_element_by_id(&rid) {
                        let rect = rail.get_bounding_client_rect();
                        let new_val = if vertical {
                            calc_slider_value(touch.client_y() as f64, rect.top(), rect.height(), min, max, step, vertical)
                        } else {
                            calc_slider_value(touch.client_x() as f64, rect.left(), rect.width(), min, max, step, vertical)
                        };
                        value.set(new_val);
                        update_dom_track(&rid, new_val, min, max, vertical);
                        if let Some(ref cb) = onchange {
                            cb.call(new_val);
                        }
                    }
                }
            }) as Box<dyn FnMut(web_sys::TouchEvent)>)
        };

        let tu_closure = {
            let mut is_dragging = is_dragging;
            let drag_listeners = drag_listeners;
            Closure::wrap(Box::new(move |_e: web_sys::TouchEvent| {
                is_dragging.set(false);
                if let Some(listeners) = drag_listeners().borrow_mut().take() {
                    let Some(window) = web_sys::window() else { return; };
                    let Some(doc) = window.document() else { return; };
                    let document: web_sys::EventTarget = doc.into();
                    let window_target: web_sys::EventTarget = window.into();
                    let _ = document.remove_event_listener_with_callback("mousemove", listeners.mm.as_ref().unchecked_ref());
                    let _ = document.remove_event_listener_with_callback("mouseup", listeners.mu.as_ref().unchecked_ref());
                    let _ = document.remove_event_listener_with_callback("touchmove", listeners.tm.as_ref().unchecked_ref());
                    let _ = document.remove_event_listener_with_callback("touchend", listeners.tu.as_ref().unchecked_ref());
                    let _ = window_target.remove_event_listener_with_callback("mousemove", listeners.mm.as_ref().unchecked_ref());
                    let _ = window_target.remove_event_listener_with_callback("mouseup", listeners.mu.as_ref().unchecked_ref());
                    let _ = window_target.remove_event_listener_with_callback("touchmove", listeners.tm.as_ref().unchecked_ref());
                    let _ = window_target.remove_event_listener_with_callback("touchend", listeners.tu.as_ref().unchecked_ref());
                }
            }) as Box<dyn FnMut(web_sys::TouchEvent)>)
        };

        let mm_ref = mm_closure.as_ref().clone();
        let mu_ref = mu_closure.as_ref().clone();
        let tm_ref = tm_closure.as_ref().clone();
        let tu_ref = tu_closure.as_ref().clone();

        let _ = document.add_event_listener_with_callback("mousemove", mm_ref.unchecked_ref());
        let _ = document.add_event_listener_with_callback("mouseup", mu_ref.unchecked_ref());
        let _ = document.add_event_listener_with_callback("touchmove", tm_ref.unchecked_ref());
        let _ = document.add_event_listener_with_callback("touchend", tu_ref.unchecked_ref());
        let _ = window_target.add_event_listener_with_callback("mousemove", mm_closure.as_ref().unchecked_ref());
        let _ = window_target.add_event_listener_with_callback("mouseup", mu_closure.as_ref().unchecked_ref());
        let _ = window_target.add_event_listener_with_callback("touchmove", tm_closure.as_ref().unchecked_ref());
        let _ = window_target.add_event_listener_with_callback("touchend", tu_closure.as_ref().unchecked_ref());

        // 存储监听器句柄以便后续清理
        *drag_listeners().borrow_mut() = Some(DragListeners {
            mm: mm_closure,
            mu: mu_closure,
            tm: tm_closure,
            tu: tu_closure,
        });
    }
}

/// Slider 滑块组件
#[allow(non_snake_case)]
pub fn Slider(props: SliderProps) -> Element {
    const CSS: &str = include_str!("../../assets/slider.css");
    let mut value = use_signal(|| props.value);
    let is_dragging = use_signal(|| false);

    // 同步外部 prop 更新到内部信号
    use_effect(use_reactive(&props.value, move |v| {
        value.set(v);
    }));

    // 生成唯一 ID 用于定位 rail 元素
    let rail_id = use_signal(|| {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        format!("ctrl-slider-rail-{}", COUNTER.fetch_add(1, Ordering::Relaxed))
    });

    let percent = use_memo(move || {
        let range = props.max - props.min;
        if range == 0 {
            return 0f64;
        }
        ((value() - props.min) as f64 / range as f64 * 100.0).clamp(0.0, 100.0)
    });

    let slider_class = {
        let mut c = String::from("ctrl-slider");
        if props.disabled {
            c.push_str(" ctrl-slider--disabled");
        }
        if props.vertical {
            c.push_str(" ctrl-slider--vertical");
        }
        if is_dragging() {
            c.push_str(" ctrl-slider--dragging");
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    #[cfg(target_arch = "wasm32")]
    let drag_listeners: Signal<Rc<RefCell<Option<DragListeners>>>> = use_signal(|| Rc::new(RefCell::new(None)));

    let on_rail_mousedown = {
        let value = value;
        let is_dragging = is_dragging;
        let rail_id = rail_id;
        let min = props.min;
        let max = props.max;
        let step = props.step;
        let vertical = props.vertical;
        let onchange = props.onchange.clone();
        #[cfg(target_arch = "wasm32")]
        let drag_listeners = drag_listeners;

        move |evt: MouseEvent| {
            if props.disabled {
                return;
            }
            evt.prevent_default();
            let client = evt.data().client_coordinates();
            #[cfg(target_arch = "wasm32")]
            {
                begin_drag(
                    value, is_dragging, rail_id, min, max, step, vertical,
                    onchange.clone(), drag_listeners,
                    client.x, client.y,
                );
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (value, is_dragging, rail_id, min, max, step, vertical, onchange, client);
            }
        }
    };

    let on_rail_touchstart = {
        let value = value;
        let is_dragging = is_dragging;
        let rail_id = rail_id;
        let min = props.min;
        let max = props.max;
        let step = props.step;
        let vertical = props.vertical;
        let onchange = props.onchange.clone();
        #[cfg(target_arch = "wasm32")]
        let drag_listeners = drag_listeners;

        move |evt: TouchEvent| {
            if props.disabled {
                return;
            }
            evt.prevent_default();
            if let Some(touch) = evt.data().touches().first() {
                let coords = touch.client_coordinates();
                #[cfg(target_arch = "wasm32")]
                {
                    begin_drag(
                        value, is_dragging, rail_id, min, max, step, vertical,
                        onchange.clone(), drag_listeners,
                        coords.x, coords.y,
                    );
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = (value, is_dragging, rail_id, min, max, step, vertical, onchange, coords);
                }
            }
        }
    };

    // 生成刻度标记
    let marks_ui = if props.marks {
        let marks_count = (props.max - props.min) / props.step + 1;
        let mut marks = Vec::new();
        for i in 0..marks_count.min(20) {
            let mark_val = props.min + i * props.step;
            let mark_pct = if props.max != props.min {
                (mark_val - props.min) as f64 / (props.max - props.min) as f64 * 100.0
            } else {
                0.0
            };
            let is_active = mark_val <= value();
            let mark_class = if is_active {
                "ctrl-slider__mark ctrl-slider__mark--active"
            } else {
                "ctrl-slider__mark"
            };
            marks.push(rsx! {
                span {
                    key: "{mark_val}",
                    class: "{mark_class}",
                    style: "left: {mark_pct}%;",
                    span { class: "ctrl-slider__mark-dot" }
                    "{mark_val}"
                }
            });
        }
        rsx! { div { class: "ctrl-slider__marks", {marks.into_iter()} } }
    } else {
        rsx! {}
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{slider_class}",
            style: "{props.style}",
            if props.show_label && !props.vertical {
                span { class: "ctrl-slider__label", "{props.min}" }
            }
            div {
                id: "{rail_id}",
                class: "ctrl-slider__rail",
                onmousedown: on_rail_mousedown,
                ontouchstart: on_rail_touchstart,
                div {
                    class: "ctrl-slider__track",
                    id: "{rail_id}-track",
                    style: if props.vertical {
                        "height: {percent}%;"
                    } else {
                        "width: {percent}%;"
                    },
                }
                div {
                    class: "ctrl-slider__handle",
                    id: "{rail_id}-handle",
                    style: if props.vertical {
                        "bottom: {percent}%;"
                    } else {
                        "left: {percent}%;"
                    },
                    if props.show_label {
                        span { class: "ctrl-slider__tooltip", "{value()}" }
                    }
                }
            }
            if props.show_label && !props.vertical {
                span { class: "ctrl-slider__label", "{props.max}" }
            }
        }
        {marks_ui}
    }
}