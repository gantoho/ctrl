use dioxus::prelude::*;
use ctrl_core::types::Size;

/// Select 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    /// 尺寸
    #[props(default = Size::default())]
    pub size: Size,

    /// 占位文本
    #[props(default = "请选择".to_string())]
    pub placeholder: String,

    /// 当前选中值
    #[props(default = "".to_string())]
    pub value: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 选中变化事件
    pub onchange: Option<EventHandler<String>>,

    /// 选项列表 (value, label, disabled)
    #[props(default = Vec::new())]
    pub options: Vec<(String, String, bool)>,
}

/// Select 下拉选择组件
#[allow(non_snake_case)]
pub fn Select(props: SelectProps) -> Element {
    const CSS: &str = include_str!("../../assets/select.css");
    let mut open = use_signal(|| false);

    let selected_label = props
        .options
        .iter()
        .find(|(v, _, _)| v == &props.value)
        .map(|(_, l, _)| l.clone())
        .unwrap_or(props.placeholder.clone());

    let mut trigger_classes = vec![
        "ctrl-select__trigger".to_string(),
        format!("ctrl-select__trigger--{}", match props.size {
            Size::Sm => "sm",
            Size::Md => "md",
            Size::Lg => "lg",
        }),
    ];
    if props.disabled {
        trigger_classes.push("ctrl-select__trigger--disabled".into());
    }
    let trigger_class = trigger_classes.join(" ");

    let mut container_class = "ctrl-select".to_string();
    if !props.class.is_empty() {
        container_class = format!("{} {}", container_class, props.class);
    }

    let arrow_class = if open() {
        "ctrl-select__arrow ctrl-select__arrow--open"
    } else {
        "ctrl-select__arrow"
    };

    let onchange = props.onchange.clone();
    let options = props.options.clone();
    let selected_val = props.value.clone();
    let disabled = props.disabled;

    rsx! {
        style { {CSS} }
        div {
            class: "{container_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            // 触发器
            div {
                class: "{trigger_class}",
                onclick: move |_| {
                    if !disabled {
                        open.set(!open());
                    }
                },
                span {
                    class: "ctrl-select__text",
                    if props.value.is_empty() {
                        span { class: "ctrl-select__placeholder", "{selected_label}" }
                    } else {
                        "{selected_label}"
                    }
                }
                svg {
                    class: "{arrow_class}",
                    width: "12",
                    height: "12",
                    view_box: "0 0 12 12",
                    fill: "none",
                    path {
                        d: "M3 4.5l3 3 3-3",
                        stroke: "var(--ctrl-text-secondary)",
                        stroke_width: "1.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                    }
                }
            }

            // 下拉面板
            if open() {
                div {
                    class: "ctrl-select__dropdown",
                    {
                        options.into_iter().map(move |(v, l, d)| {
                            let is_selected = v == selected_val;

                            let mut opt_classes = vec!["ctrl-select__option".to_string()];
                            if is_selected {
                                opt_classes.push("ctrl-select__option--selected".into());
                            }
                            if d {
                                opt_classes.push("ctrl-select__option--disabled".into());
                            }
                            let opt_class = opt_classes.join(" ");

                            let vc = v.clone();
                            let oc = onchange.clone();
                            let mut open_clone = open.clone();

                            rsx! {
                                div {
                                    key: "{v}",
                                    class: "{opt_class}",
                                    onclick: move |_| {
                                        if !d {
                                            if let Some(ref handler) = oc {
                                                handler.call(vc.clone());
                                            }
                                            open_clone.set(false);
                                        }
                                    },
                                    "{l}"
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}
