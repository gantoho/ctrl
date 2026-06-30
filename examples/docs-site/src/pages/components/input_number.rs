use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn InputNumberPage() -> Element {
    rsx! {
div { id: "input_number", style: "margin-top: 64px;",
            h1 { "InputNumber 数字输入框" }
            p { "数字输入框用于精确数值输入，内置增减按钮，支持范围限制和步长控制。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 onchange 管理数值。".to_string()),
                demo: rsx! { BasicInputNumberDemo {} },
                code: "let mut value = use_signal(|| 0);\n\nInputNumber {\n    value: value(),\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "范围限制".to_string(),
                description: Some("设置 min、max 和 step 属性。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "sm".to_string(),
                        InputNumber { value: 5, min: 0, max: 10 }
                        InputNumber { value: 50, min: 0, max: 100, step: 5 }
                        InputNumber { value: 0, min: -10, max: 10 }
                    }
                },
                code: "InputNumber { value: 5, min: 0, max: 10 }\nInputNumber { value: 50, min: 0, max: 100, step: 5 }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("禁用时不可交互。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "sm".to_string(),
                        InputNumber { value: 42, disabled: true }
                    }
                },
                code: "InputNumber { value: 42, disabled: true }".to_string(),
            }

            DemoBox {
                title: "三种尺寸".to_string(),
                description: Some("Sm / Md / Lg。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "sm".to_string(),
                        InputNumber { value: 1, size: Size::Sm }
                        InputNumber { value: 2, size: Size::Md }
                        InputNumber { value: 3, size: Size::Lg }
                    }
                },
                code: "InputNumber { value: 1, size: Size::Sm }\nInputNumber { value: 2, size: Size::Md }\nInputNumber { value: 3, size: Size::Lg }".to_string(),
            }

            h2 { "InputNumber Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("value", "i32", "0", "当前值"),
                ("min", "i32", "0", "最小值"),
                ("max", "i32", "100", "最大值"),
                ("step", "i32", "1", "步长"),
                ("disabled", "bool", "false", "是否禁用"),
                ("size", "Size", "Md", "组件尺寸"),
                ("placeholder", "String", "\"\"", "占位文本"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<i32>>", "None", "值变化回调"),
            ]}
        }
    }
}
