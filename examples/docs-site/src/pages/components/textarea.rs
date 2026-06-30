use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TextareaPage() -> Element {
    rsx! {
div { id: "textarea", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Textarea 多行输入"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 24px;",
                "多行文本输入组件。支持字数统计、最大长度限制、resize 控制等。采用受控组件模式。"
            }

            div {
                style: "padding: 12px 16px; background: var(--ctrl-primary-light); border-radius: var(--ctrl-radius-md); margin-bottom: 32px; display: flex; align-items: flex-start; gap: 8px; font-size: var(--ctrl-font-size-md);",
                span { "💡" }
                span { strong { "注意: " } "Textarea 使用受控组件模式，必须通过 value + onchange 管理值。show_count 需要配合 maxlength 使用。" }
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 onchange 实现双向绑定。".to_string()),
                demo: rsx! {
                    Textarea { placeholder: "请输入内容", rows: 3 }
                },
                code: "let mut value = use_signal(|| String::new());\n\nTextarea {\n    placeholder: \"请输入内容\",\n    rows: 3,\n    value: value(),\n    onchange: move |v: String| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "三种尺寸".to_string(),
                description: Some("Sm / Md / Lg 三种尺寸变体。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        Textarea { placeholder: "Small", rows: 2, size: Size::Sm }
                        Textarea { placeholder: "Medium（默认）", rows: 2, size: Size::Md }
                        Textarea { placeholder: "Large", rows: 2, size: Size::Lg }
                    }
                },
                code: r#"Textarea { placeholder: "Small", rows: 2, size: Size::Sm }
Textarea { placeholder: "Medium", rows: 2, size: Size::Md }
Textarea { placeholder: "Large", rows: 2, size: Size::Lg }"#.to_string(),
            }

            DemoBox {
                title: "字数统计与最大长度".to_string(),
                description: Some("设置 maxlength 限制最大字符数，show_count 显示当前字数。".to_string()),
                demo: rsx! {
                    Textarea { placeholder: "最多输入 100 个字符", rows: 3, maxlength: 100, show_count: true }
                },
                code: "Textarea {\n    placeholder: \"最多输入 100 个字符\",\n    rows: 3,\n    maxlength: 100,\n    show_count: true,\n    value: value(),\n    onchange: move |v: String| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "禁用与只读".to_string(),
                description: Some("disabled 使输入框不可交互，readonly 仅可查看。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        Textarea { placeholder: "禁用状态", rows: 2, disabled: true, value: "这段文本被禁用了".to_string() }
                        Textarea { placeholder: "只读状态", rows: 2, readonly: true, value: "这段文本是只读的".to_string() }
                    }
                },
                code: r#"Textarea { placeholder: "禁用状态", rows: 2, disabled: true }
Textarea { placeholder: "只读状态", rows: 2, readonly: true }"#.to_string(),
            }

            DemoBox {
                title: "错误状态".to_string(),
                description: Some("设置 error 为 true 显示错误样式。".to_string()),
                demo: rsx! {
                    Textarea { placeholder: "请输入内容", rows: 3, error: true }
                },
                code: "Textarea { placeholder: \"请输入内容\", rows: 3, error: true }".to_string(),
            }

            DemoBox {
                title: "Resize 控制".to_string(),
                description: Some("通过 resize 属性控制拖拽缩放行为（none / both / horizontal / vertical）。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        Textarea { placeholder: "不可拖拽 (none)", rows: 2, resize: "none".to_string() }
                        Textarea { placeholder: "默认 (vertical)", rows: 2 }
                        Textarea { placeholder: "水平拖拽 (horizontal)", rows: 2, resize: "horizontal".to_string() }
                    }
                },
                code: r#"Textarea { placeholder: "不可拖拽", rows: 2, resize: "none".to_string() }
Textarea { placeholder: "默认垂直拖拽", rows: 2 }
Textarea { placeholder: "水平拖拽", rows: 2, resize: "horizontal".to_string() }"#.to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Textarea Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("value", "String", "\"\"", "当前值（受控组件）"),
                ("placeholder", "String", "\"\"", "占位文本"),
                ("rows", "u32", "3", "可见行数（控制高度）"),
                ("size", "Size", "Md", "尺寸：Sm / Md / Lg"),
                ("maxlength", "usize", "0", "最大字符数（0 不限制）"),
                ("show_count", "bool", "false", "显示字数统计（需配合 maxlength）"),
                ("auto_height", "bool", "false", "内容自适应高度（预留）"),
                ("disabled", "bool", "false", "是否禁用"),
                ("readonly", "bool", "false", "是否只读"),
                ("error", "bool", "false", "是否错误状态"),
                ("resize", "String", "\"vertical\"", "缩放行为：none / both / horizontal / vertical"),
                ("class", "String", "\"\"", "自定义 CSS 类名"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<String>>", "None", "值变化事件（传递新值）"),
                ("onfocus", "Option<EventHandler<FocusEvent>>", "None", "获得焦点事件"),
                ("onblur", "Option<EventHandler<FocusEvent>>", "None", "失去焦点事件"),
            ]}
        }
    }
}
