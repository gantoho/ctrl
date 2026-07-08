use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn SpotlightCardPage() -> Element {
    rsx! {
div { id: "spotlight-card", style: "margin-top: 64px;",
            h1 {
                "SpotlightCard 聚光卡片"
            }
            p {
                "鼠标在卡片内移动时，跟随光标渲染一束径向光晕，营造聚光灯质感，适合功能卡片、定价卡片等强调区域。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("移动鼠标到卡片上体验光标跟随光晕效果。".to_string()),
                demo: rsx! {
                    div { style: "max-width:320px;",
                        SpotlightCard {
                            h3 { style: "margin:0 0 8px 0; color:var(--ctrl-text);", "聚光灯卡片" }
                            p { style: "margin:0; color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm); line-height:1.6;",
                                "将鼠标移到卡片上，光晕会跟随光标位置移动，移出时自动淡出。"
                            }
                        }
                    }
                },
                code: "SpotlightCard {\n    h3 { \"聚光灯卡片\" }\n    p { \"将鼠标移到卡片上体验光晕效果。\" }\n}".to_string(),
            }

            DemoBox {
                title: "自定义光晕颜色".to_string(),
                description: Some("通过 glow_color 设置不同的光晕颜色。".to_string()),
                demo: rsx! {
                    Row { gutter: 16,
                        Col { span: 8,
                            SpotlightCard { glow_color: "var(--ctrl-success)".to_string(),
                                div { style: "text-align:center; color:var(--ctrl-text);",
                                    div { style: "font-size:28px; font-weight:700; color:var(--ctrl-success);", "99.9%" }
                                    div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary); margin-top:4px;", "服务可用性" }
                                }
                            }
                        }
                        Col { span: 8,
                            SpotlightCard { glow_color: "var(--ctrl-warning)".to_string(),
                                div { style: "text-align:center; color:var(--ctrl-text);",
                                    div { style: "font-size:28px; font-weight:700; color:var(--ctrl-warning);", "2.4M" }
                                    div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary); margin-top:4px;", "月活用户" }
                                }
                            }
                        }
                        Col { span: 8,
                            SpotlightCard { glow_color: "var(--ctrl-danger)".to_string(),
                                div { style: "text-align:center; color:var(--ctrl-text);",
                                    div { style: "font-size:28px; font-weight:700; color:var(--ctrl-danger);", "128ms" }
                                    div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary); margin-top:4px;", "平均响应" }
                                }
                            }
                        }
                    }
                },
                code: "SpotlightCard { glow_color: \"var(--ctrl-success)\".to_string(),\n    // 卡片内容\n}".to_string(),
            }

            DemoBox {
                title: "自定义光晕半径".to_string(),
                description: Some("通过 radius 控制光晕的大小范围。".to_string()),
                demo: rsx! {
                    Row { gutter: 16,
                        Col { span: 12,
                            SpotlightCard { radius: 120.0,
                                p { style: "margin:0; color:var(--ctrl-text);", "小光晕 radius = 120" }
                            }
                        }
                        Col { span: 12,
                            SpotlightCard { radius: 360.0,
                                p { style: "margin:0; color:var(--ctrl-text);", "大光晕 radius = 360" }
                            }
                        }
                    }
                },
                code: "SpotlightCard { radius: 120.0, ... }\nSpotlightCard { radius: 360.0, ... }".to_string(),
            }

            h2 { "SpotlightCard Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("glow_color", "String", "var(--ctrl-primary)", "光晕颜色"),
                ("radius", "f64", "240.0", "光晕半径（px）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "卡片内容"),
            ]}
        }
    }
}
