use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn InfiniteScrollPage() -> Element {
    rsx! {
        h1 { "InfiniteScroll 无限滚动" }
        p { "滚动至底部时自动加载更多内容，适用于长列表、信息流等场景。" }

        h2 { "何时使用" }
        ul {
            li { "需要分页加载大量数据时" }
            li { "移动端信息流、聊天记录等场景" }
            li { "希望用户滚动到底部时自动触发加载" }
        }

        h2 { "代码演示" }

        DemoBox { title: "基本用法".to_string(), description: Some("滚动到底部自动触发 onload 回调，容器需要设置固定高度。".to_string()),
            demo: rsx! {
                InfiniteScrollDemo {}
            },
            code: r#"InfiniteScroll {
    loading: loading(),
    has_more: count() < 100,
    height: "400px".to_string(),
    onload: move |_| { /* 加载更多 */ },
    for i in 0..count() {
        div { key: "{i}", "Item {i}" }
    }
}"#.to_string(),
        }

        h2 { "InfiniteScroll Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("loading", "bool", "false", "是否正在加载中"),
            ("has_more", "bool", "true", "是否还有更多数据"),
            ("threshold", "f64", "100.0", "距底部多远（px）触发加载"),
            ("height", "String", "\"400px\"", "容器高度（必须设置才能滚动）"),
            ("onload", "Option<EventHandler<()>>", "—", "加载回调"),
            ("loading_text", "String", "\"加载中...\"", "加载中文案"),
            ("no_more_text", "String", "\"没有更多了\"", "没有更多数据文案"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}
    }
}

#[component]
#[allow(non_snake_case)]
fn InfiniteScrollDemo() -> Element {
    let count = use_signal(|| 20usize);
    let mut loading = use_signal(|| false);

    rsx! {
        InfiniteScroll {
            loading: loading(),
            has_more: count() < 100,
            height: "400px".to_string(),
            onload: move |_| {
                loading.set(true);
                let mut c = count;
                let mut l = loading;
                gloo_timers::callback::Timeout::new(800, move || {
                    c += 20;
                    l.set(false);
                }).forget();
            },
            for i in 0..count() {
                div {
                    key: "{i}",
                    style: "padding:12px 16px; border-bottom:1px solid var(--ctrl-border, #eee);",
                    "列表项 {i + 1}"
                }
            }
        }
    }
}
