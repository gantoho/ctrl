use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn RipplePage() -> Element {
    rsx! {
div { id: "ripple", style: "margin-top: 64px;",
            h1 {
                "Ripple 涟漪背景"
            }
            p {
                "由中心向外逐层扩散的同心圆背景装饰，常用于 Hero 区、卡片背景、加载区等。children 展示在涟漪之上。支持 prefers-reduced-motion 下停用。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("包裹内容即可获得同心圆涟漪背景。".to_string()),
                demo: rsx! {
                    Ripple {
                        style: "height:240px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); background:var(--ctrl-bg-secondary); display:flex; align-items:center; justify-content:center;".to_string(),
                        div { style: "text-align:center;",
                            div { style: "font-size:20px; font-weight:700; color:var(--ctrl-text);", "Ctrl UI" }
                            div { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "水波扩散" }
                        }
                    }
                },
                code: "Ripple {\n    style: \"height:240px;\".to_string(),\n    div { \"...\" }\n}".to_string(),
            }

            DemoBox {
                title: "层数与颜色".to_string(),
                description: Some("通过 circles 控制层数，color 自定义颜色。".to_string()),
                demo: rsx! {
                    Ripple {
                        circles: 8,
                        color: "#a855f7".to_string(),
                        style: "height:240px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); background:var(--ctrl-bg-secondary); display:flex; align-items:center; justify-content:center;".to_string(),
                        span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "circles = 8" }
                    }
                },
                code: "Ripple { circles: 8, color: \"#a855f7\".to_string(), ... }".to_string(),
            }

            h2 { "Ripple Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("circles", "usize", "6", "圆环层数"),
                ("base_size", "u32", "80", "最内层圆直径 px"),
                ("step", "u32", "70", "每层递增直径 px"),
                ("duration", "u32", "3", "单圈动画时长（秒）"),
                ("color", "String", "主题主色", "圆环颜色"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "覆盖在涟漪之上的内容"),
            ]}
        }
    }
}
