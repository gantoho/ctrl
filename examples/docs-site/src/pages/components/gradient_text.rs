use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn GradientTextPage() -> Element {
    rsx! {
div { id: "gradient-text", style: "margin-top: 64px;",
            h1 {
                "GradientText 流光文字"
            }
            p {
                "基于 background-clip:text 的渐变动画文字组件，支持自定义渐变色与流速，适合标题、品牌文案等场景。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("默认使用 Ctrl 主题色的渐变文字，animated 默认为 true。".to_string()),
                demo: rsx! {
                    GradientText { style: "font-size:36px;",
                        "Ctrl UI — Build Fast, Ship Faster"
                    }
                },
                code: "GradientText { style: \"font-size:36px;\",\n    \"Ctrl UI — Build Fast, Ship Faster\"\n}".to_string(),
            }

            DemoBox {
                title: "自定义渐变色".to_string(),
                description: Some("通过 colors 属性传入逗号分隔的颜色列表自定义渐变。".to_string()),
                demo: rsx! {
                    Space { gap: "md".to_string(), direction: Direction::Vertical,
                        GradientText { style: "font-size:32px;", colors: "#ff6b35, #f7c948, #00b4d8".to_string(), "橙黄蓝渐变" }
                        GradientText { style: "font-size:32px;", colors: "#ff0080, #7928ca, #00d2ff".to_string(), "粉紫青渐变" }
                    }
                },
                code: "GradientText { colors: \"#ff6b35, #f7c948, #00b4d8\".to_string(),\n    \"橙黄蓝渐变\"\n}".to_string(),
            }

            DemoBox {
                title: "不同速度".to_string(),
                description: Some("通过 speed 控制动画周期，值越大流动越慢。".to_string()),
                demo: rsx! {
                    Space { gap: "lg".to_string(), direction: Direction::Vertical,
                        GradientText { style: "font-size:28px;", speed: 2.0, "2 秒 — 快速流动" }
                        GradientText { style: "font-size:28px;", speed: 6.0, "6 秒 — 中速流动" }
                        GradientText { style: "font-size:28px;", speed: 12.0, "12 秒 — 缓慢流动" }
                    }
                },
                code: "GradientText { speed: 2.0, \"快速\" }\nGradientText { speed: 6.0, \"中速\" }\nGradientText { speed: 12.0, \"慢速\" }".to_string(),
            }

            DemoBox {
                title: "无动画静态渐变".to_string(),
                description: Some("animated 设为 false 时可作为静态渐变标题使用。".to_string()),
                demo: rsx! {
                    GradientText { style: "font-size:32px;", animated: false, colors: "#e63946, #457b9d".to_string(), "静态渐变标题" }
                },
                code: "GradientText { animated: false, colors: \"#e63946, #457b9d\".to_string(),\n    \"静态渐变标题\"\n}".to_string(),
            }

            h2 { "GradientText Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("colors", "String", "主题色渐变", "逗号分隔的 CSS 颜色值"),
                ("animated", "bool", "true", "是否启用流光动画"),
                ("speed", "f64", "4.0", "动画周期（秒）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "文字内容"),
            ]}
        }
    }
}
