use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn MeteorsPage() -> Element {
    rsx! {
div { id: "meteors", style: "margin-top: 64px;",
            h1 {
                "Meteors 流星"
            }
            p {
                "在容器内渲染一层斜向坠落的流星作为背景装饰，children 展示在其上层，常用于 Hero 区、空状态、卡片背景等。支持 prefers-reduced-motion 下停用。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("包裹内容即可获得流星背景。".to_string()),
                demo: rsx! {
                    Meteors {
                        style: "height:200px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); background:var(--ctrl-bg-secondary);".to_string(),
                        div { style: "height:100%; display:flex; flex-direction:column; align-items:center; justify-content:center; gap:8px;",
                            div { style: "font-size:20px; font-weight:700; color:var(--ctrl-text);", "Ctrl UI" }
                            div { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "流星划过夜空" }
                        }
                    }
                },
                code: "Meteors {\n    style: \"height:200px;\".to_string(),\n    div { \"...\" }\n}".to_string(),
            }

            DemoBox {
                title: "流星数量".to_string(),
                description: Some("通过 count 控制流星密度。".to_string()),
                demo: rsx! {
                    Meteors {
                        count: 40,
                        style: "height:180px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); background:var(--ctrl-bg-secondary);".to_string(),
                        div { style: "height:100%; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);",
                            "count = 40"
                        }
                    }
                },
                code: "Meteors { count: 40, ... }".to_string(),
            }

            h2 { "Meteors Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("count", "usize", "20", "流星数量"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式（通常设置高度/背景）"),
                ("children", "Element", "—", "覆盖在流星之上的内容"),
            ]}
        }
    }
}
