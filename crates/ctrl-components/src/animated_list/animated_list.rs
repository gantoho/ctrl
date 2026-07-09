use dioxus::prelude::*;

/// AnimatedList 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct AnimatedListProps {
    /// 相邻子项之间的入场延迟（毫秒）
    #[props(default = 120)]
    pub stagger: u32,

    /// 单项动画时长（毫秒）
    #[props(default = 500)]
    pub duration: u32,

    /// 入场方向：up / down / left / right
    #[props(default = "up".to_string())]
    pub direction: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 列表子项（每个顶层子元素依次入场）
    pub children: Element,
}

/// AnimatedList 动画列表组件
///
/// 将子项依次淡入 + 位移入场，形成交错的入场动画。常用于消息列表、通知、卡片流等。
#[allow(non_snake_case)]
pub fn AnimatedList(props: AnimatedListProps) -> Element {
    const CSS: &str = include_str!("../../assets/animated_list.css");

    let dir_class = match props.direction.as_str() {
        "down" => "ctrl-animated-list--down",
        "left" => "ctrl-animated-list--left",
        "right" => "ctrl-animated-list--right",
        _ => "ctrl-animated-list--up",
    };

    let mut container_class = format!("ctrl-animated-list {}", dir_class);
    if !props.class.is_empty() {
        container_class.push(' ');
        container_class.push_str(&props.class);
    }

    let vars = format!(
        "--ctrl-animated-list-stagger:{}ms; --ctrl-animated-list-duration:{}ms; {}",
        props.stagger, props.duration, props.style
    );

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{vars}", {props.children} }
    }
}
