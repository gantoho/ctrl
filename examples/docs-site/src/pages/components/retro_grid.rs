use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn RetroGridPage() -> Element {
    rsx! {
div { id: "retro-grid", style: "margin-top: 64px;",
            h1 {
                "RetroGrid 复古网格"
            }
            p {
                "带透视的地平线网格向前无限流动，营造复古赛博 / synthwave 氛围背景。children 展示在网格之上。支持 prefers-reduced-motion 停用动画。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("包裹内容即可获得流动网格背景。".to_string()),
                demo: rsx! {
                    RetroGrid {
                        style: "height:260px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); display:flex; align-items:center; justify-content:center;".to_string(),
                        div { style: "text-align:center; position:relative;",
                            div { style: "font-size:26px; font-weight:800; color:var(--ctrl-text);", "RETRO GRID" }
                            div { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "synthwave 氛围背景" }
                        }
                    }
                },
                code: "RetroGrid {\n    style: \"height:260px;\".to_string(),\n    div { \"RETRO GRID\" }\n}".to_string(),
            }

            DemoBox {
                title: "颜色与速度".to_string(),
                description: Some("color 自定义网格线颜色，duration 控制流动速度，angle 调整透视俯仰。".to_string()),
                demo: rsx! {
                    RetroGrid {
                        color: "#ec4899".to_string(),
                        duration: 8.0,
                        angle: 55.0,
                        style: "height:220px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); display:flex; align-items:center; justify-content:center;".to_string(),
                        span { style: "color:var(--ctrl-text); font-weight:700; position:relative;", "粉色快速网格" }
                    }
                },
                code: "RetroGrid { color: \"#ec4899\".to_string(), duration: 8.0, angle: 55.0, ... }".to_string(),
            }

            h2 { "RetroGrid Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("duration", "f64", "15.0", "网格流动一轮时长（秒）"),
                ("angle", "f64", "65.0", "透视倾斜角度（度）"),
                ("color", "String", "主题主色", "网格线颜色"),
                ("cell_size", "u32", "60", "网格单元大小 px"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "覆盖在网格之上的内容"),
            ]}
        }
    }
}
