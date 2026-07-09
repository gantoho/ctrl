use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

static IMG_CMP_ID: AtomicU16 = AtomicU16::new(1);

/// ImageComparison 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ImageComparisonProps {
    /// 前景（左侧 / before）图片地址
    pub before: String,

    /// 背景（右侧 / after）图片地址
    pub after: String,

    /// before 图片替代文本
    #[props(default = "before".to_string())]
    pub before_alt: String,

    /// after 图片替代文本
    #[props(default = "after".to_string())]
    pub after_alt: String,

    /// 初始分隔位置（0~100）
    #[props(default = 50.0)]
    pub initial: f64,

    /// 高度（CSS 尺寸，如 "320px"）
    #[props(default = "320px".to_string())]
    pub height: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// ImageComparison 图片对比组件
///
/// 通过可拖拽的分隔滑块对比前后两张图片，光标在容器内移动或拖动即可改变分隔位置。
#[allow(non_snake_case)]
pub fn ImageComparison(props: ImageComparisonProps) -> Element {
    const CSS: &str = include_str!("../../assets/image_comparison.css");

    let id = IMG_CMP_ID.fetch_add(1, Ordering::Relaxed);
    let root_id = format!("ctrl-image-comparison-{}", id);

    let mut pos = use_signal(|| props.initial);
    let mut dragging = use_signal(|| false);

    let container_class = if props.class.is_empty() {
        "ctrl-image-comparison".to_string()
    } else {
        format!("ctrl-image-comparison {}", props.class)
    };

    let vars = format!(
        "--ctrl-image-comparison-pos:{:.2}%; height:{}; {}",
        pos(), props.height, props.style
    );

    let root_id_move = root_id.clone();
    let update_from_x = move |client_x: f64| {
        let (left, w) = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id(&root_id_move))
            .and_then(|el| el.dyn_ref::<HtmlElement>().map(|e| {
                let rect = e.get_bounding_client_rect();
                (rect.left(), rect.width())
            }))
            .unwrap_or((0.0, 0.0));
        if w <= 0.0 { return; }
        let pct = ((client_x - left) / w * 100.0).clamp(0.0, 100.0);
        pos.set(pct);
    };

    let mut on_move = update_from_x.clone();
    let mut on_move_hover = update_from_x.clone();

    rsx! {
        style { {CSS} }
        div {
            id: "{root_id}",
            class: "{container_class}",
            style: "{vars}",
            onmousedown: move |e: MouseEvent| {
                dragging.set(true);
                on_move(e.data().client_coordinates().x);
            },
            onmouseup: move |_| dragging.set(false),
            onmouseleave: move |_| dragging.set(false),
            onmousemove: move |e: MouseEvent| {
                if dragging() {
                    on_move_hover(e.data().client_coordinates().x);
                }
            },

            // after（底层，完整显示）
            img {
                class: "ctrl-image-comparison__img",
                src: "{props.after}",
                alt: "{props.after_alt}",
                draggable: "false",
            }
            // before（上层，按分隔位置裁剪）
            div { class: "ctrl-image-comparison__before",
                img {
                    class: "ctrl-image-comparison__img",
                    src: "{props.before}",
                    alt: "{props.before_alt}",
                    draggable: "false",
                }
            }
            // 分隔线 + 手柄
            div { class: "ctrl-image-comparison__divider",
                div { class: "ctrl-image-comparison__handle",
                    span { class: "ctrl-image-comparison__arrow", "‹" }
                    span { class: "ctrl-image-comparison__arrow", "›" }
                }
            }
        }
    }
}
