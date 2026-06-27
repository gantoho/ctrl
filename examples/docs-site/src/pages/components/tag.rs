use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TagPage() -> Element {
    rsx! {
div { id: "tag", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Tag 标签"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "标签用于标记和分类，支持多种颜色和可关闭模式。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 color 属性设置标签颜色，默认为主题色。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                        Tag { color: "var(--ctrl-primary)".to_string(), "Primary" }
                        Tag { color: "var(--ctrl-success)".to_string(), "Success" }
                        Tag { color: "var(--ctrl-warning)".to_string(), "Warning" }
                        Tag { color: "var(--ctrl-danger)".to_string(), "Danger" }
                        Tag { color: "var(--ctrl-info)".to_string(), "Info" }
                    }
                },
                code: "Tag { color: \"var(--ctrl-primary)\".to_string(), \"Primary\" }\nTag { color: \"var(--ctrl-success)\".to_string(), \"Success\" }\nTag { color: \"var(--ctrl-warning)\".to_string(), \"Warning\" }\nTag { color: \"var(--ctrl-danger)\".to_string(), \"Danger\" }".to_string(),
            }

            DemoBox {
                title: "可关闭".to_string(),
                description: Some("设置 closable 为 true 可以显示关闭按钮，点击后标签消失。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                        Tag { color: "var(--ctrl-primary)".to_string(), closable: true, "可关闭" }
                        Tag { color: "var(--ctrl-success)".to_string(), closable: true, "成功" }
                        Tag { color: "var(--ctrl-warning)".to_string(), closable: true, "警告" }
                    }
                },
                code: "Tag { color: \"var(--ctrl-primary)\".to_string(), closable: true, \"可关闭\" }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Tag Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("color", "String", "var(--ctrl-primary)", "标签颜色（CSS 颜色值）"),
                ("closable", "bool", "false", "是否可关闭"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭事件回调"),
                ("children", "Element", "—", "标签内容"),
            ]}
        }
    }
}
