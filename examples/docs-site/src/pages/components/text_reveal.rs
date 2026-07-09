use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TextRevealPage() -> Element {
    let mut key = use_signal(|| 0u32);

    rsx! {
div { id: "text-reveal", style: "margin-top: 64px;",
            h1 {
                "TextReveal 文字揭示"
            }
            p {
                "将文本按词或按字拆分，逐个交错淡入上浮，形成揭示式入场。常用于标题、标语。"
            }

            DemoBox {
                title: "按词揭示".to_string(),
                description: Some("默认按空格分词，逐词入场。点击重放。".to_string()),
                demo: rsx! {
                    div {
                        Button {
                            variant: Variant::Outline,
                            size: Size::Sm,
                            onclick: move |_| key += 1,
                            "重放动画"
                        }
                        div { style: "margin-top:16px; font-size:28px; font-weight:800; color:var(--ctrl-text);",
                            for k in [key()] {
                                TextReveal { key: "{k}", text: "Build beautiful UI with Ctrl".to_string() }
                            }
                        }
                    }
                },
                code: "TextReveal { text: \"Build beautiful UI with Ctrl\".to_string() }".to_string(),
            }

            DemoBox {
                title: "逐字揭示".to_string(),
                description: Some("mode 设为 char 时逐字入场，适合中文短句。".to_string()),
                demo: rsx! {
                    div { style: "font-size:26px; font-weight:700; color:var(--ctrl-primary);",
                        for k in [key()] {
                            TextReveal { key: "{k}", text: "掌控每一处细节".to_string(), mode: "char".to_string(), stagger: 90 }
                        }
                    }
                },
                code: "TextReveal { text: \"掌控每一处细节\".to_string(), mode: \"char\".to_string(), stagger: 90 }".to_string(),
            }

            h2 { "TextReveal Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("text", "String", "—", "要揭示的文本"),
                ("mode", "String", "\"word\"", "揭示粒度：word / char"),
                ("stagger", "u32", "60", "相邻单元入场延迟（毫秒）"),
                ("duration", "u32", "500", "单元动画时长（毫秒）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}
