use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

static TILT_ID: AtomicU16 = AtomicU16::new(1);

/// Tilt 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TiltProps {
    /// 最大倾斜角度（度）
    #[props(default = 12.0)]
    pub max_tilt: f64,

    /// 透视距离（px），越小立体感越强
    #[props(default = 800.0)]
    pub perspective: f64,

    /// 悬停时的缩放比例
    #[props(default = 1.0)]
    pub scale: f64,

    /// 是否启用高光反射
    #[props(default = true)]
    pub glare: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 卡片内容
    pub children: Element,
}

/// Tilt 3D 倾斜卡片组件
///
/// 鼠标在卡片内移动时，卡片依据光标相对位置产生透视倾斜，营造立体悬浮效果，
/// 可选高光反射跟随光标。移出后平滑复位。
#[allow(non_snake_case)]
pub fn Tilt(props: TiltProps) -> Element {
    const CSS: &str = include_str!("../../assets/tilt.css");

    let id = TILT_ID.fetch_add(1, Ordering::Relaxed);
    let inner_id = format!("ctrl-tilt-inner-{}", id);

    // (rot_x, rot_y, glare_x%, glare_y%)
    let mut state = use_signal(|| (0.0f64, 0.0f64, 50.0f64, 50.0f64));
    let mut active = use_signal(|| false);

    let card_class = if props.class.is_empty() {
        "ctrl-tilt".to_string()
    } else {
        format!("ctrl-tilt {}", props.class)
    };

    let (rx, ry, gx, gy) = state();
    let scale = if active() { props.scale } else { 1.0 };
    let glare_opacity = if active() && props.glare { 1.0 } else { 0.0 };

    let wrapper_style = format!(
        "--ctrl-tilt-perspective:{}px; {}",
        props.perspective, props.style
    );
    let inner_style = format!(
        "--ctrl-tilt-rx:{rx:.2}deg; --ctrl-tilt-ry:{ry:.2}deg; --ctrl-tilt-scale:{scale};"
    );
    let glare_style = format!(
        "--ctrl-tilt-glare-x:{gx:.1}%; --ctrl-tilt-glare-y:{gy:.1}%; --ctrl-tilt-glare-opacity:{glare_opacity};"
    );

    let max_tilt = props.max_tilt;
    let inner_id_move = inner_id.clone();

    rsx! {
        style { {CSS} }
        div { class: "{card_class}", style: "{wrapper_style}",
            onmousemove: move |e: MouseEvent| {
                let c = e.data().element_coordinates();
                // 读取内层元素尺寸以归一化
                let (w, h) = web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| doc.get_element_by_id(&inner_id_move))
                    .and_then(|el| el.dyn_ref::<HtmlElement>().map(|e| (e.offset_width() as f64, e.offset_height() as f64)))
                    .unwrap_or((1.0, 1.0));
                if w <= 0.0 || h <= 0.0 { return; }
                // 归一化到 -0.5 .. 0.5
                let px = (c.x / w).clamp(0.0, 1.0) - 0.5;
                let py = (c.y / h).clamp(0.0, 1.0) - 0.5;
                // 光标在上方 → 卡片上边后仰（rot_x 正），左右同理
                let rot_x = -py * max_tilt * 2.0;
                let rot_y = px * max_tilt * 2.0;
                state.set((rot_x, rot_y, (px + 0.5) * 100.0, (py + 0.5) * 100.0));
            },
            onmouseenter: move |_| active.set(true),
            onmouseleave: move |_| {
                active.set(false);
                state.set((0.0, 0.0, 50.0, 50.0));
            },
            div { id: "{inner_id}", class: "ctrl-tilt__inner", style: "{inner_style}",
                div { class: "ctrl-tilt__content", {props.children} }
                if props.glare {
                    div { class: "ctrl-tilt__glare", style: "{glare_style}" }
                }
            }
        }
    }
}
