use dioxus::prelude::*;
use ctrl_core::types::Placement;
use crate::popover::Popover;

/// HoverCard 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct HoverCardProps {
    /// 卡片弹出方向（默认 Top）
    #[props(default = Placement::default())]
    pub placement: Placement,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 卡片内容
    pub content: Element,

    /// 触发元素
    pub children: Element,
}

/// HoverCard 悬停卡片组件
///
/// 鼠标悬停触发的信息卡片，基于 [`Popover`] 的 hover 模式实现，
/// 用于展示用户名片、内容摘要等上下文信息。
#[allow(non_snake_case)]
pub fn HoverCard(props: HoverCardProps) -> Element {
    rsx! {
        Popover {
            hover: true,
            placement: props.placement,
            class: props.class,
            content: props.content,
            {props.children}
        }
    }
}
