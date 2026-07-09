use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// ScrollProgress 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ScrollProgressProps {
    /// 进度条高度（px）
    #[props(default = 3)]
    pub height: u32,

    /// 进度条颜色（留空使用主题主色渐变）
    #[props(default = "".to_string())]
    pub color: String,

    /// 定位模式：为 true 时 fixed 固定在视口顶部；false 时 absolute 贴附父容器顶部
    #[props(default = true)]
    pub fixed: bool,

    /// 滚动容器的 CSS 选择器。留空时监听整个窗口（documentElement）；
    /// 当页面真正的滚动发生在某个 `overflow:auto` 容器内时，传入该容器选择器。
    #[props(default = "".to_string())]
    pub target: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// ScrollProgress 滚动进度条组件
///
/// 监听滚动，在顶部显示阅读进度。常用于文章、文档等长页面。
/// 默认监听窗口；若滚动发生在内部容器，用 `target` 指定其 CSS 选择器。
#[allow(non_snake_case)]
pub fn ScrollProgress(props: ScrollProgressProps) -> Element {
    const CSS: &str = include_str!("../../assets/scroll_progress.css");

    let mut progress = use_signal(|| 0.0f64);

    // 监听滚动，计算滚动百分比
    let mut scroll_cb: Signal<Option<Closure<dyn FnMut()>>> = use_signal(|| None);
    // 记录实际绑定的 EventTarget（window 或指定容器），用于卸载时解绑
    let mut scroll_target: Signal<Option<web_sys::EventTarget>> = use_signal(|| None);
    let target_sel = props.target.clone();
    use_effect(move || {
        let sel = target_sel.clone();
        let mut compute = move || {
            let Some(win) = web_sys::window() else { return };
            let Some(doc) = win.document() else { return };
            // 有选择器则读该容器，否则读 documentElement
            let el: web_sys::Element = if sel.is_empty() {
                match doc.document_element() { Some(e) => e, None => return }
            } else {
                match doc.query_selector(&sel).ok().flatten() { Some(e) => e, None => return }
            };
            let scroll_top = el.scroll_top() as f64;
            let scroll_height = el.scroll_height() as f64;
            let client_height = el.client_height() as f64;
            let max = scroll_height - client_height;
            let pct = if max > 0.0 { (scroll_top / max * 100.0).clamp(0.0, 100.0) } else { 0.0 };
            progress.set(pct);
        };
        compute();
        let cb = Closure::wrap(Box::new(compute) as Box<dyn FnMut()>);

        // 解析绑定目标：容器元素本身，或 window
        let et: Option<web_sys::EventTarget> = if target_sel.is_empty() {
            web_sys::window().map(|w| w.into())
        } else {
            web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.query_selector(&target_sel).ok().flatten())
                .map(|e| e.into())
        };
        if let Some(ref t) = et {
            let _ = t.add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref());
        }
        scroll_target.set(et);
        scroll_cb.set(Some(cb));
    });

    use_drop(move || {
        if let Some(cb) = scroll_cb.take() {
            if let Some(t) = scroll_target.take() {
                let _ = t.remove_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref());
            }
        }
    });

    let mut container_class = "ctrl-scroll-progress".to_string();
    if props.fixed {
        container_class.push_str(" ctrl-scroll-progress--fixed");
    }
    if !props.class.is_empty() {
        container_class.push(' ');
        container_class.push_str(&props.class);
    }

    let bar_bg = if props.color.is_empty() {
        "linear-gradient(90deg, var(--ctrl-primary), var(--ctrl-primary-hover, var(--ctrl-primary)))".to_string()
    } else {
        props.color.clone()
    };

    let vars = format!(
        "--ctrl-scroll-progress-height:{}px; --ctrl-scroll-progress-bg:{}; {}",
        props.height, bar_bg, props.style
    );
    let pct = progress();

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{vars}",
            div { class: "ctrl-scroll-progress__bar", style: "width:{pct:.2}%;" }
        }
    }
}
