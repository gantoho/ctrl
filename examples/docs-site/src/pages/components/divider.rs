use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn DividerPage() -> Element {
    rsx! {
div { id: "divider", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Divider 分割线" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "区隔内容的分割线，支持文字、虚线和垂直等变体。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; width: 100%;",
                        span { "上方内容" }
                        Divider {}
                        span { "下方内容" }
                    }
                },
                code: "Divider {}".to_string(),
            }
            DemoBox { title: "带文字".to_string(), description: Some("文字居中显示在分割线中间。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; width: 100%;",
                        span { "内容区" }
                        Divider { content: "分割文字".to_string() }
                        span { "另一区" }
                    }
                },
                code: "Divider { content: \"分割文字\".to_string() }".to_string(),
            }
            DemoBox { title: "虚线分割线".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; width: 100%;",
                        span { "上方内容" }
                        Divider { dashed: true }
                        span { "下方内容" }
                    }
                },
                code: "Divider { dashed: true }".to_string(),
            }
            DemoBox { title: "垂直分割线".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; align-items: center; gap: 8px;",
                        span { "链接" }
                        Divider { direction: "vertical".to_string() }
                        span { "菜单" }
                        Divider { direction: "vertical".to_string() }
                        span { "设置" }
                    }
                },
                code: "Divider { direction: \"vertical\".to_string() }".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("direction", "String", "\"horizontal\"", "方向（horizontal / vertical）"),
                ("content", "String", "\"\"", "中间文字，空则为纯分割线"),
                ("dashed", "bool", "false", "是否虚线"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }
    }
}
