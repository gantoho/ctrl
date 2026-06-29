use dioxus::prelude::*;

/// Badge 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct BadgeProps {
    /// 徽标内容（数字或文字）
    #[props(default = "".to_string())]
    pub count: String,

    /// 是否显示小圆点（无数字）
    #[props(default = false)]
    pub dot: bool,

    /// 最大显示数字（超过则显示 `max+`）
    #[props(default = 99)]
    pub max: u32,

    /// 徽标颜色（默认为 danger 色）
    #[props(default = "var(--ctrl-danger)".to_string())]
    pub color: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素（包裹的容器）
    pub children: Element,
}

/// Badge 徽标组件
#[allow(non_snake_case)]
pub fn Badge(props: BadgeProps) -> Element {
    const CSS: &str = include_str!("../../assets/badge.css");
    let badge_class = if props.class.is_empty() {
        "ctrl-badge__wrapper".to_string()
    } else {
        format!("ctrl-badge__wrapper {}", props.class)
    };

    let show_dot = props.dot;
    let has_content = !props.count.is_empty() || show_dot;

    // 处理数字溢出
    let display_text = if !props.count.is_empty() {
        if let Ok(n) = props.count.parse::<u32>() {
            if n > props.max {
                format!("{}+", props.max)
            } else {
                props.count.clone()
            }
        } else {
            props.count.clone()
        }
    } else {
        String::new()
    };

    let color = props.color.clone();
    let badge_style = format!("background: {color}; {extra}", extra = props.style);

    rsx! {
        style { {CSS} }
        div {
            class: "ctrl-badge__wrapper {badge_class}",
            {props.children}
            if has_content {
                if show_dot {
                    sup {
                        class: "ctrl-badge ctrl-badge--dot",
                        style: "{badge_style}",
                    }
                } else {
                    sup {
                        class: "ctrl-badge",
                        style: "{badge_style}",
                        "{display_text}"
                    }
                }
            }
        }
    }
}
