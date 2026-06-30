use dioxus::prelude::*;

/// Row 垂直对齐方式
#[derive(PartialEq, Clone, Default)]
pub enum Align {
    #[default]
    Top,
    Middle,
    Bottom,
}

/// 沟槽间隔模式
#[derive(PartialEq, Clone, Default)]
pub enum GutterMode {
    /// 传统模式：Row 负 margin + 每个 Col 左右 padding（各列四周都有间距）
    #[default]
    Padding,
    /// Gap 模式：仅在列和列之间产生间距（Row 使用 CSS gap，Col 无需额外样式）
    Gap,
}

/// Row → Col 沟槽上下文
#[derive(Clone)]
pub(super) struct GutterCtx {
    pub gutter: u32,
    pub mode: GutterMode,
}

/// Row 栅格行 Props
#[derive(Props, PartialEq, Clone)]
pub struct RowProps {
    /// 垂直对齐
    #[props(default = Align::default())]
    pub align: Align,

    /// 列间距（px）
    #[props(default = 0)]
    pub gutter: u32,

    /// 沟槽模式
    #[props(default = GutterMode::default())]
    pub gutter_mode: GutterMode,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素（Col）
    pub children: Element,
}

/// Row 栅格行组件
#[allow(non_snake_case)]
pub fn Row(props: RowProps) -> Element {
    const CSS: &str = include_str!("../../assets/grid.css");

    let align_class = match props.align {
        Align::Top => "ctrl-row--top",
        Align::Middle => "ctrl-row--middle",
        Align::Bottom => "ctrl-row--bottom",
    };

    let wrapper_class = {
        let mut c = String::from("ctrl-row");
        c.push_str(" ");
        c.push_str(align_class);
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    // 根据 gutter_mode 构建行内联样式
    let gutter_style = if props.gutter > 0 {
        match props.gutter_mode {
            GutterMode::Padding => {
                let half = (props.gutter as f64) / 2.0;
                format!("margin-left: -{}px; margin-right: -{}px;", half, half)
            }
            GutterMode::Gap => {
                format!("gap: {}px;", props.gutter)
            }
        }
    } else {
        String::new()
    };

    let full_style = if !props.style.is_empty() {
        format!("{} {}", gutter_style, props.style)
    } else {
        gutter_style
    };

    // 通过 context 向子 Col 传递 gutter 信息
    let _ctx = use_context_provider(|| GutterCtx {
        gutter: props.gutter,
        mode: props.gutter_mode,
    });

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            style: "{full_style}",
            {props.children}
        }
    }
}
