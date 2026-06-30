use dioxus::prelude::*;
use ctrl::prelude::*;
use super::codeblock::CodeBlock;

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
        Card {
            header: rsx! {
                Space { gap: "xs".to_string(), direction: Direction::Vertical,
                    h3 { "{title}" }
                    if let Some(desc) = description {
                        p { "{desc}" }
                    }
                }
            },

            Space { gap: "md".to_string(), direction: Direction::Vertical, style: "padding:16px;",
                div { style: "background:var(--ctrl-bg-secondary); padding:24px; border-radius:var(--ctrl-radius-md);",
                    {demo} {children}
                }
                Divider {}
                Button {
                    variant: Variant::Ghost,
                    block: true,
                    onclick: move |_| show_code.set(!show_code()),
                    if show_code() { "▲ 隐藏代码" } else { "▼ 显示代码" }
                }
                if show_code() {
                    CodeBlock { code: code, lang: Some("rust".to_string()) }
                }
            }
        }
    }
}
