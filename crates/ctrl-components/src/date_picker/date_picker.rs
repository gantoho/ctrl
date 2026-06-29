use dioxus::prelude::*;

/// DatePicker 日期选择器组件属性
#[derive(Props, PartialEq, Clone)]
pub struct DatePickerProps {
    /// 选中的日期（格式 YYYY-MM-DD）
    #[props(default = "".to_string())]
    pub value: String,

    /// 占位文本
    #[props(default = "请选择日期".to_string())]
    pub placeholder: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否可清除
    #[props(default = true)]
    pub clearable: bool,

    /// 日期格式
    #[props(default = "YYYY-MM-DD".to_string())]
    pub format: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 值变化回调
    pub onchange: Option<EventHandler<String>>,
}

/// 月份名称
const MONTH_NAMES: [&str; 12] = [
    "一月", "二月", "三月", "四月", "五月", "六月",
    "七月", "八月", "九月", "十月", "十一月", "十二月",
];

/// 星期名称
const WEEKDAY_NAMES: [&str; 7] = ["一", "二", "三", "四", "五", "六", "日"];

/// 获取某月的天数
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// 获取某月第一天是星期几 (0=周一, 6=周日)
fn first_day_of_week(year: i32, month: u32) -> u32 {
    let m = if month <= 2 { month as i32 + 12 } else { month as i32 };
    let y = if month <= 2 { year - 1 } else { year };
    let h = (1 + (13 * (m + 1)) / 5 + y + y / 4 - y / 100 + y / 400) % 7;
    let h = ((h % 7) + 7) % 7;
    (h + 5) as u32 % 7
}

/// 从 value 字符串解析年月，无效时回退到今天
fn parse_date_value(value: &str) -> (i32, u32) {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() == 3 {
        if let (Ok(y), Ok(m)) = (parts[0].parse::<i32>(), parts[1].parse::<u32>()) {
            if m >= 1 && m <= 12 {
                return (y, m);
            }
        }
    }
    let today = get_today();
    (today.0, today.1)
}

/// DatePicker 日期选择器组件
#[allow(non_snake_case)]
pub fn DatePicker(props: DatePickerProps) -> Element {
    const CSS: &str = include_str!("../../assets/date-picker.css");
    let panel_visible = use_signal(|| false);
    let parsed = parse_date_value(&props.value);
    let mut current_year = use_signal(|| parsed.0);
    let mut current_month = use_signal(|| parsed.1);

    let picker_class = {
        let mut c = String::from("ctrl-date-picker");
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    let input_class = {
        let mut c = String::from("ctrl-date-picker__input");
        if props.disabled {
            c.push_str(" ctrl-date-picker__input--disabled");
        }
        if panel_visible() {
            c.push_str(" ctrl-date-picker__input--focus");
        }
        c
    };

    let toggle_panel = {
        let mut visible = panel_visible.clone();
        move |_| {
            if !props.disabled {
                visible.set(!visible());
            }
        }
    };

    let clear_value = {
        let onchange = props.onchange.clone();
        move |_| {
            if let Some(ref cb) = onchange {
                cb.call(String::new());
            }
        }
    };

    let select_date = {
        let mut visible = panel_visible.clone();
        let onchange = props.onchange.clone();
        move |day: u32| {
            if let Some(ref cb) = onchange {
                let date_str = format!(
                    "{:04}-{:02}-{:02}",
                    current_year(),
                    current_month(),
                    day
                );
                cb.call(date_str);
            }
            visible.set(false);
        }
    };

    let prev_month = {
        let mut year = current_year.clone();
        let mut month = current_month.clone();
        move |_| {
            if month() == 1 {
                month.set(12);
                year.set(year() - 1);
            } else {
                month.set(month() - 1);
            }
        }
    };

    let next_month = {
        let mut year = current_year.clone();
        let mut month = current_month.clone();
        move |_| {
            if month() == 12 {
                month.set(1);
                year.set(year() + 1);
            } else {
                month.set(month() + 1);
            }
        }
    };

    let go_today = {
        let mut visible = panel_visible.clone();
        let onchange = props.onchange.clone();
        move |_| {
            if let Some(ref cb) = onchange {
                let today = get_today();
                current_year.set(today.0);
                current_month.set(today.1);
                cb.call(format!("{:04}-{:02}-{:02}", today.0, today.1, today.2));
            }
            visible.set(false);
        }
    };

    // 构建日历网格
    let calendar = {
        let year = current_year();
        let month = current_month();
        let first_wday = first_day_of_week(year, month);
        let total_days = days_in_month(year, month);
        let prev_month_days = days_in_month(
            if month == 1 { year - 1 } else { year },
            if month == 1 { 12 } else { month - 1 },
        );

        let mut cells = Vec::new();

        // 上月末尾的日期
        let start_offset = first_wday;
        for i in 0..start_offset {
            let day = prev_month_days - start_offset + i + 1;
            cells.push(rsx! {
                button {
                    key: "prev-{i}",
                    class: "ctrl-date-picker__cell ctrl-date-picker__cell--other-month",
                    disabled: true,
                    "{day}"
                }
            });
        }

        // 本月日期
        let today = get_today();
        let is_current_month = today.0 == year && today.1 == month;

        for day in 1..=total_days {
            let d = day;
            let is_today = is_current_month && today.2 == d;
            let is_selected = props.value == format!("{:04}-{:02}-{:02}", year, month, d);

            let mut cls = String::from("ctrl-date-picker__cell");
            if is_selected {
                cls.push_str(" ctrl-date-picker__cell--selected");
            }
            if is_today && !is_selected {
                cls.push_str(" ctrl-date-picker__cell--today");
            }

            let mut select = select_date.clone();
            cells.push(rsx! {
                button {
                    key: "day-{d}",
                    class: "{cls}",
                    onclick: move |_| select(d),
                    "{d}"
                }
            });
        }

        // 下月开头的日期（补齐 7×6=42 格）
        let remaining = 42 - cells.len();
        for i in 0..remaining {
            let day = i as u32 + 1;
            cells.push(rsx! {
                button {
                    key: "next-{i}",
                    class: "ctrl-date-picker__cell ctrl-date-picker__cell--other-month",
                    disabled: true,
                    "{day}"
                }
            });
        }

        cells
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{picker_class}",
            style: "{props.style}",
            div {
                class: "{input_class}",
                onclick: toggle_panel,
                if props.value.is_empty() {
                    span {
                        class: "ctrl-date-picker__placeholder",
                        "{props.placeholder}"
                    }
                } else {
                    span {
                        class: "ctrl-date-picker__value",
                        "{props.value}"
                    }
                }
                if props.clearable && !props.value.is_empty() {
                    span {
                        class: "ctrl-date-picker__clear",
                        onclick: clear_value,
                        "✕"
                    }
                }
                span { class: "ctrl-date-picker__icon", "📅" }
            }
            if panel_visible() {
                div {
                    class: "ctrl-date-picker__panel",
                    div { class: "ctrl-date-picker__header",
                        button {
                            class: "ctrl-date-picker__nav",
                            onclick: prev_month,
                            "‹"
                        }
                        span {
                            class: "ctrl-date-picker__month-year",
                            "{current_year()}年 {MONTH_NAMES[current_month() as usize - 1]}"
                        }
                        button {
                            class: "ctrl-date-picker__nav",
                            onclick: next_month,
                            "›"
                        }
                    }
                    div { class: "ctrl-date-picker__weekdays",
                        for name in WEEKDAY_NAMES.iter() {
                            span { key: "{name}", class: "ctrl-date-picker__weekday", "{name}" }
                        }
                    }
                    div { class: "ctrl-date-picker__days",
                        {calendar.into_iter()}
                    }
                    div { class: "ctrl-date-picker__footer",
                        button {
                            class: "ctrl-date-picker__today-btn",
                            onclick: go_today,
                            "今天"
                        }
                    }
                }
            }
        }
    }
}

/// 获取今天的日期 (year, month, day)
fn get_today() -> (i32, u32, u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let date = web_sys::js_sys::Date::new_0();
        (
            date.get_full_year() as i32,
            date.get_month() as u32 + 1,
            date.get_date() as u32,
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        (2024, 1, 1)
    }
}