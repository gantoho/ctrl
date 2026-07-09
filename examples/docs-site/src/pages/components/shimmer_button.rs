use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn ShimmerButtonPage() -> Element {
    rsx! {
div { id: "shimmer-button", style: "margin-top: 64px;",
            h1 {
                "ShimmerButton 微光按钮"
            }
            p {
                "一束高光周期性扫过按钮表面，用于强调主行动按钮、付费入口、升级引导等。支持 prefers-reduced-motion 下停用动画。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("高光自动循环扫过。".to_string()),
                demo: rsx! {
                    ShimmerButtonBasicDemo {}
                },
                code: "ShimmerButton {\n    onclick: move |_| { /* ... */ },\n    \"✨ 立即升级\"\n}".to_string(),
            }

            DemoBox {
                title: "自定义背景".to_string(),
                description: Some("通过 style 覆盖背景色或使用渐变。".to_string()),
                demo: rsx! {
                    div { style: "display:flex; gap:16px; flex-wrap:wrap;",
                        ShimmerButton {
                            style: "background:linear-gradient(135deg,#6366f1,#a855f7);".to_string(),
                            onclick: move |_| {},
                            "开始创作"
                        }
                        ShimmerButton {
                            style: "background:#0f172a;".to_string(),
                            onclick: move |_| {},
                            "深色按钮"
                        }
                    }
                },
                code: "ShimmerButton {\n    style: \"background:linear-gradient(135deg,#6366f1,#a855f7);\".to_string(),\n    \"开始创作\"\n}".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("disabled 时不可点击。".to_string()),
                demo: rsx! {
                    ShimmerButton { disabled: true, "已禁用" }
                },
                code: "ShimmerButton { disabled: true, \"已禁用\" }".to_string(),
            }

            h2 { "ShimmerButton Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("disabled", "bool", "false", "是否禁用"),
                ("onclick", "EventHandler<MouseEvent>", "—", "点击事件"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式（覆盖背景等）"),
                ("children", "Element", "—", "按钮内容"),
            ]}
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn ShimmerButtonBasicDemo() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { style: "display:flex; align-items:center; gap:16px;",
            ShimmerButton {
                onclick: move |_| count += 1,
                "✨ 立即升级"
            }
            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);",
                "点击 {count} 次"
            }
        }
    }
}
