use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn LoadingPage() -> Element {
    rsx! {
div { id: "loading", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Loading 加载中" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "显示加载状态，支持三种尺寸和自定义文字。" }
            DemoBox { title: "尺寸".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Loading { size: Size::Sm }
                        Loading { size: Size::Md }
                        Loading { size: Size::Lg }
                    }
                },
                code: "Loading { size: Size::Sm }\nLoading { size: Size::Md }\nLoading { size: Size::Lg }".to_string(),
            }
            DemoBox { title: "带文字".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; gap: 16px;",
                        Loading { text: "加载中...".to_string(), size: Size::Md }
                    }
                },
                code: "Loading { text: \"加载中...\".to_string() }".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("loading", "bool", "true", "是否显示加载中"),
                ("text", "String", "\"\"", "加载文案"),
                ("size", "Size", "Md", "尺寸（Sm / Md / Lg）"),
                ("fullscreen", "bool", "false", "是否全屏遮罩"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }
    }
}
