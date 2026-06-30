use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn DatePickerPage() -> Element {
    rsx! {
div { id: "date_picker", style: "margin-top: 64px;",
            h1 { "DatePicker 日期选择器" }
            p { "日期选择器用于选择日期。提供日历面板、月份导航和今天快捷按钮。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 onchange 管理日期。".to_string()),
                demo: rsx! { BasicDatePickerDemo {} },
                code: "let mut date = use_signal(|| String::new());\n\nDatePicker {\n    value: date(),\n    onchange: move |v| date.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "带初始值".to_string(),
                description: Some("设置 value 属性指定初始日期。".to_string()),
                demo: rsx! {
                    Space { gap: "sm".to_string(),
                        DatePicker { value: "2024-06-15".to_string() }
                        DatePicker { value: "2024-12-25".to_string() }
                    }
                },
                code: "DatePicker { value: \"2024-06-15\".to_string() }\nDatePicker { value: \"2024-12-25\".to_string() }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("设置 disabled 为 true 禁用选择器。".to_string()),
                demo: rsx! { DatePicker { value: "2024-06-15".to_string(), disabled: true } },
                code: "DatePicker { value: \"2024-06-15\".to_string(), disabled: true }".to_string(),
            }

            DemoBox {
                title: "不可清除".to_string(),
                description: Some("设置 clearable 为 false 隐藏清除按钮。".to_string()),
                demo: rsx! { DatePicker { value: "2024-06-15".to_string(), clearable: false } },
                code: "DatePicker { value: \"2024-06-15\".to_string(), clearable: false }".to_string(),
            }

            h2 { "DatePicker Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("value", "String", "\"\"", "选中的日期（YYYY-MM-DD）"),
                ("placeholder", "String", "\"请选择日期\"", "占位文本"),
                ("disabled", "bool", "false", "是否禁用"),
                ("clearable", "bool", "true", "是否可清除"),
                ("format", "String", "\"YYYY-MM-DD\"", "日期格式"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<String>>", "None", "值变化回调"),
            ]}
        }
    }
}
