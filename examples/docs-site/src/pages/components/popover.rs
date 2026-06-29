use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn PopoverPage() -> Element {
    rsx! {
div { id: "popover", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Popover 气泡卡片" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "点击/悬停触发的气泡卡片，可包含标题和内容。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; gap: 12px;",
                        Popover { placement: Placement::Top, title: "提示".to_string(), content: rsx! { span { "向上弹出" } },
                            Button { variant: Variant::Outline, size: Size::Sm, "Top" }
                        }
                        Popover { placement: Placement::Bottom, title: "通知".to_string(), content: rsx! { span { "向下弹出" } },
                            Button { variant: Variant::Outline, size: Size::Sm, "Bottom" }
                        }
                    }
                },
                code: "Popover { placement: \"top\".to_string(), title: \"提示\".to_string(), content: rsx! { span { \"内容\" } },\n    Button { variant: Variant::Outline, size: Size::Sm, \"Top\" }\n}".to_string(),
            }
            DemoBox { title: "overflow:hidden 容器内".to_string(), description: Some("即使祖先容器设置了 overflow: hidden，气泡卡片也不会被裁切，因为使用 position:fixed 定位。".to_string()),
                demo: rsx! {
                    div {
                        style: "overflow: hidden; border: 2px dashed var(--ctrl-border); border-radius: var(--ctrl-radius-md); padding: 20px; width: 200px; height: 80px; display: flex; align-items: center; justify-content: center;",
                        Popover { placement: Placement::Top, title: "提示".to_string(), content: rsx! { span { "不会被 overflow:hidden 裁切" } },
                            Button { variant: Variant::Primary, size: Size::Sm, "点击弹出" }
                        }
                    }
                },
                code: "div { style: \"overflow: hidden; ...\",\n    Popover { placement: \"top\".to_string(), title: \"提示\".to_string(),\n        content: rsx! { span { \"内容\" } },\n        Button { \"点击弹出\" }\n    }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("placement", "String", "\"top\"", "弹出位置（top / bottom / left / right）"),
                ("title", "String", "\"\"", "气泡标题"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "触发元素"),
                ("content", "Element", "—", "气泡内容"),
            ]}
        }
    }
}
