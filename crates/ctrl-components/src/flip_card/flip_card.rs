use dioxus::prelude::*;

/// FlipCard 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct FlipCardProps {
    /// 正面内容
    pub front: Element,

    /// 背面内容
    pub back: Element,

    /// 是否点击翻转（默认 false，即悬停翻转）
    #[props(default = false)]
    pub click_to_flip: bool,

    /// 是否纵向翻转（默认横向）
    #[props(default = false)]
    pub vertical: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式（通常用于设置宽高）
    #[props(default = "".to_string())]
    pub style: String,
}

/// FlipCard 翻转卡片组件
///
/// 通过 3D 变换在正反面之间翻转，支持悬停或点击触发、横向或纵向翻转。
/// 需通过 `style` 设置宽高，例如 `"width:240px; height:160px;"`。
#[allow(non_snake_case)]
pub fn FlipCard(props: FlipCardProps) -> Element {
    const CSS: &str = include_str!("../../assets/flip_card.css");

    let mut flipped = use_signal(|| false);

    let mut classes = vec!["ctrl-flip-card".to_string()];
    if props.vertical {
        classes.push("ctrl-flip-card--vertical".into());
    }
    if props.click_to_flip {
        if flipped() {
            classes.push("ctrl-flip-card--flipped".into());
        }
    } else {
        classes.push("ctrl-flip-card--hover".into());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let card_class = classes.join(" ");

    let click_to_flip = props.click_to_flip;

    rsx! {
        style { {CSS} }
        div {
            class: "{card_class}",
            style: "{props.style}",
            onclick: move |_| {
                if click_to_flip {
                    let cur = flipped();
                    flipped.set(!cur);
                }
            },
            div { class: "ctrl-flip-card__inner",
                div { class: "ctrl-flip-card__face ctrl-flip-card__front", {props.front} }
                div { class: "ctrl-flip-card__face ctrl-flip-card__back", {props.back} }
            }
        }
    }
}
