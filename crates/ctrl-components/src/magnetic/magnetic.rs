use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

static MAGNETIC_ID: AtomicU16 = AtomicU16::new(1);

/// Magnetic 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct MagneticProps {
    /// 磁吸强度（0~1）：光标相对中心位移映射到元素位移的比例
    #[props(default = 0.35)]
    pub strength: f64,

    /// 最大位移限制（px）
    #[props(default = 40.0)]
    pub max_offset: f64,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 磁吸内容（通常是按钮或图标）
    pub children: Element,
}

/// Magnetic 磁吸组件
///
/// 光标在元素范围内移动时，元素朝光标方向产生位移（磁吸感），移出后平滑回弹归位。
/// 常用于按钮、图标等需要强调的交互元素。
#[allow(non_snake_case)]
pub fn Magnetic(props: MagneticProps) -> Element {
    const CSS: &str = include_str!("../../assets/magnetic.css");

    let id = MAGNETIC_ID.fetch_add(1, Ordering::Relaxed);
    let inner_id = format!("ctrl-magnetic-inner-{}", id);

    // (offset_x, offset_y)
    let mut offset = use_signal(|| (0.0f64, 0.0f64));

    let card_class = if props.class.is_empty() {
        "ctrl-magnetic".to_string()
    } else {
        format!("ctrl-magnetic {}", props.class)
    };

    let (ox, oy) = offset();
    let inner_style = format!("--ctrl-magnetic-x:{ox:.2}px; --ctrl-magnetic-y:{oy:.2}px;");

    let strength = props.strength;
    let max_offset = props.max_offset;
    let inner_id_move = inner_id.clone();

    rsx! {
        style { {CSS} }
        div { class: "{card_class}", style: "{props.style}",
            onmousemove: move |e: MouseEvent| {
                let c = e.data().element_coordinates();
                let (w, h) = web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| doc.get_element_by_id(&inner_id_move))
                    .and_then(|el| el.dyn_ref::<HtmlElement>().map(|e| (e.offset_width() as f64, e.offset_height() as f64)))
                    .unwrap_or((0.0, 0.0));
                if w <= 0.0 || h <= 0.0 { return; }
                // 光标相对元素中心的位移
                let dx = (c.x - w / 2.0) * strength;
                let dy = (c.y - h / 2.0) * strength;
                let dx = dx.clamp(-max_offset, max_offset);
                let dy = dy.clamp(-max_offset, max_offset);
                offset.set((dx, dy));
            },
            onmouseleave: move |_| offset.set((0.0, 0.0)),
            div { id: "{inner_id}", class: "ctrl-magnetic__inner", style: "{inner_style}", {props.children} }
        }
    }
}
