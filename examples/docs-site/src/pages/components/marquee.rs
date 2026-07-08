use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn MarqueePage() -> Element {
    rsx! {
div { id: "marquee", style: "margin-top: 64px;",
            h1 {
                "Marquee 跑马灯"
            }
            p {
                "无缝循环滚动容器，常用于公告栏、品牌墙、Logo 展示，支持横向/纵向方向与悬停暂停。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("横向无缝滚动，鼠标悬停时暂停。".to_string()),
                demo: rsx! {
                    Marquee {
                        for i in 1..=6 {
                            Tag { key: "{i}", color: "var(--ctrl-primary)".to_string(), "标签内容 {i}" }
                        }
                    }
                },
                code: "Marquee {\n    for i in 1..=6 {\n        Tag { color: \"var(--ctrl-primary)\".to_string(), \"标签内容 {i}\" }\n    }\n}".to_string(),
            }

            DemoBox {
                title: "反向滚动".to_string(),
                description: Some("direction 设为 right 时向右滚动。".to_string()),
                demo: rsx! {
                    Marquee { direction: "right".to_string(), speed: 10.0,
                        for name in ["Rust", "Dioxus", "WebAssembly", "Ctrl UI", "TypeScript"] {
                            span { key: "{name}", style: "font-weight:600; color:var(--ctrl-text); padding:8px 16px; background:var(--ctrl-bg-secondary); border-radius:var(--ctrl-radius-md);", "{name}" }
                        }
                    }
                },
                code: "Marquee { direction: \"right\".to_string(), speed: 10.0,\n    span { \"Rust\" }\n}".to_string(),
            }

            DemoBox {
                title: "品牌墙".to_string(),
                description: Some("常用于展示合作伙伴或技术栈 Logo。".to_string()),
                demo: rsx! {
                    Marquee { speed: 14.0,
                        for i in 0..5 {
                            div { key: "{i}", style: "display:flex; align-items:center; gap:8px; padding:12px 20px; border:1px solid var(--ctrl-border); border-radius:var(--ctrl-radius-lg); background:var(--ctrl-bg);",
                                Avatar { size: Size::Sm, src: format!("https://i.pravatar.cc/40?img={}", i + 10), alt: "logo".to_string(), "" }
                                span { style: "color:var(--ctrl-text); font-weight:500;", {["Acme", "Globex", "Initech", "Umbrella", "Soylent"][i]} }
                            }
                        }
                    }
                },
                code: "Marquee { speed: 14.0,\n    div { Avatar { .. } span { \"Acme\" } }\n}".to_string(),
            }

            DemoBox {
                title: "纵向滚动".to_string(),
                description: Some("direction 设为 up 时纵向向上滚动，需要固定高度容器。".to_string()),
                demo: rsx! {
                    div { style: "height:160px; max-width:280px; border:1px solid var(--ctrl-border); border-radius:var(--ctrl-radius-md);",
                        Marquee { direction: "up".to_string(), speed: 8.0,
                            for i in 1..=5 {
                                div { key: "{i}", style: "padding:10px 16px; color:var(--ctrl-text); font-size:var(--ctrl-font-size-sm);",
                                    "📢 系统公告 {i} —— 这是一条滚动通知信息"
                                }
                            }
                        }
                    }
                },
                code: "div { style: \"height:160px;\",\n    Marquee { direction: \"up\".to_string(),\n        div { \"公告内容\" }\n    }\n}".to_string(),
            }

            h2 { "Marquee Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("direction", "String", "\"left\"", "滚动方向：left / right / up / down"),
                ("speed", "f64", "20.0", "滚动一份内容所需时长（秒），越大越慢"),
                ("pause_on_hover", "bool", "true", "悬停时暂停"),
                ("gap", "String", "var(--ctrl-spacing-lg)", "元素间距（同时作为拷贝间距，保证首尾衔接一致）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "滚动内容（自动复制一份实现无缝循环）"),
            ]}
        }
    }
}
