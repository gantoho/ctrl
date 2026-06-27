use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TimelinePage() -> Element {
    rsx! {
div { id: "timeline", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Timeline 时间线" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "垂直时间线组件，用于按时间顺序展示事件。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    Timeline {
                        TimelineItem { timestamp: "2024-01-15".to_string(), "项目立项" }
                        TimelineItem { timestamp: "2024-03-20".to_string(), color: "primary".to_string(), "需求分析完成" }
                        TimelineItem { timestamp: "2024-06-10".to_string(), color: "success".to_string(), "第一阶段上线" }
                        TimelineItem { timestamp: "2024-09-01".to_string(), "项目结项" }
                    }
                },
                code: "Timeline {\n    TimelineItem { timestamp: \"2024-01-15\".to_string(), \"事件\" }\n    TimelineItem { timestamp: \"...\".to_string(), color: \"primary\".to_string(), \"事件\" }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("class", "String", "\"\"", "自定义 CSS 类 — Timeline"),
                ("children", "Element", "—", "子元素（TimelineItem）— Timeline"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "TimelineItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("timestamp", "String", "\"\"", "时间标签"),
                ("color", "String", "\"default\"", "圆点颜色：default / primary / success / warning / danger"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "事件内容"),
            ]}
        }
    }
}
