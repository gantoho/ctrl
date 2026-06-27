use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TooltipPage() -> Element {
    rsx! {
div { id: "tooltip", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Tooltip 气泡提示"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "鼠标悬浮时显示简短提示文字，支持上下左右四个方向。"
            }

            DemoBox {
                title: "位置方向".to_string(),
                description: Some("通过 placement 控制气泡弹出方向。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; flex-wrap: wrap;",
                        Tooltip { content: "这是一段提示文字".to_string(), placement: "top".to_string(),
                            Button { variant: Variant::Outline, size: Size::Sm, "Top" }
                        }
                        Tooltip { content: "底部弹出的提示".to_string(), placement: "bottom".to_string(),
                            Button { variant: Variant::Outline, size: Size::Sm, "Bottom" }
                        }
                        Tooltip { content: "左侧提示".to_string(), placement: "left".to_string(),
                            Button { variant: Variant::Outline, size: Size::Sm, "Left" }
                        }
                        Tooltip { content: "右侧提示".to_string(), placement: "right".to_string(),
                            Button { variant: Variant::Outline, size: Size::Sm, "Right" }
                        }
                    }
                },
                code: "Tooltip { content: \"提示文字\".into(), placement: \"top\",\n    Button { \"Top\" }\n}".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Tooltip Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("content", "String", "\"\"", "提示文字"),
                ("placement", "String", "\"top\"", "弹出方向（top / bottom / left / right）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "触发元素"),
            ]}
        }
    }
}
