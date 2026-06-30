use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn TabsPage() -> Element {
    rsx! {
div { id: "tabs", style: "margin-top: 64px;",
            h1 {
                "Tabs 标签页"
            }
            p {
                "标签页用于在同一区域内切换显示不同内容。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("使用 TabNav + TabContent 组合，通过 active 控制当前标签。".to_string()),
                demo: rsx! { TabsBasicDemo {} },
                code: "let mut active = use_signal(|| \"tab1\".to_string());\nlet items = vec![\n    (\"tab1\", \"标签一\", false),\n    (\"tab2\", \"标签二\", false),\n];\n\nTabNav { items, active: active(), onchange: move |v| active.set(v) }\nTabContent { div { \"当前: {active()}\" } }".to_string(),
            }

            DemoBox {
                title: "禁用标签".to_string(),
                description: Some("在 items 中设置第三个元素为 true 可禁用某标签。".to_string()),
                demo: rsx! { TabsDisabledDemo {} },
                code: "let items = vec![\n    (\"d1\", \"可用\", false),\n    (\"d2\", \"禁用\", true),\n];".to_string(),
            }

            h2 { "Tabs Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("active", "String", "\"0\"", "当前激活的 tab key"),
                ("onchange", "Option<EventHandler<String>>", "None", "tab 切换回调"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "子元素（Tab）"),
            ]}
            h2 { "Tab Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("tab_key", "String", "\"\"", "唯一标识"),
                ("title", "String", "\"\"", "tab 标题文字"),
                ("disabled", "bool", "false", "是否禁用"),
                ("children", "Element", "—", "面板内容"),
            ]}
            h2 { "TabNav Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("items", "Vec<(String, String, bool)>", "[]", "标签项（key, title, disabled）"),
                ("active", "String", "\"0\"", "当前激活的 key"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("onchange", "Option<EventHandler<String>>", "None", "切换回调"),
            ]}
        }
    }
}
