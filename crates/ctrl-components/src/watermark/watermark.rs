use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use wasm_bindgen::{closure::Closure, JsCast};

/// Watermark 水印组件
#[derive(Props, PartialEq, Clone)]
pub struct WatermarkProps {
    /// 水印文字
    pub content: Option<String>,

    /// 水印图片 URL（优先级高于文字）
    pub image: Option<String>,

    /// 字体颜色
    #[props(default = "rgba(0, 0, 0, 0.1)".to_string())]
    pub color: String,

    /// 字体大小
    #[props(default = "14px".to_string())]
    pub font_size: String,

    /// 字体族
    #[props(default = "sans-serif".to_string())]
    pub font_family: String,

    /// 旋转角度（度），0 表示不旋转
    #[props(default = 0)]
    pub rotate: i32,

    /// 水印块间距（px），同时设置水平和垂直。被 gap_x / gap_y 覆盖
    #[props(default = 100)]
    pub gap: u32,

    /// 水平间距（px），不设置则使用 gap
    pub gap_x: Option<u32>,

    /// 垂直间距（px），不设置则使用 gap
    pub gap_y: Option<u32>,

    /// Z-index
    #[props(default = 9999)]
    pub z_index: u32,

    /// 是否启用防删除保护
    #[props(default = true)]
    pub protection: bool,

    /// 检查间隔（毫秒）
    #[props(default = 500)]
    pub check_interval: u32,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    pub children: Element,
}

/// Element Plus 同款算法：
/// - Canvas measureText() 精确测量文字
/// - rotate=0：1 个文字实例，简单网格 tile
/// - rotate≠0：3 个文字实例、砖墙式错位排列，2×(bbox+gx)×(bbox+gy) 复合 tile
///   实例 2 和 3 部分延伸到 tile 边界外，由相邻 tile 补全，形成无缝砖墙
fn render_tile(
    text: &str, font: &str, font_size: &str, color: &str,
    rotate: i32, gap_x: u32, gap_y: u32,
) -> (String, u32, u32) {
    if rotate == 0 {
        // 无旋转：简单网格
        let js = format!(
            r#"(()=>{{const c=document.createElement('canvas');const ctx=c.getContext('2d');ctx.font='500 {fs} {ff}';const m=ctx.measureText('{txt}');return[m.width,parseFloat('{fs}')];}})()"#,
            fs=font_size, ff=font, txt=text.replace('\'', "\\'"),
        );
        let (tw, th) = measure(js);
        let tile_w = tw.ceil() as u32 + gap_x;
        let tile_h = th.ceil() as u32 + gap_y;
        let cx = tile_w as f64 / 2.0;
        let cy = tile_h as f64 / 2.0;
        let svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{tw}" height="{th}"><text x="{cx}" y="{cy}" text-anchor="middle" dominant-baseline="central" font-family="{ff}" font-size="{fs}" fill="{fc}" font-weight="500">{txt}</text></svg>"#,
            tw=tile_w, th=tile_h, cx=cx, cy=cy, ff=font, fs=font_size, fc=color, txt=text,
        );
        let d = encode_svg(&svg);
        return (d, tile_w, tile_h);
    }
    // 旋转 ≠0：Element Plus 式的 3 实例砖墙复合 tile
    let js = format!(
        r#"(()=>{{const c=document.createElement('canvas');const ctx=c.getContext('2d');ctx.font='500 {fs} {ff}';const m=ctx.measureText('{txt}');const fsp=parseFloat('{fs}');const tw=m.width;const rad=Math.abs({r})*Math.PI/180;const sin=Math.abs(Math.sin(rad)),cos=Math.abs(Math.cos(rad));const bw=Math.ceil(tw*cos+fsp*sin);const bh=Math.ceil(tw*sin+fsp*cos);return[bw,bh];}})()"#,
        fs=font_size, ff=font, txt=text.replace('\'', "\\'"), r=rotate,
    );
    let (bw, bh) = measure_bbox(&js);

    let tile_w = (bw + gap_x) * 2;
    let tile_h = bh + gap_y;

    let cx1 = bw as f64 / 2.0;
    let cy1 = tile_h as f64 / 2.0;

    let cx2 = bw as f64 / 2.0 + bw as f64 + gap_x as f64;
    let cy2 = gap_y as f64 / 2.0;

    let cx3 = cx2;
    let cy3 = tile_h as f64;

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{tw}" height="{th}"><g transform="rotate({r} {cx1} {cy1})"><text x="{cx1}" y="{cy1}" text-anchor="middle" dominant-baseline="central" font-family="{ff}" font-size="{fs}" fill="{fc}" font-weight="500">{txt}</text></g><g transform="rotate({r} {cx2} {cy2})"><text x="{cx2}" y="{cy2}" text-anchor="middle" dominant-baseline="central" font-family="{ff}" font-size="{fs}" fill="{fc}" font-weight="500">{txt}</text></g><g transform="rotate({r} {cx3} {cy3})"><text x="{cx3}" y="{cy3}" text-anchor="middle" dominant-baseline="central" font-family="{ff}" font-size="{fs}" fill="{fc}" font-weight="500">{txt}</text></g></svg>"#,
        tw=tile_w, th=tile_h,
        r=rotate,
        cx1=cx1, cy1=cy1,
        cx2=cx2, cy2=cy2,
        cx3=cx3, cy3=cy3,
        ff=font, fs=font_size, fc=color, txt=text,
    );
    let d = encode_svg(&svg);
    (d, tile_w, tile_h)
}

fn measure(js: String) -> (f64, f64) {
    js_sys::eval(&js).ok().and_then(|v| {
        let arr = js_sys::Array::from(&v);
        let w = arr.get(0).as_f64().unwrap_or(0.0);
        let h = arr.get(1).as_f64().unwrap_or(14.0);
        Some((w, h))
    }).unwrap_or((0.0, 14.0))
}

fn measure_bbox(js: &str) -> (u32, u32) {
    js_sys::eval(js).ok().and_then(|v| {
        let arr = js_sys::Array::from(&v);
        let w = arr.get(0).as_f64().unwrap_or(0.0) as u32;
        let h = arr.get(1).as_f64().unwrap_or(14.0) as u32;
        Some((w, h))
    }).unwrap_or((100, 100))
}

fn encode_svg(svg: &str) -> String {
    let encoded = svg
        .replace('%', "%25")
        .replace('"', "%22")
        .replace('#', "%23")
        .replace(' ', "%20")
        .replace('\t', "%09")
        .replace('\n', "");
    format!("data:image/svg+xml,{encoded}")
}

/// 全局自增计数器，为每个 Watermark 实例生成唯一 data-id
static WM_COUNTER: AtomicU64 = AtomicU64::new(0);

#[allow(non_snake_case)]
pub fn Watermark(props: WatermarkProps) -> Element {
    const CSS: &str = include_str!("../../assets/watermark.css");

    let class = if props.class.is_empty() {
        "ctrl-watermark".to_string()
    } else {
        format!("ctrl-watermark {}", props.class)
    };

    let user_gx = props.gap_x.unwrap_or(props.gap);
    let user_gy = props.gap_y.unwrap_or(props.gap);

    let text = props.content.as_deref().unwrap_or("Ctrl UI");

    let (data_uri, tile_w, tile_h) = render_tile(
        text,
        &props.font_family,
        &props.font_size,
        &props.color,
        props.rotate,
        user_gx,
        user_gy,
    );

    let layer_style = format!(
        "position:absolute;inset:0;pointer-events:none;background-image:url('{bg}');background-repeat:repeat;background-size:{tw}px {th}px;z-index:{z};display:block;opacity:1;visibility:visible;",
        bg = data_uri,
        tw = tile_w,
        th = tile_h,
        z = props.z_index,
    );
    let container_style = format!("position:relative;overflow:hidden;{}", props.style);

    // 唯一 ID：保护循环通过它只查找自己的容器，避免不同实例互相覆写
    let wm_id = WM_COUNTER.fetch_add(1, Ordering::Relaxed);
    let selector = format!("[data-watermark-id=\"{wm_id}\"]");

    let protection = props.protection;
    let interval = props.check_interval;
    let layer_clone = layer_style.clone();
    let container_clone = container_style.clone();

    use_effect(move || {
        if !protection {
            return;
        }

        // 每次 effect 被调用时 clone 一份，供内部 FnMut 闭包 move 使用
        let sel = selector.clone();
        let saved_layer = layer_clone.clone();
        let saved_container = container_clone.clone();

        let closure = Closure::<dyn FnMut()>::new(move || {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let container_el = match document.query_selector(&sel).ok().flatten().and_then(|c| c.dyn_ref::<web_sys::Element>().map(|e| e.clone())) {
                Some(el) => el,
                None => return,
            };

            // 保护容器样式
            let cs = container_el.get_attribute("style").unwrap_or_default();
            if cs != saved_container {
                let _ = container_el.set_attribute("style", &saved_container);
            }

            // 保护水印层
            let layers = container_el.query_selector_all(".ctrl-watermark__layer").unwrap();
            if layers.length() == 0 {
                let layer = document.create_element("div").unwrap();
                layer.set_class_name("ctrl-watermark__layer");
                let _ = layer.set_attribute("style", &saved_layer);
                let first_child = container_el.first_child();
                if let Some(ref fc) = first_child {
                    let _ = container_el.insert_before(&layer, Some(fc));
                } else {
                    let _ = container_el.append_child(&layer);
                }
            } else {
                for j in 0..layers.length() {
                    if let Some(ll) = layers.item(j) {
                        if let Some(le) = ll.dyn_ref::<web_sys::Element>() {
                            let cur = le.get_attribute("style").unwrap_or_default();
                            if cur != saved_layer {
                                let _ = le.set_attribute("style", &saved_layer);
                            }
                        }
                    }
                }
            }
        });

        let _ = web_sys::window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            interval as i32,
        );

        closure.forget();
    });

    rsx! {
        style { {CSS} }
        div {
            class: "{class}",
            style: "{container_style}",
            "data-watermark-id": "{wm_id}",
            div { class: "ctrl-watermark__layer", style: "{layer_style}" }
            div { class: "ctrl-watermark__content", {props.children} }
        }
    }
}
