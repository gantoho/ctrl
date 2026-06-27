use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn RatePage() -> Element {
    rsx! {
div { id: "rate", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Rate 评分" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "评分组件用于收集用户对内容的评价。支持半星、自定义图标和只读状态。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 onchange 管理评分。".to_string()),
                demo: rsx! { BasicRateDemo {} },
                code: "let mut value = use_signal(|| 3.0);\n\nRate {\n    value: value(),\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "半星模式".to_string(),
                description: Some("设置 allow_half 允许半星评分。鼠标悬停在星的左半区选半星，右半区选整星。".to_string()),
                demo: rsx! { HalfRateDemo {} },
                code: "let mut value = use_signal(|| 2.5);\n\nRate {\n    value: value(),\n    allow_half: true,\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "禁用/只读".to_string(),
                description: Some("禁用或只读状态下的评分。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        Rate { value: 4.0, disabled: true }
                        Rate { value: 5.0, disabled: true }
                    }
                },
                code: "Rate { value: 4, disabled: true }\nRate { value: 5, disabled: true }".to_string(),
            }

            DemoBox {
                title: "显示文本".to_string(),
                description: Some("设置 show_text 显示评论文本。".to_string()),
                demo: rsx! { Rate { value: 4.0, show_text: true } },
                code: "Rate { value: 4, show_text: true }".to_string(),
            }

            DemoBox {
                title: "自定义图标".to_string(),
                description: Some("通过 icon_full、icon_empty 设置自定义图标（支持图片 URL）。".to_string()),
                demo: rsx! { CustomIconRateDemo {} },
                code: "Rate {\n    value: value(),\n    icon_full: \"...\".to_string(),\n    icon_empty: \"...\".to_string(),\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "自定义图标 + 半星".to_string(),
                description: Some("半星模式下额外提供 icon_half 图标。".to_string()),
                demo: rsx! { CustomIconHalfRateDemo {} },
                code: "Rate {\n    value: value(),\n    allow_half: true,\n    icon_full: \"...\".to_string(),\n    icon_half: \"...\".to_string(),\n    icon_empty: \"...\".to_string(),\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Rate Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("value", "f64", "0.0", "当前评分值"),
                ("count", "i32", "5", "星星数量"),
                ("allow_half", "bool", "false", "是否允许半星"),
                ("disabled", "bool", "false", "是否禁用"),
                ("show_text", "bool", "false", "是否显示评论文本"),
                ("icon_full", "Option<String>", "None", "全选图标 URL"),
                ("icon_half", "Option<String>", "None", "半选图标 URL"),
                ("icon_empty", "Option<String>", "None", "未选图标 URL"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<f64>>", "None", "值变化回调"),
            ]}
        }
    }
}
