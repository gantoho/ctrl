use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn OrbitingCirclesPage() -> Element {
    rsx! {
div { id: "orbiting-circles", style: "margin-top: 64px;",
            h1 {
                "OrbitingCircles 环绕轨道"
            }
            p {
                "若干元素沿以中心为圆心的圆形轨道匀速绕行，同一半径上的元素自动均匀分布。常用于展示技术栈、生态集成、连接关系等。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("传入 items 指定每个元素与其轨道半径，中心内容用 children。".to_string()),
                demo: rsx! {
                    div { style: "display:flex; justify-content:center;",
                        OrbitingCircles {
                            items: vec![
                                OrbitItem::new("⚛️", 60.0),
                                OrbitItem::new("🦀", 60.0),
                                OrbitItem::new("📦", 120.0).reverse(),
                                OrbitItem::new("🎨", 120.0).reverse(),
                                OrbitItem::new("⚡", 120.0).reverse(),
                            ],
                            div { style: "width:64px; height:64px; border-radius:50%; background:var(--ctrl-primary); color:#fff; display:flex; align-items:center; justify-content:center; font-weight:800; font-size:20px;",
                                "Ctrl"
                            }
                        }
                    }
                },
                code: "OrbitingCircles {\n    items: vec![\n        OrbitItem::new(\"⚛️\", 60.0),\n        OrbitItem::new(\"📦\", 120.0).reverse(),\n    ],\n    div { \"Ctrl\" }\n}".to_string(),
            }

            DemoBox {
                title: "速度与轨道".to_string(),
                description: Some("duration 控制绕行速度，show_path 关闭轨道虚线。".to_string()),
                demo: rsx! {
                    div { style: "display:flex; justify-content:center;",
                        OrbitingCircles {
                            size: 260.0,
                            duration: 10.0,
                            show_path: false,
                            items: vec![
                                OrbitItem::new("🌍", 90.0),
                                OrbitItem::new("🚀", 90.0),
                                OrbitItem::new("🛰️", 90.0),
                            ],
                            div { style: "font-size:36px;", "☀️" }
                        }
                    }
                },
                code: "OrbitingCircles { size: 260.0, duration: 10.0, show_path: false, items: vec![...] }".to_string(),
            }

            h2 { "OrbitingCircles Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("items", "Vec<OrbitItem>", "—", "环绕元素列表"),
                ("size", "f64", "320.0", "容器边长 px"),
                ("duration", "f64", "20.0", "绕行一圈时长（秒）"),
                ("show_path", "bool", "true", "是否绘制轨道虚线圆环"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "中心内容"),
            ]}

            h2 { "OrbitItem" }
            PropsTable { headers: vec!["方法 / 字段".to_string(), "类型".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("OrbitItem::new(content, radius)", "fn", "创建正向环绕元素", ""),
                (".reverse()", "fn", "设为反向环绕", ""),
                ("content", "String", "显示内容（emoji/文本）", ""),
                ("radius", "f64", "轨道半径 px", ""),
            ]}
        }
    }
}
