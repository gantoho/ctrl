use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn DropdownPage() -> Element {
    rsx! {
div { id: "dropdown", style: "margin-top: 64px;",
            h1 { "Dropdown 下拉菜单" }
            p { "点击后展开的下拉菜单，支持选项和分割线。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    DropdownDocs {}
                },
                code: "let mut selected = use_signal(|| String::new());\ndiv {\n    Dropdown {\n        trigger: rsx! { Button { \"打开菜单\" } },\n        DropdownItem {\n            onclick: move |_| selected.set(\"选项一\".to_string()),\n            \"选项一\"\n        }\n        DropdownItem {\n            onclick: move |_| selected.set(\"选项二\".to_string()),\n            \"选项二\"\n        }\n        DropdownDivider {}\n        DropdownItem { disabled: true, \"禁用项\" }\n    }\n    if !selected().is_empty() {\n        p { \"已选：{selected}\" }\n    }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("placement", "String", "\"bottom\"", "弹出位置（bottom / bottom-start / bottom-end / top / top-start / top-end）"),
                ("trigger", "Element", "—", "触发元素（通常是 Button）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "菜单项（DropdownItem / DropdownDivider）"),
            ]}
            h2 { "DropdownItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("disabled", "bool", "false", "是否禁用"),
                ("onclick", "Option<EventHandler<()>>", "None", "点击事件"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "菜单项文字"),
            ]}
        }
    }
}
