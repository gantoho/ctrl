use dioxus::prelude::*;

/// Rate 评分组件属性
#[derive(Props, PartialEq, Clone)]
pub struct RateProps {
    /// 当前分值
    #[props(default = 0.0)]
    pub value: f64,

    /// 星星总数
    #[props(default = 5)]
    pub count: i32,

    /// 是否允许半星
    #[props(default = false)]
    pub allow_half: bool,

    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否显示分值文字
    #[props(default = false)]
    pub show_text: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 值变化回调
    pub onchange: Option<EventHandler<f64>>,

    /// 自定义全选图标 URL
    #[props(default = None)]
    pub icon_full: Option<String>,

    /// 自定义半选图标 URL（仅 allow_half 为 true 时生效）
    #[props(default = None)]
    pub icon_half: Option<String>,

    /// 自定义未选图标 URL
    #[props(default = None)]
    pub icon_empty: Option<String>,
}

/// 渲染单个星星
fn render_star(
    index: i32,
    current: f64,
    allow_half: bool,
    interactive: bool,
    icon_full: &Option<String>,
    icon_half: &Option<String>,
    icon_empty: &Option<String>,
) -> Element {
    let star_index = (index + 1) as f64;
    let has_custom_icons = icon_full.is_some() || icon_half.is_some() || icon_empty.is_some();

    if allow_half && current >= star_index - 0.5 && current < star_index {
        // 半星
        if has_custom_icons {
            let half_src = icon_half
                .clone()
                .or_else(|| icon_full.clone())
                .unwrap_or_default();
            rsx! {
                span { class: "ctrl-rate__star",
                    img { src: "{half_src}", class: "ctrl-rate__star-img" }
                }
            }
        } else {
            rsx! {
                span {
                    class: "ctrl-rate__star ctrl-rate__star--half",
                    "★"
                    span { class: "ctrl-rate__star-half", "★" }
                }
            }
        }
    } else if current >= star_index {
        // 全星
        if let Some(ref icon) = icon_full {
            rsx! {
                span { class: "ctrl-rate__star",
                    img { src: "{icon}", class: "ctrl-rate__star-img" }
                }
            }
        } else {
            rsx! {
                span {
                    class: "ctrl-rate__star ctrl-rate__star--active",
                    "★"
                }
            }
        }
    } else {
        // 空星
        if let Some(ref icon) = icon_empty {
            let star_class = if interactive {
                "ctrl-rate__star"
            } else {
                "ctrl-rate__star ctrl-rate__star--readonly"
            };
            rsx! {
                span { class: "{star_class}",
                    img { src: "{icon}", class: "ctrl-rate__star-img" }
                }
            }
        } else {
            let star_class = if interactive {
                "ctrl-rate__star"
            } else {
                "ctrl-rate__star ctrl-rate__star--readonly"
            };
            rsx! {
                span { class: "{star_class}", "★" }
            }
        }
    }
}

/// Rate 评分组件
#[allow(non_snake_case)]
pub fn Rate(props: RateProps) -> Element {
    const CSS: &str = include_str!("../../assets/rate.css");
    let mut value = use_signal(|| props.value);
    let mut hover_value = use_signal(|| 0.0f64);

    // 同步外部 prop 更新到内部信号
    use_effect(use_reactive(&props.value, move |v| {
        value.set(v);
    }));

    let is_interactive = !props.readonly && !props.disabled;

    let rate_class = {
        let mut c = String::from("ctrl-rate");
        if props.disabled {
            c.push_str(" ctrl-rate--disabled");
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    let text_labels = ["极差", "失望", "一般", "满意", "惊喜"];

    let display_text = if props.show_text {
        let dv = if hover_value() > 0.0 { hover_value() } else { value() };
        if dv > 0.0 && dv <= 5.0 {
            let idx = (dv.ceil() as usize).saturating_sub(1).min(4);
            text_labels[idx].to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{rate_class}",
            style: "{props.style}",
            for i in 0..props.count {
                {
                    let idx = i;
                    let display_val = if hover_value() > 0.0 { hover_value() } else { value() };
                    let star = render_star(
                        idx,
                        display_val,
                        props.allow_half,
                        is_interactive,
                        &props.icon_full,
                        &props.icon_half,
                        &props.icon_empty,
                    );

                    if is_interactive && props.allow_half {
                        // 半星模式：将每颗星分为左右两个热区，精确选中 0.5 或 1.0
                        rsx! {
                            span {
                                key: "{idx}",
                                class: "ctrl-rate__star-item",
                                // 左半区 → idx + 0.5
                                span {
                                    class: "ctrl-rate__star-half-zone",
                                    onmouseenter: move |_| hover_value.set(idx as f64 + 0.5),
                                    onmouseleave: move |_| {
                                        if hover_value() == idx as f64 + 0.5 {
                                            hover_value.set(0.0);
                                        }
                                    },
                                    onclick: move |_| {
                                        let new_val = idx as f64 + 0.5;
                                        value.set(new_val);
                                        hover_value.set(0.0);
                                        if let Some(ref cb) = props.onchange {
                                            cb.call(new_val);
                                        }
                                    },
                                }
                                // 右半区 → idx + 1.0
                                span {
                                    class: "ctrl-rate__star-full-zone",
                                    onmouseenter: move |_| hover_value.set(idx as f64 + 1.0),
                                    onmouseleave: move |_| {
                                        if hover_value() == idx as f64 + 1.0 {
                                            hover_value.set(0.0);
                                        }
                                    },
                                    onclick: move |_| {
                                        let new_val = idx as f64 + 1.0;
                                        value.set(new_val);
                                        hover_value.set(0.0);
                                        if let Some(ref cb) = props.onchange {
                                            cb.call(new_val);
                                        }
                                    },
                                }
                                {star}
                            }
                        }
                    } else if is_interactive {
                        // 非半星模式：整颗星响应
                        rsx! {
                            span {
                                key: "{idx}",
                                onmouseenter: move |_| hover_value.set((idx + 1) as f64),
                                onmouseleave: move |_| {
                                    if hover_value() == (idx + 1) as f64 {
                                        hover_value.set(0.0);
                                    }
                                },
                                onclick: move |_| {
                                    let new_val = (idx + 1) as f64;
                                    value.set(new_val);
                                    hover_value.set(0.0);
                                    if let Some(ref cb) = props.onchange {
                                        cb.call(new_val);
                                    }
                                },
                                {star}
                            }
                        }
                    } else {
                        rsx! {
                            span { key: "{idx}", {star} }
                        }
                    }
                }
            }
            if props.show_text {
                span {
                    class: "ctrl-rate__text",
                    "{display_text}"
                }
            }
        }
    }
}