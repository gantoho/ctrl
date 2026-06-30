use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};

static COLLAPSE_ID: AtomicU16 = AtomicU16::new(1);

/// Collapse 折叠面板组件属性
#[derive(Props, PartialEq, Clone)]
pub struct CollapseProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 是否无边框
    #[props(default = false)]
    pub borderless: bool,

    /// 是否启用手风琴模式（同时只有一个展开项）
    #[props(default = false)]
    pub accordion: bool,

    /// 子元素（CollapseItem）
    pub children: Element,
}

/// 单个折叠项属性
#[derive(Props, PartialEq, Clone)]
pub struct CollapseItemProps {
    /// 标题
    #[props(default = "".to_string())]
    pub title: String,

    /// 是否展开
    #[props(default = false)]
    pub expanded: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否显示箭头
    #[props(default = true)]
    pub show_arrow: bool,

    /// 是否启用展开/收起动画（默认 true）
    #[props(default = true)]
    pub animated: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（展开后显示的内容）
    pub children: Element,
}

// ── 手风琴上下文 ──
#[derive(Clone)]
struct CollapseCtx {
    accordion: bool,
    active_id: Signal<Option<u16>>,
}

/// Collapse 折叠面板组件
#[allow(non_snake_case)]
pub fn Collapse(props: CollapseProps) -> Element {
    const CSS: &str = include_str!("../../assets/collapse.css");

    let active_id = use_signal(|| None::<u16>);
    use_context_provider(|| CollapseCtx {
        accordion: props.accordion,
        active_id,
    });

    let wrapper_class = if props.borderless {
        if props.class.is_empty() {
            "ctrl-collapse ctrl-collapse--borderless".to_string()
        } else {
            format!("ctrl-collapse ctrl-collapse--borderless {}", props.class)
        }
    } else {
        if props.class.is_empty() {
            "ctrl-collapse".to_string()
        } else {
            format!("ctrl-collapse {}", props.class)
        }
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            {props.children}
        }
    }
}

/// 单个折叠项组件
#[allow(non_snake_case)]
pub fn CollapseItem(props: CollapseItemProps) -> Element {
    let ctx = try_use_context::<CollapseCtx>();
    let mut expanded = use_signal(|| props.expanded);

    // 同步外部 prop 更新到内部信号
    use_effect(use_reactive(&props.expanded, move |e| {
        expanded.set(e);
    }));

    let item_id = COLLAPSE_ID.fetch_add(1, Ordering::Relaxed);
    let content_id = format!("ctrl-collapse-{}", item_id);

    // 提前提取上下文中的字段，避免多次 move
    let accordion_mode = ctx.as_ref().map(|c| c.accordion).unwrap_or(false);
    let mut active_id_signal = ctx.as_ref().map(|c| c.active_id.clone());

    // 手风琴模式下，首次遇到 expanded:true 的项设为当前激活项
    if accordion_mode && props.expanded {
        if let Some(ref mut active) = active_id_signal {
            if active().is_none() {
                active.set(Some(item_id));
            }
        }
    }

    // 最终展开状态：手风琴模式由上下文驱动，否则由自身信号驱动
    let active_for_memo = active_id_signal.clone();
    let is_expanded = use_memo(move || {
        if let Some(ref active) = active_for_memo {
            if accordion_mode {
                active() == Some(item_id)
            } else {
                expanded()
            }
        } else {
            expanded()
        }
    });

    let header_class = if props.disabled {
        "ctrl-collapse__header ctrl-collapse__header--disabled"
    } else {
        "ctrl-collapse__header"
    };

    let arrow_class = if is_expanded() {
        "ctrl-collapse__arrow ctrl-collapse__arrow--expanded"
    } else {
        "ctrl-collapse__arrow"
    };

    let body_class = if props.animated {
        if is_expanded() {
            "ctrl-collapse__body ctrl-collapse__body--expanded"
        } else {
            "ctrl-collapse__body"
        }
    } else {
        if is_expanded() {
            "ctrl-collapse__body ctrl-collapse__body--expanded ctrl-collapse__body--no-animation"
        } else {
            "ctrl-collapse__body ctrl-collapse__body--no-animation"
        }
    };

    let title = props.title.clone();
    let item_class = if props.class.is_empty() {
        "ctrl-collapse__item".to_string()
    } else {
        format!("ctrl-collapse__item {}", props.class)
    };

    // 通过 use_effect 监听 is_expanded 变化，用 web_sys 精确控制 max-height
    let id_for_effect = content_id.clone();
    use_effect(use_reactive((&is_expanded,), move |(is_expanded,)| {
        let id = id_for_effect.clone();
        let expanded = is_expanded();
        wasm_bindgen_futures::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(1).await;
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    if let Some(el) = doc.get_element_by_id(&id) {
                        if expanded {
                            let h = el.scroll_height();
                            el.set_attribute("style", &format!("max-height: {}px", h.max(0))).ok();
                        } else {
                            el.set_attribute("style", "max-height: 0px").ok();
                        }
                    }
                }
            }
        });
    }));

    rsx! {
        div {
            class: "{item_class}",
            div {
                class: "{header_class}",
                onclick: move |_| {
                    if !props.disabled {
                        if accordion_mode {
                            if let Some(ref mut active) = active_id_signal {
                                if active() == Some(item_id) {
                                    active.set(None);
                                } else {
                                    active.set(Some(item_id));
                                }
                            }
                        } else {
                            expanded.set(!expanded());
                        }
                    }
                },
                "{title}"
                if props.show_arrow {
                    span {
                        class: "{arrow_class}",
                        "▶"
                    }
                }
            }
            div {
                class: "{body_class}",
                div {
                    id: "{content_id}",
                    class: "ctrl-collapse__content",
                    div {
                        class: "ctrl-collapse__content-inner",
                        {props.children}
                    }
                }
            }
        }
    }
}