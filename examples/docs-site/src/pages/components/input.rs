use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::components::codeblock::CodeBlock;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn InputPage() -> Element {
    rsx! {
div { id: "input", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Input 输入框"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 24px;",
                "输入框用于文本输入。采用受控组件模式，支持尺寸、状态和事件绑定。"
            }

            div {
                style: "padding: 12px 16px; background: var(--ctrl-primary-light); border-radius: var(--ctrl-radius-md); margin-bottom: 32px; display: flex; align-items: flex-start; gap: 8px; font-size: var(--ctrl-font-size-md);",
                span { "💡" }
                span { strong { "注意: " } "Input 使用受控组件模式，必须通过 value + oninput 管理值。oninput 闭包参数需显式标注为 FormEvent。" }
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 oninput 实现双向绑定。".to_string()),
                demo: rsx! { BasicInputDemo {} },
                code: "let mut value = use_signal(|| String::new());\n\nInput {\n    placeholder: \"请输入内容\",\n    value: value(),\n    oninput: move |evt: FormEvent| value.set(evt.value()),\n}".to_string(),
            }

            DemoBox {
                title: "三种尺寸".to_string(),
                description: Some("Sm 28px / Md 36px / Lg 44px".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 360px;",
                        Input { placeholder: "Small", size: Size::Sm }
                        Input { placeholder: "Medium", size: Size::Md }
                        Input { placeholder: "Large", size: Size::Lg }
                    }
                },
                code: "Input { placeholder: \"Small\", size: Size::Sm }\nInput { placeholder: \"Medium\", size: Size::Md }\nInput { placeholder: \"Large\", size: Size::Lg }".to_string(),
            }

            DemoBox {
                title: "状态".to_string(),
                description: Some("禁用、只读、错误状态。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 360px;",
                        Input { placeholder: "禁用状态", disabled: true }
                        Input { placeholder: "只读状态", readonly: true, value: "只读内容".to_string() }
                        Input { placeholder: "错误状态", error: true }
                    }
                },
                code: "Input { placeholder: \"禁用状态\", disabled: true }\nInput { placeholder: \"只读状态\", readonly: true, value: \"只读内容\" }\nInput { placeholder: \"错误状态\", error: true }".to_string(),
            }

            DemoBox {
                title: "表单验证".to_string(),
                description: Some("点击提交时验证邮箱和密码，实时清除错误状态。".to_string()),
                demo: rsx! { FormValidationDemo {} },
                code: "// 代码见页面下方".to_string(),
            }

            h3 { style: "font-size: 1rem; font-weight: 600; color: var(--ctrl-text); margin: 24px 0 12px;", "表单验证完整代码" }
            CodeBlock { code: [
                "let mut email = use_signal(|| String::new());",
                "let mut password = use_signal(|| String::new());",
                "let mut errors = use_signal(|| (false, false));",
                "let mut submitted = use_signal(|| false);",
                "",
                "let validate = move |_| {",
                "    let e = (email().trim().is_empty(), password().is_empty());",
                "    errors.set(e);",
                "    if !e.0 && !e.1 { submitted.set(true); }",
                "};",
                "",
                "rsx! {",
                "    Input { placeholder: \"邮箱\", value: email(), error: errors().0,",
                "        oninput: move |evt: FormEvent| { email.set(evt.value()); },",
                "    }",
                "    Input { r#type: \"password\", placeholder: \"密码\", value: password(), error: errors().1,",
                "        oninput: move |evt: FormEvent| { password.set(evt.value()); },",
                "    }",
                "    Button { variant: Variant::Primary, block: true, onclick: validate, \"提交\" }",
                "}",
            ].join("\n"), lang: Some("rust".to_string()) }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 48px 0 20px;", "Input Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("size", "Size", "Md", "输入框尺寸"),
                ("value", "String", "\"\"", "当前值（受控）"),
                ("placeholder", "String", "\"\"", "占位文本"),
                ("disabled", "bool", "false", "是否禁用"),
                ("readonly", "bool", "false", "是否只读"),
                ("error", "bool", "false", "是否错误状态"),
                ("r#type", "String", "\"text\"", "原生 input type"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("oninput", "Option<EventHandler>", "None", "输入事件"),
            ]}
        }
    }
}
