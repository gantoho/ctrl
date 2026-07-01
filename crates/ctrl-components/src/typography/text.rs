use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TextProps {
    /// 文本类型：default / secondary / success / warning / danger
    #[props(default = "default".to_string())]
    pub r#type: String,

    /// 加粗
    #[props(default = false)]
    pub strong: bool,

    /// 斜体
    #[props(default = false)]
    pub italic: bool,

    /// 下划线
    #[props(default = false)]
    pub underline: bool,

    /// 删除线
    #[props(default = false)]
    pub delete: bool,

    /// 代码样式
    #[props(default = false)]
    pub code: bool,

    /// 高亮标记
    #[props(default = false)]
    pub mark: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    pub children: Element,
}

#[allow(non_snake_case)]
pub fn Text(props: TextProps) -> Element {
    let mut classes = vec!["ctrl-text".to_string()];

    if props.r#type != "default" {
        classes.push(format!("ctrl-text--{}", props.r#type));
    }
    if props.strong {
        classes.push("ctrl-text--strong".to_string());
    }
    if props.italic {
        classes.push("ctrl-text--italic".to_string());
    }
    if props.underline {
        classes.push("ctrl-text--underline".to_string());
    }
    if props.delete {
        classes.push("ctrl-text--delete".to_string());
    }
    if props.code {
        classes.push("ctrl-text--code".to_string());
    }
    if props.mark {
        classes.push("ctrl-text--mark".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class);
    }

    let class = classes.join(" ");

    rsx! {
        span { class: "{class}", style: "{props.style}", {props.children} }
    }
}
