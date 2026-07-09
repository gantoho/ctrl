use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn DotPatternPage() -> Element {
    rsx! {
div { id: "dot-pattern", style: "margin-top: 64px;",
            h1 {
                "DotPattern 点阵背景"
            }
            p {
                "使用纯 CSS 渐变绘制点阵或网格装饰背景，常用于 Hero 区、卡片、分区背景。children 展示在图案之上，可选中心径向淡出遮罩使边缘渐隐。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("包裹内容即可获得点阵背景，默认带中心淡出。".to_string()),
                demo: rsx! {
                    DotPattern {
                        style: "height:220px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); display:flex; align-items:center; justify-content:center;".to_string(),
                        div { style: "text-align:center; position:relative;",
                            div { style: "font-size:22px; font-weight:800; color:var(--ctrl-text);", "点阵背景" }
                            div { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "干净的装饰底纹" }
                        }
                    }
                },
                code: "DotPattern {\n    style: \"height:220px;\".to_string(),\n    div { \"...\" }\n}".to_string(),
            }

            DemoBox {
                title: "网格与自定义".to_string(),
                description: Some("grid 切换为网格线，gap 控制间距，color 自定义颜色。".to_string()),
                demo: rsx! {
                    div { style: "display:grid; grid-template-columns:1fr 1fr; gap:16px;",
                        DotPattern {
                            grid: true,
                            gap: 24,
                            style: "height:160px; border-radius:var(--ctrl-radius-md); border:1px solid var(--ctrl-border);".to_string(),
                            div { style: "height:100%; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary); position:relative;", "网格线" }
                        }
                        DotPattern {
                            gap: 16,
                            dot_size: 2,
                            color: "var(--ctrl-primary)".to_string(),
                            fade: false,
                            style: "height:160px; border-radius:var(--ctrl-radius-md); border:1px solid var(--ctrl-border);".to_string(),
                            div { style: "height:100%; display:flex; align-items:center; justify-content:center; color:var(--ctrl-text-secondary); position:relative;", "主色密点 · 无淡出" }
                        }
                    }
                },
                code: "DotPattern { grid: true, gap: 24, ... }\nDotPattern { gap: 16, dot_size: 2, color: \"var(--ctrl-primary)\".into(), fade: false, ... }".to_string(),
            }

            h2 { "DotPattern Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("gap", "u32", "20", "图案单元间距 px"),
                ("dot_size", "u32", "1", "点半径 / 线宽 px"),
                ("color", "String", "主题边框色", "点 / 线颜色"),
                ("grid", "bool", "false", "使用网格线代替圆点"),
                ("fade", "bool", "true", "中心径向淡出遮罩"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "覆盖在图案之上的内容"),
            ]}
        }
    }
}
