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
            li { "返回顶部按钮的触发条件" }
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

        DemoBox { title: "固定在底部".to_string(), description: Some("设置 offset_bottom 使元素固定在底部。".to_string()),
            demo: rsx! {
                Affix {
                    offset_bottom: 10.0,
                    Button { "固定在底部" }
                }
            },
            code: r#"Affix {
    offset_bottom: 10.0,
    Button { "固定在底部" }
}"#.to_string(),
        }

        h2 { "Affix Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("offset_top", "f64", "0.0", "距顶部偏移量（px），达到时固定"),
            ("offset_bottom", "f64", "-1.0", "距底部偏移量（px），达到时固定"),
            ("onchange", "Option<EventHandler<bool>>", "—", "固定状态变化回调"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}
    }
}
