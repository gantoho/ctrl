use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn BadgePage() -> Element {
    rsx! {
div { id: "badge", style: "margin-top: 64px;",
            h1 {
                "Badge 徽标"
            }
            p {
                "徽标用于在子元素右上角显示数字或圆点，表示新消息数或状态。"
            }

            DemoBox {
                title: "数字徽标".to_string(),
                description: Some("通过 count 设置显示的数字，超过 max 则显示 N+。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(),
                        Badge { count: "5".to_string(),
                            div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "消息" }
                        }
                        Badge { count: "120".to_string(), max: 99,
                            div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "通知" }
                        }
                    }
                },
                code: "Badge { count: \"5\", div { \"消息\" } }\nBadge { count: \"120\", max: 99, div { \"通知\" } }".to_string(),
            }

            DemoBox {
                title: "圆点徽标".to_string(),
                description: Some("dot 为 true 时只显示一个小圆点，用于状态提示。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(),
                        Badge { dot: true,
                            div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "状态" }
                        }
                    }
                },
                code: "Badge { dot: true, div { \"状态\" } }".to_string(),
            }

            DemoBox {
                title: "自定义颜色".to_string(),
                description: Some("通过 color 属性自定义徽标背景色。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(),
                        Badge { count: "3".to_string(), color: "var(--ctrl-success)".to_string(),
                            div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "完成" }
                        }
                        Badge { count: "99+".to_string(), color: "var(--ctrl-warning)".to_string(),
                            div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "警告" }
                        }
                    }
                },
                code: "Badge { count: \"3\", color: \"var(--ctrl-success)\", div { \"完成\" } }\nBadge { count: \"99+\", color: \"var(--ctrl-warning)\", div { \"警告\" } }".to_string(),
            }

            h2 { "Badge Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("count", "String", "\"\"", "徽标数字/文字"),
                ("dot", "bool", "false", "是否显示小圆点"),
                ("max", "u32", "99", "最大显示数字"),
                ("color", "String", "var(--ctrl-danger)", "徽标背景色"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "包裹的子元素"),
            ]}
        }
    }
}
