use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn AffixPage() -> Element {
    rsx! {
        h1 { "Affix 固钉" }
        p { "将元素固定在页面可视范围内，常用于导航栏、操作按钮等需要始终可见的元素。" }

        h2 { "何时使用" }
        ul {
            li { "页面滚动时需要保持某个元素可见" }
            li { "固定顶部导航栏或底部操作栏" }
            li { "在指定滚动容器内固定元素" }
        }

        h2 { "代码演示" }

        DemoBox { title: "基本用法".to_string(), description: Some("滚动页面，元素固定在顶部。".to_string()),
            demo: rsx! {
                Affix {
                    offset_top: 10.0,
                    onchange: |_fixed: bool| {},
                    Button { "固定在顶部" }
                }
            },
            code: r#"Affix {
    offset_top: 10.0,
    onchange: |fixed| { /* ... */ },
    Button { "固定在顶部" }
}"#.to_string(),
        }

        DemoBox { title: "固定在底部".to_string(), description: Some("设置 position 为 Bottom 使元素固定在底部。".to_string()),
            demo: rsx! {
                Affix {
                    position: AffixPosition::Bottom,
                    offset_top: 20.0,
                    Button { "固定在底部" }
                }
            },
            code: r#"Affix {
    position: AffixPosition::Bottom,
    offset_top: 20.0,
    Button { "固定在底部" }
}"#.to_string(),
        }

        DemoBox { title: "指定滚动容器".to_string(), description: Some("通过 target 指定滚动容器的 CSS 选择器，元素会在该容器内固定。".to_string()),
            demo: rsx! {
                div {
                    id: "affix-demo-container",
                    style: "height: 200px; overflow-y: auto; border: 1px solid var(--ctrl-border, #d9d9d9); border-radius: 6px; padding: 16px;",
                    div { style: "height: 60px; padding: 12px; background: var(--ctrl-bg-secondary, #f5f5f5); margin-bottom: 8px; border-radius: 4px;",
                        "滚动内容 1"
                    }
                    Affix {
                        target: "#affix-demo-container".to_string(),
                        offset_top: 0.0,
                        Button { "容器内固定" }
                    }
                    div { style: "height: 60px; padding: 12px; background: var(--ctrl-bg-secondary, #f5f5f5); margin-top: 8px; margin-bottom: 8px; border-radius: 4px;",
                        "滚动内容 2"
                    }
                    for i in 3..=8 {
                        div {
                            key: "{i}",
                            style: "height: 60px; padding: 12px; background: var(--ctrl-bg-secondary, #f5f5f5); margin-top: 8px; border-radius: 4px;",
                            "滚动内容 {i}"
                        }
                    }
                }
            },
            code: r##"Affix {
    target: "#my-scroll-container".to_string(),
    offset_top: 0.0,
    Button { "容器内固定" }
}"##.to_string(),
        }

        DemoBox { title: "禁用状态".to_string(), description: Some("通过 disabled 禁用固钉功能。".to_string()),
            demo: rsx! {
                Affix {
                    disabled: true,
                    offset_top: 10.0,
                    Button { "禁用状态（不会固定）" }
                }
            },
            code: r#"Affix {
    disabled: true,
    offset_top: 10.0,
    Button { "禁用状态" }
}"#.to_string(),
        }

        h2 { "Affix Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("offset_top", "f64", "0.0", "距顶部/底部偏移量（px）"),
            ("offset_bottom", "f64", "-1.0", "距底部偏移量（px），优先于 offset_top"),
            ("position", "AffixPosition", "Top", "固定方向：Top / Bottom"),
            ("target", "String", "\"\"", "滚动容器 CSS 选择器，默认 window"),
            ("disabled", "bool", "false", "是否禁用固钉"),
            ("z_index", "u32", "10", "固定时的 z-index"),
            ("onchange", "Option<EventHandler<bool>>", "—", "固定状态变化回调"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}
    }
}
