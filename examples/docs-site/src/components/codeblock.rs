use dioxus::prelude::*;

/// 代码展示组件
#[component]
#[allow(non_snake_case)]
pub fn CodeBlock(code: String, lang: Option<String>) -> Element {
    let lang_attr = lang.unwrap_or_else(|| "rust".to_string());

    let style_str = [
        "margin:0",
        "padding:16px 20px",
        "background:var(--ctrl-bg)",
        "color:var(--ctrl-text)",
        "font-size:0.8125rem",
        "line-height:1.6",
        "overflow-x:auto",
        "border:1px solid var(--ctrl-border)",
        "border-radius:var(--ctrl-radius-md)",
    ].join("; ");

    rsx! {
        pre {
            style: "{style_str}",
            code {
                class: "language-{lang_attr}",
                "{code}"
            }
        }
    }
}
