use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::overlay::{self, OverlayClosures};

static TP_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// TimePicker 时间选择器组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TimePickerProps {
    /// 当前值，格式 "HH:MM:SS" 或 "HH:MM"
    #[props(default = "".to_string())]
    pub value: String,

    /// 占位文本
    #[props(default = "请选择时间".to_string())]
    pub placeholder: String,

    /// 格式：HH:mm:ss / HH:mm
    #[props(default = "HH:mm:ss".to_string())]
    pub format: String,

    /// 是否显示秒
    #[props(default = true)]
    pub show_second: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 尺寸：sm / md / lg
    #[props(default = "md".to_string())]
    pub size: String,

    /// 小时步长
    #[props(default = 1)]
    pub hour_step: u32,

    /// 分钟步长
    #[props(default = 1)]
    pub minute_step: u32,

    /// 秒步长
    #[props(default = 1)]
    pub second_step: u32,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 值变化回调
    pub onchange: Option<EventHandler<String>>,
}

/// TimePicker 时间选择器组件
#[allow(non_snake_case)]
pub fn TimePicker(props: TimePickerProps) -> Element {
    const CSS: &str = include_str!("../../assets/time_picker.css");

    let mut inner_value = use_signal(|| props.value.clone());
    let mut open = use_signal(|| false);

    let tp_id = use_signal(|| {
        let id = TP_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("tp-{}", id)
    });

    let mut hour = use_signal(|| 0u32);
    let mut minute = use_signal(|| 0u32);
    let mut second = use_signal(|| 0u32);

    // 从 inner_value 解析时/分/秒
    let parse_time = {
        let iv = inner_value;
        move || {
            let v = iv();
            let parts: Vec<&str> = v.split(':').collect();
            let h: u32 = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
            let m: u32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            let s: u32 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            (h.min(23), m.min(59), s.min(59))
        }
    };

    // 提交时间值
    let mut commit_time = {
        let mut inner_value = inner_value;
        let show_second = props.show_second;
        let onchange = props.onchange.clone();
        move |h: u32, m: u32, s: u32| {
            let formatted = if show_second {
                format!("{:02}:{:02}:{:02}", h, m, s)
            } else {
                format!("{:02}:{:02}", h, m)
            };
            inner_value.set(formatted.clone());
            if let Some(ref cb) = onchange {
                cb.call(formatted);
            }
        }
    };

    // 打开面板时同步信号
    let mut sync_to_signals = {
        let parse_time = parse_time.clone();
        move || {
            let (h, m, s) = parse_time();
            hour.set(h);
            minute.set(m);
            second.set(s);
        }
    };

    // 显示用的格式化值
    let display_value = {
        let inner_value = inner_value;
        let show_second = props.show_second;
        move || {
            let v = inner_value();
            if v.is_empty() {
                String::new()
            } else if show_second {
                v
            } else {
                let parts: Vec<&str> = v.split(':').collect();
                if parts.len() >= 2 {
                    format!("{}:{}", parts[0], parts[1])
                } else {
                    v
                }
            }
        }
    };

    let hours: Vec<u32> = (0..24u32).step_by(props.hour_step as usize).collect();
    let minutes: Vec<u32> = (0..60u32).step_by(props.minute_step as usize).collect();
    let seconds: Vec<u32> = (0..60u32).step_by(props.second_step as usize).collect();

    let size_class = match props.size.as_str() {
        "sm" => "ctrl-time-picker--sm",
        "lg" => "ctrl-time-picker--lg",
        _ => "ctrl-time-picker--md",
    };

    let mut classes = vec!["ctrl-time-picker".to_string(), size_class.to_string()];
    if props.disabled {
        classes.push("ctrl-time-picker--disabled".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let class_str = classes.join(" ");

    let panel_id = format!("{}-panel", tp_id());

    // ── fixed 弹层：不受 overflow:hidden 裁切 + document capture scroll 跟随 ──
    overlay::use_fixed_panel_effect(&panel_id, &tp_id(), open.clone(), 4.0, false);

    // ── 点击外部关闭 ──
    let listeners = use_signal(|| Rc::new(RefCell::new(OverlayClosures::new())));
    {
        let store = listeners.clone();
        let tid = tp_id();
        let pid = panel_id.clone();
        let v = open.clone();
        use_effect(move || {
            if v() {
                overlay::setup_click_outside(&store.read(), &tid, &pid, v.clone());
            } else {
                store.read().borrow_mut().cleanup();
            }
        });
    }

    // ── 滚动选中项到可视范围的辅助函数 ──
    let scroll_to_active = {
        let pid = panel_id.clone();
        move || {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    if let Some(panel) = doc.get_element_by_id(&pid) {
                        if let Ok(active_list) = panel.query_selector_all(".ctrl-time-picker__item--active") {
                            let len = active_list.length();
                            for i in 0..len {
                                if let Some(el) = active_list.get(i) {
                                    let elem: web_sys::Element = el.unchecked_into();
                                    let mut opts = web_sys::ScrollIntoViewOptions::new();
                                    opts.set_block(web_sys::ScrollLogicalPosition::Center);
                                    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                                    elem.scroll_into_view_with_scroll_into_view_options(&opts);
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    // ── 打开面板时滚动到选中项 ──
    {
        let v = open.clone();
        let scroll = scroll_to_active.clone();
        use_effect(move || {
            if v() {
                scroll();
            }
        });
    }
    use_drop(move || { listeners.read().borrow_mut().cleanup(); });

    rsx! {
        style { {CSS} }
        div {
            class: "{class_str}",
            style: "{props.style}",
            id: tp_id(),
            div {
                class: "ctrl-time-picker__input-wrap",
                onclick: move |evt| {
                    evt.stop_propagation();
                    if !props.disabled {
                        sync_to_signals();
                        open.set(!open());
                    }
                },
                input {
                    class: "ctrl-time-picker__input",
                    placeholder: "{props.placeholder}",
                    value: display_value(),
                    readonly: true,
                    disabled: props.disabled,
                }
                span { class: "ctrl-time-picker__icon",
                    dangerous_inner_html: r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="2"/><path d="M12 7v5l3 3" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>"#,
                }
            }
            div { id: "{panel_id}", class: "ctrl-time-picker__panel",
                div { class: "ctrl-time-picker__columns",
                        div { class: "ctrl-time-picker__column",
                            div { class: "ctrl-time-picker__column-title", "时" }
                            div { class: "ctrl-time-picker__list",
                                for h_ in &hours {
                                    div {
                                        class: if *h_ == hour() { "ctrl-time-picker__item ctrl-time-picker__item--active" } else { "ctrl-time-picker__item" },
                                        onclick: {
                                            let h = *h_;
                                            let mut parse = parse_time.clone();
                                            let mut commit = commit_time.clone();
                                            move |_| {
                                                let (_, m, s) = parse();
                                                hour.set(h);
                                                commit(h, m, s);
                                            }
                                        },
                                        "{h_:02}"
                                    }
                                }
                            }
                        }
                        div { class: "ctrl-time-picker__column",
                            div { class: "ctrl-time-picker__column-title", "分" }
                            div { class: "ctrl-time-picker__list",
                                for m_ in &minutes {
                                    div {
                                        class: if *m_ == minute() { "ctrl-time-picker__item ctrl-time-picker__item--active" } else { "ctrl-time-picker__item" },
                                        onclick: {
                                            let m = *m_;
                                            let mut parse = parse_time.clone();
                                            let mut commit = commit_time.clone();
                                            move |_| {
                                                let (h, _, s) = parse();
                                                minute.set(m);
                                                commit(h, m, s);
                                            }
                                        },
                                        "{m_:02}"
                                    }
                                }
                            }
                        }
                        if props.show_second {
                            div { class: "ctrl-time-picker__column",
                                div { class: "ctrl-time-picker__column-title", "秒" }
                                div { class: "ctrl-time-picker__list",
                                    for s_ in &seconds {
                                        div {
                                            class: if *s_ == second() { "ctrl-time-picker__item ctrl-time-picker__item--active" } else { "ctrl-time-picker__item" },
                                            onclick: {
                                                let s = *s_;
                                                let mut parse = parse_time.clone();
                                                let mut commit = commit_time.clone();
                                                move |_| {
                                                    let (h, m, _) = parse();
                                                    second.set(s);
                                                    commit(h, m, s);
                                                }
                                            },
                                            "{s_:02}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "ctrl-time-picker__footer",
                        {
                            let has_step = props.hour_step != 1 || props.minute_step != 1 || (props.show_second && props.second_step != 1);
                            rsx! {
                                button {
                                    class: "ctrl-time-picker__btn",
                                    disabled: has_step,
                                    title: if has_step { "步长模式下不可用" } else { "此刻" },
                                    onclick: {
                                        let mut commit = commit_time.clone();
                                        let scroll = scroll_to_active.clone();
                                        move |_| {
                                            let now = js_sys::Date::new_0();
                                            let h = now.get_hours();
                                            let m = now.get_minutes();
                                            let s = now.get_seconds();
                                            hour.set(h);
                                            minute.set(m);
                                            second.set(s);
                                            commit(h, m, s);
                                            let scroll = scroll.clone();
                                            gloo_timers::callback::Timeout::new(0, move || {
                                                scroll();
                                            }).forget();
                                        }
                                    },
                                    "此刻"
                                }
                            }
                        }
                        button {
                            class: "ctrl-time-picker__btn ctrl-time-picker__btn--primary",
                            onclick: move |_| open.set(false),
                            "确定"
                        }
                    }
            }
        }
    }
}
