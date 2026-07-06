use std::rc::Rc;
use std::cell::RefCell;
use dioxus::prelude::*;
use ctrl_core::types::Placement;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

// ═══════════════════════════════════════════════════════════
// TourStep — 引导步骤定义
// ═══════════════════════════════════════════════════════════

/// 引导漫游中的单个步骤
#[derive(Clone, PartialEq)]
pub struct TourStep {
    /// 目标元素的 CSS 选择器
    pub target: String,
    /// 步骤标题
    pub title: String,
    /// 步骤描述
    pub description: String,
    /// 提示卡相对于目标元素的弹出位置
    pub placement: Placement,
}

// ═══════════════════════════════════════════════════════════
// TargetRect — 元素位置信息
// ═══════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq)]
struct TargetRect {
    top: f64,
    left: f64,
    width: f64,
    height: f64,
}

// ═══════════════════════════════════════════════════════════
// TourAPI — 通过 use_tour() 获取
// ═══════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct TourAPI {
    pub(crate) visible: Signal<bool>,
    pub(crate) current: Signal<usize>,
    pub(crate) steps: Signal<Vec<TourStep>>,
    pub(crate) onfinish: Signal<Option<EventHandler<()>>>,
}

/// 获取 Tour 命令式 API
pub fn use_tour() -> TourAPI {
    use_context::<TourAPI>()
}

impl TourAPI {
    /// 开始引导，设置步骤并从第 0 步开始
    pub fn start(&mut self, steps: Vec<TourStep>) {
        self.steps.set(steps);
        self.current.set(0);
        self.visible.set(true);
    }

    /// 关闭引导
    pub fn close(&mut self) {
        self.visible.set(false);
    }

    /// 跳转到下一步
    pub fn next(&mut self) {
        let idx = *self.current.read();
        let len = self.steps.read().len();
        if idx + 1 < len {
            self.current.set(idx + 1);
        }
    }

    /// 跳转到上一步
    pub fn prev(&mut self) {
        let idx = *self.current.read();
        if idx > 0 {
            self.current.set(idx - 1);
        }
    }

    /// 完成引导
    pub fn finish(&mut self) {
        let onfinish = self.onfinish.read().clone();
        if let Some(ref handler) = onfinish {
            handler.call(());
        }
        self.visible.set(false);
    }
}

// ═══════════════════════════════════════════════════════════
// TourProvider
// ═══════════════════════════════════════════════════════════

/// TourProvider 属性
#[derive(Props, PartialEq, Clone)]
pub struct TourProviderProps {
    pub children: Element,
}

/// 引导漫游命令式 Provider
#[allow(non_snake_case)]
pub fn TourProvider(props: TourProviderProps) -> Element {
    let visible = use_signal(|| false);
    let current = use_signal(|| 0_usize);
    let steps = use_signal(Vec::<TourStep>::new);
    let onfinish = use_signal(|| None::<EventHandler<()>>);

    let api = TourAPI {
        visible,
        current,
        steps,
        onfinish,
    };
    use_context_provider(|| api);

    let vis = visible();
    let steps_val = steps.read().clone();
    let cur = current();
    let onfinish_handler = onfinish.read().clone();

    let mut close_api = api;

    rsx! {
        {props.children}
        Tour {
            steps: steps_val,
            current: cur,
            open: vis,
            onclose: EventHandler::new(move |_| { close_api.close(); }),
            onfinish: onfinish_handler,
        }
    }
}

// ═══════════════════════════════════════════════════════════
// TourProps — 声明式 API
// ═══════════════════════════════════════════════════════════

/// Tour 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TourProps {
    /// 步骤列表
    #[props(default)]
    pub steps: Vec<TourStep>,

    /// 当前步骤索引
    #[props(default = 0)]
    pub current: usize,

    /// 是否打开
    #[props(default = false)]
    pub open: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 关闭事件
    pub onclose: Option<EventHandler<()>>,

    /// 完成事件
    pub onfinish: Option<EventHandler<()>>,
}

// ═══════════════════════════════════════════════════════════
// Tour 组件
// ═══════════════════════════════════════════════════════════

/// Tour 引导漫游组件
#[allow(non_snake_case)]
pub fn Tour(props: TourProps) -> Element {
    const CSS: &str = include_str!("../../assets/tour.css");

    // ── 所有 hooks 必须无条件调用，且位于任何提前 return 之前 ──
    let mut current = use_signal(|| props.current);
    let target_rect = use_signal(|| Option::<TargetRect>::None);

    // 同步 current：外部受控值变化时、或每次打开引导时，重置到 props.current
    use_effect(use_reactive(
        (&props.open, &props.current),
        move |(open, ext): (bool, usize)| {
            if open {
                current.set(ext);
            }
        },
    ));

    // 读取当前步骤索引（同时让组件订阅 current 的变化）
    let idx = current();

    // 计算当前步骤目标元素位置：open / 步骤索引 / 步骤列表变化时重算
    {
        let mut rect = target_rect;
        use_effect(use_reactive(
            (&props.open, &idx, &props.steps),
            move |(open, idx, steps): (bool, usize, Vec<TourStep>)| {
                if open {
                    if let Some(step) = steps.get(idx) {
                        calculate_target_rect(&step.target, &mut rect);
                        return;
                    }
                }
                rect.set(None);
            },
        ));
    }

    // 键盘导航：open / 步骤数变化时重新注册监听（先清理旧的）
    let keydown_listeners: Signal<Rc<RefCell<Option<Closure<dyn FnMut(web_sys::KeyboardEvent)>>>>> =
        use_signal(|| Rc::new(RefCell::new(None)));
    {
        let listeners = keydown_listeners;
        let onclose = props.onclose.clone();
        let mut cur = current;
        use_effect(use_reactive(
            (&props.open, &props.steps.len()),
            move |(open, total): (bool, usize)| {
                // 先移除旧监听
                if let Some(old) = listeners.read().borrow_mut().take() {
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        let _ = doc.remove_event_listener_with_callback(
                            "keydown",
                            old.as_ref().unchecked_ref(),
                        );
                    }
                }

                if !open {
                    return;
                }

                let onclose = onclose.clone();
                let closure = Closure::new(move |evt: web_sys::KeyboardEvent| {
                    match evt.key().as_str() {
                        "Escape" => {
                            if let Some(ref h) = onclose {
                                h.call(());
                            }
                        }
                        "ArrowRight" | "ArrowDown" => {
                            if cur() + 1 < total {
                                cur.set(cur() + 1);
                            }
                        }
                        "ArrowLeft" | "ArrowUp" => {
                            if cur() > 0 {
                                cur.set(cur() - 1);
                            }
                        }
                        _ => {}
                    }
                });

                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    let _ = doc.add_event_listener_with_callback(
                        "keydown",
                        closure.as_ref().unchecked_ref(),
                    );
                }
                *listeners.read().borrow_mut() = Some(closure);
            },
        ));
    }

    // 组件卸载时清理键盘监听
    use_drop(move || {
        if let Some(cb) = keydown_listeners.read().borrow_mut().take() {
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                let _ = doc.remove_event_listener_with_callback(
                    "keydown",
                    cb.as_ref().unchecked_ref(),
                );
            }
        }
    });

    // ── 关闭或无步骤时不渲染（hooks 已全部执行，不违反 Hooks 规则）──
    if !props.open || props.steps.is_empty() {
        return rsx! {};
    }

    let steps = &props.steps;
    let total = steps.len();
    // 防止索引越界（如步骤数减少）
    let display_idx = idx.min(total - 1);
    let step = steps.get(display_idx);
    let is_first = display_idx == 0;
    let is_last = display_idx + 1 >= total;

    // 计算提示卡位置
    let placement = step.map(|s| s.placement).unwrap_or_default();
    let tooltip_style = compute_tooltip_style(target_rect(), placement);

    // 计算聚光灯样式
    let spotlight_style = target_rect()
        .map(|r| {
            format!(
                "top:{:.2}px;left:{:.2}px;width:{:.2}px;height:{:.2}px;",
                r.top, r.left, r.width, r.height
            )
        })
        .unwrap_or_default();

    // 检查目标元素是否存在
    let target_found = target_rect().is_some();

    let onclose = props.onclose.clone();
    let onfinish = props.onfinish.clone();

    let step_title = step.map(|s| s.title.as_str()).unwrap_or("");
    let step_description = step.map(|s| s.description.as_str()).unwrap_or("");
    let step_target = step.map(|s| s.target.as_str()).unwrap_or("");

    let overlay_class = if props.class.is_empty() {
        "ctrl-tour__overlay".to_string()
    } else {
        format!("ctrl-tour__overlay {}", props.class)
    };

    let overlay_style = props.style.clone();

    rsx! {
        style { {CSS} }

        // 遮罩层
        div {
            class: "{overlay_class}",
            style: if !overlay_style.is_empty() { overlay_style.as_str() } else { "" },
            div { class: "ctrl-tour__mask" }
            div {
                class: "ctrl-tour__spotlight",
                style: "{spotlight_style}",
            }
        }

        // 提示卡片
        div {
            class: "ctrl-tour__tooltip",
            style: "{tooltip_style}",

            div {
                class: "ctrl-tour__tooltip-header",
                div {
                    class: "ctrl-tour__tooltip-title",
                    "{step_title}"
                }
                button {
                    class: "ctrl-tour__tooltip-close",
                    onclick: {
                        let handler = onclose.clone();
                        move |_| {
                            if let Some(ref h) = handler { h.call(()); }
                        }
                    },
                    "\u{2715}"
                }
            }

            div {
                class: "ctrl-tour__tooltip-body",
                if !target_found {
                    div {
                        style: "color: #faad14; font-weight: 600; margin-bottom: 8px;",
                        "目标元素 \"{step_target}\" 未找到。请在页面中添加对应的 DOM 元素。"
                    }
                }
                "{step_description}"
            }

            div {
                class: "ctrl-tour__tooltip-footer",
                span {
                    class: "ctrl-tour__tooltip-steps",
                    "Step {display_idx + 1} of {total}"
                }
                div {
                    class: "ctrl-tour__tooltip-actions",
                    button {
                        class: "ctrl-tour__btn ctrl-tour__btn--skip",
                        onclick: {
                            let handler = onclose.clone();
                            move |_| {
                                if let Some(ref h) = handler { h.call(()); }
                            }
                        },
                        "Skip"
                    }
                    if !is_first {
                        button {
                            class: "ctrl-tour__btn ctrl-tour__btn--prev",
                            onclick: move |_| {
                                if current() > 0 { current.set(current() - 1); }
                            },
                            "Prev"
                        }
                    }
                    if is_last {
                        button {
                            class: "ctrl-tour__btn ctrl-tour__btn--finish",
                            onclick: {
                                let h_finish = onfinish.clone();
                                let h_close = onclose.clone();
                                move |_| {
                                    if let Some(ref h) = h_finish { h.call(()); }
                                    if let Some(ref h) = h_close { h.call(()); }
                                }
                            },
                            "Finish"
                        }
                    } else {
                        button {
                            class: "ctrl-tour__btn ctrl-tour__btn--next",
                            onclick: move |_| {
                                current.set(current() + 1);
                            },
                            "Next"
                        }
                    }
                }
            }

            div {
                class: "ctrl-tour__tooltip-arrow",
                style: "{compute_arrow_style(target_rect(), placement)}",
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════
// 工具函数
// ═══════════════════════════════════════════════════════════

fn calculate_target_rect(selector: &str, target_rect: &mut Signal<Option<TargetRect>>) {
    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => {
            target_rect.set(None);
            return;
        }
    };

    let el = match doc.query_selector(selector) {
        Ok(Some(el)) => el,
        _ => {
            target_rect.set(None);
            return;
        }
    };

    let html_el = match el.dyn_ref::<HtmlElement>() {
        Some(h) => h,
        None => {
            target_rect.set(None);
            return;
        }
    };

    let rect = html_el.get_bounding_client_rect();
    target_rect.set(Some(TargetRect {
        top: rect.top(),
        left: rect.left(),
        width: rect.width(),
        height: rect.height(),
    }));
}

fn compute_tooltip_style(target_rect: Option<TargetRect>, placement: Placement) -> String {
    let rect = match target_rect {
        Some(r) => r,
        // 目标未找到时隐藏卡片（Dioxus 增量 style 只增不减，必须显式输出 visibility）
        None => return "visibility:hidden;".to_string(),
    };

    let gap: f64 = 12.0;
    let arrow_size: f64 = 12.0;

    // 使用 CSS transform 做居中 / 对齐，无需估算 tooltip 尺寸，位置始终精确
    let (top, left, transform) = match placement {
        Placement::Top => (
            rect.top - gap - arrow_size,
            rect.left + rect.width / 2.0,
            "translate(-50%, -100%)",
        ),
        Placement::Bottom => (
            rect.top + rect.height + gap + arrow_size,
            rect.left + rect.width / 2.0,
            "translateX(-50%)",
        ),
        Placement::Left => (
            rect.top + rect.height / 2.0,
            rect.left - gap - arrow_size,
            "translate(-100%, -50%)",
        ),
        Placement::Right => (
            rect.top + rect.height / 2.0,
            rect.left + rect.width + gap + arrow_size,
            "translateY(-50%)",
        ),
        Placement::TopStart => (
            rect.top - gap - arrow_size,
            rect.left,
            "translateY(-100%)",
        ),
        Placement::TopEnd => (
            rect.top - gap - arrow_size,
            rect.left + rect.width,
            "translate(-100%, -100%)",
        ),
        Placement::BottomStart => (
            rect.top + rect.height + gap + arrow_size,
            rect.left,
            "none",
        ),
        Placement::BottomEnd => (
            rect.top + rect.height + gap + arrow_size,
            rect.left + rect.width,
            "translateX(-100%)",
        ),
        _ => (
            rect.top + rect.height + gap + arrow_size,
            rect.left + rect.width / 2.0,
            "translateX(-50%)",
        ),
    };

    // 限制锚点不超出视口
    let vw = web_sys::window()
        .and_then(|w| w.inner_width().ok().and_then(|v| v.as_f64()))
        .unwrap_or(1024.0);
    let vh = web_sys::window()
        .and_then(|w| w.inner_height().ok().and_then(|v| v.as_f64()))
        .unwrap_or(768.0);

    let clamped_top = top.max(0.0).min(vh);
    let clamped_left = left.max(0.0).min(vw);

    // 显式 set visibility:visible，覆盖首帧残留的 visibility:hidden
    format!(
        "top:{:.2}px;left:{:.2}px;transform:{transform};visibility:visible;",
        clamped_top, clamped_left,
    )
}

fn compute_arrow_style(target_rect: Option<TargetRect>, placement: Placement) -> String {
    if target_rect.is_none() {
        // 显式 hide（Dioxus 增量 style 不会自动清理之前的定位值）
        return "display:none;top:auto;bottom:auto;left:auto;right:auto;margin-left:0;margin-top:0;".to_string();
    }

    let arrow_size: f64 = 12.0;
    let half_arrow: f64 = arrow_size / 2.0;

    let pos = match placement {
        Placement::Top => format!(
            "top:auto;bottom:-{:.0}px;left:50%;margin-left:-{:.0}px;",
            half_arrow, half_arrow
        ),
        Placement::Bottom => format!(
            "bottom:auto;top:-{:.0}px;left:50%;margin-left:-{:.0}px;",
            half_arrow, half_arrow
        ),
        Placement::Left => format!(
            "left:auto;right:-{:.0}px;top:50%;margin-top:-{:.0}px;",
            half_arrow, half_arrow
        ),
        Placement::Right => format!(
            "right:auto;left:-{:.0}px;top:50%;margin-top:-{:.0}px;",
            half_arrow, half_arrow
        ),
        Placement::TopStart => format!("top:auto;bottom:-{:.0}px;left:24px;right:auto;", half_arrow),
        Placement::TopEnd => format!("top:auto;bottom:-{:.0}px;right:24px;left:auto;", half_arrow),
        Placement::BottomStart => format!("bottom:auto;top:-{:.0}px;left:24px;right:auto;", half_arrow),
        Placement::BottomEnd => format!("bottom:auto;top:-{:.0}px;right:24px;left:auto;", half_arrow),
        _ => return "display:none;".to_string(),
    };

    // 显式 set display:block 以确保之前残留的 display:none 被覆盖
    format!("display:block;{}", pos)
}
