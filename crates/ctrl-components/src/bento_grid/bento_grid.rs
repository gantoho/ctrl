use dioxus::prelude::*;

/// BentoGrid 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct BentoGridProps {
    /// 网格列数
    #[props(default = 3)]
    pub columns: u32,

    /// 单元间距（CSS 尺寸）
    #[props(default = "16px".to_string())]
    pub gap: String,

    /// 行最小高度（CSS 尺寸）
    #[props(default = "180px".to_string())]
    pub row_height: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 单元格（BentoCell）
    pub children: Element,
}

/// BentoGrid 便当网格布局组件
///
/// 大小不一的卡片按网格排布，通过 BentoCell 的 col_span / row_span 控制跨列跨行，
/// 常用于产品特性、功能亮点的杂志式展示。
#[allow(non_snake_case)]
pub fn BentoGrid(props: BentoGridProps) -> Element {
    const CSS: &str = include_str!("../../assets/bento_grid.css");

    let container_class = if props.class.is_empty() {
        "ctrl-bento-grid".to_string()
    } else {
        format!("ctrl-bento-grid {}", props.class)
    };

    let vars = format!(
        "grid-template-columns: repeat({}, minmax(0, 1fr)); gap: {}; grid-auto-rows: minmax({}, auto); {}",
        props.columns, props.gap, props.row_height, props.style
    );

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{vars}", {props.children} }
    }
}

/// BentoCell 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct BentoCellProps {
    /// 跨列数
    #[props(default = 1)]
    pub col_span: u32,

    /// 跨行数
    #[props(default = 1)]
    pub row_span: u32,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 单元内容
    pub children: Element,
}

/// BentoCell 便当网格单元
///
/// BentoGrid 的子项，通过 col_span / row_span 控制占据的网格区域大小。
#[allow(non_snake_case)]
pub fn BentoCell(props: BentoCellProps) -> Element {
    let cell_class = if props.class.is_empty() {
        "ctrl-bento-grid__cell".to_string()
    } else {
        format!("ctrl-bento-grid__cell {}", props.class)
    };

    let vars = format!(
        "grid-column: span {}; grid-row: span {}; {}",
        props.col_span, props.row_span, props.style
    );

    rsx! {
        div { class: "{cell_class}", style: "{vars}", {props.children} }
    }
}
