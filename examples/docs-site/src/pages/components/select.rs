use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn SelectPage() -> Element {
    rsx! {
div { id: "select", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Select 下拉选择"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "下拉选择器用于从一组选项中选择一项。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 options 属性传入选项列表，onchange 获取选中值。".to_string()),
                demo: rsx! { BasicSelectDemo {} },
                code: "let options = vec![\n    (\"a\".to_string(), \"选项 A\".to_string(), false),\n    (\"b\".to_string(), \"选项 B\".to_string(), false),\n    (\"c\".to_string(), \"选项 C\".to_string(), false),\n];\n\nSelect { options, placeholder: \"请选择\", value: value(), onchange: move |v| value.set(v) }".to_string(),
            }

            DemoBox {
                title: "尺寸".to_string(),
                description: Some("Sm / Md / Lg".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 240px;",
                        Select { size: Size::Sm, options: vec![("1".into(), "小".into(), false)], placeholder: "Small".to_string() }
                        Select { size: Size::Md, options: vec![("1".into(), "中".into(), false)], placeholder: "Medium".to_string() }
                        Select { size: Size::Lg, options: vec![("1".into(), "大".into(), false)], placeholder: "Large".to_string() }
                    }
                },
                code: "Select { size: Size::Sm, options, placeholder: \"Small\" }\nSelect { size: Size::Md, options, placeholder: \"Medium\" }\nSelect { size: Size::Lg, options, placeholder: \"Large\" }".to_string(),
            }

            DemoBox {
                title: "禁用".to_string(),
                description: Some("整体禁用或单项禁用。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 240px;",
                        Select {
                            disabled: true,
                            placeholder: "整个禁用".to_string(),
                            options: vec![("1".into(), "选项".into(), false)],
                        }
                    }
                },
                code: "Select { disabled: true, options, placeholder: \"整个禁用\" }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Select Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("options", "Vec<(String,String,bool)>", "[]", "选项列表 (值,标签,禁用)"),
                ("value", "String", "\"\"", "当前选中值"),
                ("placeholder", "String", "\"请选择\"", "占位文本"),
                ("size", "Size", "Md", "选择器尺寸"),
                ("disabled", "bool", "false", "是否禁用"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<String>>", "None", "选中变化事件"),
            ]}
        }
    }
}
