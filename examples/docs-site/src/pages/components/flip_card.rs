use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn FlipCardPage() -> Element {
    rsx! {
div { id: "flip-card", style: "margin-top: 64px;",
            h1 {
                "FlipCard 翻转卡片"
            }
            p {
                "通过 3D 变换在正反面之间翻转，支持悬停或点击触发、横向或纵向翻转，常用于展示卡片详情、翻牌互动等。需通过 style 设置宽高。"
            }

            DemoBox {
                title: "悬停翻转".to_string(),
                description: Some("鼠标悬停时翻转到背面。".to_string()),
                demo: rsx! {
                    FlipCard {
                        style: "width:240px; height:160px;".to_string(),
                        front: rsx! {
                            div { style: "width:100%; height:100%; display:flex; align-items:center; justify-content:center; background:var(--ctrl-primary); color:#fff; font-size:18px; font-weight:600;",
                                "正面 · 悬停我"
                            }
                        },
                        back: rsx! {
                            div { style: "width:100%; height:100%; display:flex; align-items:center; justify-content:center; background:var(--ctrl-bg-secondary); color:var(--ctrl-text); font-size:14px; border:1px solid var(--ctrl-border);",
                                "背面 · 详细信息"
                            }
                        },
                    }
                },
                code: "FlipCard {\n    style: \"width:240px; height:160px;\".to_string(),\n    front: rsx! { /* 正面 */ },\n    back: rsx! { /* 背面 */ },\n}".to_string(),
            }

            DemoBox {
                title: "点击翻转".to_string(),
                description: Some("click_to_flip 时点击卡片翻转。".to_string()),
                demo: rsx! {
                    FlipCard {
                        click_to_flip: true,
                        style: "width:240px; height:160px;".to_string(),
                        front: rsx! {
                            div { style: "width:100%; height:100%; display:flex; align-items:center; justify-content:center; background:linear-gradient(135deg,#6366f1,#a855f7); color:#fff; font-weight:600;",
                                "点击翻转 ↻"
                            }
                        },
                        back: rsx! {
                            div { style: "width:100%; height:100%; display:flex; align-items:center; justify-content:center; background:var(--ctrl-bg-secondary); color:var(--ctrl-text); border:1px solid var(--ctrl-border);",
                                "再次点击返回"
                            }
                        },
                    }
                },
                code: "FlipCard {\n    click_to_flip: true,\n    front: rsx! { ... },\n    back: rsx! { ... },\n}".to_string(),
            }

            DemoBox {
                title: "纵向翻转".to_string(),
                description: Some("vertical 时绕水平轴翻转。".to_string()),
                demo: rsx! {
                    FlipCard {
                        vertical: true,
                        style: "width:240px; height:160px;".to_string(),
                        front: rsx! {
                            div { style: "width:100%; height:100%; display:flex; align-items:center; justify-content:center; background:var(--ctrl-success); color:#fff; font-weight:600;",
                                "正面（纵向）"
                            }
                        },
                        back: rsx! {
                            div { style: "width:100%; height:100%; display:flex; align-items:center; justify-content:center; background:var(--ctrl-bg-secondary); color:var(--ctrl-text); border:1px solid var(--ctrl-border);",
                                "背面（纵向）"
                            }
                        },
                    }
                },
                code: "FlipCard { vertical: true, front: rsx! { ... }, back: rsx! { ... } }".to_string(),
            }

            h2 { "FlipCard Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("front", "Element", "—", "正面内容"),
                ("back", "Element", "—", "背面内容"),
                ("click_to_flip", "bool", "false", "点击翻转（默认悬停翻转）"),
                ("vertical", "bool", "false", "纵向翻转（默认横向）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式（设置宽高）"),
            ]}
        }
    }
}
