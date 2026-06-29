use dioxus::prelude::*;
use ctrl_core::types::Size;

/// Segmented 分段控制器组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SegmentedProps {
    /// 当前选中值
    #[props(default = "".to_string())]
    pub value: String,

    /// 选项列表：[(label, value)]
    pub options: Vec<(String, String)>,

    /// 尺寸
    #[props(default = Size::Md)]
    pub size: Size,

    /// 是否占满宽度
    #[props(default = false)]
    pub block: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 值变化回调
    pub onchange: Option<EventHandler<String>>,
}

/// Segmented 分段控制器组件
///
/// 用于在多个选项之间切换，类似 iOS 的分段控件。
///
/// # 示例
///
/// ```rust
/// Segmented {
///     value: "day".to_string(),
///     options: vec![
///         ("日".to_string(), "day".to_string()),
///         ("周".to_string(), "week".to_string()),
///         ("月".to_string(), "month".to_string()),
///     ],
///     onchange: |v| log::info!("{}", v),
/// }
/// ```
#[allow(non_snake_case)]
pub fn Segmented(props: SegmentedProps) -> Element {
    const CSS: &str = include_str!("../../assets/segmented.css");
    let mut selected = use_signal(|| props.value.clone());

    let seg_class = {
        let mut c = format!("ctrl-segmented ctrl-segmented--{}", props.size);
        if props.block {
            c.push_str(" ctrl-segmented--block");
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    let onchange = props.onchange.clone();

    rsx! {
        style { {CSS} }
        div {
            class: "{seg_class}",
            style: "{props.style}",
            for (label, val) in props.options.iter() {
                {
                    let val = val.clone();
                    let label = label.clone();
                    let is_active = selected() == val;
                    let item_class = if is_active {
                        "ctrl-segmented__item ctrl-segmented__item--active".to_string()
                    } else if props.disabled {
                        "ctrl-segmented__item ctrl-segmented__item--disabled".to_string()
                    } else {
                        "ctrl-segmented__item".to_string()
                    };
                    let onchange = onchange.clone();
                    let disabled = props.disabled;

                    rsx! {
                        button {
                            key: "{val}",
                            class: "{item_class}",
                            disabled: disabled,
                            onclick: move |_| {
                                selected.set(val.clone());
                                if let Some(ref cb) = onchange {
                                    cb.call(val.clone());
                                }
                            },
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}