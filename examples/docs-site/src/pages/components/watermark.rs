use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn WatermarkPage() -> Element {
    rsx! {
        h1 { "Watermark 水印" }
        p { "在内容上覆盖水印，防止信息简单截图泄露，支持文字和图片水印，具备防删除保护能力。" }

        DemoBox { title: "文字水印".to_string(), description: None,
            demo: rsx! {
                Watermark { content: Some("abcdefghijklmnopqrstuvwxyz".to_string()), gap: 0,
                    div { style: "height:180px; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-lg);",
                        "带水印的内容区域"
                    }
                }
            },
            code: "Watermark { content: Some(\"Ctrl UI\"),\n    div { style: \"height:180px; ...\", \"带水印的内容区域\" }\n}".to_string(),
        }

        DemoBox { title: "自定义颜色".to_string(), description: Some("通过 color 属性自定义水印文字颜色。".to_string()),
            demo: rsx! {
                Watermark { content: Some("机密".to_string()), color: "rgba(239, 68, 68, 0.12)".to_string(),
                    div { style: "height:180px; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary);",
                        "红色水印"
                    }
                }
            },
            code: "Watermark { content: Some(\"机密\"), color: \"rgba(239, 68, 68, 0.12)\" }".to_string(),
        }

        DemoBox { title: "自定义旋转角度".to_string(), description: Some("通过 rotate 调整水印倾斜角度。拖动滑块查看不同角度效果。".to_string()),
            demo: rsx! {
                {
                    let mut angle = use_signal(|| -45i32);
                    rsx! {
                        div {
                            div { style: "margin-bottom:16px; display:flex; align-items:center; gap:12px;",
                                span { style: "font-size:14px; font-weight:500; white-space:nowrap;", "旋转角度: {angle}°" }
                                div { style: "flex:1;",
                                    Slider { value: angle(), min: -90, max: 90, step: 1, onchange: move |v: i32| angle.set(v) }
                                }
                            }
                            Watermark { content: Some("Ctrl UI".to_string()), rotate: angle(), gap: 0, color: "rgba(0, 0, 0, 0.08)".to_string(),
                                div { style: "height:200px; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary); text-align:center; padding:0 40px;",
                                    "拖动上方滑块调整旋转角度 (当前: {angle}°)"
                                }
                            }
                        }
                    }
                }
            },
            code: "Watermark { content: Some(\"Ctrl UI\"), rotate: -45, gap: 0 }".to_string(),
        }

        DemoBox { title: "密集水印".to_string(), description: Some("通过 gap 控制水印密度，值越小越密集。".to_string()),
            demo: rsx! {
                Watermark { content: Some("Ctrl".to_string()), gap: 60, color: "rgba(0, 0, 0, 0.06)".to_string(),
                    div { style: "height:180px; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary);",
                        "密集水印"
                    }
                }
            },
            code: "Watermark { content: Some(\"Ctrl\"), gap: 60 }".to_string(),
        }

        DemoBox { title: "防删除保护".to_string(), description: Some("默认启用防删除保护，水印层被删除或隐藏后会自动恢复。可通过 protection 属性关闭。".to_string()),
            demo: rsx! {
                Watermark { content: Some("防删除水印".to_string()), protection: true, check_interval: 500,
                    div { style: "height:180px; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary);",
                        "尝试删除或隐藏此水印（会自动恢复）"
                    }
                }
            },
            code: "Watermark { content: Some(\"防删除水印\"),\n    protection: true,\n    check_interval: 500,\n    // ...\n}".to_string(),
        }

        h2 { "Watermark Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("content", "Option<String>", "None", "水印文字内容"),
            ("image", "Option<String>", "None", "水印图片 URL（优先级高于文字）"),
            ("color", "String", "\"rgba(0,0,0,0.1)\"", "文字颜色"),
            ("font_size", "String", "\"14px\"", "字体大小"),
            ("font_family", "String", "\"sans-serif\"", "字体族"),
            ("rotate", "i32", "-30", "旋转角度（度）"),
            ("gap", "u32", "100", "水印块间距（px）"),
            ("z_index", "u32", "9999", "水印层 z-index"),
            ("protection", "bool", "true", "是否启用防删除保护"),
            ("check_interval", "u32", "500", "保护检查间隔（毫秒）"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}
    }
}