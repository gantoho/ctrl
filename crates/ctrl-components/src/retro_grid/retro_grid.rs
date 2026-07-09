use dioxus::prelude::*;

/// RetroGrid 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct RetroGridProps {
    /// 网格向前流动一轮的时长（秒）
    #[props(default = 15.0)]
    pub duration: f64,

    /// 网格透视倾斜角度（度）
    #[props(default = 65.0)]
    pub angle: f64,

    /// 网格线颜色（留空使用主题主色）
    #[props(default = "".to_string())]
    pub color: String,

    /// 网格单元大小（px）
    #[props(default = 60)]
    pub cell_size: u32,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 覆盖在网格之上的内容
    pub children: Element,
}

/// RetroGrid 复古网格组件
///
/// 带透视的地平线网格向前无限流动，营造复古赛博 / synthwave 氛围背景。
/// `children` 展示在网格之上。
#[allow(non_snake_case)]
pub fn RetroGrid(props: RetroGridProps) -> Element {
    const CSS: &str = include_str!("../../assets/retro_grid.css");

    let container_class = if props.class.is_empty() {
        "ctrl-retro-grid".to_string()
    } else {
        format!("ctrl-retro-grid {}", props.class)
    };

    let color = if props.color.is_empty() {
        "var(--ctrl-primary)".to_string()
    } else {
        props.color.clone()
    };

    let vars = format!(
        "--ctrl-retro-duration:{}s; --ctrl-retro-angle:{}deg; --ctrl-retro-color:{}; --ctrl-retro-cell:{}px; {}",
        props.duration, props.angle, color, props.cell_size, props.style
    );

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{vars}",
            div { class: "ctrl-retro-grid__wrap",
                div { class: "ctrl-retro-grid__lines" }
            }
            div { class: "ctrl-retro-grid__fade" }
            div { class: "ctrl-retro-grid__content", {props.children} }
        }
    }
}
