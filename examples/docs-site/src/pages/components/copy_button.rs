use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn CopyButtonPage() -> Element {
    rsx! {
div { id: "copy-button", style: "margin-top: 64px;",
            h1 {
                "CopyButton 复制按钮"
            }
            p {
                "点击将指定文本复制到剪贴板，并在短时间内展示成功态（文案与图标切换），常用于复制代码、令牌、分享链接等。复用 Button 保持视觉一致。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("点击复制，2 秒后自动恢复。".to_string()),
                demo: rsx! {
                    CopyButton { text: "https://ctrl-ui.dev".to_string() }
                },
                code: "CopyButton { text: \"https://ctrl-ui.dev\".to_string() }".to_string(),
            }

            DemoBox {
                title: "变体与尺寸".to_string(),
                description: Some("通过 variant 与 size 控制外观。".to_string()),
                demo: rsx! {
                    Space { gap: "md".to_string(),
                        CopyButton { text: "primary".to_string(), variant: Variant::Primary }
                        CopyButton { text: "outline".to_string(), variant: Variant::Outline }
                        CopyButton { text: "ghost".to_string(), variant: Variant::Ghost, size: Size::Sm }
                    }
                },
                code: "CopyButton { text: \"...\".to_string(), variant: Variant::Primary }\nCopyButton { text: \"...\".to_string(), variant: Variant::Ghost, size: Size::Sm }".to_string(),
            }

            DemoBox {
                title: "自定义文案与无图标".to_string(),
                description: Some("通过 label / copied_label 自定义文案，show_icon 控制图标显示。".to_string()),
                demo: rsx! {
                    Space { gap: "md".to_string(),
                        CopyButton {
                            text: "token-abc-123".to_string(),
                            label: "复制令牌".to_string(),
                            copied_label: "已复制到剪贴板".to_string(),
                        }
                        CopyButton {
                            text: "no-icon".to_string(),
                            show_icon: false,
                        }
                    }
                },
                code: "CopyButton {\n    text: \"token-abc-123\".to_string(),\n    label: \"复制令牌\".to_string(),\n    copied_label: \"已复制到剪贴板\".to_string(),\n}\nCopyButton { text: \"no-icon\".to_string(), show_icon: false }".to_string(),
            }

            DemoBox {
                title: "复制回调".to_string(),
                description: Some("oncopy 在复制成功后触发，返回被复制的文本。".to_string()),
                demo: rsx! {
                    CopyButtonCallbackDemo {}
                },
                code: "CopyButton {\n    text: content(),\n    oncopy: move |t: String| log.set(format!(\"已复制：{}\", t)),\n}".to_string(),
            }

            h2 { "CopyButton Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("text", "String", "—", "待复制的文本内容"),
                ("label", "String", "复制", "默认文案"),
                ("copied_label", "String", "已复制", "复制成功后的文案"),
                ("show_icon", "bool", "true", "是否显示图标"),
                ("duration", "u32", "2000", "成功态持续时间（毫秒）"),
                ("variant", "Variant", "Outline", "按钮变体"),
                ("size", "Size", "Md", "按钮尺寸"),
                ("oncopy", "EventHandler<String>", "—", "复制成功回调（返回被复制文本）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn CopyButtonCallbackDemo() -> Element {
    let mut log = use_signal(|| "（尚未复制）".to_string());

    rsx! {
        div { style: "display:flex; align-items:center; gap:16px; flex-wrap:wrap;",
            CopyButton {
                text: "Ctrl UI 组件库".to_string(),
                oncopy: move |t: String| log.set(format!("已复制：{}", t)),
            }
            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);",
                "{log}"
            }
        }
    }
}
