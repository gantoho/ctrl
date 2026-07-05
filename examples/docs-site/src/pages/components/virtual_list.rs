use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn VirtualListPage() -> Element {
    rsx! {
        h1 { "VirtualList 虚拟列表" }
        p { "高性能虚拟滚动列表，只渲染可视区域内的元素，轻松支撑万级数据量。" }

        h2 { "何时使用" }
        ul {
            li { "列表数据量较大（超过 100 条）时，用于提升渲染性能" }
            li { "需要展示大量日志、消息、搜索结果等长列表" }
        }

        h2 { "代码演示" }

        DemoBox { title: "基本用法".to_string(), description: Some("使用 render_item 回调渲染每一项，内部自动计算可见范围。".to_string()),
            demo: rsx! {
                VirtualList {
                    count: 10000,
                    item_height: 40.0,
                    height: 400.0,
                    render_item: Callback::new(move |i: usize| {
                        rsx! {
                            div { style: "height:40px; padding:0 16px; display:flex; align-items:center; border-bottom:1px solid var(--ctrl-border, #eee);",
                                "Item {i}"
                            }
                        }
                    }),
                }
            },
            code: r#"VirtualList {
    count: 10000,
    item_height: 40.0,
    height: 400.0,
    render_item: Callback::new(move |i: usize| {
        rsx! { div { style: "height:40px;", "Item {i}" } }
    }),
}"#.to_string(),
        }

        DemoBox { title: "自定义高度和缓冲区".to_string(), description: Some("通过 item_height、height、overscan 自定义配置。".to_string()),
            demo: rsx! {
                VirtualList {
                    count: 5000,
                    item_height: 60.0,
                    height: 300.0,
                    overscan: 10,
                    render_item: Callback::new(move |i: usize| {
                        rsx! {
                            div { style: "height:60px; padding:0 16px; display:flex; align-items:center; border-bottom:1px solid var(--ctrl-border, #eee);",
                                "Row {i}"
                            }
                        }
                    }),
                }
            },
            code: r#"VirtualList {
    count: 5000,
    item_height: 60.0,
    height: 300.0,
    overscan: 10,
    render_item: Callback::new(move |i: usize| {
        rsx! { div { style: "height:60px;", "Row {i}" } }
    }),
}"#.to_string(),
        }

        h2 { "VirtualList Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("count", "usize", "—", "数据总条数"),
            ("item_height", "f64", "40.0", "每项高度（px）"),
            ("height", "f64", "400.0", "可视区域高度（px）"),
            ("overscan", "usize", "5", "上下额外渲染的缓冲项数"),
            ("render_item", "Callback<usize, Element>", "—", "渲染每一项的回调"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}
    }
}
