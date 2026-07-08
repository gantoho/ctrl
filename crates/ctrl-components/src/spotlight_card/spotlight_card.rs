use dioxus::prelude::*;

/// SpotlightCard 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SpotlightCardProps {
    /// 光晕颜色
    #[props(default = "var(--ctrl-primary)".to_string())]
    pub glow_color: String,

    /// 光晕半径（px）
    #[props(default = 240.0)]
    pub radius: f64,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 卡片内容
    pub children: Element,
}

/// SpotlightCard 聚光灯卡片组件
///
/// 鼠标在卡片内移动时，跟随光标渲染一束径向光晕，营造聚光灯效果。
#[allow(non_snake_case)]
pub fn SpotlightCard(props: SpotlightCardProps) -> Element {
    const CSS: &str = include_str!("../../assets/spotlight_card.css");

    let mut pos = use_signal(|| (0.0f64, 0.0f64));
    let mut active = use_signal(|| false);

    let card_class = if props.class.is_empty() {
        "ctrl-spotlight-card".to_string()
    } else {
        format!("ctrl-spotlight-card {}", props.class)
    };

    let (x, y) = pos();
    let opacity = if active() { 1.0 } else { 0.0 };
    let card_style = format!(
        "--ctrl-spotlight-x: {x}px; --ctrl-spotlight-y: {y}px; --ctrl-spotlight-color: {color}; --ctrl-spotlight-radius: {radius}px; --ctrl-spotlight-opacity: {opacity}; {extra}",
        color = props.glow_color,
        radius = props.radius,
        extra = props.style,
    );

    rsx! {
        style { {CSS} }
        div {
            class: "{card_class}",
            style: "{card_style}",
            onmousemove: move |e: MouseEvent| {
                let c = e.data().element_coordinates();
                pos.set((c.x, c.y));
            },
            onmouseenter: move |_| active.set(true),
            onmouseleave: move |_| active.set(false),
            div { class: "ctrl-spotlight-card__glow" }
            div { class: "ctrl-spotlight-card__content", {props.children} }
        }
    }
}
