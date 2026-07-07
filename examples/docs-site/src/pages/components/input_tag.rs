use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn InputTagPage() -> Element {
    rsx! {
        div { id: "input-tag", style: "margin-top: 64px;",
            h1 { "InputTag 标签输入" }
            p { "输入文本后按回车或逗号生成标签，点击标签上的 X 可删除。" }

            DemoBox {
                title: "基础用法".to_string(),
                description: Some("输入文本后按回车添加标签，按退格键删除最后一个标签。".to_string()),
                demo: rsx! {
                    InputTag {
                        placeholder: "输入后按回车添加".to_string(),
                    }
                },
                code: r#"InputTag {
    placeholder: "输入后按回车添加".to_string(),
}"#.to_string(),
            }

            DemoBox {
                title: "预设值".to_string(),
                description: Some("通过 value 属性传入初始标签列表。".to_string()),
                demo: rsx! {
                    InputTag {
                        value: vec!["Vue".to_string(), "React".to_string(), "Angular".to_string()],
                    }
                },
                code: r#"InputTag {
    value: vec!["Vue".into(), "React".into(), "Angular".into()],
}"#.to_string(),
            }

            DemoBox {
                title: "限制数量".to_string(),
                description: Some("通过 max 属性限制最多标签数量。此示例最多 3 个标签。".to_string()),
                demo: rsx! {
                    InputTag {
                        max: Some(3),
                        placeholder: "最多 3 个标签".to_string(),
                    }
                },
                code: r#"InputTag {
    max: Some(3),
}"#.to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("设置 disabled 属性禁用组件。".to_string()),
                demo: rsx! {
                    InputTag {
                        value: vec!["React".to_string(), "TypeScript".to_string()],
                        disabled: true,
                    }
                },
                code: r#"InputTag {
    value: vec!["React".into(), "TypeScript".into()],
    disabled: true,
}"#.to_string(),
            }

            h2 { "InputTag Props" }
            PropsTable {
                headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()],
                rows: vec![
                    ("value", "Vec<String>", "[]", "标签列表"),
                    ("onchange", "EventHandler<Vec<String>>", "None", "标签变化回调"),
                    ("placeholder", "String", "\"输入后按回车添加\"", "输入框占位提示"),
                    ("disabled", "bool", "false", "是否禁用"),
                    ("max", "Option<usize>", "None", "最多标签数量"),
                    ("class", "String", "\"\"", "自定义类名"),
                ],
            }
        }
    }
}
