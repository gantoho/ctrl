use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static VL_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// VirtualList 虚拟列表组件属性
#[derive(Props, PartialEq, Clone)]
pub struct VirtualListProps {
    /// 数据总条数
    pub count: usize,

    /// 每项预估高度（px），用于初始计算
    #[props(default = 40.0)]
    pub item_height: f64,

    /// 可视区域高度（px）
    #[props(default = 400.0)]
    pub height: f64,

    /// 额外渲染的缓冲项数（上下各多渲染 overscan 项，减少快速滚动白屏）
    #[props(default = 5)]
    pub overscan: usize,

    /// 渲染每一项的回调，参数为索引
    pub render_item: Callback<usize, Element>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// VirtualList 虚拟列表 —— 只渲染可视区域内的元素，支撑万级数据量
///
/// ```rust,ignore
/// VirtualList {
///     count: 10000,
///     item_height: 40.0,
///     height: 400.0,
///     render_item: Callback::new(move |i: usize| {
///         rsx! { div { style: "height:40px;", "Item {i}" } }
///     }),
/// }
/// ```
#[allow(non_snake_case)]
pub fn VirtualList(props: VirtualListProps) -> Element {
    const CSS: &str = include_str!("../../assets/virtual_list.css");

    let vl_id = use_signal(|| {
        let id = VL_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("vl-{}", id)
    });

    let item_height = props.item_height;
    let total_height = item_height * props.count as f64;
    let height = props.height;
    let overscan = props.overscan;
    let count = props.count;

    let mut start = use_signal(|| 0usize);
    let mut end = use_signal(|| ((height / item_height).ceil() as usize + overscan * 2).min(count));
    let mut offset_top = use_signal(|| 0.0_f64);

    let id_str = vl_id().clone();

    rsx! {
        style { {CSS} }
        div {
            class: "ctrl-virtual-list {props.class}",
            style: "height: {height}px; {props.style}",
            id: "{id_str}",
            onscroll: move |_| {
                if let Some(win) = web_sys::window() {
                    if let Some(doc) = win.document() {
                        if let Some(el) = doc.get_element_by_id(&id_str) {
                            let st = el.scroll_top() as f64;
                            let s = ((st / item_height).floor() as usize).saturating_sub(overscan);
                            let vis = ((height / item_height).ceil() as usize) + overscan * 2;
                            let e = (s + vis).min(count);
                            let off = s as f64 * item_height;
                            start.set(s);
                            end.set(e);
                            offset_top.set(off);
                        }
                    }
                }
            },
            div {
                class: "ctrl-virtual-list__phantom",
                style: "height: {total_height}px;"
            }
            div {
                class: "ctrl-virtual-list__content",
                style: "transform: translateY({offset_top()}px);",
                for i in start()..end() {
                    div {
                        class: "ctrl-virtual-list__item",
                        key: "{i}",
                        {props.render_item.call(i)}
                    }
                }
            }
        }
    }
}
