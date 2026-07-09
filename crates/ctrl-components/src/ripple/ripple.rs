use dioxus::prelude::*;

/// Ripple 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct RippleProps {
    /// 圆环层数
    #[props(default = 6)]
    pub circles: usize,

    /// 最内层圆的直径，单位 px
    #[props(default = 80)]
    pub base_size: u32,

    /// 每层递增的直径，单位 px
    #[props(default = 70)]
    pub step: u32,

    /// 单圈动画时长，单位秒
    #[props(default = 3)]
    pub duration: u32,

    /// 圆环颜色（留空使用主题主色）
    #[props(default = "".to_string())]
    pub color: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 覆盖在涟漪之上的内容
    pub children: Element,
}

/// Ripple 涟漪背景组件
///
/// 由中心向外逐层扩散的同心圆背景装饰，常用于 Hero 区、卡片背景、加载区等。
/// `children` 展示在涟漪之上。
#[allow(non_snake_case)]
pub fn Ripple(props: RippleProps) -> Element {
    const CSS: &str = include_str!("../../assets/ripple.css");

    let container_class = if props.class.is_empty() {
        "ctrl-ripple".to_string()
    } else {
        format!("ctrl-ripple {}", props.class)
    };

    let mut vars = format!("--ctrl-ripple-duration:{}s;", props.duration);
    if !props.color.is_empty() {
        vars.push_str(&format!("--ctrl-ripple-color:{};", props.color));
    }
    if !props.style.is_empty() {
        vars.push(' ');
        vars.push_str(&props.style);
    }

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{vars}",
            div { class: "ctrl-ripple__layer",
                for i in 0..props.circles {
                    {
                        let size = props.base_size + props.step * i as u32;
                        let opacity = 1.0 - (i as f32 / props.circles as f32);
                        let delay = i as f32 * 0.06;
                        let border = if i == props.circles - 1 { "dashed" } else { "solid" };
                        let circle_style = format!(
                            "width:{size}px; height:{size}px; opacity:{opacity:.2}; animation-delay:{delay:.2}s; border-style:{border};"
                        );
                        rsx! {
                            div { key: "{i}", class: "ctrl-ripple__circle", style: "{circle_style}" }
                        }
                    }
                }
            }
            div { class: "ctrl-ripple__content", {props.children} }
        }
    }
}
