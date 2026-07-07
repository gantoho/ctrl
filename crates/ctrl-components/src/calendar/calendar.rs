use dioxus::prelude::*;

const MONTH_NAMES: [&str; 12] = [
    "一月", "二月", "三月", "四月", "五月", "六月",
    "七月", "八月", "九月", "十月", "十一月", "十二月",
];
const WEEKDAY_NAMES: [&str; 7] = ["一", "二", "三", "四", "五", "六", "日"];

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Zeller's congruence: returns the day of week (0=Mon, 6=Sun)
fn first_day_of_week(year: i32, month: u32) -> u32 {
    let m = if month <= 2 {
        month as i32 + 12
    } else {
        month as i32
    };
    let y = if month <= 2 { year - 1 } else { year };
    let h = (1 + (13 * (m + 1)) / 5 + y + y / 4 - y / 100 + y / 400) % 7;
    // Convert: Zeller's 0=Sat,1=Sun,...,6=Fri → we want 0=Mon,...,6=Sun
    ((h + 5) % 7) as u32
}

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
        // Non-WASM fallback for testing / SSR
        (2026, 7, 7)
    }
}

fn parse_date(s: &str) -> Option<(i32, u32, u32)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let year = parts[0].parse::<i32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;
    let day = parts[2].parse::<u32>().ok()?;
    if month < 1 || month > 12 || day < 1 || day > days_in_month(year, month) {
        return None;
    }
    Some((year, month, day))
}

/// Calendar 日历组件属性
#[derive(Props, PartialEq, Clone)]
pub struct CalendarProps {
    /// 选中的日期，格式 "YYYY-MM-DD"
    #[props(default = "".to_string())]
    pub value: String,

    /// 日期变化回调，返回 "YYYY-MM-DD" 格式字符串
    pub onchange: Option<EventHandler<String>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// Calendar 日历组件
///
/// 月历视图，支持上一月/下一月切换、日期点击选中。
#[allow(non_snake_case)]
pub fn Calendar(props: CalendarProps) -> Element {
    const CSS: &str = include_str!("../../assets/calendar.css");
    let mut inner_value = use_signal(|| props.value.clone());
    let mut year = use_signal(|| {
        if let Some((y, _, _)) = parse_date(&props.value) {
            y
        } else {
            get_today().0
        }
    });
    let mut month = use_signal(|| {
        if let Some((_, m, _)) = parse_date(&props.value) {
            m
        } else {
            get_today().1
        }
    });

    let wrapper_class = if props.class.is_empty() {
        "ctrl-calendar".to_string()
    } else {
        format!("ctrl-calendar {}", props.class)
    };

    let prev_year = move |_| {
        year.set(year() - 1);
    };

    let next_year = move |_| {
        year.set(year() + 1);
    };

    // 点击标题回到当天所在月份（仅跳转视图，不选中、不触发 onchange）
    let go_today = move |_| {
        let today = get_today();
        year.set(today.0);
        month.set(today.1);
    };

    let prev_month = move |_| {
        if month() == 1 {
            month.set(12);
            year.set(year() - 1);
        } else {
            month.set(month() - 1);
        }
    };

    let next_month = move |_| {
        if month() == 12 {
            month.set(1);
            year.set(year() + 1);
        } else {
            month.set(month() + 1);
        }
    };

    let select_date = move |day: u32| {
        let date_str = format!("{:04}-{:02}-{:02}", year(), month(), day);
        inner_value.set(date_str.clone());
        if let Some(ref cb) = props.onchange {
            cb.call(date_str);
        }
    };

    // 构建日期网格
    let cells = {
        let y = year();
        let m = month();
        let first_wday = first_day_of_week(y, m);
        let total_days = days_in_month(y, m);
        let prev_month_days = days_in_month(if m == 1 { y - 1 } else { y }, if m == 1 { 12 } else { m - 1 });

        let today = get_today();
        let is_current_month = today.0 == y && today.1 == m;
        let mut cells = Vec::new();

        // 上月填充
        for i in 0..first_wday {
            let day = prev_month_days - first_wday + i + 1;
            cells.push(rsx! {
                button {
                    key: "prev-{i}",
                    class: "ctrl-calendar__cell ctrl-calendar__cell--disabled",
                    disabled: true,
                    "{day}"
                }
            });
        }

        // 当月日期
        for d in 1..=total_days {
            let is_today = is_current_month && today.2 == d;
            let date_str = format!("{:04}-{:02}-{:02}", y, m, d);
            let is_selected = inner_value() == date_str;

            let mut cls = "ctrl-calendar__cell".to_string();
            if is_selected {
                cls.push_str(" ctrl-calendar__cell--selected");
            } else if is_today {
                cls.push_str(" ctrl-calendar__cell--today");
            }

            let day = d;
            let mut select = select_date.clone();
            cells.push(rsx! {
                button {
                    key: "day-{d}",
                    class: "{cls}",
                    onclick: move |_| select(day),
                    "{day}"
                }
            });
        }

        // 下月填充（补齐到 42 格）
        let remaining = 42 - cells.len();
        for i in 0..remaining {
            let day = i as u32 + 1;
            cells.push(rsx! {
                button {
                    key: "next-{i}",
                    class: "ctrl-calendar__cell ctrl-calendar__cell--disabled",
                    disabled: true,
                    "{day}"
                }
            });
        }
        cells
    };

    rsx! {
        style { {CSS} }
        div { class: "{wrapper_class}",
            // 头部：年 / 月切换
            div { class: "ctrl-calendar__header",
                button {
                    class: "ctrl-calendar__nav ctrl-calendar__nav--year",
                    onclick: prev_year,
                    "«"
                }
                button {
                    class: "ctrl-calendar__nav",
                    onclick: prev_month,
                    "<"
                }
                span {
                    class: "ctrl-calendar__month",
                    title: "回到今天",
                    onclick: go_today,
                    "{year()} 年 {MONTH_NAMES[month() as usize - 1]}"
                }
                button {
                    class: "ctrl-calendar__nav",
                    onclick: next_month,
                    ">"
                }
                button {
                    class: "ctrl-calendar__nav ctrl-calendar__nav--year",
                    onclick: next_year,
                    "»"
                }
            }

            // 星期表头
            div { class: "ctrl-calendar__weekdays",
                for name in WEEKDAY_NAMES {
                    span { key: "{name}", class: "ctrl-calendar__weekday", "{name}" }
                }
            }

            // 日期网格
            div { class: "ctrl-calendar__body",
                {cells.into_iter()}
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════
// CalendarConfig — 命令式 API 配置
// ═══════════════════════════════════════════════════════════

/// Calendar 命令式 API 配置
#[derive(Clone)]
pub struct CalendarConfig {
    /// 初始选中日期，格式 "YYYY-MM-DD"
    pub value: String,
    /// 弹层标题（为空则不显示标题栏）
    pub title: String,
    /// 点击遮罩是否关闭
    pub mask_closable: bool,
    /// 日期选中回调，返回 "YYYY-MM-DD"
    pub onchange: Option<EventHandler<String>>,
    /// 关闭回调
    pub onclose: Option<EventHandler<()>>,
}

impl Default for CalendarConfig {
    fn default() -> Self {
        Self {
            value: String::new(),
            title: "选择日期".to_string(),
            mask_closable: true,
            onchange: None,
            onclose: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════
// CalendarAPI — 通过 use_calendar() 获取
// ═══════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct CalendarAPI {
    pub(crate) visible: Signal<bool>,
    pub(crate) config: Signal<CalendarConfig>,
}

/// 获取 Calendar 命令式 API
pub fn use_calendar() -> CalendarAPI {
    use_context::<CalendarAPI>()
}

impl CalendarAPI {
    /// 以浮层方式打开日历
    pub fn open(&mut self, config: CalendarConfig) {
        self.config.set(config);
        self.visible.set(true);
    }

    /// 关闭日历浮层
    pub fn close(&mut self) {
        self.visible.set(false);
    }
}

// ═══════════════════════════════════════════════════════════
// CalendarProvider — 包裹在应用根节点，提供命令式调用
// ═══════════════════════════════════════════════════════════

/// CalendarProvider 属性
#[derive(Props, PartialEq, Clone)]
pub struct CalendarProviderProps {
    pub children: Element,
}

/// 日历命令式 Provider，包裹在应用根节点。
///
/// 通过 use_calendar() 获取 API 弹出日历选择日期：
///
/// ```rust
/// let mut calendar = use_calendar();
/// calendar.open(CalendarConfig {
///     title: "选择日期".into(),
///     onchange: Some(EventHandler::new(|date: String| { /* 处理选中日期 */ })),
///     ..Default::default()
/// });
/// ```
#[allow(non_snake_case)]
pub fn CalendarProvider(props: CalendarProviderProps) -> Element {
    const CSS: &str = include_str!("../../assets/calendar.css");
    let visible = use_signal(|| false);
    let config = use_signal(CalendarConfig::default);

    let api = CalendarAPI { visible, config };
    use_context_provider(|| api);

    let vis = visible();
    let cfg = config.read().clone();
    let title = cfg.title.clone();
    let value = cfg.value.clone();
    let mask_closable = cfg.mask_closable;
    let onclose = cfg.onclose.clone();
    let onchange = cfg.onchange.clone();

    let on_mask = {
        let mut a = api;
        let oc = onclose.clone();
        move |_| {
            if mask_closable {
                if let Some(ref h) = oc { h.call(()); }
                a.close();
            }
        }
    };

    let on_close_btn = {
        let mut a = api;
        let oc = onclose.clone();
        move |_| {
            if let Some(ref h) = oc { h.call(()); }
            a.close();
        }
    };

    let on_select = {
        let mut a = api;
        let oc = onchange.clone();
        move |date: String| {
            if let Some(ref h) = oc { h.call(date); }
            a.close();
        }
    };

    rsx! {
        {props.children}
        if vis {
            style { {CSS} }
            div { class: "ctrl-calendar-popup__overlay",
                div {
                    class: "ctrl-calendar-popup__mask",
                    onclick: on_mask,
                }
                div { class: "ctrl-calendar-popup__panel",
                    if !title.is_empty() {
                        div { class: "ctrl-calendar-popup__header",
                            span { class: "ctrl-calendar-popup__title", "{title}" }
                            button {
                                class: "ctrl-calendar-popup__close",
                                onclick: on_close_btn,
                                "✕"
                            }
                        }
                    }
                    Calendar {
                        value: value,
                        onchange: on_select,
                    }
                }
            }
        }
    }
}
