use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct HeadingProps {
    /// 标题级别 1-6，默认 1
    #[props(default = 1)]
    pub level: u8,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    pub children: Element,
}

#[allow(non_snake_case)]
pub fn Heading(props: HeadingProps) -> Element {
    const CSS: &str = include_str!("../../assets/typography.css");
    let level = props.level.min(6).max(1);
    let tag = format!("h{}", level);
    let class = if props.class.is_empty() {
        format!("ctrl-heading ctrl-heading--h{}", level)
    } else {
        format!("ctrl-heading ctrl-heading--h{} {}", level, props.class)
    };

    rsx! {
        style { {CSS} }
        {match tag.as_str() {
            "h1" => rsx! { h1 { class: "{class}", style: "{props.style}", {props.children} } },
            "h2" => rsx! { h2 { class: "{class}", style: "{props.style}", {props.children} } },
            "h3" => rsx! { h3 { class: "{class}", style: "{props.style}", {props.children} } },
            "h4" => rsx! { h4 { class: "{class}", style: "{props.style}", {props.children} } },
            "h5" => rsx! { h5 { class: "{class}", style: "{props.style}", {props.children} } },
            _ => rsx! { h6 { class: "{class}", style: "{props.style}", {props.children} } },
        }}
    }
}
