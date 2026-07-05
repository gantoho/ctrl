use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static IS_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// InfiniteScroll 无限滚动组件属性
#[derive(Props, PartialEq, Clone)]
pub struct InfiniteScrollProps {
    /// 是否正在加载中
    #[props(default = false)]
    pub loading: bool,

    /// 是否还有更多数据
    #[props(default = true)]
    pub has_more: bool,

    /// 距离底部多远（px）触发加载
    #[props(default = 100.0)]
    pub threshold: f64,

    /// 容器高度
    #[props(default = "400px".to_string())]
    pub height: String,

    /// 加载回调
    pub onload: Option<EventHandler<()>>,

    /// 子元素
    pub children: Element,

    /// 加载中提示文案
    #[props(default = "加载中...".to_string())]
    pub loading_text: String,

    /// 没有更多数据提示文案
    #[props(default = "没有更多了".to_string())]
    pub no_more_text: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// InfiniteScroll 无限滚动 —— 滚动到底部自动加载更多
#[allow(non_snake_case)]
pub fn InfiniteScroll(props: InfiniteScrollProps) -> Element {
    const CSS: &str = include_str!("../../assets/infinite_scroll.css");

    let is_id = use_signal(|| {
        let id = IS_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("is-{}", id)
    });

    let loading = props.loading;
    let has_more = props.has_more;
    let threshold = props.threshold;
    let onload = props.onload.clone();
    let id_str = is_id().clone();

    rsx! {
        style { {CSS} }
        div {
            class: "ctrl-infinite-scroll {props.class}",
            style: "height: {props.height}; {props.style}",
            id: "{id_str}",
            onscroll: move |_| {
                if loading || !has_more {
                    return;
                }
                if let Some(win) = web_sys::window() {
                    if let Some(doc) = win.document() {
                        if let Some(el) = doc.get_element_by_id(&id_str) {
                            let scroll_bottom = el.scroll_height() - el.scroll_top() - el.client_height();
                            if scroll_bottom as f64 <= threshold {
                                if let Some(ref cb) = onload {
                                    cb.call(());
                                }
                            }
                        }
                    }
                }
            },
            {&props.children}
            if loading {
                div { class: "ctrl-infinite-scroll__loading",
                    div { class: "ctrl-infinite-scroll__spinner" }
                    span { class: "ctrl-infinite-scroll__text", "{props.loading_text}" }
                }
            }
            if !has_more && !loading {
                div { class: "ctrl-infinite-scroll__no-more",
                    span { class: "ctrl-infinite-scroll__text", "{props.no_more_text}" }
                }
            }
        }
    }
}
