use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn CircularProgressPage() -> Element {
    rsx! {
div { id: "circular-progress", style: "margin-top: 64px;",
            h1 {
                "CircularProgress 环形进度"
            }
            p {
                "基于 SVG 绘制的圆环进度组件，支持标准环形与底部留缺口的仪表盘模式，可自定义尺寸、环宽、颜色与中心内容。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("传入 percent（0-100）展示环形进度，默认显示中心百分比。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(),
                        CircularProgress { percent: 25.0 }
                        CircularProgress { percent: 60.0 }
                        CircularProgress { percent: 100.0, color: "var(--ctrl-success)".to_string() }
                    }
                },
                code: "CircularProgress { percent: 25.0 }\nCircularProgress { percent: 60.0 }\nCircularProgress { percent: 100.0, color: \"var(--ctrl-success)\".to_string() }".to_string(),
            }

            DemoBox {
                title: "仪表盘模式".to_string(),
                description: Some("dashboard 为 true 时底部留 90° 缺口，适合展示评分、健康度等。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(),
                        CircularProgress { percent: 75.0, dashboard: true }
                        CircularProgress { percent: 90.0, dashboard: true, color: "var(--ctrl-warning)".to_string() }
                        CircularProgress { percent: 45.0, dashboard: true, color: "var(--ctrl-danger)".to_string() }
                    }
                },
                code: "CircularProgress { percent: 75.0, dashboard: true }\nCircularProgress { percent: 90.0, dashboard: true, color: \"var(--ctrl-warning)\".to_string() }".to_string(),
            }

            DemoBox {
                title: "尺寸与环宽".to_string(),
                description: Some("通过 size 与 stroke_width 调整圆环大小和粗细。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(), align: "center".to_string(),
                        CircularProgress { percent: 66.0, size: 72.0, stroke_width: 6.0 }
                        CircularProgress { percent: 66.0, size: 120.0, stroke_width: 10.0 }
                        CircularProgress { percent: 66.0, size: 160.0, stroke_width: 16.0 }
                    }
                },
                code: "CircularProgress { percent: 66.0, size: 72.0, stroke_width: 6.0 }\nCircularProgress { percent: 66.0, size: 160.0, stroke_width: 16.0 }".to_string(),
            }

            DemoBox {
                title: "自定义中心内容".to_string(),
                description: Some("通过 children 覆盖默认百分比文字，展示任意内容。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(),
                        CircularProgress { percent: 80.0, size: 140.0, color: "var(--ctrl-success)".to_string(),
                            div { style: "text-align:center;",
                                div { style: "font-size:28px; font-weight:700; color:var(--ctrl-success);", "80" }
                                div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary);", "健康分" }
                            }
                        }
                        CircularProgress { percent: 100.0, size: 140.0,
                            div { style: "font-size:32px;", "✓" }
                        }
                    }
                },
                code: "CircularProgress { percent: 80.0, size: 140.0,\n    div { div { \"80\" } div { \"健康分\" } }\n}".to_string(),
            }

            h2 { "CircularProgress Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("percent", "f64", "0.0", "进度值（0-100）"),
                ("size", "f64", "120.0", "直径（px）"),
                ("stroke_width", "f64", "8.0", "环宽度（px）"),
                ("color", "String", "var(--ctrl-primary)", "进度颜色"),
                ("track_color", "String", "var(--ctrl-bg-secondary)", "轨道颜色"),
                ("dashboard", "bool", "false", "仪表盘模式（底部留缺口）"),
                ("show_text", "bool", "true", "是否显示中心百分比"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Option<Element>", "None", "自定义中心内容"),
            ]}
        }
    }
}
