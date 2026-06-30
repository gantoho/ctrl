use dioxus::prelude::*;
use ctrl_core::types::Direction;

/// Divider 分割线组件属性
#[derive(Props, PartialEq, Clone)]
pub struct DividerProps {
    /// 分割线方向
    #[props(default = Direction::Horizontal)]
    pub direction: Direction,

    /// 分割线中间的文字（空字符串则为纯分割线）
    #[props(default = "".to_string())]
    pub content: String,

    /// 文字位置：left / center / right
    #[props(default = "center".to_string())]
    pub orientation: String,

    /// 上间距：sm / md / lg / xl / 或自定义 CSS 值
    #[props(default = "md".to_string())]
    pub gap_top: String,

    /// 右间距：sm / md / lg / xl / 或自定义 CSS 值
    #[props(default = "md".to_string())]
    pub gap_right: String,

    /// 下间距：sm / md / lg / xl / 或自定义 CSS 值
    #[props(default = "md".to_string())]
    pub gap_bottom: String,

    /// 左间距：sm / md / lg / xl / 或自定义 CSS 值
    #[props(default = "md".to_string())]
    pub gap_left: String,

    /// 是否为虚线
    #[props(default = false)]
    pub dashed: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// Divider 分割线组件
#[allow(non_snake_case)]
pub fn Divider(props: DividerProps) -> Element {
    const CSS: &str = include_str!("../../assets/divider.css");
    let has_text = !props.content.is_empty();
    let is_vertical = props.direction == Direction::Vertical;

    fn resolve_gap(g: &str) -> String {
        match g {
            "0" => "0".to_string(),
            "sm" => "var(--ctrl-spacing-sm)".to_string(),
            "md" => "var(--ctrl-spacing-md)".to_string(),
            "lg" => "var(--ctrl-spacing-lg)".to_string(),
            "xl" => "var(--ctrl-spacing-xl)".to_string(),
            other => other.to_string(),
        }
    }
    let gt = resolve_gap(&props.gap_top);
    let gr = resolve_gap(&props.gap_right);
    let gb = resolve_gap(&props.gap_bottom);
    let gl = resolve_gap(&props.gap_left);

    let mut classes = vec!["ctrl-divider".to_string()];

    if has_text {
        classes.push("ctrl-divider--with-text".into());
    } else if !is_vertical {
        classes.push("ctrl-divider--plain".into());
    }

    if is_vertical {
        classes.push("ctrl-divider--vertical".into());
    }

    if props.dashed {
        classes.push("ctrl-divider--dashed".into());
    }

    match props.orientation.as_str() {
        "left" => classes.push("ctrl-divider--left".into()),
        "right" => classes.push("ctrl-divider--right".into()),
        _ => {}
    }

    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }

    let class_str = classes.join(" ");

    rsx! {
        style { {CSS} }
        div {
            class: "{class_str}",
            style: "--ctrl-divider-gap-top: {gt}; --ctrl-divider-gap-right: {gr}; --ctrl-divider-gap-bottom: {gb}; --ctrl-divider-gap-left: {gl}; {props.style}",
            "{props.content}"
        }
    }
}
