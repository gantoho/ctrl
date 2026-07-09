use dioxus::prelude::*;

/// TextReveal 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TextRevealProps {
    /// 要揭示的文本
    pub text: String,

    /// 按词还是按字揭示："word"（默认，按空格分词）/ "char"（逐字）
    #[props(default = "word".to_string())]
    pub mode: String,

    /// 相邻单元的入场延迟（毫秒）
    #[props(default = 60)]
    pub stagger: u32,

    /// 单个单元动画时长（毫秒）
    #[props(default = 500)]
    pub duration: u32,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// TextReveal 文字揭示组件
///
/// 将文本按词或按字拆分，逐个交错淡入上浮，形成揭示式入场。常用于标题、标语。
#[allow(non_snake_case)]
pub fn TextReveal(props: TextRevealProps) -> Element {
    const CSS: &str = include_str!("../../assets/text_reveal.css");

    let container_class = if props.class.is_empty() {
        "ctrl-text-reveal".to_string()
    } else {
        format!("ctrl-text-reveal {}", props.class)
    };

    let vars = format!(
        "--ctrl-text-reveal-duration:{}ms; {}",
        props.duration, props.style
    );

    // 拆分为单元；按词时保留空格作为词间距
    let by_char = props.mode == "char";
    let units: Vec<String> = if by_char {
        props.text.chars().map(|c| c.to_string()).collect()
    } else {
        props.text.split(' ').map(|w| w.to_string()).collect()
    };
    let stagger = props.stagger;

    rsx! {
        style { {CSS} }
        span { class: "{container_class}", style: "{vars}",
            for (i, unit) in units.iter().enumerate() {
                {
                    let delay = i as u32 * stagger;
                    let unit_style = format!("animation-delay:{}ms;", delay);
                    let display = if by_char && unit == " " { "\u{00a0}".to_string() } else { unit.clone() };
                    rsx! {
                        span { key: "{i}", class: "ctrl-text-reveal__unit", style: "{unit_style}", "{display}" }
                        if !by_char && i < units.len() - 1 {
                            span { class: "ctrl-text-reveal__space", "\u{00a0}" }
                        }
                    }
                }
            }
        }
    }
}
