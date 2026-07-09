use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn AnimatedListPage() -> Element {
    let mut key = use_signal(|| 0u32);

    rsx! {
div { id: "animated-list", style: "margin-top: 64px;",
            h1 {
                "AnimatedList 动画列表"
            }
            p {
                "将子项依次淡入 + 位移入场，形成交错的入场动画。常用于消息列表、通知、卡片流等。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("每个顶层子元素依次入场。点击按钮可重放动画。".to_string()),
                demo: rsx! {
                    div {
                        Button {
                            variant: Variant::Outline,
                            size: Size::Sm,
                            onclick: move |_| key += 1,
                            "重放动画"
                        }
                        div { style: "margin-top:16px;",
                            for k in [key()] {
                                AnimatedList { key: "{k}",
                                    for i in 1..=5 {
                                        div { style: "padding:12px 16px; margin-bottom:8px; border-radius:var(--ctrl-radius-md); background:var(--ctrl-bg); border:1px solid var(--ctrl-border);",
                                            "列表项 {i}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                code: "AnimatedList {\n    for i in 1..=5 {\n        div { \"列表项 {i}\" }\n    }\n}".to_string(),
            }

            DemoBox {
                title: "方向与节奏".to_string(),
                description: Some("direction 设置入场方向，stagger 控制间隔。".to_string()),
                demo: rsx! {
                    for k in [key()] {
                        AnimatedList { key: "{k}", direction: "left".to_string(), stagger: 90,
                            for name in ["设计", "开发", "测试", "上线"] {
                                div { style: "padding:10px 16px; margin-bottom:8px; border-radius:var(--ctrl-radius-md); background:var(--ctrl-primary); color:#fff; display:inline-block; margin-right:8px;",
                                    "{name}"
                                }
                            }
                        }
                    }
                },
                code: "AnimatedList { direction: \"left\".to_string(), stagger: 90, ... }".to_string(),
            }

            h2 { "AnimatedList Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("stagger", "u32", "120", "相邻子项入场延迟（毫秒）"),
                ("duration", "u32", "500", "单项动画时长（毫秒）"),
                ("direction", "String", "\"up\"", "入场方向：up/down/left/right"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "列表子项"),
            ]}
        }
    }
}
