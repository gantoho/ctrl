use dioxus::prelude::*;

/// Meteors 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct MeteorsProps {
    /// 流星数量
    #[props(default = 20)]
    pub count: usize,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 覆盖在流星之上的内容
    pub children: Element,
}

/// Meteors 流星组件
///
/// 在容器内渲染一层斜向坠落的流星作为背景装饰，`children` 展示在其上层。
#[allow(non_snake_case)]
pub fn Meteors(props: MeteorsProps) -> Element {
    const CSS: &str = include_str!("../../assets/meteors.css");

    let container_class = if props.class.is_empty() {
        "ctrl-meteors".to_string()
    } else {
        format!("ctrl-meteors {}", props.class)
    };

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{props.style}",
            div { class: "ctrl-meteors__layer",
                for i in 0..props.count {
                    {
                        // 基于索引的确定性伪随机分布
                        let left = (i * 41 + 7) % 100;
                        let delay = (i * 313) % 5000;
                        let duration = 3000 + (i * 271) % 5000;
                        let meteor_style = format!(
                            "left:{}%; animation-delay:{}ms; animation-duration:{}ms;",
                            left, delay, duration
                        );
                        rsx! {
                            span { key: "{i}", class: "ctrl-meteors__meteor", style: "{meteor_style}" }
                        }
                    }
                }
            }
            div { class: "ctrl-meteors__content", {props.children} }
        }
    }
}
