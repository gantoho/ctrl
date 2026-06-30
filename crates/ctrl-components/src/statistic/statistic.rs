use dioxus::prelude::*;

/// 趋势方向
#[derive(PartialEq, Clone, Default)]
pub enum StatisticTrend {
    #[default]
    None,
    Up,
    Down,
}

/// Statistic 统计数值组件属性
#[derive(Props, PartialEq, Clone)]
pub struct StatisticProps {
    /// 标题
    #[props(default = "".to_string())]
    pub title: String,

    /// 数值（支持任意格式的字符串，如 "128"、"88.5%"）
    #[props(default = "".to_string())]
    pub value: String,

    /// 小数精度（>0 时自动格式化数值，仅当 value 可解析为 f64 时生效）
    #[props(default = 0)]
    pub precision: usize,

    /// 趋势方向
    #[props(default = StatisticTrend::default())]
    pub trend: StatisticTrend,

    /// 是否居中
    #[props(default = false)]
    pub center: bool,

    /// 前缀（渲染在数值前面）
    pub prefix: Option<Element>,

    /// 后缀（渲染在数值后面）
    pub suffix: Option<Element>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 数值自定义样式
    #[props(default = "".to_string())]
    pub value_style: String,
}

/// 格式化数值（按精度）
fn format_value(raw: &str, precision: usize) -> String {
    if precision == 0 {
        return raw.to_string();
    }
    // 尝试解析为 f64 进行格式化
    if let Ok(num) = raw.parse::<f64>() {
        format!("{:.precision$}", num)
    } else {
        raw.to_string()
    }
}

/// Statistic 统计数值组件
///
/// 用于突出展示统计数据，支持标题、数值、前缀/后缀和趋势箭头。
///
/// ### 使用示例
/// ```rust
/// Statistic {
///     title: "总销售额".to_string(),
///     value: "128,000".to_string(),
///     suffix: rsx! { span { "元" } },
///     trend: StatisticTrend::Up,
/// }
/// ```
#[allow(non_snake_case)]
pub fn Statistic(props: StatisticProps) -> Element {
    const CSS: &str = include_str!("../../assets/statistic.css");

    let display_value = format_value(&props.value, props.precision);

    let wrapper_class = {
        let mut c = String::from("ctrl-statistic");
        if props.center {
            c.push_str(" ctrl-statistic--center");
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    let value_style = if !props.value_style.is_empty() {
        format!("{}", props.value_style)
    } else {
        String::new()
    };

    // 趋势箭头
    let trend_ui = match props.trend {
        StatisticTrend::Up => rsx! {
            span { class: "ctrl-statistic__trend ctrl-statistic__trend--up", "↑" }
        },
        StatisticTrend::Down => rsx! {
            span { class: "ctrl-statistic__trend ctrl-statistic__trend--down", "↓" }
        },
        StatisticTrend::None => rsx! {},
    };

    let has_title = !props.title.is_empty();

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            if has_title {
                div { class: "ctrl-statistic__title", "{props.title}" }
            }
            div { class: "ctrl-statistic__content",
                if let Some(ref prefix) = props.prefix {
                    span { class: "ctrl-statistic__prefix", {prefix.clone()} }
                }
                span {
                    class: "ctrl-statistic__value",
                    style: if !value_style.is_empty() { value_style.as_str() },
                    "{display_value}"
                }
                if let Some(ref suffix) = props.suffix {
                    span { class: "ctrl-statistic__suffix", {suffix.clone()} }
                }
                {trend_ui}
            }
        }
    }
}
