use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn DrawerPage() -> Element {
    rsx! {
div { id: "drawer", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Drawer 抽屉" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "从屏幕边缘滑出的浮层面板，适用于表单、详情等场景。" }
            DemoBox { title: "基本用法".to_string(), description: Some("点击按钮打开右侧抽屉。".to_string()),
                demo: rsx! {
                    DrawerDocs {}
                },
                code: "let mut visible = use_signal(|| false);\nButton { onclick: move |_| visible.set(true), \"打开\" }\nDrawer { visible: visible(), title: \"标题\".to_string(),\n    onclose: move |_| visible.set(false),\n    p { \"内容\" }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("visible", "bool", "false", "是否打开"),
                ("title", "String", "\"\"", "抽屉标题"),
                ("placement", "String", "\"right\"", "位置（left / right / top / bottom）"),
                ("size", "String", "\"380px\"", "宽度或高度"),
                ("show_close", "bool", "true", "是否显示关闭按钮"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭回调"),
                ("footer", "Option<Element>", "None", "底部操作区"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "抽屉内容"),
            ]}
        }
    }
}
