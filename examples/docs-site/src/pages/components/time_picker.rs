use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TimePickerPage() -> Element {
    let mut time = use_signal(|| "".to_string());
    rsx! {
        h1 { "TimePicker 时间选择器" }
        p { "点击输入框弹出下拉面板，滚动选择时、分、秒。" }

        DemoBox { title: "基础用法".to_string(), description: None,
            demo: rsx! {
                TimePicker {
                    value: "14:30:00".to_string(),
                    onchange: move |v| time.set(v),
                }
            },
            code: "TimePicker { value: \"14:30:00\", onchange: move |v| ... }".to_string(),
        }

        DemoBox { title: "不显示秒".to_string(), description: Some("设置 show_second: false 只选择时和分。".to_string()),
            demo: rsx! {
                TimePicker {
                    value: "09:15".to_string(),
                    show_second: false,
                }
            },
            code: "TimePicker { value: \"09:15\", show_second: false }".to_string(),
        }

        DemoBox { title: "步长控制".to_string(), description: Some("通过 hour_step / minute_step 控制选项粒度。如 15 分钟粒度和 2 小时粒度。".to_string()),
            demo: rsx! {
                Space { direction: Direction::Vertical, gap: "md".to_string(),
                    TimePicker { value: "10:30:00".to_string(), minute_step: 15, placeholder: "15分钟步长".to_string() }
                    TimePicker { value: "08:00:00".to_string(), hour_step: 2, placeholder: "2小时步长".to_string() }
                }
            },
            code: "TimePicker { value: \"10:30\", minute_step: 15 }\nTimePicker { value: \"08:00\", hour_step: 2 }".to_string(),
        }

        DemoBox { title: "尺寸".to_string(), description: Some("提供 sm / md / lg 三种尺寸。".to_string()),
            demo: rsx! {
                Space { direction: Direction::Vertical, gap: "md".to_string(),
                    TimePicker { size: "sm".to_string(), value: "12:00:00".to_string() }
                    TimePicker { size: "md".to_string(), value: "12:00:00".to_string() }
                    TimePicker { size: "lg".to_string(), value: "12:00:00".to_string() }
                }
            },
            code: "TimePicker { size: \"sm\", ... }\nTimePicker { size: \"md\", ... }\nTimePicker { size: \"lg\", ... }".to_string(),
        }

        DemoBox { title: "禁用状态".to_string(), description: Some("设置 disabled: true 禁用选择器。".to_string()),
            demo: rsx! {
                TimePicker { value: "08:30:00".to_string(), disabled: true }
            },
            code: "TimePicker { value: \"08:30:00\", disabled: true }".to_string(),
        }

        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("value", "String", "\"\"", "当前值"),
            ("placeholder", "String", "\"请选择时间\"", "占位文本"),
            ("format", "String", "\"HH:mm:ss\"", "格式字符串"),
            ("show_second", "bool", "true", "是否显示秒列"),
            ("disabled", "bool", "false", "是否禁用"),
            ("size", "String", "\"md\"", "尺寸：sm / md / lg"),
            ("hour_step", "u32", "1", "小时步长"),
            ("minute_step", "u32", "1", "分钟步长"),
            ("second_step", "u32", "1", "秒步长"),
            ("onchange", "Option<EventHandler>", "None", "值变化回调"),
            ("class", "String", "\"\"", "自定义类名"),
            ("style", "String", "\"\"", "自定义样式"),
        ] }
    }
}
