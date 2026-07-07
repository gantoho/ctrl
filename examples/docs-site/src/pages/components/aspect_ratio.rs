use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn AspectRatioPage() -> Element {
    rsx! {
div { id: "aspect-ratio", style: "margin-top: 64px;",
            h1 {
                "AspectRatio 宽高比"
            }
            p {
                "保持固定宽高比的容器组件，适合嵌入图片、视频、地图等需要等比缩放的内容。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 ratio 属性设置宽高比。".to_string()),
                demo: rsx! {
                    Row { gutter: 16,
                        Col { span: 8,
                            AspectRatio { ratio: "16 / 9".to_string(),
                                div { style: "background:var(--ctrl-bg-secondary); border:1px solid var(--ctrl-border); border-radius:var(--ctrl-radius-md); display:flex; align-items:center; justify-content:center; width:100%; height:100%;",
                                    span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-md);", "16:9" }
                                }
                            }
                        }
                        Col { span: 8,
                            AspectRatio { ratio: "4 / 3".to_string(),
                                div { style: "background:var(--ctrl-bg-secondary); border:1px solid var(--ctrl-border); border-radius:var(--ctrl-radius-md); display:flex; align-items:center; justify-content:center; width:100%; height:100%;",
                                    span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-md);", "4:3" }
                                }
                            }
                        }
                        Col { span: 8,
                            AspectRatio { ratio: "1 / 1".to_string(),
                                div { style: "background:var(--ctrl-bg-secondary); border:1px solid var(--ctrl-border); border-radius:var(--ctrl-radius-md); display:flex; align-items:center; justify-content:center; width:100%; height:100%;",
                                    span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-md);", "1:1" }
                                }
                            }
                        }
                    }
                },
                code: "AspectRatio { ratio: \"16 / 9\".to_string(),\n    div { /* 你的内容 */ }\n}".to_string(),
            }

            DemoBox {
                title: "图片容器".to_string(),
                description: Some("保证图片在不同宽度下保持原始宽高比，配合 object-fit 使用。".to_string()),
                demo: rsx! {
                    Row { gutter: 16,
                        Col { span: 12,
                            AspectRatio { ratio: "16 / 9".to_string(),
                                img {
                                    src: "https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=600&h=338&fit=crop",
                                    alt: "风景图",
                                    style: "width:100%; height:100%; object-fit:cover;"
                                }
                            }
                        }
                        Col { span: 12,
                            AspectRatio { ratio: "1 / 1".to_string(),
                                img {
                                    src: "https://images.unsplash.com/photo-1469474968028-56623f02e42e?w=400&h=400&fit=crop",
                                    alt: "自然图",
                                    style: "width:100%; height:100%; object-fit:cover;"
                                }
                            }
                        }
                    }
                },
                code: "AspectRatio { ratio: \"16 / 9\".to_string(),\n    img { src: \"...\", style: \"width:100%; height:100%; object-fit:cover;\" }\n}".to_string(),
            }

            DemoBox {
                title: "视频/iframe 容器".to_string(),
                description: Some("通用适配，可嵌入 iframe 地图或视频播放器。".to_string()),
                demo: rsx! {
                    div { style: "max-width:560px;",
                        AspectRatio { ratio: "16 / 9".to_string(),
                            div { style: "background:var(--ctrl-bg-secondary); border:1px dashed var(--ctrl-border); border-radius:var(--ctrl-radius-md); display:flex; align-items:center; justify-content:center; width:100%; height:100%;",
                                span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "视频 / iframe 占位区域" }
                            }
                        }
                    }
                },
                code: "AspectRatio { ratio: \"16 / 9\".to_string(),\n    iframe { src: \"https://...\", style: \"width:100%; height:100%; border:none;\" }\n}".to_string(),
            }

            DemoBox {
                title: "自定义比例".to_string(),
                description: Some("支持任意 CSS aspect-ratio 值，如 21/9、3/2 等。".to_string()),
                demo: rsx! {
                    Row { gutter: 16,
                        Col { span: 12,
                            AspectRatio { ratio: "21 / 9".to_string(),
                                div { style: "background:var(--ctrl-bg-secondary); border:1px solid var(--ctrl-border); border-radius:var(--ctrl-radius-md); display:flex; align-items:center; justify-content:center; width:100%; height:100%;",
                                    span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "超宽 21:9" }
                                }
                            }
                        }
                        Col { span: 12,
                            AspectRatio { ratio: "3 / 2".to_string(),
                                div { style: "background:var(--ctrl-bg-secondary); border:1px solid var(--ctrl-border); border-radius:var(--ctrl-radius-md); display:flex; align-items:center; justify-content:center; width:100%; height:100%;",
                                    span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "3:2" }
                                }
                            }
                        }
                    }
                },
                code: "AspectRatio { ratio: \"21 / 9\".to_string(), ... }\nAspectRatio { ratio: \"3 / 2\".to_string(), ... }".to_string(),
            }

            h2 { "AspectRatio Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("ratio", "String", "\"16 / 9\"", "宽高比（如 \"16 / 9\"、\"4 / 3\"、\"1 / 1\"）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "内部内容"),
            ]}
        }
    }
}
