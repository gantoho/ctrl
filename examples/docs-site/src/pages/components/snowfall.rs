use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn SnowfallPage() -> Element {
    rsx! {
div { id: "snowfall", style: "margin-top: 64px;",
            h1 { "Snowfall 飘雪" }
            p {
                "雪花从顶部飘落到底部，每片大小、速度、水平漂移各不相同。常用于节日 / 冬季主题的背景装饰。children 展示在雪花之上。支持 prefers-reduced-motion 停用。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("包裹内容即可获得飘雪背景。".to_string()),
                demo: rsx! {
                    Snowfall {
                        style: "height:260px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); display:flex; align-items:center; justify-content:center;".to_string(),
                        div { style: "text-align:center; position:relative; z-index:1;",
                            div { style: "font-size:26px; font-weight:800; color:var(--ctrl-text);", "❄️" }
                            div { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm); margin-top:8px;", "飘雪季节" }
                        }
                    }
                },
                code: "Snowfall {\n    style: \"height:260px;\".to_string(),\n    div { \"❄️ 飘雪季节\" }\n}".to_string(),
            }

            DemoBox {
                title: "密度与速度".to_string(),
                description: Some("count 控制雪花数量，color 自定义颜色，min_duration/max_duration 控制速度范围。".to_string()),
                demo: rsx! {
                    Snowfall {
                        count: 30,
                        color: "#93c5fd".to_string(),
                        min_size: 3.0,
                        max_size: 8.0,
                        min_duration: 4.0,
                        max_duration: 10.0,
                        style: "height:220px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); display:flex; align-items:center; justify-content:center;".to_string(),
                        span { style: "color:var(--ctrl-text); font-weight:700; position:relative; z-index:1;", "蓝色轻雪" }
                    }
                },
                code: "Snowfall { count: 30, color: \"#93c5fd\".to_string(), min_duration: 4.0, max_duration: 10.0, ... }".to_string(),
            }

            h2 { "Snowfall Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("count", "u32", "60", "雪花数量"),
                ("color", "String", "\"#ffffff\"", "雪花颜色"),
                ("min_size", "f64", "2.0", "最小尺寸 px"),
                ("max_size", "f64", "6.0", "最大尺寸 px"),
                ("min_duration", "f64", "6.0", "最短落底时间（秒）"),
                ("max_duration", "f64", "14.0", "最长落底时间（秒）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "雪花之上的内容"),
            ]}
        }
    }
}
