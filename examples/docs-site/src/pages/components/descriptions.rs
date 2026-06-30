use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn DescriptionsPage() -> Element {
    rsx! {
div { id: "descriptions", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Descriptions 描述列表"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "用于标签-值成对展示信息。适合详情页，支持标题、尺寸、无边框模式和标签列宽度控制。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("使用 items 数组传递标签-值对。".to_string()),
                demo: rsx! {
                    Descriptions {
                        title: "用户信息".to_string(),
                        items: vec![
                            DescriptionsItem { label: "姓名".into(), value: rsx! { "张三" } },
                            DescriptionsItem { label: "手机号".into(), value: rsx! { "138****8888" } },
                            DescriptionsItem { label: "部门".into(), value: rsx! { "技术部" } },
                            DescriptionsItem { label: "地址".into(), value: rsx! { "上海市浦东新区" } },
                        ],
                    }
                },
                code: r#"Descriptions {
    title: "用户信息".to_string(),
    items: vec![
        DescriptionsItem { label: "姓名".into(), value: rsx! { "张三" } },
        DescriptionsItem { label: "手机号".into(), value: rsx! { "138****8888" } },
        DescriptionsItem { label: "部门".into(), value: rsx! { "技术部" } },
        DescriptionsItem { label: "地址".into(), value: rsx! { "上海市浦东新区" } },
    ],
}"#.to_string(),
            }

            DemoBox {
                title: "无边框".to_string(),
                description: Some("设置 bordered = false 移除边框和背景色。".to_string()),
                demo: rsx! {
                    Descriptions {
                        title: "订单信息".to_string(),
                        bordered: false,
                        items: vec![
                            DescriptionsItem { label: "订单号".into(), value: rsx! { "20240628001" } },
                            DescriptionsItem { label: "状态".into(), value: rsx! {
                                Tag { color: "var(--ctrl-success)".to_string(), "已完成" }
                            }},
                            DescriptionsItem { label: "金额".into(), value: rsx! { "¥ 1,280.00" } },
                            DescriptionsItem { label: "创建时间".into(), value: rsx! { "2024-06-28 14:30" } },
                        ],
                    }
                },
                code: r#"Descriptions {
    title: "订单信息".to_string(),
    bordered: false,
    items: vec![
        DescriptionsItem { label: "订单号".into(), value: rsx! { "20240628001" } },
        DescriptionsItem { label: "状态".into(), value: rsx! { Tag { color: "var(--ctrl-success)".into(), "已完成" } } },
        DescriptionsItem { label: "金额".into(), value: rsx! { "¥ 1,280.00" } },
        DescriptionsItem { label: "创建时间".into(), value: rsx! { "2024-06-28 14:30" } },
    ],
}"#.to_string(),
            }

            DemoBox {
                title: "三种尺寸".to_string(),
                description: Some("通过 size 控制尺寸：Sm / Md（默认） / Lg。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 24px;",
                        Descriptions {
                            title: "小尺寸（Sm）".to_string(),
                            size: Size::Sm,
                            items: vec![
                                DescriptionsItem { label: "项目".into(), value: rsx! { "Ctrl UI" } },
                                DescriptionsItem { label: "版本".into(), value: rsx! { "1.0.0" } },
                            ],
                        }
                        Descriptions {
                            title: "大尺寸（Lg）".to_string(),
                            size: Size::Lg,
                            items: vec![
                                DescriptionsItem { label: "项目".into(), value: rsx! { "Ctrl UI" } },
                                DescriptionsItem { label: "版本".into(), value: rsx! { "1.0.0" } },
                            ],
                        }
                    }
                },
                code: r#"// Sm 小尺寸
Descriptions {
    title: "...".to_string(),
    size: Size::Sm,
    items: vec![...],
}
// Lg 大尺寸
Descriptions {
    title: "...".to_string(),
    size: Size::Lg,
    items: vec![...],
}"#.to_string(),
            }

            DemoBox {
                title: "自定义列宽".to_string(),
                description: Some("通过 label_width（1-10）设置标签列宽度比例，默认 3（30%）。".to_string()),
                demo: rsx! {
                    Descriptions {
                        title: "资产信息".to_string(),
                        label_width: 5,
                        items: vec![
                            DescriptionsItem { label: "可用余额".into(), value: rsx! { "¥ 12,580.00" } },
                            DescriptionsItem { label: "冻结金额".into(), value: rsx! { "¥ 0.00" } },
                            DescriptionsItem { label: "累计收益".into(), value: rsx! { "¥ 3,245.68" } },
                        ],
                    }
                },
                code: r#"Descriptions {
    title: "资产信息".to_string(),
    label_width: 5,   // 标签 50%，值 50%
    items: vec![
        DescriptionsItem { label: "可用余额".into(), value: rsx! { "¥ 12,580.00" } },
    ],
}"#.to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Descriptions Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "标题"),
                ("size", "Size", "Md", "尺寸：Sm / Md / Lg"),
                ("bordered", "bool", "true", "是否显示边框"),
                ("label_width", "u8", "3", "标签列宽度比例（1-10，如 3=30%）"),
                ("items", "Vec<DescriptionsItem>", "[]", "描述项数组"),
                ("class", "String", "\"\"", "自定义 CSS 类名"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "DescriptionsItem" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("label", "String", "—", "标签文本"),
                ("value", "Element", "—", "值内容（支持任意 Dioxus 元素）"),
            ]}
        }
    }
}
