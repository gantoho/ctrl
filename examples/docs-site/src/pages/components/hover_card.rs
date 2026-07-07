use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn HoverCardPage() -> Element {
    rsx! {
div { id: "hover-card", style: "margin-top: 64px;",
            h1 {
                "HoverCard 悬停卡片"
            }
            p {
                "鼠标悬停时显示的卡片，用于展示用户名片、商品预览、内容摘要等上下文信息。"
            }

            DemoBox {
                title: "用户名片".to_string(),
                description: Some("鼠标悬浮在触发元素上时展示用户信息卡片。".to_string()),
                demo: rsx! {
                    Space { gap: "lg".to_string(),
                        for i in 0..3 {
                            HoverCard {
                                key: "{i}",
                                placement: Placement::Bottom,
                                content: rsx! {
                                    div { style: "width: 240px;",
                                        div { style: "display: flex; align-items: center; gap: 12px; margin-bottom: 12px;",
                                            Avatar { size: Size::Lg, src: format!("https://i.pravatar.cc/96?img={}", i + 1), alt: "用户头像".to_string(), "" }
                                            div {
                                                div { style: "font-weight: 600; color: var(--ctrl-text);",
                                                    {["Alice Chen", "Bob Wang", "Carol Li"][i]}
                                                }
                                                div { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                                                    {["@alice", "@bob", "@carol"][i]}
                                                }
                                            }
                                        }
                                        p { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin: 0 0 12px 0; line-height: 1.5;",
                                            {["全栈工程师，热爱开源，专注于 Rust + TypeScript 技术栈。", "后端架构师，分布式系统与微服务领域专家。", "前端技术专家，WebGL 与数据可视化方向。"][i]}
                                        }
                                        Space { gap: "xs".to_string(),
                                            Tag { color: "var(--ctrl-primary)".to_string(), "Rust" }
                                            Tag { color: "var(--ctrl-success)".to_string(), "Dioxus" }
                                        }
                                    }
                                },
                                Avatar { size: Size::Lg, src: format!("https://i.pravatar.cc/96?img={}", i + 1), alt: "用户头像".to_string(), "" }
                            }
                        }
                    }
                },
                code: "HoverCard { placement: Placement::Bottom, content: rsx! { /* 名片内容 */ },\n    Avatar { size: Size::Lg, src: \"...\".to_string(), \"\" }\n}".to_string(),
            }

            DemoBox {
                title: "商品预览".to_string(),
                description: Some("悬浮在文字链接上时展示商品的图片、价格与描述。".to_string()),
                demo: rsx! {
                    HoverCard {
                        placement: Placement::Top,
                        content: rsx! {
                            div { style: "width: 220px;",
                                div { style: "width: 100%; height: 120px; border-radius: var(--ctrl-radius-md); background: var(--ctrl-bg-secondary); display: flex; align-items: center; justify-content: center; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm); margin-bottom: 12px;",
                                    "商品图"
                                }
                                div { style: "font-weight: 600; color: var(--ctrl-text); margin-bottom: 4px;",
                                    "Ctrl UI Pro 授权"
                                }
                                div { style: "display: flex; align-items: baseline; gap: 8px;",
                                    span { style: "font-size: 20px; font-weight: 700; color: var(--ctrl-danger);", "¥299" }
                                    span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); text-decoration: line-through;", "¥599" }
                                }
                            }
                        },
                        Button { variant: Variant::Outline, size: Size::Sm, "查看商品" }
                    }
                },
                code: "HoverCard { placement: Placement::Top, content: rsx! { /* 商品预览 */ },\n    Button { variant: Variant::Outline, size: Size::Sm, \"查看商品\" }\n}".to_string(),
            }

            DemoBox {
                title: "放置方向".to_string(),
                description: Some("通过 placement 控制卡片相对触发元素的方向。".to_string()),
                demo: rsx! {
                    Space { gap: "sm".to_string(),
                        HoverCard { placement: Placement::Top, content: rsx! { span { "向上弹出" } },
                            Button { variant: Variant::Outline, size: Size::Sm, "Top" }
                        }
                        HoverCard { placement: Placement::Bottom, content: rsx! { span { "向下弹出" } },
                            Button { variant: Variant::Outline, size: Size::Sm, "Bottom" }
                        }
                        HoverCard { placement: Placement::Left, content: rsx! { span { "向左弹出" } },
                            Button { variant: Variant::Outline, size: Size::Sm, "Left" }
                        }
                        HoverCard { placement: Placement::Right, content: rsx! { span { "向右弹出" } },
                            Button { variant: Variant::Outline, size: Size::Sm, "Right" }
                        }
                    }
                },
                code: "HoverCard { placement: Placement::Top, content: rsx! { span { \"内容\" } },\n    Button { \"Top\" }\n}".to_string(),
            }

            h2 { "HoverCard Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("placement", "Placement", "Top", "卡片弹出方向（Top / Bottom / Left / Right）"),
                ("content", "Element", "—", "卡片内容"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "触发元素"),
            ]}
        }
    }
}
