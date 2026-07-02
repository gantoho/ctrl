use dioxus::prelude::*;

/// 锚点项
#[derive(Props, PartialEq, Clone)]
pub struct AnchorItem {
    /// 锚点唯一标识
    pub id: String,
    /// 显示文本
    pub title: String,
    /// 链接 href（如 #section1）
    pub href: String,
}

/// Anchor 锚点导航
#[derive(Props, PartialEq, Clone)]
pub struct AnchorProps {
    /// 锚点项列表
    pub items: Vec<AnchorItem>,

    /// 滚动偏移量（如顶部固定 header 高度）
    #[props(default = 0)]
    pub offset_top: u32,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

#[allow(non_snake_case)]
pub fn Anchor(props: AnchorProps) -> Element {
    const CSS: &str = include_str!("../../assets/anchor.css");
    let mut active = use_signal(|| String::new());

    let class = if props.class.is_empty() {
        "ctrl-anchor".to_string()
    } else {
        format!("ctrl-anchor {}", props.class)
    };

    rsx! {
        style { {CSS} }
        nav { class: "{class}", style: "{props.style}",
            for item in &props.items {
                {
                    let anchor_id = item.id.clone();
                    let href = item.href.clone();
                    let title = item.title.clone();
                    let offset = props.offset_top;

                    rsx! {
                        a {
                            key: "{anchor_id}",
                            class: if active() == anchor_id { "ctrl-anchor__item ctrl-anchor__item--active" }
                                   else { "ctrl-anchor__item" },
                            onclick: move |evt| {
                                evt.prevent_default();
                                active.set(anchor_id.clone());
                                scroll_to_anchor(&href, offset);
                            },
                            span { class: "ctrl-anchor__line" }
                            span { class: "ctrl-anchor__title", "{title}" }
                        }
                    }
                }
            }
        }
    }
}

/// 平滑滚动到锚点元素
#[allow(unused_variables)]
fn scroll_to_anchor(href: &str, offset: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let selector = href.strip_prefix('#').unwrap_or(href);
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(el) = document.get_element_by_id(selector) {
                    // 更新 URL hash
                    let url = format!("#{}", selector);
                    let _ = window
                        .history()
                        .ok()
                        .and_then(|h| h.replace_state_with_url(
                            &wasm_bindgen::JsValue::NULL,
                            "",
                            Some(&url),
                        ).ok());

                    // scrollIntoView 平滑滚动
                    el.scroll_into_view();
                    // 滚动后再回调 offset
                    let _ = window.scroll_by_with_x_and_y(0.0, -(offset as f64));
                }
            }
        }
    }
}
