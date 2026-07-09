use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlImageElement};

static LENS_ID: AtomicU16 = AtomicU16::new(1);

/// Lens 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct LensProps {
    /// 被放大的图片地址
    pub src: String,

    /// 图片替代文本
    #[props(default = "".to_string())]
    pub alt: String,

    /// 放大倍率
    #[props(default = 2.5)]
    pub zoom: f64,

    /// 镜片大小（px，宽高相等）
    #[props(default = 160.0)]
    pub size: f64,

    /// 高度（CSS 尺寸）
    #[props(default = "320px".to_string())]
    pub height: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// Lens 放大镜组件
///
/// 鼠标悬停在图片上时，浮动圆形镜片显示局部高倍放大的画面。光标跟随，适合产品细节查看、
/// 电商图片放大等场景。背后实现：镜片使用同一图片作 background-image，按光标比例偏移
/// background-position，background-size = zoom * 原图尺寸。
#[allow(non_snake_case)]
pub fn Lens(props: LensProps) -> Element {
    const CSS: &str = include_str!("../../assets/lens.css");

    let id = LENS_ID.fetch_add(1, Ordering::Relaxed);
    let root_id = format!("ctrl-lens-root-{}", id);
    let img_id = format!("ctrl-lens-img-{}", id);

    // (visible, bg_size_w, bg_size_h, bg_pos_x, bg_pos_y, center_x, center_y) —— 均为 px
    let mut state = use_signal(|| (false, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64));

    let container_class = if props.class.is_empty() {
        "ctrl-lens".to_string()
    } else {
        format!("ctrl-lens {}", props.class)
    };

    let (hovering, bg_w, bg_h, pos_x, pos_y, cx, cy) = state();
    let lens_style = if hovering {
        format!(
            "display:block; width:{size}px; height:{size}px; left:{left:.1}px; top:{top:.1}px; \
             background-image: url({src}); background-size: {bg_w:.1}px {bg_h:.1}px; background-position: {pos_x:.1}px {pos_y:.1}px;",
            size = props.size,
            left = cx - props.size / 2.0,
            top = cy - props.size / 2.0,
            src = props.src,
            bg_w = bg_w,
            bg_h = bg_h,
            pos_x = pos_x,
            pos_y = pos_y,
        )
    } else {
        "display:none;".to_string()
    };

    let root_id_move = root_id.clone();
    let img_id_move = img_id.clone();
    let zoom = props.zoom;
    let lens_size = props.size;

    rsx! {
        style { {CSS} }
        div {
            id: "{root_id}",
            class: "{container_class}",
            style: "height:{props.height}; {props.style}",
            onmousemove: move |e: MouseEvent| {
                let c = e.data().element_coordinates();
                let window = web_sys::window();
                let doc = window.as_ref().and_then(|w| w.document());
                let (cw, ch) = doc.as_ref()
                    .and_then(|d| d.get_element_by_id(&root_id_move))
                    .and_then(|el| el.dyn_ref::<HtmlElement>().map(|e| (e.offset_width() as f64, e.offset_height() as f64)))
                    .unwrap_or((1.0, 1.0));
                // 图片自然尺寸，用于复刻 object-fit: cover 的缩放
                let (iw, ih) = doc.as_ref()
                    .and_then(|d| d.get_element_by_id(&img_id_move))
                    .and_then(|el| el.dyn_ref::<HtmlImageElement>().map(|e| (e.natural_width() as f64, e.natural_height() as f64)))
                    .filter(|(w, h)| *w > 0.0 && *h > 0.0)
                    .unwrap_or((cw, ch));
                if cw <= 0.0 || ch <= 0.0 { return; }
                let cxp = c.x.clamp(0.0, cw);
                let cyp = c.y.clamp(0.0, ch);
                // cover 缩放：图片按最大比例铺满容器
                let s = (cw / iw).max(ch / ih);
                let dw = iw * s; // cover 渲染后的图片显示尺寸
                let dh = ih * s;
                // 居中裁切偏移（容器左上角相对显示图左上角）
                let ox = (dw - cw) / 2.0;
                let oy = (dh - ch) / 2.0;
                // 光标点在（未放大的）显示图坐标
                let px = ox + cxp;
                let py = oy + cyp;
                // 放大后的背景尺寸与该点位置
                let bg_w = dw * zoom;
                let bg_h = dh * zoom;
                let pos_x = lens_size / 2.0 - px * zoom;
                let pos_y = lens_size / 2.0 - py * zoom;
                state.set((true, bg_w, bg_h, pos_x, pos_y, cxp, cyp));
            },
            onmouseleave: move |_| state.set((false, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
            img {
                id: "{img_id}",
                class: "ctrl-lens__img",
                src: "{props.src}",
                alt: "{props.alt}",
            },
            div {
                class: "ctrl-lens__lens",
                style: "{lens_style}",
            }
        }
    }
}
