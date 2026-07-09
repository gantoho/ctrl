use dioxus::prelude::*;

/// CircularProgress 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct CircularProgressProps {
    /// 进度值 0-100
    #[props(default = 0.0)]
    pub percent: f64,

    /// 直径（px）
    #[props(default = 120.0)]
    pub size: f64,

    /// 环宽度（px）
    #[props(default = 8.0)]
    pub stroke_width: f64,

    /// 进度颜色（默认 primary）
    #[props(default = "var(--ctrl-primary)".to_string())]
    pub color: String,

    /// 轨道颜色
    #[props(default = "var(--ctrl-bg-secondary)".to_string())]
    pub track_color: String,

    /// 仪表盘模式（底部留 90° 缺口）
    #[props(default = false)]
    pub dashboard: bool,

    /// 是否显示中心百分比文字
    #[props(default = true)]
    pub show_text: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 自定义中心内容（覆盖默认百分比文字）
    pub children: Option<Element>,
}

/// CircularProgress 环形进度组件
///
/// 基于 SVG `stroke-dasharray` 绘制圆环进度，支持标准环形与底部留缺口的仪表盘模式。
#[allow(non_snake_case)]
pub fn CircularProgress(props: CircularProgressProps) -> Element {
    const CSS: &str = include_str!("../../assets/circular_progress.css");

    let percent = props.percent.clamp(0.0, 100.0);
    let size = props.size;
    let sw = props.stroke_width;
    let r = (size - sw) / 2.0;
    let center = size / 2.0;
    let circ = 2.0 * std::f64::consts::PI * r;

    // 仪表盘模式：可见弧占 75%（270°），缺口居中于底部
    let arc_ratio = if props.dashboard { 0.75 } else { 1.0 };
    let rotate = if props.dashboard { 135.0 } else { -90.0 };
    let track_dash = format!("{} {}", circ * arc_ratio, circ);
    let bar_dash = format!("{} {}", circ * arc_ratio * percent / 100.0, circ);
    let transform = format!("rotate({rotate} {center} {center})");

    let wrapper_class = if props.class.is_empty() {
        "ctrl-circular-progress".to_string()
    } else {
        format!("ctrl-circular-progress {}", props.class)
    };
    let wrapper_style = format!("width: {size}px; height: {size}px; {}", props.style);

    rsx! {
        style { {CSS} }
        div { class: "{wrapper_class}", style: "{wrapper_style}",
            svg {
                class: "ctrl-circular-progress__svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 {size} {size}",
                circle {
                    class: "ctrl-circular-progress__track",
                    cx: "{center}", cy: "{center}", r: "{r}",
                    fill: "none",
                    stroke: "{props.track_color}",
                    stroke_width: "{sw}",
                    stroke_dasharray: "{track_dash}",
                    stroke_linecap: "round",
                    transform: "{transform}",
                }
                circle {
                    class: "ctrl-circular-progress__bar",
                    cx: "{center}", cy: "{center}", r: "{r}",
                    fill: "none",
                    stroke: "{props.color}",
                    stroke_width: "{sw}",
                    stroke_dasharray: "{bar_dash}",
                    stroke_linecap: "round",
                    transform: "{transform}",
                }
            }
            div { class: "ctrl-circular-progress__content",
                if let Some(children) = props.children {
                    {children}
                } else if props.show_text {
                    span { class: "ctrl-circular-progress__text", "{percent:.0}%" }
                }
            }
        }
    }
}
