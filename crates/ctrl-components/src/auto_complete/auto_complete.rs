use dioxus::prelude::*;
use std::sync::atomic::{AtomicU16, Ordering};
use std::rc::Rc;
use std::cell::RefCell;
use ctrl_core::types::Size;
use crate::overlay::{self, OverlayClosures};
use wasm_bindgen::JsCast;

static AUTO_COMPLETE_ID: AtomicU16 = AtomicU16::new(1);

/// 自动补全选项
#[derive(Debug, Clone, PartialEq)]
pub struct AutoCompleteOption {
    pub value: String,
    pub label: String,
}

/// AutoComplete 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct AutoCompleteProps {
    /// 当前值
    #[props(default = "".to_string())]
    pub value: String,

    /// 占位文本
    #[props(default = "请输入".to_string())]
    pub placeholder: String,

    /// 选项列表
    #[props(default = Vec::new())]
    pub options: Vec<AutoCompleteOption>,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 尺寸
    #[props(default = Size::default())]
    pub size: Size,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 值变化事件
    pub onchange: Option<EventHandler<String>>,

    /// 选项选中事件
    pub onselect: Option<EventHandler<AutoCompleteOption>>,
}

/// 过滤选项，匹配 label 中包含搜索文本的选项
fn filter_options(options: &[AutoCompleteOption], query: &str) -> Vec<AutoCompleteOption> {
    if query.is_empty() {
        return options.to_vec();
    }
    let lower = query.to_lowercase();
    options
        .iter()
        .filter(|o| o.label.to_lowercase().contains(&lower))
        .cloned()
        .collect()
}

/// HTML 实体转义（避免 XSS）
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// 构建高亮 HTML：将 label 中匹配 query 的部分用 <mark> 包裹
fn highlight_label(label: &str, query: &str) -> String {
    if query.is_empty() {
        return escape_html(label);
    }
    let lower_label = label.to_lowercase();
    let lower_query = query.to_lowercase();
    let mut result = String::new();
    let mut last_end = 0;

    let matches: Vec<(usize, usize)> = lower_label.match_indices(&lower_query).map(|(s, m)| (s, m.len())).collect();
    if matches.is_empty() {
        result.push_str(&escape_html(label));
        return result;
    }

    for (start, _) in &matches {
        let start = *start;
        let end = start + query.len();
        if start >= last_end {
            result.push_str(&escape_html(&label[last_end..start]));
            result.push_str("<mark>");
            result.push_str(&escape_html(&label[start..end]));
            result.push_str("</mark>");
            last_end = end;
        }
    }
    if last_end < label.len() {
        result.push_str(&escape_html(&label[last_end..]));
    }
    result
}

/// AutoComplete 自动补全组件
#[allow(non_snake_case)]
pub fn AutoComplete(props: AutoCompleteProps) -> Element {
    const CSS: &str = include_str!("../../assets/auto_complete.css");
    let mut open = use_signal(|| false);
    let mut input_value = use_signal(|| props.value.clone());
    let mut highlighted_index = use_signal(|| -1i32);
    #[allow(unused_mut)]
    let mut loading = use_signal(|| false);

    let id = AUTO_COMPLETE_ID.fetch_add(1, Ordering::Relaxed);
    let container_id = format!("ctrl-auto-complete-{}", id);
    let trigger_id = format!("ctrl-auto-complete-trigger-{}", id);
    let dropdown_id = format!("ctrl-auto-complete-dropdown-{}", id);

    let filtered = use_memo(move || filter_options(&props.options, &input_value()));

    let mut container_class = "ctrl-auto-complete".to_string();
    if !props.class.is_empty() {
        container_class = format!("{} {}", container_class, props.class);
    }
    if open() {
        container_class = format!("{} ctrl-auto-complete--open", container_class);
    }
    if props.disabled {
        container_class = format!("{} ctrl-auto-complete--disabled", container_class);
    }

    let input_class = format!("ctrl-auto-complete__input ctrl-auto-complete__input--{}", match props.size {
        Size::Sm => "sm", Size::Md => "md", Size::Lg => "lg", _ => "md",
    });

    let onchange = props.onchange.clone();
    let onselect = props.onselect.clone();
    let placeholder = props.placeholder.clone();
    let disabled = props.disabled;

    // 同步外部 value 到 input_value
    {
        let external_value = props.value.clone();
        let mut iv = input_value;
        use_effect(move || {
            iv.set(external_value.clone());
        });
    }

    // ── fixed 弹层：不受 overflow:hidden 裁切 + document capture scroll 跟随 + 匹配 trigger 宽度 ──
    overlay::use_fixed_panel_effect(&dropdown_id, &trigger_id, open.clone(), 4.0, true);

    // ── z-index：打开时提升容器层级 ──
    {
        let cid = container_id.clone();
        let o = open.clone();
        use_effect(move || {
            let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
            let Some(el) = doc.get_element_by_id(&cid) else { return };
            let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() else { return };
            if o() {
                let _ = html_el.style().set_property("z-index", "100");
            } else {
                let _ = html_el.style().set_property("z-index", "auto");
            }
        });
    }

    // ── click-outside ──
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

    // 过滤后结果
    let filtered_items = filtered();

    // 限制 highlighted_index 在有效范围
    {
        let mut hi = highlighted_index;
        let len = filtered_items.len() as i32;
        use_effect(move || {
            if hi() >= len { hi.set(len - 1); }
            if hi() < -1 { hi.set(-1); }
        });
    }

    // 滚动高亮选项到可见区域
    {
        let hi = highlighted_index;
        let did = dropdown_id.clone();
        use_effect(move || {
            let index = hi();
            if index < 0 { return; }
            let idx = index as usize;
            let did = did.clone();
            spawn(async move {
                let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
                let Some(dropdown) = doc.get_element_by_id(&did) else { return };
                let selector = format!(".ctrl-auto-complete__option[data-index=\"{}\"]", idx);
                let Some(el) = dropdown.query_selector(&selector).ok().flatten() else { return };
                el.scroll_into_view_with_bool(false);
            });
        });
    }

    let highlight_idx = highlighted_index;
    let query = input_value();

    rsx! {
        style { {CSS} }
        div {
            id: "{container_id}",
            class: "{container_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            // Trigger: the input wrapper
            div {
                id: "{trigger_id}",
                class: "ctrl-auto-complete__trigger",
                input {
                    class: "{input_class}",
                    value: "{input_value}",
                    placeholder: "{placeholder}",
                    disabled: disabled,
                    autocomplete: "off",
                    oninput: move |e: FormEvent| {
                        let val = e.value();
                        input_value.set(val.clone());
                        if let Some(ref handler) = onchange {
                            handler.call(val.clone());
                        }
                        open.set(true);
                        highlighted_index.set(-1);
                    },
                    onfocus: move |_| {
                        if !disabled {
                            open.set(true);
                            highlighted_index.set(-1);
                        }
                    },
                    onkeydown: move |event: KeyboardEvent| {
                        use dioxus::html::Key;
                        match event.key() {
                            Key::Enter => {
                                let idx = highlighted_index();
                                let items = filtered_items.clone();
                                if idx >= 0 && (idx as usize) < items.len() {
                                    let opt = &items[idx as usize];
                                    input_value.set(opt.label.clone());
                                    if let Some(ref handler) = onselect {
                                        handler.call(opt.clone());
                                    }
                                    if let Some(ref handler) = onchange {
                                        handler.call(opt.label.clone());
                                    }
                                    open.set(false);
                                    highlighted_index.set(-1);
                                } else {
                                    open.set(false);
                                }
                            }
                            Key::Escape => {
                                open.set(false);
                                highlighted_index.set(-1);
                                input_value.set(props.value.clone());
                            }
                            Key::ArrowDown => {
                                event.prevent_default();
                                highlighted_index.set(highlighted_index() + 1);
                            }
                            Key::ArrowUp => {
                                event.prevent_default();
                                highlighted_index.set(highlighted_index() - 1);
                            }
                            _ => {}
                        }
                    },
                }
                // 下拉箭头
                svg {
                    class: if open() { "ctrl-auto-complete__arrow ctrl-auto-complete__arrow--open" } else { "ctrl-auto-complete__arrow" },
                    width: "12", height: "12", view_box: "0 0 12 12", fill: "none",
                    path {
                        d: "M3 4.5l3 3 3-3", stroke: "var(--ctrl-text-secondary)",
                        stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
                    }
                }
                // 加载指示器
                if loading() {
                    span {
                        class: "ctrl-auto-complete__loading",
                        svg {
                            width: "14", height: "14", view_box: "0 0 24 24", fill: "none",
                            circle {
                                cx: "12", cy: "12", r: "10",
                                stroke: "var(--ctrl-primary)", stroke_width: "3",
                                stroke_dasharray: "31.4 31.4",
                                stroke_linecap: "round",
                            }
                        }
                    }
                }
                // 清除按钮
                if !input_value().is_empty() && !disabled {
                    span {
                        class: "ctrl-auto-complete__clear",
                        onclick: move |e: MouseEvent| {
                            e.stop_propagation();
                            input_value.set("".to_string());
                            if let Some(ref handler) = onchange {
                                handler.call("".to_string());
                            }
                            open.set(false);
                            highlighted_index.set(-1);
                        },
                        svg {
                            width: "10", height: "10", view_box: "0 0 10 10", fill: "none",
                            stroke: "var(--ctrl-text-secondary)",
                            stroke_width: "1.5", stroke_linecap: "round",
                            path { d: "M2 2l6 6M8 2l-6 6" }
                        }
                    }
                }
            }
            // 下拉面板
            div {
                id: "{dropdown_id}",
                class: "ctrl-auto-complete__dropdown",
                if filtered_items.is_empty() {
                    div {
                        class: "ctrl-auto-complete__option ctrl-auto-complete__option--empty",
                        "无匹配结果"
                    }
                } else {
                    { filtered_items.clone().into_iter().enumerate().map(move |(idx, opt)| {
                        let is_highlighted = highlight_idx() == idx as i32;
                        let mut opt_classes = vec!["ctrl-auto-complete__option".to_string()];
                        if is_highlighted {
                            opt_classes.push("ctrl-auto-complete__option--highlighted".into());
                        }
                        let opt_class = opt_classes.join(" ");
                        let opt_clone = opt.clone();
                        let mut iv = input_value;
                        let mut o = open.clone();
                        let mut hi = highlighted_index.clone();
                        let os = onselect.clone();
                        let oc = onchange.clone();
                        let q = query.clone();
                        let label_html = highlight_label(&opt.label, &q);
                        rsx! {
                            div {
                                key: "{opt.value}",
                                class: "{opt_class}",
                                "data-index": "{idx}",
                                onmouseenter: move |_| { hi.set(idx as i32); },
                                onmousedown: move |e: MouseEvent| {
                                    // prevent default to avoid input losing focus before click
                                    e.prevent_default();
                                },
                                onclick: move |_| {
                                    let option = opt_clone.clone();
                                    iv.set(option.label.clone());
                                    if let Some(ref handler) = os {
                                        handler.call(option.clone());
                                    }
                                    if let Some(ref handler) = oc {
                                        handler.call(option.label.clone());
                                    }
                                    o.set(false);
                                    hi.set(-1);
                                },
                                span {
                                    class: "ctrl-auto-complete__option-label",
                                    dangerous_inner_html: "{label_html}",
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
