use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn ColorPickerPage() -> Element {
    rsx! {
        h1 { "ColorPicker 颜色选择器" }
        p { "用于选择或输入颜色值，内置预设色板。" }

        DemoBox { title: "基本用法".to_string(), description: None,
            demo: rsx! {
                ColorPicker {}
            },
            code: "ColorPicker {}".to_string(),
        }

        DemoBox { title: "预设颜色".to_string(), description: Some("内置 20 个预设颜色块，点击即可选中。".to_string()),
            demo: rsx! {
                ColorPicker { value: "#3B82F6".to_string() }
            },
            code: "ColorPicker { value: \"#3B82F6\" }".to_string(),
        }

        DemoBox { title: "纯色块模式".to_string(), description: Some("show_text 为 false 时隐藏 hex 输入框，仅显示色块。".to_string()),
            demo: rsx! {
                ColorPicker { show_text: false, value: "#EF4444".to_string() }
            },
            code: "ColorPicker { show_text: false, value: \"#EF4444\" }".to_string(),
        }

        DemoBox { title: "禁用状态".to_string(), description: None,
            demo: rsx! {
                ColorPicker { disabled: true, value: "#10B981".to_string() }
            },
            code: "ColorPicker { disabled: true, value: \"#10B981\" }".to_string(),
        }

        h2 { "ColorPicker Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("value", "String", "\"#3FC99E\"", "当前颜色值"),
            ("show_text", "bool", "true", "是否显示 hex 输入框"),
            ("placeholder", "String", "\"选择颜色\"", "占位文字"),
            ("disabled", "bool", "false", "是否禁用"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}
    }
}
