use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn MenuPage() -> Element {
    rsx! {
div { id: "menu", style: "margin-top: 64px;",
            h1 { "Menu 导航菜单" }
            p { "垂直/水平导航菜单，适用于侧边栏或顶部导航。" }
            DemoBox { title: "垂直菜单".to_string(), description: None,
                demo: rsx! {
                    MenuDocs {}
                },
                code: "Menu {\n    MenuItem { \"首页\" }\n    MenuItem { \"组件\" }\n    MenuItem { \"文档\" }\n    MenuItem { disabled: true, \"禁用项\" }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("direction", "String", "\"vertical\"", "方向（vertical / horizontal）— Menu"),
                ("active", "String", "\"\"", "当前激活项 key — Menu"),
                ("onchange", "Option<EventHandler<String>>", "None", "切换回调 — Menu"),
                ("class", "String", "\"\"", "自定义 CSS 类 — Menu"),
                ("children", "Element", "—", "子元素（MenuItem / Submenu）— Menu"),
            ]}
            h2 { "MenuItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("item_key", "String", "\"\"", "唯一标识"),
                ("disabled", "bool", "false", "是否禁用"),
                ("onclick", "Option<EventHandler<()>>", "None", "点击事件"),
                ("icon", "Option<Element>", "None", "图标插槽"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "菜单文字"),
            ]}
        }
    }
}
