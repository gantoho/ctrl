use dioxus::prelude::*;

/// 时间差值
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeRemaining {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

/// Countdown 倒计时组件
#[derive(Props, PartialEq, Clone)]
pub struct CountdownProps {
    /// 目标时间戳（毫秒）
    pub deadline: f64,

    /// 显示格式：如 "dd:hh:mm:ss"
    /// d=天, h=时, m=分, s=秒
    #[props(default = "hh:mm:ss".to_string())]
    pub format: String,

    /// 倒计时结束文案
    #[props(default = "00:00:00".to_string())]
    pub finished_text: String,

    /// 数字样式：normal / split
    #[props(default = "split".to_string())]
    pub variant: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 倒计时结束回调
    pub onfinish: Option<EventHandler<()>>,
}

#[allow(non_snake_case)]
pub fn Countdown(props: CountdownProps) -> Element {
    const CSS: &str = include_str!("../../assets/countdown.css");
    let remaining = use_signal(|| calc_remaining(props.deadline));
    let deadline = props.deadline;
    let onfinish = props.onfinish.clone();

    // 每秒刷新剩余时间，结束时触发 onfinish
    // 将 Interval 存入 Signal，组件销毁时自动 drop 停止定时器
    let _interval = use_signal(|| {
        let mut r = remaining;
        let cb = onfinish.clone();

        gloo_timers::callback::Interval::new(1000, move || {
            let rem = calc_remaining(deadline);
            let is_dead = rem.days == 0 && rem.hours == 0 && rem.minutes == 0 && rem.seconds == 0;
            // 首次进入结束状态时触发回调
            if is_dead && r.peek().days + r.peek().hours + r.peek().minutes + r.peek().seconds > 0 {
                if let Some(ref handler) = cb {
                    handler.call(());
                }
            }
            r.set(rem);
        })
    });

    let class = if props.class.is_empty() {
        "ctrl-countdown".to_string()
    } else {
        format!("ctrl-countdown {}", props.class)
    };

    let rem = remaining();
    let values = build_time_parts(&props.format, &rem);

    // 预计算 split 数字字符
    let split_digits: Vec<(String, Vec<char>)> = values.iter()
        .map(|(label, val)| (label.clone(), format!("{:02}", val).chars().collect()))
        .collect();

    rsx! {
        style { {CSS} }
        div { class: "{class}", style: "{props.style}",
            for (label, digits) in &split_digits {
                span { class: "ctrl-countdown__item",
                    if props.variant == "split" {
                        span { class: "ctrl-countdown__split",
                            for ch in digits {
                                span { class: "ctrl-countdown__digit", "{ch}" }
                            }
                        }
                    } else {
                        span { class: "ctrl-countdown__num",
                            {digits.iter().collect::<String>()}
                        }
                    }
                    span { class: "ctrl-countdown__label", "{label}" }
                }
            }
        }
    }
}

fn calc_remaining(deadline: f64) -> TimeRemaining {
    let diff = (deadline - now_ms()).max(0.0);
    let total_secs = (diff / 1000.0) as u64;
    TimeRemaining {
        days: total_secs / 86400,
        hours: (total_secs % 86400) / 3600,
        minutes: (total_secs % 3600) / 60,
        seconds: total_secs % 60,
    }
}

fn now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as f64
    }
}

fn build_time_parts(format: &str, rem: &TimeRemaining) -> Vec<(String, u64)> {
    let mut parts = Vec::new();
    let mut chars = format.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            'd' => {
                // 跳过连续重复的 d，只取一个
                while let Some(&next) = chars.peek() { if next == 'd' { chars.next(); } else { break; } }
                parts.push(("天".to_string(), rem.days));
            }
            'h' => {
                while let Some(&next) = chars.peek() { if next == 'h' { chars.next(); } else { break; } }
                parts.push(("时".to_string(), rem.hours));
            }
            'm' => {
                while let Some(&next) = chars.peek() { if next == 'm' { chars.next(); } else { break; } }
                parts.push(("分".to_string(), rem.minutes));
            }
            's' => {
                while let Some(&next) = chars.peek() { if next == 's' { chars.next(); } else { break; } }
                parts.push(("秒".to_string(), rem.seconds));
            }
            _ => {
                let mut sep = ch.to_string();
                while let Some(&next) = chars.peek() {
                    if "dhms".contains(next) { break; }
                    sep.push(chars.next().unwrap());
                }
                if let Some(last) = parts.last_mut() {
                    last.0.push_str(&sep);
                }
            }
        }
    }
    parts
}
