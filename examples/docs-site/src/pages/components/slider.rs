use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn SliderPage() -> Element {
    rsx! {
div { id: "slider", style: "margin-top: 64px;",
            h1 { "Slider 滑块" }
            p { "滑块用于在数值范围内进行选择。支持水平/垂直方向、步长、刻度标记和禁用状态。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 onchange 管理滑块值。".to_string()),
                demo: rsx! { BasicSliderDemo {} },
                code: "let mut value = use_signal(|| 50);\n\nSlider {\n    value: value(),\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "自定义范围".to_string(),
                description: Some("设置 min、max 和 step 属性。".to_string()),
                demo: rsx! { CustomRangeSliderDemo {} },
                code: "Slider { value: 30, min: 0, max: 100, step: 5, show_label: true }\nSlider { value: 0, min: -50, max: 50, step: 10, show_label: true }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("设置 disabled 为 true 禁用滑块。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "lg".to_string(),
                        Slider { value: 30, disabled: true }
                        Slider { value: 70, disabled: true }
                    }
                },
                code: "Slider { value: 30, disabled: true }".to_string(),
            }

            DemoBox {
                title: "显示标签".to_string(),
                description: Some("设置 show_label 显示当前值和最大值。".to_string()),
                demo: rsx! {
                    div { style: "max-width: 400px;", Slider { value: 60, show_label: true } }
                },
                code: "Slider { value: 60, show_label: true }".to_string(),
            }

            h2 { "Slider Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("value", "i32", "0", "当前值"),
                ("min", "i32", "0", "最小值"),
                ("max", "i32", "100", "最大值"),
                ("step", "i32", "1", "步长"),
                ("disabled", "bool", "false", "是否禁用"),
                ("vertical", "bool", "false", "是否垂直"),
                ("marks", "bool", "false", "是否显示刻度"),
                ("show_label", "bool", "false", "是否显示数值标签"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<i32>>", "None", "值变化回调"),
            ]}
        }
    }
}
