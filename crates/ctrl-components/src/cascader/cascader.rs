use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use std::rc::Rc;
use std::cell::RefCell;
use ctrl_core::types::Size;
use crate::overlay::{self, OverlayClosures};
use wasm_bindgen::JsCast;

static CASCADER_ID: AtomicU16 = AtomicU16::new(1);

/// 级联选择器选项
#[derive(PartialEq, Clone)]
pub struct CascaderOption {
    pub value: String,
    pub label: String,
    pub children: Vec<CascaderOption>,
    pub disabled: bool,
}

impl CascaderOption {
    /// 叶子节点（无子级）
    pub fn leaf(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            children: Vec::new(),
            disabled: false,
        }
    }

    /// 父节点（含子级）
    pub fn parent(
        value: impl Into<String>,
        label: impl Into<String>,
        children: Vec<CascaderOption>,
    ) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            children,
            disabled: false,
        }
    }

    /// 标记为禁用
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Cascader 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct CascaderProps {
    /// 当前选中路径（受控）
    #[props(default = None)]
    pub value: Option<Vec<String>>,

    /// 级联选项树
    #[props(default = Vec::new())]
    pub options: Vec<CascaderOption>,

    /// 占位文本
    #[props(default = "请选择".to_string())]
    pub placeholder: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 尺寸
    #[props(default = Size::default())]
    pub size: Size,

    /// 是否可清空
    #[props(default = false)]
    pub clearable: bool,

    /// 展开下一级的触发方式：false 为点击展开（默认），true 为悬停展开
    #[props(default = false)]
    pub expand_on_hover: bool,

    /// 自定义 CSS 类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义内联样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 选中值变化回调
    pub onchange: Option<EventHandler<Vec<String>>>,
}

/// 沿 value 路径查找对应的 label 序列，用于拼接展示文本。
fn labels_of(options: &[CascaderOption], path: &[String]) -> Vec<String> {
    let mut labels = Vec::new();
    let mut level = options;
    for value in path {
        match level.iter().find(|o| &o.value == value) {
            Some(opt) => {
                labels.push(opt.label.clone());
                level = &opt.children;
            }
            None => break,
        }
    }
    labels
}

/// 根据导航路径逐级展开，构建需要展示的列。
/// 第 0 列为根选项，之后每列为上一列被选中节点的子级。
fn build_columns(options: &[CascaderOption], path: &[String]) -> Vec<Vec<CascaderOption>> {
    let mut columns = vec![options.to_vec()];
    for (depth, value) in path.iter().enumerate() {
        let children = columns[depth]
            .iter()
            .find(|o| &o.value == value)
            .map(|o| o.children.clone())
            .unwrap_or_default();
        if children.is_empty() {
            break;
        }
        columns.push(children);
    }
    columns
}

/// Cascader 级联选择器
#[allow(non_snake_case)]
pub fn Cascader(props: CascaderProps) -> Element {
    const CSS: &str = include_str!("../../assets/cascader.css");

    let id = CASCADER_ID.fetch_add(1, Ordering::Relaxed);
    let container_id = format!("ctrl-cascader-{}", id);
    let trigger_id = format!("ctrl-cascader-trigger-{}", id);
    let dropdown_id = format!("ctrl-cascader-dropdown-{}", id);

    let mut open = use_signal(|| false);
    // 已确认的选中路径，决定触发器展示文本。
    let mut selected: Signal<Vec<String>> = use_signal(|| props.value.clone().unwrap_or_default());
    // 面板导航路径，决定展开哪些列。
    let mut active_path: Signal<Vec<String>> =
        use_signal(|| props.value.clone().unwrap_or_default());

    // 受控同步：外部 value 变化时更新内部状态。
    use_effect(use_reactive(&props.value, move |v| {
        let val = v.clone().unwrap_or_default();
        selected.set(val.clone());
        active_path.set(val);
    }));

    let disabled = props.disabled;
    let clearable = props.clearable;
    let expand_on_hover = props.expand_on_hover;
    let options = props.options.clone();
    let onchange = props.onchange.clone();

    // ── 展示文本 ──
    let selected_path = selected();
    let display_text = if selected_path.is_empty() {
        props.placeholder.clone()
    } else {
        let labels = labels_of(&options, &selected_path);
        if labels.is_empty() {
            props.placeholder.clone()
        } else {
            labels.join(" / ")
        }
    };
    let is_empty = selected_path.is_empty();

    // ── 触发器样式 ──
    let size_cls = match props.size {
        Size::Sm => "sm",
        Size::Lg => "lg",
        _ => "md",
    };
    let mut trigger_classes = vec![
        "ctrl-cascader__trigger".to_string(),
        format!("ctrl-cascader__trigger--{}", size_cls),
    ];
    if disabled {
        trigger_classes.push("ctrl-cascader__trigger--disabled".into());
    }
    let trigger_class = trigger_classes.join(" ");

    let mut container_class = "ctrl-cascader".to_string();
    if !props.class.is_empty() {
        container_class = format!("{} {}", container_class, props.class);
    }

    let arrow_class = if open() {
        "ctrl-cascader__arrow ctrl-cascader__arrow--open"
    } else {
        "ctrl-cascader__arrow"
    };

    // ── fixed 弹层定位（复用公共能力，不匹配 trigger 宽度）──
    overlay::use_fixed_panel_effect(&dropdown_id, &trigger_id, open.clone(), 4.0, false);

    // ── z-index：打开时提升容器层级 ──
    {
        let cid = container_id.clone();
        let o = open.clone();
        use_effect(move || {
            let doc = match web_sys::window().and_then(|w| w.document()) {
                Some(d) => d,
                None => return,
            };
            let Some(el) = doc.get_element_by_id(&cid) else { return };
            let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() else { return };
            let _ = html_el
                .style()
                .set_property("z-index", if o() { "100" } else { "auto" });
        });
    }

    // ── click-outside（复用 overlay 的 mousedown/mouseup 方案）──
    let listeners = use_signal(|| Rc::new(RefCell::new(OverlayClosures::new())));
    {
        let store = listeners.clone();
        let tid = trigger_id.clone();
        let did = dropdown_id.clone();
        let o = open.clone();
        use_effect(move || {
            if o() {
                overlay::setup_click_outside(&store.read(), &tid, &did, o.clone());
            } else {
                store.read().borrow_mut().cleanup();
            }
        });
    }
    use_drop(move || {
        listeners.read().borrow_mut().cleanup();
    });

    // ── 构建级联列 ──
    let columns = build_columns(&options, &active_path());

    rsx! {
        style { {CSS} }
        div {
            id: "{container_id}",
            class: "{container_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },

            // ── 触发器 ──
            div {
                id: "{trigger_id}",
                class: "{trigger_class}",
                onclick: move |_| {
                    if disabled { return; }
                    let next = !open();
                    if next {
                        // 打开时将面板导航同步到当前选中路径
                        active_path.set(selected());
                    }
                    open.set(next);
                },
                span {
                    class: "ctrl-cascader__text",
                    if is_empty {
                        span { class: "ctrl-cascader__placeholder", "{display_text}" }
                    } else {
                        "{display_text}"
                    }
                }
                if clearable && !is_empty {
                    span {
                        class: "ctrl-cascader__clear",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            if disabled { return; }
                            selected.set(Vec::new());
                            active_path.set(Vec::new());
                            if let Some(ref cb) = onchange {
                                cb.call(Vec::new());
                            }
                        },
                        "×"
                    }
                }
                svg {
                    class: "{arrow_class}",
                    width: "12",
                    height: "12",
                    view_box: "0 0 12 12",
                    fill: "none",
                    path {
                        d: "M3 4.5l3 3 3-3",
                        stroke: "var(--ctrl-text-secondary)",
                        stroke_width: "1.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                    }
                }
            }

            // ── 下拉面板 ──
            div {
                id: "{dropdown_id}",
                class: "ctrl-cascader__dropdown",
                div {
                    class: "ctrl-cascader__panel",
                    for (depth , column) in columns.iter().enumerate() {
                        ul {
                            key: "{depth}",
                            class: "ctrl-cascader__column",
                            for opt in column.iter() {
                                {
                                    let opt_val = opt.value.clone();
                                    let opt_label = opt.label.clone();
                                    let opt_disabled = opt.disabled;
                                    let has_children = !opt.children.is_empty();
                                    let current = active_path();
                                    let is_active = depth < current.len() && current[depth] == opt_val;

                                    let mut item_class = "ctrl-cascader__item".to_string();
                                    if is_active {
                                        item_class.push_str(" ctrl-cascader__item--active");
                                    }
                                    if opt_disabled {
                                        item_class.push_str(" ctrl-cascader__item--disabled");
                                    }

                                    let cb = onchange.clone();
                                    rsx! {
                                        li {
                                            key: "{opt_val}",
                                            class: "{item_class}",
                                            onmouseenter: {
                                                let opt_val = opt_val.clone();
                                                move |_| {
                                                    // 悬停展开：仅对可展开的父级生效
                                                    if !expand_on_hover || opt_disabled || !has_children {
                                                        return;
                                                    }
                                                    let mut new_path: Vec<String> =
                                                        active_path().into_iter().take(depth).collect();
                                                    new_path.push(opt_val.clone());
                                                    active_path.set(new_path);
                                                }
                                            },
                                            onclick: move |_| {
                                                if opt_disabled { return; }
                                                let mut new_path: Vec<String> =
                                                    active_path().into_iter().take(depth).collect();
                                                new_path.push(opt_val.clone());
                                                if has_children {
                                                    active_path.set(new_path);
                                                } else {
                                                    active_path.set(new_path.clone());
                                                    selected.set(new_path.clone());
                                                    open.set(false);
                                                    if let Some(ref handler) = cb {
                                                        handler.call(new_path);
                                                    }
                                                }
                                            },
                                            span {
                                                class: "ctrl-cascader__item-label",
                                                "{opt_label}"
                                            }
                                            if has_children {
                                                span { class: "ctrl-cascader__item-arrow", "›" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
