use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn ProgressPage() -> Element {
    rsx! {
div { id: "progress", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Progress 进度条"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "进度条用于展示任务完成进度，支持颜色自定义和百分比文字。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 percent 设置 0~100 的进度值。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",
                        Progress { percent: 20.0 }
                        Progress { percent: 60.0, color: "var(--ctrl-success)".to_string() }
                        Progress { percent: 90.0, color: "var(--ctrl-warning)".to_string(), show_text: true }
                    }
                },
                code: "Progress { percent: 20.0 }\nProgress { percent: 60.0, color: \"var(--ctrl-success)\" }\nProgress { percent: 90.0, show_text: true }".to_string(),
            }

            DemoBox {
                title: "动态进度".to_string(),
                description: Some("点击按钮实时调整进度值。".to_string()),
                demo: rsx! { ProgressDynamicDemo {} },
                code: "let mut percent = use_signal(|| 30.0);\n\nProgress { percent: percent(), show_text: true }\nButton { onclick: move |_| percent.set((percent() + 10.0).min(100.0)), \"+10\" }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Progress Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("percent", "f64", "0.0", "进度值 0~100"),
                ("color", "String", "var(--ctrl-primary)", "进度条颜色"),
                ("show_text", "bool", "false", "是否显示百分比文字"),
                ("height", "u32", "8", "进度条高度(px)"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}
