use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn SegmentedPage() -> Element {
    rsx! {
div { id: "segmented", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Segmented 分段控制器" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "分段控制器用于在多个选项间切换，类似 iOS 的 Segmented Control。支持三种尺寸和禁用状态。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 onchange 管理选中项。".to_string()),
                demo: rsx! { BasicSegmentedDemo {} },
                code: "let mut value = use_signal(|| String::new());\nlet options = vec![(\"日\".to_string(), \"day\".to_string()), (\"周\".to_string(), \"week\".to_string())];\n\nSegmented {\n    options: options,\n    value: value(),\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "尺寸".to_string(),
                description: Some("Sm / Md / Lg 三种尺寸。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        Segmented { options: vec![("小".to_string(), "sm".to_string()), ("中".to_string(), "md".to_string()), ("大".to_string(), "lg".to_string())], size: Size::Sm }
                        Segmented { options: vec![("小".to_string(), "sm".to_string()), ("中".to_string(), "md".to_string()), ("大".to_string(), "lg".to_string())], size: Size::Md }
                        Segmented { options: vec![("小".to_string(), "sm".to_string()), ("中".to_string(), "md".to_string()), ("大".to_string(), "lg".to_string())], size: Size::Lg }
                    }
                },
                code: "Segmented { options: vec![...], size: Size::Sm }\nSegmented { options: vec![...], size: Size::Md }\nSegmented { options: vec![...], size: Size::Lg }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("整体禁用或禁用单个选项。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        Segmented { options: vec![("日".to_string(), "day".to_string()), ("周".to_string(), "week".to_string()), ("月".to_string(), "month".to_string())], value: "周".to_string(), disabled: true }
                    }
                },
                code: "Segmented { options: vec![...], disabled: true }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Segmented Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("options", "Vec<String>", "[]", "选项列表"),
                ("value", "String", "\"\"", "当前选中值"),
                ("size", "Size", "Md", "组件尺寸"),
                ("disabled", "bool", "false", "是否禁用"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<String>>", "None", "值变化回调"),
            ]}
        }
    }
}
