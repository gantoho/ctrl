use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TypewriterPage() -> Element {
    rsx! {
div { id: "typewriter", style: "margin-top: 64px;",
            h1 {
                "Typewriter 打字机"
            }
            p {
                "循环地逐字输入、停留、删除文本列表，营造打字机动画效果，常用于首屏标语、动态介绍等场景。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("传入 words 列表，组件会循环打印每一句。".to_string()),
                demo: rsx! {
                    div { style: "font-size:28px; font-weight:600;",
                        span { style: "color:var(--ctrl-text-secondary);", "我们专注于 " }
                        Typewriter {
                            words: vec!["高性能 Web".to_string(), "跨平台应用".to_string(), "极致体验".to_string()],
                        }
                    }
                },
                code: "Typewriter {\n    words: vec![\"高性能 Web\".to_string(), \"跨平台应用\".to_string(), \"极致体验\".to_string()],\n}".to_string(),
            }

            DemoBox {
                title: "调整速度".to_string(),
                description: Some("通过 type_speed / delete_speed / pause 控制节奏（毫秒）。".to_string()),
                demo: rsx! {
                    Space { gap: "md".to_string(), direction: Direction::Vertical,
                        div { style: "font-size:22px; color:var(--ctrl-text);",
                            Typewriter {
                                words: vec!["快速打字 fast".to_string(), "迅捷删除 quick".to_string()],
                                type_speed: 60.0,
                                delete_speed: 30.0,
                                pause: 800.0,
                            }
                        }
                        div { style: "font-size:22px; color:var(--ctrl-text);",
                            Typewriter {
                                words: vec!["慢速打字 slow".to_string(), "从容删除 easy".to_string()],
                                type_speed: 220.0,
                                delete_speed: 120.0,
                                pause: 2000.0,
                            }
                        }
                    }
                },
                code: "Typewriter {\n    words: vec![\"快速打字\".to_string()],\n    type_speed: 60.0, delete_speed: 30.0, pause: 800.0,\n}".to_string(),
            }

            DemoBox {
                title: "隐藏光标".to_string(),
                description: Some("cursor 设为 false 时不显示闪烁光标。".to_string()),
                demo: rsx! {
                    div { style: "font-size:24px; color:var(--ctrl-primary); font-weight:600;",
                        Typewriter {
                            words: vec!["无光标模式".to_string(), "简洁纯净".to_string()],
                            cursor: false,
                        }
                    }
                },
                code: "Typewriter {\n    words: vec![\"无光标模式\".to_string()],\n    cursor: false,\n}".to_string(),
            }

            h2 { "Typewriter Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("words", "Vec<String>", "—", "循环播放的文本列表"),
                ("type_speed", "f64", "130.0", "打字速度（毫秒/字）"),
                ("delete_speed", "f64", "60.0", "删除速度（毫秒/字）"),
                ("pause", "f64", "1600.0", "完整单词停留时长（毫秒）"),
                ("cursor", "bool", "true", "是否显示光标"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}
