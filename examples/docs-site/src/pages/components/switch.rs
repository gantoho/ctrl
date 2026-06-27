use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn SwitchPage() -> Element {
    rsx! {
div { id: "switch", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Switch 开关"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "开关用于在两种状态间切换。支持三种尺寸和禁用状态。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 checked 和 onchange 管理开关状态。".to_string()),
                demo: rsx! { BasicSwitchDemo {} },
                code: "let mut on = use_signal(|| false);\n\nSwitch {\n    checked: on(),\n    onchange: move |v| on.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "三种尺寸".to_string(),
                description: Some("Sm / Md / Lg".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Switch { size: Size::Sm }
                        Switch { size: Size::Md, checked: true }
                        Switch { size: Size::Lg }
                    }
                },
                code: "Switch { size: Size::Sm }\nSwitch { size: Size::Md, checked: true }\nSwitch { size: Size::Lg }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("禁用时不可交互，透明度降低。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Switch { disabled: true }
                        Switch { disabled: true, checked: true }
                    }
                },
                code: "Switch { disabled: true }\nSwitch { disabled: true, checked: true }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Switch Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("checked", "bool", "false", "是否选中"),
                ("disabled", "bool", "false", "是否禁用"),
                ("size", "Size", "Md", "开关尺寸"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<bool>>", "None", "状态变化事件"),
            ]}
        }
    }
}
