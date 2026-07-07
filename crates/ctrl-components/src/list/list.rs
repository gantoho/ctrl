use dioxus::prelude::*;

/// List 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ListProps {
    /// 是否显示外边框
    #[props(default = true)]
    pub bordered: bool,

    /// 是否显示列表项之间的分隔线
    #[props(default = true)]
    pub split: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 列表项（通常为 ListItem）
    pub children: Element,
}

/// List 列表组件
#[allow(non_snake_case)]
pub fn List(props: ListProps) -> Element {
    const CSS: &str = include_str!("../../assets/list.css");
    let mut classes = vec!["ctrl-list".to_string()];
    if props.bordered {
        classes.push("ctrl-list--bordered".to_string());
    }
    if props.split {
        classes.push("ctrl-list--split".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let list_class = classes.join(" ");

    rsx! {
        style { {CSS} }
        div {
            class: "{list_class}",
            style: "{props.style}",
            {props.children}
        }
    }
}

/// ListItem 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ListItemProps {
    /// 右侧额外内容（如操作按钮）
    pub extra: Option<Element>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 列表项主体内容
    pub children: Element,
}

/// ListItem 列表项组件
#[allow(non_snake_case)]
pub fn ListItem(props: ListItemProps) -> Element {
    let item_class = if props.class.is_empty() {
        "ctrl-list__item".to_string()
    } else {
        format!("ctrl-list__item {}", props.class)
    };

    rsx! {
        div {
            class: "{item_class}",
            style: "{props.style}",
            div { class: "ctrl-list__item-content", {props.children} }
            if let Some(extra) = props.extra {
                div { class: "ctrl-list__item-extra", {extra} }
            }
        }
    }
}
