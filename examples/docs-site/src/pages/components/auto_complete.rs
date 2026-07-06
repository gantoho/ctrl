use dioxus::prelude::*;
use ctrl::prelude::*;
use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn AutoCompletePage() -> Element {
    rsx! {
        h1 { "AutoComplete 自动补全" }
        p { "输入时自动匹配并展示候选项，支持键盘导航和自定义选项。适用于搜索、地址补全等场景。" }

        h2 { "基本用法" }

        DemoBox {
            title: "基础示例".to_string(),
            description: Some("输入文字自动筛选匹配项，支持键盘上下选择，回车确认。".to_string()),
            demo: rsx! {
                div { style: "width: 300px;",
                    AutoComplete {
                        placeholder: "请输入水果名称".to_string(),
                        options: vec![
                            AutoCompleteOption { value: "apple".to_string(), label: "Apple 苹果".to_string() },
                            AutoCompleteOption { value: "banana".to_string(), label: "Banana 香蕉".to_string() },
                            AutoCompleteOption { value: "cherry".to_string(), label: "Cherry 樱桃".to_string() },
                            AutoCompleteOption { value: "grape".to_string(), label: "Grape 葡萄".to_string() },
                            AutoCompleteOption { value: "orange".to_string(), label: "Orange 橙子".to_string() },
                            AutoCompleteOption { value: "peach".to_string(), label: "Peach 桃子".to_string() },
                        ],
                    }
                }
            },
            code: "AutoComplete {\n    placeholder: \"请输入水果名称\".to_string(),\n    options: vec![\n        AutoCompleteOption { value: \"apple\".to_string(), label: \"Apple 苹果\".to_string() },\n        AutoCompleteOption { value: \"banana\".to_string(), label: \"Banana 香蕉\".to_string() },\n    ],\n}".to_string(),
        }

        DemoBox {
            title: "禁用状态".to_string(),
            description: Some("设置 disabled 为 true 禁用输入。".to_string()),
            demo: rsx! {
                div { style: "width: 300px;",
                    AutoComplete {
                        disabled: true,
                        value: "Apple 苹果".to_string(),
                        placeholder: "禁用状态".to_string(),
                        options: vec![
                            AutoCompleteOption { value: "apple".to_string(), label: "Apple 苹果".to_string() },
                        ],
                    }
                }
            },
            code: "AutoComplete {\n    disabled: true,\n    value: \"Apple 苹果\".to_string(),\n    placeholder: \"禁用状态\".to_string(),\n}".to_string(),
        }

        h2 { "AutoComplete Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("value", "String", "\"\"", "当前输入值"),
            ("placeholder", "String", "\"请输入\"", "占位提示文本"),
            ("options", "Vec<AutoCompleteOption>", "vec![]", "可选列表"),
            ("disabled", "bool", "false", "是否禁用"),
            ("size", "Size", "Md", "输入框尺寸"),
            ("class", "String", "\"\"", "自定义 CSS 类名"),
            ("style", "String", "\"\"", "自定义内联样式"),
            ("onchange", "Option<EventHandler<String>>", "None", "输入值变化回调"),
            ("onselect", "Option<EventHandler<AutoCompleteOption>>", "None", "选项选中回调"),
        ]}
    }
}
