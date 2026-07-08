use dioxus::prelude::*;

/// 内容拷贝份数，多份拷贝保证在宽容器 / 短内容下也能铺满，避免出现空白
const COPIES: usize = 4;

/// Marquee 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct MarqueeProps {
    /// 滚动方向：left（向左）/ right（向右）/ up（向上）/ down（向下）
    #[props(default = "left".to_string())]
    pub direction: String,

    /// 滚动速度（秒）：滚动一份内容所需时长，越大越慢
    #[props(default = 20.0)]
    pub speed: f64,

    /// 悬停时暂停
    #[props(default = true)]
    pub pause_on_hover: bool,

    /// 元素间距（同时作为拷贝之间的间距，保证首尾衔接一致）
    #[props(default = "var(--ctrl-spacing-lg)".to_string())]
    pub gap: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 滚动内容（自动复制多份实现无缝循环）
    pub children: Element,
}

/// Marquee 跑马灯组件
///
/// 无缝循环滚动容器。渲染多份内容拷贝，每份以 `translateX(calc(-100% - gap))`
/// 独立平移，容器与拷贝内部共用同一 gap，从而在任意内容长度下都保持首尾无缝衔接。
#[allow(non_snake_case)]
pub fn Marquee(props: MarqueeProps) -> Element {
    const CSS: &str = include_str!("../../assets/marquee.css");

    let vertical = props.direction == "up" || props.direction == "down";
    let reverse = props.direction == "right" || props.direction == "down";

    let mut classes = vec!["ctrl-marquee".to_string()];
    if vertical {
        classes.push("ctrl-marquee--vertical".to_string());
    }
    if props.pause_on_hover {
        classes.push("ctrl-marquee--pause".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let marquee_class = classes.join(" ");

    let container_style = format!("--ctrl-marquee-gap: {gap}; {extra}", gap = props.gap, extra = props.style);
    let group_style = format!(
        "animation-duration: {dur}s; animation-direction: {dir};",
        dur = props.speed,
        dir = if reverse { "reverse" } else { "normal" },
    );

    rsx! {
        style { {CSS} }
        div { class: "{marquee_class}", style: "{container_style}",
            for i in 0..COPIES {
                div {
                    key: "{i}",
                    class: "ctrl-marquee__group",
                    style: "{group_style}",
                    "aria-hidden": if i > 0 { "true" } else { "false" },
                    {props.children.clone()}
                }
            }
        }
    }
}
