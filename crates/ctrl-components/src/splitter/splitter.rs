use dioxus::prelude::*;
use ctrl_core::types::Direction;
use std::rc::Rc;
use std::cell::{Cell, RefCell};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;

// ═══════════════════════════════════════════════════════════
// Internal types
// ═══════════════════════════════════════════════════════════

/// Panel registration info collected from child SplitterPanel components
#[derive(Clone)]
struct PanelReg {
    default_size: f64,
    min_size: f64,
    max_size: f64,
    resizable: bool,
    collapsible: bool,
    collapsed: Signal<bool>,
    /// 折叠前保存的上次尺寸，用于恢复
    last_size: f64,
}

impl PartialEq for PanelReg {
    fn eq(&self, other: &Self) -> bool {
        self.default_size == other.default_size
            && self.min_size == other.min_size
            && self.max_size == other.max_size
            && self.resizable == other.resizable
            && self.collapsible == other.collapsible
            && self.collapsed == other.collapsed
    }
}

/// Context shared between Splitter and SplitterPanel children
#[derive(Clone)]
struct SplitterCtx {
    sizes: Signal<Vec<f64>>,
    direction: Direction,
    counter: Rc<Cell<usize>>,
    panels: Rc<RefCell<Vec<PanelReg>>>,
    onresize: Option<EventHandler<Vec<f64>>>,
    container_id: Signal<String>,
}

impl PartialEq for SplitterCtx {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            &*self.sizes as *const _,
            &*other.sizes as *const _,
        )
    }
}

#[cfg(target_arch = "wasm32")]
struct DragState {
    mm: Closure<dyn FnMut(web_sys::MouseEvent)>,
    mu: Closure<dyn FnMut(web_sys::MouseEvent)>,
    tm: Closure<dyn FnMut(web_sys::TouchEvent)>,
    tu: Closure<dyn FnMut(web_sys::TouchEvent)>,
}

/// Compute default sizes from registered panel info.
/// If any panel specifies a non‑zero default_size, use those normalized to 100%.
/// Otherwise, distribute equally.
fn compute_default_sizes(infos: &[PanelReg]) -> Vec<f64> {
    let n = infos.len();
    if n == 0 {
        return vec![];
    }
    let sum: f64 = infos.iter().map(|p| p.default_size).sum();
    if sum > 0.0 {
        infos.iter()
            .map(|p| p.default_size / sum * 100.0)
            .collect()
    } else {
        let equal = 100.0 / n as f64;
        vec![equal; n]
    }
}

/// Clamp a panel size to its min/max constraints.
#[allow(dead_code)]
fn clamp_size(size: f64, info: &PanelReg) -> f64 {
    let min = info.min_size;
    let max = if info.max_size > 0.0 { info.max_size } else { f64::MAX };
    size.clamp(min, max)
}

// ═══════════════════════════════════════════════════════════
// SplitterPanel Props
// ═══════════════════════════════════════════════════════════

/// SplitterPanel 面板属性
#[derive(Props, PartialEq, Clone)]
pub struct SplitterPanelProps {
    /// 默认尺寸（百分比，如 50.0 表示 50%；设为 0 则按均分计算）
    #[props(default = 0.0)]
    pub default_size: f64,

    /// 最小尺寸（百分比）
    #[props(default = 0.0)]
    pub min_size: f64,

    /// 最大尺寸（百分比，0 表示无上限）
    #[props(default = 0.0)]
    pub max_size: f64,

    /// 是否允许拖拽调整尺寸
    #[props(default = true)]
    pub resizable: bool,

    /// 是否可折叠
    #[props(default = false)]
    pub collapsible: bool,

    /// 初始是否折叠
    #[props(default = false)]
    pub collapsed: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素（面板内容）
    pub children: Element,
}

// ═══════════════════════════════════════════════════════════
// Splitter Props
// ═══════════════════════════════════════════════════════════

/// Splitter 可拖拽分隔面板属性
#[derive(Props, PartialEq, Clone)]
pub struct SplitterProps {
    /// 排列方向
    #[props(default = Direction::Horizontal)]
    pub direction: Direction,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 尺寸变化回调，参数为当前各面板尺寸 Vec<f64>
    pub onresize: Option<EventHandler<Vec<f64>>>,

    /// 子元素（SplitterPanel 组件）
    pub children: Element,
}

// ═══════════════════════════════════════════════════════════
// Collapse / Expand logic
// ═══════════════════════════════════════════════════════════

/// 折叠或展开指定面板，按比例从其余面板中重新分配空间。
fn toggle_collapse(
    panel_idx: usize,
    mut sizes: Signal<Vec<f64>>,
    panels: Rc<RefCell<Vec<PanelReg>>>,
    onresize: Option<EventHandler<Vec<f64>>>,
) {
    let mut guard = panels.borrow_mut();
    let n = sizes.read().len();
    if panel_idx >= n || panel_idx >= guard.len() {
        return;
    }

    let is_collapsed = (guard[panel_idx].collapsed)();
    let current_size = sizes.read()[panel_idx];

    if is_collapsed {
        // ── 展开：恢复为上次保存的尺寸 ──
        let restore_size = guard[panel_idx].last_size.max(guard[panel_idx].min_size);
        let delta = restore_size - current_size; // current_size 应为 0

        // 从其余面板按比例扣除空间
        let mut new_sizes = sizes.read().clone();
        new_sizes[panel_idx] = restore_size;

        let other_total: f64 = new_sizes.iter().enumerate()
            .filter(|(i, _)| *i != panel_idx)
            .map(|(_, s)| *s)
            .sum();

        if other_total > 0.0 && delta > 0.0 {
            for (i, s) in new_sizes.iter_mut().enumerate() {
                if i != panel_idx {
                    let ratio = *s / other_total;
                    *s = (*s - delta * ratio).max(0.0);
                }
            }
            // 确保所有面板总和为 100
            let total: f64 = new_sizes.iter().sum();
            if total > 0.0 {
                let factor = 100.0 / total;
                for s in &mut new_sizes {
                    *s *= factor;
                }
            }
        }

        let mut cs = guard[panel_idx].collapsed;
        cs.set(false);
        drop(guard);

        sizes.set(new_sizes.clone());
        if let Some(ref cb) = onresize {
            cb.call(new_sizes);
        }
    } else {
        // ── 折叠：保存当前尺寸，将空间按比例分配给其余面板 ──
        guard[panel_idx].last_size = current_size;

        let mut new_sizes = sizes.read().clone();
        let freed = current_size;
        new_sizes[panel_idx] = 0.0;

        let other_total: f64 = new_sizes.iter().enumerate()
            .filter(|(i, _)| *i != panel_idx)
            .map(|(_, s)| *s)
            .sum();

        if other_total > 0.0 && freed > 0.0 {
            for (i, s) in new_sizes.iter_mut().enumerate() {
                if i != panel_idx {
                    let ratio = *s / other_total;
                    *s += freed * ratio;
                }
            }
        }

        let mut cs = guard[panel_idx].collapsed;
        cs.set(true);
        drop(guard);

        sizes.set(new_sizes.clone());
        if let Some(ref cb) = onresize {
            cb.call(new_sizes);
        }
    }
}

// ═══════════════════════════════════════════════════════════
// Drag logic (wasm32)
// ═══════════════════════════════════════════════════════════

#[cfg(target_arch = "wasm32")]
fn begin_divider_drag(
    sizes: Signal<Vec<f64>>,
    divider_idx: usize,
    container_id: String,
    direction: Direction,
    panels: Rc<RefCell<Vec<PanelReg>>>,
    onresize: Option<EventHandler<Vec<f64>>>,
    drag_listeners: Signal<Rc<RefCell<Option<DragState>>>>,
    client_x: f64,
    client_y: f64,
) {
    let Some(window) = web_sys::window() else { return; };
    let Some(doc) = window.document() else { return; };
    let Some(container) = doc.get_element_by_id(&container_id) else { return; };

    let rect = container.get_bounding_client_rect();
    let container_size = if direction == Direction::Horizontal {
        rect.width()
    } else {
        rect.height()
    };
    if container_size <= 0.0 {
        return;
    }

    let start_pos = if direction == Direction::Horizontal {
        client_x
    } else {
        client_y
    };

    let initial_sizes = sizes.read().clone();
    let n = initial_sizes.len();

    // divider_idx 对应的分隔条在面板 (divider_idx-1) 和 divider_idx 之间
    let left_i = divider_idx.saturating_sub(1);
    let right_i = divider_idx;
    if right_i >= n {
        return;
    }

    let left_initial = initial_sizes[left_i];
    let right_initial = initial_sizes[right_i];
    // 拖拽过程中只调整这两个面板，保持其余面板不变
    let two_sum = left_initial + right_initial;

    let doc_target: web_sys::EventTarget = doc.clone().into();

    // ── mousemove ──
    let mm_closure = {
        let mut sizes = sizes;
        let panels = panels.clone();
        let container = container.clone();
        let onresize = onresize.clone();
        Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            let rect = container.get_bounding_client_rect();
            let current_size = if direction == Direction::Horizontal {
                rect.width()
            } else {
                rect.height()
            };
            if current_size <= 0.0 {
                return;
            }
            let current_pos = if direction == Direction::Horizontal {
                e.client_x() as f64
            } else {
                e.client_y() as f64
            };
            let delta_pct = (current_pos - start_pos) / current_size * 100.0;

            let guard = panels.borrow();
            let mut new_sizes = sizes.read().clone();
            let ns = new_sizes.len();
            if left_i < ns && right_i < ns {
                let left_info = &guard[left_i];
                let right_info = &guard[right_i];

                // 计算新尺寸
                let mut new_left = left_initial + delta_pct;
                let mut new_right = two_sum - new_left;

                // 对两侧分别做 clamp
                let clamped_left = clamp_size(new_left, left_info);
                let clamped_right = clamp_size(new_right, right_info);

                // 如果某一侧触发了 clamp，另一侧吸收差额
                let left_hit = (clamped_left - new_left).abs() > 0.01;
                let right_hit = (clamped_right - new_right).abs() > 0.01;

                if left_hit && !right_hit {
                    new_left = clamped_left;
                    new_right = two_sum - new_left;
                    new_right = clamp_size(new_right, right_info);
                } else if right_hit && !left_hit {
                    new_right = clamped_right;
                    new_left = two_sum - new_right;
                    new_left = clamp_size(new_left, left_info);
                } else {
                    // 两侧都未触发或都触发 → 使用 clamp 结果
                    new_left = clamped_left;
                    new_right = clamped_right;
                    // 如果都触发，用 clamp 结果但需要平衡
                    if left_hit && right_hit {
                        // 优先保证不超出各自约束，允许总和偏移
                        new_left = clamped_left;
                        new_right = clamped_right;
                    }
                }

                new_sizes[left_i] = new_left;
                new_sizes[right_i] = new_right;
            }

            sizes.set(new_sizes.clone());
            if let Some(ref cb) = onresize {
                cb.call(new_sizes);
            }
        }) as Box<dyn FnMut(web_sys::MouseEvent)>)
    };

    // ── mouseup ──
    let mu_closure = {
        let sizes = sizes;
        let drag_listeners = drag_listeners;
        let onresize = onresize.clone();
        Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
            let final_sizes = sizes.read().clone();
            if let Some(ref cb) = onresize {
                cb.call(final_sizes);
            }
            if let Some(state) = drag_listeners().borrow_mut().take() {
                let Some(doc) = web_sys::window().unwrap().document() else { return; };
                let d: web_sys::EventTarget = doc.into();
                let _ = d.remove_event_listener_with_callback(
                    "mousemove", state.mm.as_ref().unchecked_ref(),
                );
                let _ = d.remove_event_listener_with_callback(
                    "mouseup", state.mu.as_ref().unchecked_ref(),
                );
                let _ = d.remove_event_listener_with_callback(
                    "touchmove", state.tm.as_ref().unchecked_ref(),
                );
                let _ = d.remove_event_listener_with_callback(
                    "touchend", state.tu.as_ref().unchecked_ref(),
                );
            }
        }) as Box<dyn FnMut(web_sys::MouseEvent)>)
    };

    // ── touchmove ──
    let tm_closure = {
        let mut sizes = sizes;
        let panels = panels.clone();
        let container = container.clone();
        let onresize = onresize.clone();
        Closure::wrap(Box::new(move |e: web_sys::TouchEvent| {
            e.prevent_default();
            if let Some(touch) = e.touches().get(0) {
                let rect = container.get_bounding_client_rect();
                let current_size = if direction == Direction::Horizontal {
                    rect.width()
                } else {
                    rect.height()
                };
                if current_size <= 0.0 {
                    return;
                }
                let current_pos = if direction == Direction::Horizontal {
                    touch.client_x() as f64
                } else {
                    touch.client_y() as f64
                };
                let delta_pct = (current_pos - start_pos) / current_size * 100.0;

                let guard = panels.borrow();
                let mut new_sizes = sizes.read().clone();
                let ns = new_sizes.len();
                if left_i < ns && right_i < ns {
                    let left_info = &guard[left_i];
                    let right_info = &guard[right_i];

                    let mut new_left = left_initial + delta_pct;
                    let mut new_right = two_sum - new_left;

                    let clamped_left = clamp_size(new_left, left_info);
                    let clamped_right = clamp_size(new_right, right_info);

                    let left_hit = (clamped_left - new_left).abs() > 0.01;
                    let right_hit = (clamped_right - new_right).abs() > 0.01;

                    if left_hit && !right_hit {
                        new_left = clamped_left;
                        new_right = two_sum - new_left;
                        new_right = clamp_size(new_right, right_info);
                    } else if right_hit && !left_hit {
                        new_right = clamped_right;
                        new_left = two_sum - new_right;
                        new_left = clamp_size(new_left, left_info);
                    } else {
                        new_left = clamped_left;
                        new_right = clamped_right;
                    }

                    new_sizes[left_i] = new_left;
                    new_sizes[right_i] = new_right;
                }

                sizes.set(new_sizes.clone());
                if let Some(ref cb) = onresize {
                    cb.call(new_sizes);
                }
            }
        }) as Box<dyn FnMut(web_sys::TouchEvent)>)
    };

    // ── touchend ──
    let tu_closure = {
        let sizes = sizes;
        let drag_listeners = drag_listeners;
        let onresize = onresize.clone();
        Closure::wrap(Box::new(move |_e: web_sys::TouchEvent| {
            let final_sizes = sizes.read().clone();
            if let Some(ref cb) = onresize {
                cb.call(final_sizes);
            }
            if let Some(state) = drag_listeners().borrow_mut().take() {
                let Some(doc) = web_sys::window().unwrap().document() else { return; };
                let d: web_sys::EventTarget = doc.into();
                let _ = d.remove_event_listener_with_callback(
                    "mousemove", state.mm.as_ref().unchecked_ref(),
                );
                let _ = d.remove_event_listener_with_callback(
                    "mouseup", state.mu.as_ref().unchecked_ref(),
                );
                let _ = d.remove_event_listener_with_callback(
                    "touchmove", state.tm.as_ref().unchecked_ref(),
                );
                let _ = d.remove_event_listener_with_callback(
                    "touchend", state.tu.as_ref().unchecked_ref(),
                );
            }
        }) as Box<dyn FnMut(web_sys::TouchEvent)>)
    };

    let mm_ref = mm_closure.as_ref().clone();
    let mu_ref = mu_closure.as_ref().clone();
    let tm_ref = tm_closure.as_ref().clone();
    let tu_ref = tu_closure.as_ref().clone();

    let _ = doc_target.add_event_listener_with_callback("mousemove", mm_ref.unchecked_ref());
    let _ = doc_target.add_event_listener_with_callback("mouseup", mu_ref.unchecked_ref());
    let _ = doc_target.add_event_listener_with_callback("touchmove", tm_ref.unchecked_ref());
    let _ = doc_target.add_event_listener_with_callback("touchend", tu_ref.unchecked_ref());

    *drag_listeners().borrow_mut() = Some(DragState {
        mm: mm_closure,
        mu: mu_closure,
        tm: tm_closure,
        tu: tu_closure,
    });
}

// ═══════════════════════════════════════════════════════════
// Splitter component
// ═══════════════════════════════════════════════════════════

/// Splitter 可拖拽分隔面板组件
#[allow(non_snake_case)]
pub fn Splitter(props: SplitterProps) -> Element {
    const CSS: &str = include_str!("../../assets/splitter.css");

    let sizes = use_signal(Vec::<f64>::new);
    let counter: Rc<Cell<usize>> = Rc::new(Cell::new(0));
    let panels: Rc<RefCell<Vec<PanelReg>>> = Rc::new(RefCell::new(Vec::new()));

    let container_id = use_signal(|| {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static ID_CTR: AtomicUsize = AtomicUsize::new(0);
        format!("ctrl-splitter-{}", ID_CTR.fetch_add(1, Ordering::Relaxed))
    });

    let ctx = SplitterCtx {
        sizes,
        direction: props.direction,
        counter: counter.clone(),
        panels: panels.clone(),
        onresize: props.onresize.clone(),
        container_id: container_id.clone(),
    };
    use_context_provider(|| ctx.clone());

    // Build root class
    let root_class = {
        let mut c = String::from("ctrl-splitter");
        match props.direction {
            Direction::Horizontal => c.push_str(" ctrl-splitter--horizontal"),
            Direction::Vertical => c.push_str(" ctrl-splitter--vertical"),
            _ => {}
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    // 重置计数器（不清理注册表！保留 collapsed Signal）
    counter.set(0);

    // 初始化：仅在首次渲染后通过 effect 计算默认尺寸
    let mut initialized = use_signal(|| false);
    use_effect({
        let mut sizes = sizes;
        let panels = panels.clone();
        move || {
            if !initialized() {
                let infos = panels.borrow();
                if !infos.is_empty() {
                    let computed = compute_default_sizes(&infos);
                    sizes.set(computed);
                    initialized.set(true);
                }
            }
        }
    });

    rsx! {
        style { {CSS} }
        div {
            id: "{container_id}",
            class: "{root_class}",
            style: "{props.style}",
            {props.children}
        }
    }
}

// ═══════════════════════════════════════════════════════════
// SplitterPanel component
// ═══════════════════════════════════════════════════════════

/// 获取折叠按钮对应的箭头字符（根据方向自动旋转）
fn collapse_arrow(dir: Direction, is_left: bool, is_collapsed: bool) -> &'static str {
    match dir {
        Direction::Horizontal => {
            match (is_left, is_collapsed) {
                (true, true) => "▶",   // 左面板折叠 → 向右展开
                (true, false) => "◀",  // 左面板展开 → 向左折叠
                (false, true) => "◀",  // 右面板折叠 → 向左展开
                (false, false) => "▶", // 右面板展开 → 向右折叠
            }
        }
        Direction::Vertical => {
            match (is_left, is_collapsed) {
                (true, true) => "▼",   // 上面板折叠 → 向下展开
                (true, false) => "▲",  // 上面板展开 → 向上折叠
                (false, true) => "▲",  // 下面板折叠 → 向上展开
                (false, false) => "▼", // 下面板展开 → 向下折叠
            }
        }
        _ => "◀",
    }
}

/// SplitterPanel 面板组件
#[allow(non_snake_case)]
pub fn SplitterPanel(props: SplitterPanelProps) -> Element {
    let ctx = try_use_context::<SplitterCtx>();

    let (panel_idx, panel_size, is_collapsed, _resizable) =
        if let Some(ref ctx) = ctx {
            // 稳定索引分配（use_signal 闭包仅执行一次）
            let idx = use_signal(|| {
                let c = ctx.counter.get();
                ctx.counter.set(c + 1);
                c
            });
            let i = idx();

            // 注册 / 更新面板信息（就地更新，保留 collapsed Signal）
            {
                let mut infos = ctx.panels.borrow_mut();
                let is_new = i >= infos.len();

                // 新建占位条目（仅首次）
                while infos.len() <= i {
                    infos.push(PanelReg {
                        default_size: props.default_size,
                        min_size: props.min_size,
                        max_size: props.max_size,
                        resizable: props.resizable,
                        collapsible: props.collapsible,
                        collapsed: Signal::new(props.collapsed),
                        last_size: 0.0,
                    });
                }

                if !is_new {
                    // 更新已有条目：只更新 props 属性，不碰 collapsed Signal
                    infos[i].default_size = props.default_size;
                    infos[i].min_size = props.min_size;
                    infos[i].max_size = props.max_size;
                    infos[i].resizable = props.resizable;
                    infos[i].collapsible = props.collapsible;
                    // collapsed Signal 由 toggle_collapse 管理，不在此处修改
                }
            }

            let sizes = ctx.sizes.read();
            let size = if i < sizes.len() { sizes[i] } else { 0.0 };
            let collapsed = {
                let infos = ctx.panels.borrow();
                infos.get(i).map(|r| (r.collapsed)()).unwrap_or(false)
            };
            let resizable = {
                let infos = ctx.panels.borrow();
                infos.get(i).map(|r| r.resizable).unwrap_or(true)
            };

            (i, size, collapsed, resizable)
        } else {
            (0, 100.0, false, true)
        };

    // ── 构建分隔条（在非首个面板前渲染） ──
    let divider = if panel_idx > 0 && ctx.is_some() {
        let ctx_ref = ctx.as_ref().unwrap();
        let direction = ctx_ref.direction;
        let divider_idx = panel_idx;

        // 读取相邻面板的 collapsible 和 collapsed 状态
        let (left_collapsible, right_collapsible, left_collapsed, right_collapsed) = {
            let infos = ctx_ref.panels.borrow();
            let idx_l = divider_idx.saturating_sub(1);
            let idx_r = divider_idx;
            let left = infos.get(idx_l);
            let right = infos.get(idx_r);
            (
                left.map(|r| r.collapsible).unwrap_or(false),
                right.map(|r| r.collapsible).unwrap_or(false),
                left.map(|r| (r.collapsed)()).unwrap_or(false),
                right.map(|r| (r.collapsed)()).unwrap_or(false),
            )
        };

        let sizes = ctx_ref.sizes;
        let panels = ctx_ref.panels.clone();
        let container_id = ctx_ref.container_id;
        let onresize = ctx_ref.onresize.clone();

        #[cfg(target_arch = "wasm32")]
        let drag_listeners: Signal<Rc<RefCell<Option<DragState>>>> = use_signal(|| Rc::new(RefCell::new(None)));

        let on_mousedown = {
            let sizes = sizes;
            let panels = panels.clone();
            #[allow(unused_variables)]
            let container_id = container_id;
            let onresize = onresize.clone();
            #[cfg(target_arch = "wasm32")]
            let drag_listeners = drag_listeners;

            move |evt: MouseEvent| {
                evt.prevent_default();
                let client = evt.data().client_coordinates();
                #[cfg(target_arch = "wasm32")]
                {
                    begin_divider_drag(
                        sizes,
                        divider_idx,
                        container_id(),
                        direction,
                        panels.clone(),
                        onresize.clone(),
                        drag_listeners,
                        client.x,
                        client.y,
                    );
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = (&sizes, divider_idx, &direction, &panels, &onresize, client);
                }
            }
        };

        let on_touchstart = {
            let sizes = sizes;
            let panels = panels.clone();
            #[allow(unused_variables)]
            let container_id = container_id;
            let onresize = onresize.clone();
            #[cfg(target_arch = "wasm32")]
            let drag_listeners = drag_listeners;

            move |evt: TouchEvent| {
                evt.prevent_default();
                if let Some(touch) = evt.data().touches().first() {
                    let coords = touch.client_coordinates();
                    #[cfg(target_arch = "wasm32")]
                    {
                        begin_divider_drag(
                            sizes,
                            divider_idx,
                            container_id(),
                            direction,
                            panels.clone(),
                            onresize.clone(),
                            drag_listeners,
                            coords.x,
                            coords.y,
                        );
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let _ = (&sizes, divider_idx, &direction, &panels, &onresize, coords);
                    }
                }
            }
        };

        // 折叠左侧面板（panel_idx - 1）
        let collapse_left = {
            let sizes = sizes;
            let panels = panels.clone();
            let onresize = onresize.clone();
            let left_i = panel_idx - 1;
            move |_| {
                toggle_collapse(left_i, sizes, panels.clone(), onresize.clone());
            }
        };

        // 折叠右侧面板（panel_idx）
        let collapse_right = {
            let sizes = sizes;
            let panels = panels.clone();
            let onresize = onresize.clone();
            let right_i = panel_idx;
            move |_| {
                toggle_collapse(right_i, sizes, panels.clone(), onresize.clone());
            }
        };

        // ── 按钮显示逻辑 ──
        // 规则：
        //   - 面板折叠后，其展开按钮只出现在该面板左侧的分隔条上
        //   - 当一侧面板已折叠时，另一侧的折叠按钮隐藏（避免分隔条紧邻时重叠）
        //   - 首个面板(index=0)折叠时例外：展开按钮出现在其右侧分隔条上
        let is_left_first = divider_idx.saturating_sub(1) == 0;

        let show_left = left_collapsible && (
            // 左侧面板折叠中 → 展开按钮：仅当它是首个面板（无左侧分隔条）
            (left_collapsed && is_left_first)
            // 左侧面板未折叠 → 折叠按钮：仅当右侧也未被折叠（避免重叠）
            || (!left_collapsed && !right_collapsed)
        );

        let show_right = right_collapsible && (
            // 右侧面板折叠中 → 展开按钮：始终在此显示（这是它的左侧分隔条）
            right_collapsed
            // 右侧面板未折叠 → 折叠按钮：仅当左侧也未被折叠
            || (!right_collapsed && !left_collapsed)
        );

        let has_toggles = show_left || show_right;

        Some(rsx! {
            div {
                class: "ctrl-splitter__divider",
                onmousedown: on_mousedown,
                ontouchstart: on_touchstart,

                if has_toggles {
                    div {
                        class: "ctrl-splitter__divider-toggles",

                        if show_left {
                            button {
                                class: if left_collapsed {
                                    "ctrl-splitter__collapse-btn ctrl-splitter__collapse-btn--expand"
                                } else {
                                    "ctrl-splitter__collapse-btn"
                                },
                                onclick: collapse_left,
                                title: if left_collapsed { "展开" } else { "折叠" },
                                {collapse_arrow(direction, true, left_collapsed)}
                            }
                        }

                        if show_right {
                            button {
                                class: if right_collapsed {
                                    "ctrl-splitter__collapse-btn ctrl-splitter__collapse-btn--expand"
                                } else {
                                    "ctrl-splitter__collapse-btn"
                                },
                                onclick: collapse_right,
                                title: if right_collapsed { "展开" } else { "折叠" },
                                {collapse_arrow(direction, false, right_collapsed)}
                            }
                        }
                    }
                }
            }
        })
    } else {
        None
    };

    // ── Panel class and style ──
    let panel_class = {
        let mut c = String::from("ctrl-splitter__panel");
        if is_collapsed {
            c.push_str(" ctrl-splitter__panel--collapsed");
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    let panel_style = if ctx.is_some() {
        let dir = ctx.as_ref().map(|c| c.direction).unwrap_or(Direction::Horizontal);
        let size_prop = if dir == Direction::Horizontal { "width" } else { "height" };
        let mut s = format!("flex: 0 0 {}%; {}: {}%;", panel_size, size_prop, panel_size);
        if is_collapsed {
            s.push_str(" overflow: hidden; min-width: 0; min-height: 0;");
        }
        if !props.style.is_empty() {
            s.push_str(" ");
            s.push_str(&props.style);
        }
        s
    } else {
        props.style.clone()
    };

    // ── 渲染：分隔条（如有）+ 面板 ──
    rsx! {
        if let Some(div) = divider {
            {div}
        }
        div {
            class: "{panel_class}",
            style: "{panel_style}",
            {props.children}
        }
    }
}
