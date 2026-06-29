use dioxus::prelude::*;
use ctrl_core::types::Direction;

/// Space 间距组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SpaceProps {
    /// 间距大小：sm / md / lg / 或自定义 CSS 值
    #[props(default = "md".to_string())]
    pub gap: String,

    /// 排列方向
    #[props(default = Direction::Horizontal)]
    pub direction: Direction,

    /// 对齐方式：start / center / end / baseline
    #[props(default = "center".to_string())]
    pub align: String,

    /// 是否换行
    #[props(default = false)]
    pub wrap: bool,

    /// 是否占满整行
    #[props(default = false)]
    pub block: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素
    pub children: Element,
}

/// Space 间距组件
///
/// 用于设置子元素之间的间距，支持水平和垂直排列。
///
/// # 示例
///
/// ```rust
/// Space { gap: "md".to_string(),
///     Button { "按钮1" }
///     Button { "按钮2" }
///     Button { "按钮3" }
/// }
/// ```
#[allow(non_snake_case)]
pub fn Space(props: SpaceProps) -> Element {
    const CSS: &str = include_str!("../../assets/space.css");

    let gap_value = match props.gap.as_str() {
        "sm" => "var(--ctrl-spacing-sm)",
        "md" => "var(--ctrl-spacing-md)",
        "lg" => "var(--ctrl-spacing-lg)",
        "xl" => "var(--ctrl-spacing-xl)",
        other => other,
    };

    let space_class = {
        let mut c = String::from("ctrl-space");
        if props.block {
            c.push_str(" ctrl-space--block");
        }
        if props.direction == Direction::Vertical {
            c.push_str(" ctrl-space--vertical");
        } else {
            c.push_str(" ctrl-space--horizontal");
        }
        if props.wrap {
            c.push_str(" ctrl-space--wrap");
        }
        match props.align.as_str() {
            "start" => c.push_str(" ctrl-space--align-start"),
            "end" => c.push_str(" ctrl-space--align-end"),
            "baseline" => c.push_str(" ctrl-space--align-baseline"),
            _ => {}
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{space_class}",
            style: "gap: {gap_value}; {props.style}",
            {props.children}
        }
    }
}