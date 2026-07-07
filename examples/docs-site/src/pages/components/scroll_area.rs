use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn ScrollAreaPage() -> Element {
    rsx! {
div { id: "scroll-area", style: "margin-top: 64px;",
            h1 {
                "ScrollArea 滚动区域"
            }
            p {
                "提供主题化滚动条的滚动容器，支持纵向/横向滚动、吸附对齐。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("设置 height，内容超出时自动出现主题化滚动条。".to_string()),
                demo: rsx! {
                    div { style: "max-width:400px;",
                        ScrollArea { height: "160px".to_string(), style: "border:1px solid var(--ctrl-border); border-radius:var(--ctrl-radius-md);".to_string(),
                            for i in 1..=12 {
                                div { key: "{i}",
                                    style: "padding:10px 16px; border-bottom:1px solid var(--ctrl-border); font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text);",
                                    "列表项 {i} —— 内容展示区域"
                                }
                            }
                        }
                    }
                },
                code: "ScrollArea { height: \"160px\".to_string(),\n    // 可滚动内容\n}".to_string(),
            }

            DemoBox {
                title: "横向滚动".to_string(),
                description: Some("horizontal 为 true 时启用横向滚动卡片列表。".to_string()),
                demo: rsx! {
                    div { style: "max-width:400px;",
                        ScrollArea { horizontal: true,
                            div { style: "display:flex; gap:12px; padding:8px 0; white-space:nowrap;",
                                for i in 1..=6 {
                                    div { key: "{i}",
                                        style: "flex-shrink:0; width:120px; height:100px; background:var(--ctrl-bg-secondary); border:1px solid var(--ctrl-border); border-radius:var(--ctrl-radius-md); display:flex; align-items:center; justify-content:center; font-size:var(--ctrl-font-size-md); color:var(--ctrl-text-secondary);",
                                        "卡片 {i}"
                                    }
                                }
                            }
                        }
                    }
                },
                code: "ScrollArea { horizontal: true,\n    div { style: \"display:flex; gap:12px;\", /* 卡片列表 */ }\n}".to_string(),
            }

            DemoBox {
                title: "双向滚动".to_string(),
                description: Some("通过 style 覆盖为 overflow:auto，同时启用纵向和横向滚动。".to_string()),
                demo: rsx! {
                    div { style: "max-width:400px;",
                        ScrollArea { height: "160px".to_string(), style: "overflow:auto; border:1px solid var(--ctrl-border); border-radius:var(--ctrl-radius-md);".to_string(),
                            div { style: "width:640px;",
                                for i in 1..=6 {
                                    div { key: "{i}",
                                        style: "padding:8px 16px; display:flex; gap:24px; border-bottom:1px solid var(--ctrl-border); font-size:var(--ctrl-font-size-sm);",
                                        span { style: "width:120px; flex-shrink:0; color:var(--ctrl-text);", "行 {i}" }
                                        span { style: "width:200px; flex-shrink:0; color:var(--ctrl-text-secondary);", "列 A 数据" }
                                        span { style: "width:200px; flex-shrink:0; color:var(--ctrl-text-secondary);", "列 B 数据" }
                                        span { style: "width:200px; flex-shrink:0; color:var(--ctrl-text-secondary);", "列 C 数据" }
                                    }
                                }
                            }
                        }
                    }
                },
                code: "ScrollArea { height: \"160px\".to_string(), style: \"overflow:auto;\".to_string(),\n    div { style: \"width:640px;\", /* 宽内容 */ }\n}".to_string(),
            }

            DemoBox {
                title: "吸附滚动".to_string(),
                description: Some("配合 style 传入 scroll-snap-type 启用滚动吸附，常用于轮播或引导区。".to_string()),
                demo: rsx! {
                    div { style: "max-width:400px;",
                        ScrollArea { horizontal: true, style: "scroll-snap-type:x mandatory;".to_string(),
                            div { style: "display:flex; gap:12px; padding:8px 0;",
                                for i in 1..=4 {
                                    div { key: "{i}",
                                        style: "flex-shrink:0; width:300px; height:120px; scroll-snap-align:start; background:linear-gradient(135deg, var(--ctrl-primary), color-mix(in srgb, var(--ctrl-primary) 60%, var(--ctrl-info))); border-radius:var(--ctrl-radius-md); display:flex; align-items:center; justify-content:center; font-size:var(--ctrl-font-size-lg); color:#fff; font-weight:600;",
                                        "Slide {i}"
                                    }
                                }
                            }
                        }
                    }
                },
                code: "ScrollArea { horizontal: true, style: \"scroll-snap-type:x mandatory;\".to_string(),\n    div { style: \"display:flex; gap:12px;\",\n        div { style: \"width:300px; scroll-snap-align:start;\", \"Slide 1\" }\n    }\n}".to_string(),
            }

            h2 { "ScrollArea Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("height", "String", "\"\"", "固定高度（如 \"160px\"）"),
                ("max_height", "String", "\"\"", "最大高度"),
                ("horizontal", "bool", "false", "是否开启横向滚动"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "滚动内容"),
            ]}
        }
    }
}
