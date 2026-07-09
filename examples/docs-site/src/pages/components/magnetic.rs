use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn MagneticPage() -> Element {
    rsx! {
div { id: "magnetic", style: "margin-top: 64px;",
            h1 {
                "Magnetic 磁吸"
            }
            p {
                "光标在元素范围内移动时，元素朝光标方向产生位移（磁吸感），移出后平滑回弹归位。常用于按钮、图标等需要强调的交互元素。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("将按钮包裹进 Magnetic，鼠标靠近即被吸附。".to_string()),
                demo: rsx! {
                    div { style: "display:flex; justify-content:center; gap:32px; padding:32px; flex-wrap:wrap;",
                        Magnetic {
                            Button { variant: Variant::Primary, "磁吸按钮" }
                        }
                        Magnetic {
                            div { style: "width:56px; height:56px; border-radius:50%; background:var(--ctrl-bg); border:1px solid var(--ctrl-border); display:flex; align-items:center; justify-content:center; font-size:22px; box-shadow:var(--ctrl-shadow-md);",
                                "★"
                            }
                        }
                    }
                },
                code: "Magnetic {\n    Button { variant: Variant::Primary, \"磁吸按钮\" }\n}".to_string(),
            }

            DemoBox {
                title: "强度与范围".to_string(),
                description: Some("strength 控制吸附灵敏度，max_offset 限制最大位移。".to_string()),
                demo: rsx! {
                    div { style: "display:flex; justify-content:center; gap:32px; padding:32px; flex-wrap:wrap;",
                        Magnetic {
                            strength: 0.6,
                            max_offset: 60.0,
                            Button { variant: Variant::Secondary, "强磁吸" }
                        }
                        Magnetic {
                            strength: 0.15,
                            max_offset: 15.0,
                            Button { variant: Variant::Outline, "弱磁吸" }
                        }
                    }
                },
                code: "Magnetic { strength: 0.6, max_offset: 60.0, ... }\nMagnetic { strength: 0.15, max_offset: 15.0, ... }".to_string(),
            }

            h2 { "Magnetic Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("strength", "f64", "0.35", "磁吸强度（光标位移映射比例）"),
                ("max_offset", "f64", "40.0", "最大位移限制 px"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "磁吸内容"),
            ]}
        }
    }
}
