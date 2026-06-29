use dioxus::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::atomic::{AtomicU16, Ordering};
use wasm_bindgen::JsCast;
use crate::overlay::{self, OverlayClosures};

static DP_ID: AtomicU16 = AtomicU16::new(1);

/// DatePicker 日期选择器组件属性
#[derive(Props, PartialEq, Clone)]
pub struct DatePickerProps {
    #[props(default = "".to_string())]
    pub value: String,
    #[props(default = "请选择日期".to_string())]
    pub placeholder: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = true)]
    pub clearable: bool,
    #[props(default = "YYYY-MM-DD".to_string())]
    pub format: String,
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = "".to_string())]
    pub style: String,
    pub onchange: Option<EventHandler<String>>,
}

const MONTH_NAMES: [&str; 12] = ["一月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "十二月"];
const WEEKDAY_NAMES: [&str; 7] = ["一", "二", "三", "四", "五", "六", "日"];

fn days_in_month(year: i32, month: u32) -> u32 {
    match month { 1|3|5|7|8|10|12 => 31, 4|6|9|11 => 30, 2 => if (year%4==0 && year%100!=0) || year%400==0 {29} else {28}, _ => 30 }
}

fn first_day_of_week(year: i32, month: u32) -> u32 {
    let m = if month <= 2 { month as i32 + 12 } else { month as i32 };
    let y = if month <= 2 { year - 1 } else { year };
    let h = (1 + (13*(m+1))/5 + y + y/4 - y/100 + y/400) % 7;
    let h = ((h%7)+7)%7;
    (h+5) as u32 % 7
}

fn parse_date_value(value: &str) -> (i32, u32) {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() == 3 {
        if let (Ok(y), Ok(m)) = (parts[0].parse::<i32>(), parts[1].parse::<u32>()) {
            if m >= 1 && m <= 12 { return (y, m); }
        }
    }
    let today = get_today();
    (today.0, today.1)
}

fn get_today() -> (i32, u32, u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let date = web_sys::js_sys::Date::new_0();
        (date.get_full_year() as i32, date.get_month() as u32 + 1, date.get_date() as u32)
    }
    #[cfg(not(target_arch = "wasm32"))]
    { (2024, 1, 1) }
}

#[allow(non_snake_case)]
pub fn DatePicker(props: DatePickerProps) -> Element {
    const CSS: &str = include_str!("../../assets/date-picker.css");
    let pid = DP_ID.fetch_add(1, Ordering::Relaxed);
    let panel_id = format!("ctrl-date-picker-panel-{}", pid);
    let panel_visible = use_signal(|| false);
    let mut inner_value = use_signal(|| props.value.clone());
    let parsed = parse_date_value(&inner_value());
    let mut current_year = use_signal(|| parsed.0);
    let mut current_month = use_signal(|| parsed.1);

    use_effect(use_reactive(&props.value, move |v| {
        inner_value.set(v.clone());
        let (y, m) = parse_date_value(&v);
        current_year.set(y);
        current_month.set(m);
    }));

    let picker_class = {
        let mut c = "ctrl-date-picker".to_string();
        if !props.class.is_empty() { c = format!("{} {}", c, props.class); }
        c
    };

    let input_class = {
        let mut c = "ctrl-date-picker__input".to_string();
        if props.disabled { c.push_str(" ctrl-date-picker__input--disabled"); }
        if panel_visible() { c.push_str(" ctrl-date-picker__input--focus"); }
        c
    };

    let toggle_panel = {
        let mut visible = panel_visible.clone();
        move |_| { if !props.disabled { visible.set(!visible()); } }
    };

    let clear_value = {
        let mut iv = inner_value.clone();
        let onchange = props.onchange.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            iv.set(String::new());
            if let Some(ref cb) = onchange { cb.call(String::new()); }
        }
    };

    let select_date = {
        let mut iv = inner_value.clone();
        let mut visible = panel_visible.clone();
        let onchange = props.onchange.clone();
        move |day: u32| {
            let date_str = format!("{:04}-{:02}-{:02}", current_year(), current_month(), day);
            iv.set(date_str.clone());
            if let Some(ref cb) = onchange { cb.call(date_str); }
            visible.set(false);
        }
    };

    let prev_month = {
        let mut year = current_year.clone();
        let mut month = current_month.clone();
        move |_| { if month() == 1 { month.set(12); year.set(year()-1); } else { month.set(month()-1); } }
    };

    let next_month = {
        let mut year = current_year.clone();
        let mut month = current_month.clone();
        move |_| { if month() == 12 { month.set(1); year.set(year()+1); } else { month.set(month()+1); } }
    };

    let go_today = {
        let mut iv = inner_value.clone();
        let mut year = current_year.clone();
        let mut month = current_month.clone();
        let mut visible = panel_visible.clone();
        let onchange = props.onchange.clone();
        move |_| {
            let today = get_today();
            year.set(today.0); month.set(today.1);
            let date_str = format!("{:04}-{:02}-{:02}", today.0, today.1, today.2);
            iv.set(date_str.clone());
            if let Some(ref cb) = onchange { cb.call(date_str); }
            visible.set(false);
        }
    };

    let calendar = {
        let year = current_year();
        let month = current_month();
        let first_wday = first_day_of_week(year, month);
        let total_days = days_in_month(year, month);
        let prev_month_days = days_in_month(if month==1 {year-1} else {year}, if month==1 {12} else {month-1});
        let mut cells = Vec::new();
        let start_offset = first_wday;
        for i in 0..start_offset {
            let day = prev_month_days - start_offset + i + 1;
            cells.push(rsx! { button { key: "prev-{i}", class: "ctrl-date-picker__cell ctrl-date-picker__cell--other-month", disabled: true, "{day}" }});
        }
        let today = get_today();
        let is_current_month = today.0 == year && today.1 == month;
        for day in 1..=total_days {
            let d = day;
            let is_today = is_current_month && today.2 == d;
            let is_selected = inner_value() == format!("{:04}-{:02}-{:02}", year, month, d);
            let mut cls = "ctrl-date-picker__cell".to_string();
            if is_selected { cls.push_str(" ctrl-date-picker__cell--selected"); }
            if is_today && !is_selected { cls.push_str(" ctrl-date-picker__cell--today"); }
            let mut select = select_date.clone();
            cells.push(rsx! { button { key: "day-{d}", class: "{cls}", onclick: move |_| select(d), "{d}" }});
        }
        let remaining = 42 - cells.len();
        for i in 0..remaining {
            let day = i as u32 + 1;
            cells.push(rsx! { button { key: "next-{i}", class: "ctrl-date-picker__cell ctrl-date-picker__cell--other-month", disabled: true, "{day}" }});
        }
        cells
    };

    // ── 点击外部关闭（使用 overlay 工具：mousedown/mouseup）──
    let listeners = use_signal(|| Rc::new(RefCell::new(OverlayClosures::new())));
    {
        let store = listeners.clone();
        let pv = panel_visible.clone();
        use_effect(move || {
            if pv() {
                overlay::setup_click_outside_custom(&store.read(),
                    move |el: &web_sys::Element| {
                        let mut current: Option<web_sys::Element> = Some(el.clone());
                        while let Some(e) = current {
                            if let Some(html_el) = e.dyn_ref::<web_sys::HtmlElement>() {
                                if html_el.class_name().contains("ctrl-date-picker") {
                                    return true;
                                }
                            }
                            current = e.parent_element();
                        }
                        false
                    },
                    pv.clone());
            } else {
                store.read().borrow_mut().cleanup();
            }
        });
    }
    use_drop(move || { listeners.read().borrow_mut().cleanup(); });

    // ── 控制 panel visibility ──
    overlay::use_visibility_effect(&panel_id, panel_visible.clone());

    rsx! {
        style { {CSS} }
        div { class: "{picker_class}", style: "{props.style}",
            div { class: "{input_class}", onclick: toggle_panel,
                if inner_value().is_empty() {
                    span { class: "ctrl-date-picker__placeholder", "{props.placeholder}" }
                } else {
                    span { class: "ctrl-date-picker__value", "{inner_value()}" }
                }
                if props.clearable && !inner_value().is_empty() {
                    span { class: "ctrl-date-picker__clear", onclick: clear_value, "✕" }
                }
                span { class: "ctrl-date-picker__icon", "📅" }
            }
            div {
                id: "{panel_id}",
                class: "ctrl-date-picker__panel",
                div { class: "ctrl-date-picker__header",
                    button { class: "ctrl-date-picker__nav", onclick: prev_month, "‹" }
                    span { class: "ctrl-date-picker__month-year", "{current_year()}年 {MONTH_NAMES[current_month() as usize - 1]}" }
                    button { class: "ctrl-date-picker__nav", onclick: next_month, "›" }
                }
                div { class: "ctrl-date-picker__weekdays",
                    for name in WEEKDAY_NAMES.iter() {
                        span { key: "{name}", class: "ctrl-date-picker__weekday", "{name}" }
                    }
                }
                div { class: "ctrl-date-picker__days", {calendar.iter()} }
                div { class: "ctrl-date-picker__footer",
                    button { class: "ctrl-date-picker__today-btn", onclick: go_today, "今天" }
                }
            }
        }
    }
}
