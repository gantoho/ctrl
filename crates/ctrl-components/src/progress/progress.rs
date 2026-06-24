use dioxus::prelude::*;

/// Progress 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ProgressProps {
    /// 进度值 0-100
    #[props(default = 0.0)]
    pub percent: f64,

    /// 进度条颜色（默认 primary）
    #[props(default = "var(--ctrl-primary)".to_string())]
    pub color: String,

    /// 是否显示百分比文字
    #[props(default = false)]
    pub show_text: bool,

    /// 进度条高度（px）
    #[props(default = 8)]
    pub height: u32,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// Progress 进度条组件
#[allow(non_snake_case)]
pub fn Progress(props: ProgressProps) -> Element {
    let wrapper_class = if props.class.is_empty() {
        "ctrl-progress".to_string()
    } else {
        format!("ctrl-progress {}", props.class)
    };

    let percent = props.percent.clamp(0.0, 100.0);
    let color = props.color.clone();
    let track_style = format!("height: {}px; --ctrl-progress-color: {color};", props.height, color = color);
    let bar_style = format!("width: {percent}%;", percent = percent);
    let wrapper_style = if props.style.is_empty() {
        track_style.clone()
    } else {
        format!("{} {}", track_style, props.style)
    };

    let text = format!("{:.0}%", percent);

    rsx! {
        div {
            class: "{wrapper_class}",
            style: "{wrapper_style}",
            div {
                class: "ctrl-progress__track",
                div {
                    class: "ctrl-progress__bar",
                    style: "{bar_style}",
                }
            }
            if props.show_text {
                span {
                    class: "ctrl-progress__text",
                    "{text}"
                }
            }
        }
    }
}
