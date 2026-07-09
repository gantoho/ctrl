use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn BorderBeamPage() -> Element {
    rsx! {
div { id: "border-beam", style: "margin-top: 64px;",
            h1 {
                "BorderBeam 边框光束"
            }
            p {
                "在容器边框轨道上循环流动一束光点，用于强调卡片、AI 输入框、特色定价卡等。使用 @property 驱动 conic-gradient 角度旋转，绕行无断裂，并在 prefers-reduced-motion 下自动停用动画。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("包裹任意内容即可获得流动边框光束。".to_string()),
                demo: rsx! {
                    BorderBeam {
                        style: "width:280px;".to_string(),
                        div { style: "padding:24px;",
                            div { style: "font-weight:600; color:var(--ctrl-text); margin-bottom:6px;", "Ctrl UI" }
                            div { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "一束光沿着边框缓缓流动。" }
                        }
                    }
                },
                code: "BorderBeam {\n    div { style: \"padding:24px;\", \"...\" }\n}".to_string(),
            }

            DemoBox {
                title: "速度与厚度".to_string(),
                description: Some("通过 duration 控制绕行速度，size 控制轨道厚度，reverse 反向。".to_string()),
                demo: rsx! {
                    div { style: "display:flex; gap:16px; flex-wrap:wrap;",
                        BorderBeam {
                            duration: 2,
                            style: "width:180px;".to_string(),
                            div { style: "padding:20px; text-align:center; color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "快速 2s" }
                        }
                        BorderBeam {
                            size: 4,
                            style: "width:180px;".to_string(),
                            div { style: "padding:20px; text-align:center; color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "粗轨道 4px" }
                        }
                        BorderBeam {
                            reverse: true,
                            style: "width:180px;".to_string(),
                            div { style: "padding:20px; text-align:center; color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "反向绕行" }
                        }
                    }
                },
                code: "BorderBeam { duration: 2, ... }\nBorderBeam { size: 4, ... }\nBorderBeam { reverse: true, ... }".to_string(),
            }

            DemoBox {
                title: "自定义颜色".to_string(),
                description: Some("通过 color_from / color_to 自定义光束渐变色。".to_string()),
                demo: rsx! {
                    div { style: "display:flex; gap:16px; flex-wrap:wrap;",
                        BorderBeam {
                            color_from: "#06b6d4".to_string(),
                            color_to: "#3b82f6".to_string(),
                            style: "width:200px;".to_string(),
                            div { style: "padding:20px; text-align:center; color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "青→蓝" }
                        }
                        BorderBeam {
                            color_from: "#f59e0b".to_string(),
                            color_to: "#ef4444".to_string(),
                            radius: "999px".to_string(),
                            style: "width:200px;".to_string(),
                            div { style: "padding:20px; text-align:center; color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "橙→红 · 胶囊" }
                        }
                    }
                },
                code: "BorderBeam {\n    color_from: \"#06b6d4\".to_string(),\n    color_to: \"#3b82f6\".to_string(),\n}\nBorderBeam {\n    color_from: \"#f59e0b\".to_string(),\n    color_to: \"#ef4444\".to_string(),\n    radius: \"999px\".to_string(),\n}".to_string(),
            }

            DemoBox {
                title: "结合按钮".to_string(),
                description: Some("用 BorderBeam 包裹 Button，通过内边距让光束在按钮四周绕行，适合强调主行动按钮。".to_string()),
                demo: rsx! {
                    div { style: "display:flex; gap:20px; flex-wrap:wrap; align-items:center;",
                        BorderBeam {
                            radius: "0.5rem".to_string(),
                            style: "display:inline-block;".to_string(),
                            Button {
                                variant: Variant::Primary,
                                onclick: move |_| {},
                                "✨ AI 生成"
                            }
                        }
                        BorderBeam {
                            color_from: "#06b6d4".to_string(),
                            color_to: "#3b82f6".to_string(),
                            radius: "999px".to_string(),
                            style: "display:inline-block;".to_string(),
                            Button {
                                variant: Variant::Outline,
                                style: "border-radius:999px;".to_string(),
                                onclick: move |_| {},
                                "立即体验"
                            }
                        }
                    }
                },
                code: "BorderBeam {\n    radius: \"0.5rem\".to_string(),\n    style: \"display:inline-block;\".to_string(),\n    Button { variant: Variant::Primary, \"✨ AI 生成\" }\n}".to_string(),
            }

            h2 { "BorderBeam Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("size", "u32", "2", "边框（光束轨道）厚度，px"),
                ("duration", "u32", "4", "绕行一周时长，秒"),
                ("color_from", "String", "主题主色", "光束渐变起始色"),
                ("color_to", "String", "#a855f7", "光束渐变结束色"),
                ("radius", "String", "--ctrl-radius-lg", "圆角"),
                ("reverse", "bool", "false", "是否反向绕行"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}
