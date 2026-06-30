use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn RadioPage() -> Element {
    rsx! {
div { id: "radio", style: "margin-top: 64px;",
            h1 {
                "Radio 单选框"
            }
            p {
                "单选框用于在一组互斥选项中选择一项。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value + onchange 管理选中值。".to_string()),
                demo: rsx! { BasicRadioDemo {} },
                code: "let mut selected = use_signal(|| \"a\".to_string());\n\nRadio { value: \"a\", label: \"选项 A\", checked: selected() == \"a\", onchange: move |v| selected.set(v) }\nRadio { value: \"b\", label: \"选项 B\", checked: selected() == \"b\", onchange: move |v| selected.set(v) }\nRadio { value: \"c\", label: \"选项 C\", checked: selected() == \"c\", onchange: move |v| selected.set(v) }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("禁用时不可点击。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "xs".to_string(),
                        Radio { value: "a", label: "选项 A".to_string(), checked: true, disabled: true }
                        Radio { value: "b", label: "选项 B".to_string(), disabled: true }
                    }
                },
                code: "Radio { value: \"a\", label: \"选项 A\".to_string(), checked: true, disabled: true }\nRadio { value: \"b\", label: \"选项 B\".to_string(), disabled: true }".to_string(),
            }

            h2 { "Radio Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("checked", "bool", "false", "是否选中"),
                ("disabled", "bool", "false", "是否禁用"),
                ("value", "String", "\"\"", "单选值"),
                ("label", "String", "\"\"", "标签文本"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<String>>", "None", "选中变化事件"),
            ]}
        }
    }
}
