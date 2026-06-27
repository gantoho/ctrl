use dioxus::prelude::*;

/// Timeline 时间线组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TimelineProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（TimelineItem）
    pub children: Element,
}

/// 时间线条目属性
#[derive(Props, PartialEq, Clone)]
pub struct TimelineItemProps {
    /// 时间标签
    #[props(default = "".to_string())]
    pub timestamp: String,

    /// 圆点颜色：default / primary / success / warning / danger
    #[props(default = "default".to_string())]
    pub color: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（内容）
    pub children: Element,
}

/// Timeline 时间线组件
#[allow(non_snake_case)]
pub fn Timeline(props: TimelineProps) -> Element {
    const CSS: &str = include_str!("../../assets/timeline.css");

    let wrapper_class = if props.class.is_empty() {
        "ctrl-timeline".to_string()
    } else {
        format!("ctrl-timeline {}", props.class)
    };

    rsx! {
        style { {CSS} }
        ul {
            class: "{wrapper_class}",
            {props.children}
        }
    }
}

/// 时间线条目
#[allow(non_snake_case)]
pub fn TimelineItem(props: TimelineItemProps) -> Element {
    let color_class = match props.color.as_str() {
        "primary" => "ctrl-timeline__item--primary",
        "success" => "ctrl-timeline__item--success",
        "warning" => "ctrl-timeline__item--warning",
        "danger" => "ctrl-timeline__item--danger",
        _ => "",
    };

    let item_class = if color_class.is_empty() {
        "ctrl-timeline__item".to_string()
    } else {
        format!("ctrl-timeline__item {}", color_class)
    };

    rsx! {
        li {
            class: if props.class.is_empty() { item_class } else { format!("{} {}", item_class, props.class) },
            if !props.timestamp.is_empty() {
                span {
                    class: "ctrl-timeline__timestamp",
                    "{props.timestamp}"
                }
            }
            div {
                class: "ctrl-timeline__dot",
            }
            div {
                class: "ctrl-timeline__content",
                {props.children}
            }
        }
    }
}
