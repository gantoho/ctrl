use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn SpinPage() -> Element {
    let mut spinning = use_signal(|| true);
    rsx! {
        h1 { "Spin 加载中" }
        p { "包裹内容区域，在加载状态时显示旋转动画指示器和半透明遮罩。" }

        DemoBox { title: "基础用法".to_string(), description: None,
            demo: rsx! {
                Spin { spinning: true,
                    div { style: "height:120px; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary);",
                        "内容区域"
                    }
                }
            },
            code: "Spin { spinning: true,\n    div { \"内容区域\" }\n}".to_string(),
        }

        DemoBox { title: "尺寸".to_string(), description: Some("提供 sm / md / lg 三种尺寸。".to_string()),
            demo: rsx! {
                Space { direction: Direction::Horizontal, gap: "lg".to_string(),
                    Spin { spinning: true, size: "sm".to_string(),
                        div { style: "width:80px;height:80px;display:flex;align-items:center;justify-content:center;", "sm" }
                    }
                    Spin { spinning: true, size: "md".to_string(),
                        div { style: "width:80px;height:80px;display:flex;align-items:center;justify-content:center;", "md" }
                    }
                    Spin { spinning: true, size: "lg".to_string(),
                        div { style: "width:80px;height:80px;display:flex;align-items:center;justify-content:center;", "lg" }
                    }
                }
            },
            code: "Spin { spinning: true, size: \"sm\", div { ... } }\nSpin { spinning: true, size: \"md\", div { ... } }\nSpin { spinning: true, size: \"lg\", div { ... } }".to_string(),
        }

        DemoBox { title: "加载文案".to_string(), description: Some("通过 tip 设置加载提示文字。".to_string()),
            demo: rsx! {
                Spin { spinning: true, tip: "数据加载中，请稍候...".to_string(),
                    div { style: "height:120px; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary);",
                        "内容区域"
                    }
                }
            },
            code: "Spin { spinning: true, tip: \"数据加载中，请稍候...\",\n    div { \"内容区域\" }\n}".to_string(),
        }

        DemoBox { title: "自定义颜色".to_string(), description: Some("通过 color 属性自定义加载图标的颜色。".to_string()),
            demo: rsx! {
                Spin { spinning: true, color: "#e74c3c".to_string(),
                    div { style: "height:120px; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary);",
                        "内容区域"
                    }
                }
            },
            code: "Spin { spinning: true, color: \"#e74c3c\",\n    div { \"内容区域\" }\n}".to_string(),
        }

        DemoBox { title: "切换状态".to_string(), description: Some("通过 spinning 属性动态控制加载状态。".to_string()),
            demo: rsx! {
                div {
                    Button {
                        size: Size::Sm,
                        onclick: move |_| spinning.set(!spinning()),
                        if spinning() { "停止加载" } else { "开始加载" }
                    }
                    div { style: "margin-top:12px;",
                        Spin { spinning: spinning(),
                            div { style: "height:120px; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary);",
                                if spinning() { "加载中..." } else { "加载完成" }
                            }
                        }
                    }
                }
            },
            code: r#"let mut spinning = use_signal(|| true);
Spin { spinning: spinning(),
    div { "内容" }
}"#.to_string(),
        }

        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("spinning", "bool", "true", "是否显示加载状态"),
            ("tip", "Option<String>", "None", "加载提示文案"),
            ("size", "String", "\"md\"", "尺寸：sm / md / lg"),
            ("color", "Option<String>", "None", "自定义颜色"),
            ("delay", "u32", "0", "延迟显示时间（ms），0 不延迟"),
            ("class", "String", "\"\"", "自定义类名"),
            ("style", "String", "\"\"", "自定义样式"),
        ] }
    }
}
