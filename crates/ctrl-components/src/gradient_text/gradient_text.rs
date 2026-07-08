use dioxus::prelude::*;

/// GradientText 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct GradientTextProps {
    /// 渐变色值列表，逗号分隔的 CSS 颜色（如 "#f00, #0ff, #f0f"）
    #[props(default = "var(--ctrl-primary), var(--ctrl-info), var(--ctrl-success)".to_string())]
    pub colors: String,

    /// 是否启用流光动画
    #[props(default = true)]
    pub animated: bool,

    /// 动画周期（秒）
    #[props(default = 4.0)]
    pub speed: f64,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 文本内容
    pub children: Element,
}

/// GradientText 流光渐变文字组件
///
/// 使用 background-clip:text 将渐变裁切到文字上，可选流动动画。
#[allow(non_snake_case)]
pub fn GradientText(props: GradientTextProps) -> Element {
    const CSS: &str = include_str!("../../assets/gradient_text.css");

    let mut classes = vec!["ctrl-gradient-text".to_string()];
    if props.animated {
        classes.push("ctrl-gradient-text--animated".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let text_class = classes.join(" ");

    let text_style = format!(
        "background-image: linear-gradient(90deg, {colors}, {first}); animation-duration: {speed}s; {extra}",
        colors = props.colors,
        first = first_color(&props.colors),
        speed = props.speed,
        extra = props.style,
    );

    rsx! {
        style { {CSS} }
        span {
            class: "{text_class}",
            style: "{text_style}",
            {props.children}
        }
    }
}

/// 取第一个颜色，用于渐变末尾衔接实现无缝循环
fn first_color(colors: &str) -> String {
    colors.split(',').next().unwrap_or("").trim().to_string()
}
