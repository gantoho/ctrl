use dioxus::prelude::*;

use super::row::{GutterCtx, GutterMode};

/// Col 栅格列 Props
#[derive(Props, PartialEq, Clone)]
pub struct ColProps {
    /// 列占位（1-24），默认 24
    #[props(default = 24)]
    pub span: u8,

    /// 列偏移（0-23），默认 0
    #[props(default = 0)]
    pub offset: u8,

    /// xs 断点 span（<576px），不设置则使用 span
    #[props(default = None)]
    pub xs: Option<u8>,

    /// sm 断点 span（≥576px）
    #[props(default = None)]
    pub sm: Option<u8>,

    /// md 断点 span（≥768px）
    #[props(default = None)]
    pub md: Option<u8>,

    /// lg 断点 span（≥992px）
    #[props(default = None)]
    pub lg: Option<u8>,

    /// xl 断点 span（≥1200px）
    #[props(default = None)]
    pub xl: Option<u8>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素
    pub children: Element,
}

/// 构建 Col 的 class 字符串（不含 span 时避免与内联冲突）
fn build_col_class(props: &ColProps, include_span: bool) -> String {
    let mut classes = String::from("ctrl-col");

    if include_span {
        let span = props.span.clamp(1, 24);
        classes.push_str(&format!(" ctrl-col--span-{span}"));
    }

    if let Some(s) = props.xs {
        classes.push_str(&format!(" ctrl-col--xs-{}", s.clamp(1, 24)));
    }
    if let Some(s) = props.sm {
        classes.push_str(&format!(" ctrl-col--sm-{}", s.clamp(1, 24)));
    }
    if let Some(s) = props.md {
        classes.push_str(&format!(" ctrl-col--md-{}", s.clamp(1, 24)));
    }
    if let Some(s) = props.lg {
        classes.push_str(&format!(" ctrl-col--lg-{}", s.clamp(1, 24)));
    }
    if let Some(s) = props.xl {
        classes.push_str(&format!(" ctrl-col--xl-{}", s.clamp(1, 24)));
    }

    if !props.class.is_empty() {
        classes.push_str(" ");
        classes.push_str(&props.class);
    }

    classes
}

/// Col 栅格列组件
#[allow(non_snake_case)]
pub fn Col(props: ColProps) -> Element {
    let span = props.span.clamp(1, 24);
    let offset = props.offset.clamp(0, 23);

    // CSS Grid 列线从 1 开始
    // 无 offset：class 上的 grid-column: span N 控制，内联不写
    // 有 offset：内联 grid-column: {start} / span {span}，class 不加 span 类
    let (classes, grid_style) = if offset > 0 {
        let start = offset + 1;
        (
            build_col_class(&props, false),
            format!("grid-column: {start} / span {span};"),
        )
    } else {
        (
            build_col_class(&props, true),
            String::new(),
        )
    };

    // 从父 Row 读取 gutter 信息，仅在 Padding 模式下加 padding
    let ctx = use_context::<GutterCtx>();
    let gutter_pad = if ctx.gutter > 0 && ctx.mode == GutterMode::Padding {
        let half = (ctx.gutter as f64) / 2.0;
        format!("padding-left: {}px; padding-right: {}px;", half, half)
    } else {
        String::new()
    };

    let full_style = if !props.style.is_empty() {
        format!("{} {} {}", grid_style, gutter_pad, props.style)
    } else {
        format!("{} {}", grid_style, gutter_pad)
    };

    rsx! {
        div {
            class: "{classes}",
            style: "{full_style}",
            {props.children}
        }
    }
}
