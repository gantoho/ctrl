use dioxus::prelude::*;

/// DotPattern 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct DotPatternProps {
    /// 图案单元间距（px）
    #[props(default = 20)]
    pub gap: u32,

    /// 点半径（px）
    #[props(default = 1)]
    pub dot_size: u32,

    /// 点颜色（留空使用主题边框色）
    #[props(default = "".to_string())]
    pub color: String,

    /// 是否使用网格线代替圆点
    #[props(default = false)]
    pub grid: bool,

    /// 是否启用中心径向淡出遮罩
    #[props(default = true)]
    pub fade: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 覆盖在图案之上的内容
    pub children: Element,
}

/// DotPattern 点阵背景组件
///
/// 使用纯 CSS 渐变绘制点阵或网格装饰背景，常用于 Hero 区、卡片、分区背景。
/// `children` 展示在图案之上；可选中心径向淡出遮罩，使边缘渐隐。
#[allow(non_snake_case)]
pub fn DotPattern(props: DotPatternProps) -> Element {
    const CSS: &str = include_str!("../../assets/dot_pattern.css");

    let mut container_class = if props.grid {
        "ctrl-dot-pattern ctrl-dot-pattern--grid".to_string()
    } else {
        "ctrl-dot-pattern".to_string()
    };
    if props.fade {
        container_class.push_str(" ctrl-dot-pattern--fade");
    }
    if !props.class.is_empty() {
        container_class.push(' ');
        container_class.push_str(&props.class);
    }

    let color = if props.color.is_empty() {
        "var(--ctrl-border)".to_string()
    } else {
        props.color.clone()
    };

    let vars = format!(
        "--ctrl-dot-gap:{}px; --ctrl-dot-size:{}px; --ctrl-dot-color:{}; {}",
        props.gap, props.dot_size, color, props.style
    );

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{vars}",
            div { class: "ctrl-dot-pattern__layer" }
            div { class: "ctrl-dot-pattern__content", {props.children} }
        }
    }
}
