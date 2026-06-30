use dioxus::prelude::*;
use ctrl_core::types::Size;

/// 描述列表项：标签 + 值内容
#[derive(Clone, PartialEq)]
pub struct DescriptionsItem {
    /// 标签
    pub label: String,
    /// 值内容
    pub value: Element,
}

/// Descriptions 描述列表 Props
#[derive(Props, PartialEq, Clone)]
pub struct DescriptionsProps {
    /// 标题
    #[props(default = "".to_string())]
    pub title: String,

    /// 尺寸
    #[props(default = Size::default())]
    pub size: Size,

    /// 是否显示边框（默认 true）
    #[props(default = true)]
    pub bordered: bool,

    /// 标签列宽度比例（1-10，默认 3，即 30%）
    #[props(default = 3)]
    pub label_width: u8,

    /// 描述项列表
    #[props(default = Vec::new())]
    pub items: Vec<DescriptionsItem>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// Descriptions 描述列表组件
///
/// 用于标签-值成对展示，适合详情页信息陈列。
///
/// ### 使用示例
/// ```rust
/// Descriptions {
///     title: "用户信息".to_string(),
///     items: vec![
///         DescriptionsItem { label: "姓名".into(), value: rsx! { "张三" } },
///         DescriptionsItem { label: "年龄".into(), value: rsx! { "28" } },
///         DescriptionsItem { label: "地址".into(), value: rsx! { "上海市" } },
///     ],
/// }
/// ```
#[allow(non_snake_case)]
pub fn Descriptions(props: DescriptionsProps) -> Element {
    const CSS: &str = include_str!("../../assets/descriptions.css");

    let size_class = match props.size {
        Size::Sm => "ctrl-descriptions--sm",
        Size::Lg => "ctrl-descriptions--lg",
        _ => "",
    };

    let bordered_class = if !props.bordered {
        "ctrl-descriptions--borderless"
    } else {
        ""
    };

    let wrapper_class = {
        let mut c = String::from("ctrl-descriptions");
        if !size_class.is_empty() {
            c.push_str(" ");
            c.push_str(size_class);
        }
        if !bordered_class.is_empty() {
            c.push_str(" ");
            c.push_str(bordered_class);
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    let label_pct = props.label_width.clamp(1, 10) as u32 * 10;
    let value_pct = 100 - label_pct;

    let has_title = !props.title.is_empty();

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            if has_title {
                h3 {
                    style: "font-size: var(--ctrl-font-size-lg, 16px); font-weight: 600; color: var(--ctrl-text); margin: 0 0 16px 0;",
                    "{props.title}"
                }
            }
            table { class: "ctrl-descriptions__table",
                tbody {
                    for (i, item) in props.items.iter().enumerate() {
                        tr { key: "desc-row-{i}", class: "ctrl-descriptions__row",
                            td {
                                class: "ctrl-descriptions__label",
                                style: "width: {label_pct}%;",
                                "{item.label}"
                            }
                            td {
                                class: "ctrl-descriptions__value",
                                style: "width: {value_pct}%;",
                                {item.value.clone()}
                            }
                        }
                    }
                }
            }
        }
    }
}
