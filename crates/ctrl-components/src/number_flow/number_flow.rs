use dioxus::prelude::*;

/// NumberFlow 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct NumberFlowProps {
    /// 显示的数值（字符串形式，支持千分位、小数点等分隔符）
    pub value: String,

    /// 前缀（如 "$"、"¥"）
    #[props(default = "".to_string())]
    pub prefix: String,

    /// 后缀（如 "%"、"K"）
    #[props(default = "".to_string())]
    pub suffix: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// NumberFlow 数字滚动组件
///
/// 每个数字位使用垂直滚动列（0-9），数值变化时通过 CSS transform 过渡实现滚动动画。
#[allow(non_snake_case)]
pub fn NumberFlow(props: NumberFlowProps) -> Element {
    const CSS: &str = include_str!("../../assets/number_flow.css");
    let flow_class = if props.class.is_empty() {
        "ctrl-number-flow".to_string()
    } else {
        format!("ctrl-number-flow {}", props.class)
    };

    rsx! {
        style { {CSS} }
        span {
            class: "{flow_class}",
            style: "{props.style}",
            if !props.prefix.is_empty() {
                span { class: "ctrl-number-flow__affix", "{props.prefix}" }
            }
            for (i, c) in props.value.chars().enumerate() {
                if c.is_ascii_digit() {
                    span {
                        key: "{i}",
                        class: "ctrl-number-flow__digit",
                        span {
                            class: "ctrl-number-flow__digit-col",
                            style: "transform: translateY(-{c.to_digit(10).unwrap()}em);",
                            for d in 0..10 {
                                span { key: "{d}", "{d}" }
                            }
                        }
                    }
                } else {
                    span { key: "{i}", class: "ctrl-number-flow__sep", "{c}" }
                }
            }
            if !props.suffix.is_empty() {
                span { class: "ctrl-number-flow__affix", "{props.suffix}" }
            }
        }
    }
}
