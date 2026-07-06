use dioxus::prelude::*;
use ctrl::prelude::*;
use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn SplitterPage() -> Element {
    rsx! {
        h1 { "Splitter 分隔面板" }
        p { "可拖拽的分隔面板布局，用于创建可调整大小的多面板界面。支持水平/垂直布局、折叠面板和嵌套使用。" }

        h2 { "基本用法" }

        DemoBox {
            title: "水平布局".to_string(),
            description: Some("默认方向为水平排列，拖拽中间的分隔条调整两侧面板大小。".to_string()),
            demo: rsx! {
                div { style: "width: 100%; height: 200px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md);",
                    Splitter { direction: Direction::Horizontal,
                        SplitterPanel { default_size: 30.0, min_size: 10.0,
                            div { style: "padding: 16px;",
                                p { "左侧面板 30%" }
                            }
                        }
                        SplitterPanel { default_size: 70.0, min_size: 20.0,
                            div { style: "padding: 16px;",
                                p { "右侧面板 70%" }
                            }
                        }
                    }
                }
            },
            code: "Splitter { direction: Direction::Horizontal,\n    SplitterPanel { default_size: 30.0, min_size: 10.0,\n        div { \"左侧面板 30%\" }\n    }\n    SplitterPanel { default_size: 70.0, min_size: 20.0,\n        div { \"右侧面板 70%\" }\n    }\n}".to_string(),
        }

        DemoBox {
            title: "垂直布局".to_string(),
            description: Some("设置 direction 为 Vertical 垂直排列。".to_string()),
            demo: rsx! {
                div { style: "width: 100%; height: 250px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md);",
                    Splitter { direction: Direction::Vertical,
                        SplitterPanel { default_size: 40.0, min_size: 10.0,
                            div { style: "padding: 16px;",
                                p { "上方面板 40%" }
                            }
                        }
                        SplitterPanel { default_size: 60.0, min_size: 20.0,
                            div { style: "padding: 16px;",
                                p { "下方面板 60%" }
                            }
                        }
                    }
                }
            },
            code: "Splitter { direction: Direction::Vertical,\n    SplitterPanel { default_size: 40.0,\n        div { \"上面板 40%\" }\n    }\n    SplitterPanel { default_size: 60.0,\n        div { \"下面板 60%\" }\n    }\n}".to_string(),
        }

        DemoBox {
            title: "可折叠面板".to_string(),
            description: Some("设置 collapsible 为 true 可在分隔条上显示折叠按钮。".to_string()),
            demo: rsx! {
                div { style: "width: 100%; height: 200px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md);",
                    Splitter { direction: Direction::Horizontal,
                        SplitterPanel { collapsible: true, default_size: 33.0, min_size: 10.0,
                            div { style: "padding: 16px;",
                                p { "面板 A" }
                            }
                        }
                        SplitterPanel { collapsible: true, default_size: 33.0, min_size: 10.0,
                            div { style: "padding: 16px;",
                                p { "面板 B" }
                            }
                        }
                        SplitterPanel { default_size: 34.0, min_size: 10.0,
                            div { style: "padding: 16px;",
                                p { "面板 C" }
                            }
                        }
                    }
                }
            },
            code: "Splitter {\n    SplitterPanel { collapsible: true, default_size: 33.0,\n        div { \"面板 A\" }\n    }\n    SplitterPanel { collapsible: true, default_size: 33.0,\n        div { \"面板 B\" }\n    }\n    SplitterPanel { default_size: 34.0,\n        div { \"面板 C\" }\n    }\n}".to_string(),
        }

        h2 { "Splitter Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("direction", "Direction", "Horizontal", "布局方向"),
            ("class", "String", "\"\"", "自定义 CSS 类名"),
            ("style", "String", "\"\"", "自定义内联样式"),
            ("onresize", "Option<EventHandler<Vec<f64>>>", "None", "尺寸变化回调"),
        ]}

        h2 { "SplitterPanel Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("default_size", "f64", "0.0", "默认尺寸百分比"),
            ("min_size", "f64", "0.0", "最小尺寸百分比"),
            ("max_size", "f64", "0.0", "最大尺寸百分比"),
            ("resizable", "bool", "true", "是否可拖拽调整"),
            ("collapsible", "bool", "false", "是否可折叠"),
            ("collapsed", "bool", "false", "初始是否折叠"),
            ("class", "String", "\"\"", "自定义 CSS 类名"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}
    }
}
