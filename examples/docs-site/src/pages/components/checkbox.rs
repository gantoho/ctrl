use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::components::codeblock::CodeBlock;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn CheckboxPage() -> Element {
    rsx! {
div { id: "checkbox", style: "margin-top: 64px;",
            h1 {
                "Checkbox 复选框"
            }
            p {
                "复选框用于多选场景。支持选中、未选中、半选和禁用状态。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 checked 和 onchange 管理选中状态。".to_string()),
                demo: rsx! { BasicCheckboxDemo {} },
                code: "let mut checked = use_signal(|| false);\n\nCheckbox {\n    checked: checked(),\n    label: \"同意协议\".to_string(),\n    onchange: move |v| checked.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "状态".to_string(),
                description: Some("未选中、选中、半选、禁用。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "sm".to_string(),
                        Checkbox { label: "未选中".to_string() }
                        Checkbox { checked: true, label: "已选中".to_string() }
                        Checkbox { indeterminate: true, label: "半选状态".to_string() }
                        Checkbox { disabled: true, label: "禁用未选中".to_string() }
                        Checkbox { checked: true, disabled: true, label: "禁用已选中".to_string() }
                    }
                },
                code: "Checkbox { label: \"未选中\" }\nCheckbox { checked: true, label: \"已选中\" }\nCheckbox { indeterminate: true, label: \"半选\" }\nCheckbox { disabled: true, label: \"禁用\" }".to_string(),
            }

            DemoBox {
                title: "全选示例".to_string(),
                description: Some("使用 Checkbox 实现全选/取消全选功能。".to_string()),
                demo: rsx! { CheckAllDemo {} },
                code: "// 全选逻辑代码见页面下方".to_string(),
            }

            h3 { "全选示例代码" }
            CodeBlock { code: [
                "let items = vec![\"选项 A\", \"选项 B\", \"选项 C\"];",
                "let mut checked = use_signal(|| vec![false; items.len()]);",
                "",
                "// 计算全选/半选状态",
                "let all = checked().iter().all(|&c| c);",
                "let some = checked().iter().any(|&c| c);",
                "let indet = some && !all;",
                "",
                "// 全选 Checkbox",
                "Checkbox {",
                "    checked: all, indeterminate: indet, label: \"全选\".to_string(),",
                "    onchange: move |v| checked.set(vec![v; items.len()]),",
                "}",
                "// 子选项",
                "for (i, item) in items.iter().enumerate() {",
                "    Checkbox { label: item.to_string(), checked: checked()[i],",
                "        onchange: move |v| { let mut c = checked(); c[i] = v; checked.set(c); },",
                "    }",
                "}",
            ].join("\n"), lang: Some("rust".to_string()) }

            h2 { "Checkbox Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("checked", "bool", "false", "是否选中"),
                ("disabled", "bool", "false", "是否禁用"),
                ("indeterminate", "bool", "false", "半选状态"),
                ("label", "String", "\"\"", "标签文本"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<bool>>", "None", "状态变化事件"),
            ]}
        }
    }
}
