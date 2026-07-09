use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TiltPage() -> Element {
    rsx! {
div { id: "tilt", style: "margin-top: 64px;",
            h1 {
                "Tilt 3D 倾斜"
            }
            p {
                "鼠标在卡片内移动时，卡片依据光标相对位置产生透视倾斜，营造立体悬浮效果，并可选高光反射跟随光标。移出后平滑复位。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("将内容包裹进 Tilt 即可获得跟随鼠标的 3D 倾斜。".to_string()),
                demo: rsx! {
                    div { style: "display:flex; justify-content:center; padding:24px;",
                        Tilt {
                            div { style: "width:260px; height:160px; border-radius:var(--ctrl-radius-lg); background:linear-gradient(135deg, var(--ctrl-primary), #a855f7); display:flex; align-items:center; justify-content:center; color:#fff; font-weight:700; font-size:18px; box-shadow:var(--ctrl-shadow-lg);",
                                "移动鼠标试试"
                            }
                        }
                    }
                },
                code: "Tilt {\n    div { class: \"card\", \"移动鼠标试试\" }\n}".to_string(),
            }

            DemoBox {
                title: "倾斜幅度与缩放".to_string(),
                description: Some("max_tilt 控制最大角度，scale 设置悬停缩放，perspective 调整立体强度。".to_string()),
                demo: rsx! {
                    div { style: "display:flex; justify-content:center; gap:24px; padding:24px; flex-wrap:wrap;",
                        Tilt {
                            max_tilt: 20.0,
                            scale: 1.06,
                            perspective: 600.0,
                            div { style: "width:180px; height:180px; border-radius:var(--ctrl-radius-lg); background:var(--ctrl-bg); border:1px solid var(--ctrl-border); display:flex; align-items:center; justify-content:center; color:var(--ctrl-text); box-shadow:var(--ctrl-shadow-md);",
                                "强倾斜 + 放大"
                            }
                        }
                        Tilt {
                            max_tilt: 6.0,
                            glare: false,
                            div { style: "width:180px; height:180px; border-radius:var(--ctrl-radius-lg); background:var(--ctrl-bg); border:1px solid var(--ctrl-border); display:flex; align-items:center; justify-content:center; color:var(--ctrl-text); box-shadow:var(--ctrl-shadow-md);",
                                "轻倾斜 · 无高光"
                            }
                        }
                    }
                },
                code: "Tilt { max_tilt: 20.0, scale: 1.06, perspective: 600.0, ... }\nTilt { max_tilt: 6.0, glare: false, ... }".to_string(),
            }

            h2 { "Tilt Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("max_tilt", "f64", "12.0", "最大倾斜角度（度）"),
                ("perspective", "f64", "800.0", "透视距离 px，越小越立体"),
                ("scale", "f64", "1.0", "悬停时缩放比例"),
                ("glare", "bool", "true", "是否启用高光反射"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "卡片内容"),
            ]}
        }
    }
}
