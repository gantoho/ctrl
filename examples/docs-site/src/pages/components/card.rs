use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn CardPage() -> Element {
    rsx! {
div { id: "card", style: "margin-top: 64px;",
            h1 {
                "Card 卡片"
            }
            p {
                "卡片用于承载和展示信息，支持标题、边框、阴影等样式。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("默认带边框，无阴影。".to_string()),
                demo: rsx! {
                    Card { title: "卡片标题".to_string(),
                        p { "这是卡片的内容区域，可以放置任何元素。" }
                    }
                },
                code: "Card { title: \"卡片标题\".to_string(),\n    p { \"这是卡片内容\" }\n}".to_string(),
            }

            DemoBox {
                title: "带阴影".to_string(),
                description: Some("设置 shadow 为 true 显示投影。".to_string()),
                demo: rsx! {
                    Space { gap: "md".to_string(),
                        Card { style: "flex: 1;".to_string(), title: "默认卡片".to_string(),
                            p { "bordered 为默认值 true" }
                        }
                        Card { style: "flex: 1;".to_string(), shadow: true, title: "阴影卡片".to_string(),
                            p { "shadow: true" }
                        }
                    }
                },
                code: "Card { title: \"阴影卡片\".to_string(), shadow: true,\n    p { \"卡片内容\" }\n}".to_string(),
            }

            h2 { "Card Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "卡片标题"),
                ("bordered", "bool", "true", "是否显示边框"),
                ("shadow", "bool", "false", "是否带阴影"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("header", "Option<Element>", "None", "自定义头部插槽"),
                ("children", "Element", "—", "卡片内容"),
            ]}
        }
    }
}
