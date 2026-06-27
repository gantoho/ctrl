use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn StepsPage() -> Element {
    rsx! {
div { id: "steps", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Steps 步骤条" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "用于展示流程进度，支持水平和垂直排列。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    Steps {
                        Step { title: "步骤一".to_string(), description: "第一步描述".to_string() }
                        Step { title: "步骤二".to_string(), description: "第二步描述".to_string() }
                        Step { title: "步骤三".to_string(), description: "最后一步".to_string() }
                    }
                },
                code: "Steps {\n    Step { title: \"步骤一\".to_string(), description: \"描述\".to_string() }\n    Step { title: \"步骤二\".to_string() }\n    Step { title: \"步骤三\".to_string() }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("current", "i32", "-1", "当前步骤索引（Steps），-1 表示未开始"),
                ("direction", "String", "\"horizontal\"", "方向（horizontal / vertical）— Steps"),
                ("class", "String", "\"\"", "自定义 CSS 类 — Steps"),
                ("children", "Element", "—", "子元素（Step）— Steps"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Step Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "步骤标题"),
                ("description", "String", "\"\"", "步骤描述"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }
    }
}
