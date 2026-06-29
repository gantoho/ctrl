use dioxus::prelude::*;

use super::codeblock::CodeBlock;

/// DemoBox —— 组件交互演示区 + 源代码展示
#[component]
#[allow(non_snake_case)]
pub fn DemoBox(
    title: String,
    description: Option<String>,
    demo: Element,
    code: String,
    children: Element,
) -> Element {
    let mut show_code = use_signal(|| false);

    rsx! {
        div {
            style: "margin-bottom: 32px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-lg);",

            div { style: "padding: 16px 20px; border-bottom: 1px solid var(--ctrl-border);",
                h3 { style: "font-size: 1rem; font-weight: 600; color: var(--ctrl-text); margin-bottom: 4px;", "{title}" }
                if let Some(desc) = description {
                    p { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin: 0;", "{desc}" }
                }
            }

            div { style: "padding: 24px; background: var(--ctrl-bg-secondary);", {demo} {children} }

            div { style: "border-top: 1px solid var(--ctrl-border);",
                button {
                    style: "width: 100%; padding: 10px 20px; background: transparent; border: none; cursor: pointer; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); display: flex; align-items: center; gap: 6px; font-family: inherit; transition: color var(--ctrl-transition);",
                    onclick: move |_| show_code.set(!show_code()),
                    if show_code() { "▼ 隐藏代码" } else { "▶ 显示代码" }
                }
            }

            if show_code() {
                div { style: "border-top: 1px solid var(--ctrl-border);",
                    CodeBlock { code: code, lang: Some("rust".to_string()) }
                }
            }
        }
    }
}
