use dioxus::prelude::*;

/// Props 表格组件
#[component]
#[allow(non_snake_case)]
pub fn PropsTable(headers: Vec<String>, rows: Vec<(&'static str, &'static str, &'static str, &'static str)>) -> Element {
    rsx! {
        div { style: "overflow-x: auto; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); margin-bottom: 40px;",
            table { style: "width: 100%; border-collapse: collapse; font-size: var(--ctrl-font-size-md);",
                thead {
                    tr { style: "background: var(--ctrl-bg-secondary);",
                        {headers.iter().map(|h| rsx! {
                            th { style: "padding: 10px 16px; text-align: left; font-weight: 600; color: var(--ctrl-text); border-bottom: 1px solid var(--ctrl-border);", "{h}" }
                        })}
                    }
                }
                tbody {
                    {rows.iter().map(|(name, type_str, default, desc)| rsx! {
                        tr { style: "border-bottom: 1px solid var(--ctrl-border);",
                            td { style: "padding: 10px 16px; color: var(--ctrl-primary); font-weight: 500; font-family: monospace;", "{name}" }
                            td { style: "padding: 10px 16px; color: var(--ctrl-text-secondary); font-family: monospace;", "{type_str}" }
                            td { style: "padding: 10px 16px; color: var(--ctrl-text-secondary); font-family: monospace;", "{default}" }
                            td { style: "padding: 10px 16px; color: var(--ctrl-text);", "{desc}" }
                        }
                    })}
                }
            }
        }
    }
}