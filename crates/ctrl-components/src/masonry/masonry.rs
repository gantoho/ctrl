use dioxus::prelude::*;

/// Masonry 瀑布流组件属性
#[derive(Props, PartialEq, Clone)]
pub struct MasonryProps {
    /// 默认列数
    #[props(default = 3)]
    pub cols: u32,

    /// 列间距（px）
    #[props(default = 16.0)]
    pub gap: f64,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 瀑布流中的子项
    pub children: Element,
}

/// Masonry 瀑布流组件
///
/// 使用 CSS columns 实现瀑布流布局，子元素按列自上而下排列，各列高度自动平衡。
/// 适合图片墙、卡片列表等不定高内容的展示场景。
#[allow(non_snake_case)]
pub fn Masonry(props: MasonryProps) -> Element {
    const CSS: &str = include_str!("../../assets/masonry.css");

    let container_class = if props.class.is_empty() {
        "ctrl-masonry".to_string()
    } else {
        format!("ctrl-masonry {}", props.class)
    };

    let inline_style = format!(
        "--ctrl-masonry-cols:{};--ctrl-masonry-gap:{}px;{}",
        props.cols, props.gap, props.style
    );

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{inline_style}",
            {props.children}
        }
    }
}
