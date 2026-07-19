use dioxus::prelude::*;
use wasm_bindgen::UnwrapThrowExt;

/// 一个颜色样本
#[derive(Clone, PartialEq)]
pub struct ColorSample {
    /// 颜色值（hex，如 "#3b82f6" 或 CSS 颜色值）
    pub color: String,
    /// 标签（如 "Blue-500"）
    pub label: String,
    /// 额外文本（如预设名称）
    pub extra: String,
}

impl ColorSample {
    pub fn new(color: impl Into<String>, label: impl Into<String>) -> Self {
        Self { color: color.into(), label: label.into(), extra: String::new() }
    }

    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = extra.into();
        self
    }
}

/// ColorSwatch 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ColorSwatchProps {
    /// 颜色样本列表
    pub colors: Vec<ColorSample>,

    /// 是否显示 hex 值文本
    #[props(default = true)]
    pub show_value: bool,

    /// 每列数量
    #[props(default = 4)]
    pub columns: u32,

    /// 色块宽度（CSS 尺寸）
    #[props(default = "64px".to_string())]
    pub swatch_size: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// ColorSwatch 色卡组件
///
/// 颜色样本网格，点击任意色块复制其 hex / CSS 颜色值到剪贴板。
/// 常用于设计系统展示或主题色概览。
#[allow(non_snake_case)]
pub fn ColorSwatch(props: ColorSwatchProps) -> Element {
    const CSS: &str = include_str!("../../assets/color_swatch.css");

    let copied = use_signal(|| None::<usize>);

    let container_class = if props.class.is_empty() {
        "ctrl-color-swatch".to_string()
    } else {
        format!("ctrl-color-swatch {}", props.class)
    };

    let vars = format!(
        "grid-template-columns: repeat({}, 1fr); --ctrl-color-swatch-size: {}; {}",
        props.columns, props.swatch_size, props.style
    );

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{vars}",
            for (i, sample) in props.colors.iter().enumerate() {
                        {
                            let color_style = sample.color.clone();
                            let color = sample.color.clone();
                            let label = sample.label.clone();
                            let extra = sample.extra.clone();
                            let is_copied = copied() == Some(i);

                            rsx! {
                                div {
                                    key: "{i}",
                                    class: if is_copied { "ctrl-color-swatch__item is-copied" } else { "ctrl-color-swatch__item" },
                                    onclick: move |_| {
                                        let c = color.clone();
                                        wasm_bindgen_futures::spawn_local(async move {
                                            let clipboard = web_sys::window()
                                                .unwrap_throw()
                                                .navigator()
                                                .clipboard();
                                            let _ = clipboard.write_text(&c);
                                        });
                                        let mut c = copied;
                                        c.set(Some(i));
                                    },
                                    div {
                                        class: "ctrl-color-swatch__block",
                                        style: "background-color:{color_style};",
                                    }
                                    if props.show_value {
                                        span { class: "ctrl-color-swatch__label", "{label}" }
                                        if !extra.is_empty() {
                                            span { class: "ctrl-color-swatch__extra", "{extra}" }
                                        }
                                    }
                                    if is_copied {
                                        div {
                                            class: "ctrl-color-swatch__copied",
                                            "已复制"
                                        }
                                    }
                                }
                            }
                        }
            }
        }
    }
}
