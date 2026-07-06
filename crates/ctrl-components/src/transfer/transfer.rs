use dioxus::prelude::*;

/// Transfer 数据项
#[derive(PartialEq, Clone)]
pub struct TransferItem {
    /// 唯一标识
    pub key: String,
    /// 显示文本
    pub label: String,
    /// 是否禁用
    pub disabled: bool,
}

/// Transfer 穿梭框组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TransferProps {
    /// 数据源（所有可选项）
    #[props(default = Vec::new())]
    pub data_source: Vec<TransferItem>,

    /// 右侧面板已选中的 key 列表
    #[props(default = Vec::new())]
    pub target_keys: Vec<String>,

    /// 左右面板标题
    #[props(default = None)]
    pub titles: Option<(String, String)>,

    /// 是否显示搜索框
    #[props(default = false)]
    pub show_search: bool,

    /// 搜索框占位文本
    #[props(default = "Search".to_string())]
    pub search_placeholder: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// target_keys 变化回调
    pub onchange: Option<EventHandler<Vec<String>>>,
}

/// 渲染一个列表项（checkbox + label）
fn render_item(
    item: &TransferItem,
    checked: bool,
    disabled: bool,
    on_toggle: EventHandler<()>,
) -> Element {
    let item_key = item.key.clone();
    let item_label = item.label.clone();
    let item_disabled = item.disabled || disabled;

    let mut item_classes = vec!["ctrl-transfer__item".to_string()];
    if item_disabled {
        item_classes.push("ctrl-transfer__item--disabled".into());
    }

    let mut box_classes = vec!["ctrl-transfer__checkbox".to_string()];
    if checked {
        box_classes.push("ctrl-transfer__checkbox--checked".into());
    }
    if item_disabled {
        box_classes.push("ctrl-transfer__checkbox--disabled".into());
    }

    let box_class = box_classes.join(" ");
    let item_class = item_classes.join(" ");

    let on_toggle_clone = on_toggle.clone();
    let item_disabled_for_click = item_disabled;

    rsx! {
        div {
            class: "{item_class}",
            key: "{item_key}",
            div {
                class: "{box_class}",
                onclick: move |evt| {
                    evt.stop_propagation();
                    if !item_disabled_for_click {
                        on_toggle_clone.call(());
                    }
                },
                if checked {
                    svg {
                        view_box: "0 0 16 16",
                        width: "12",
                        height: "12",
                        fill: "none",
                        stroke: "var(--ctrl-text-on-primary)",
                        stroke_width: "2",
                        path { d: "M3 8l3.5 3.5L13 5" }
                    }
                }
            }
            span { class: "ctrl-transfer__item-label", "{item_label}" }
        }
    }
}

/// 渲染"全选"复选框行
fn render_select_all(
    checked: bool,
    indeterminate: bool,
    label: &str,
    on_toggle: EventHandler<()>,
) -> Element {
    let mut box_classes = vec!["ctrl-transfer__checkbox".to_string()];
    if checked || indeterminate {
        box_classes.push("ctrl-transfer__checkbox--checked".into());
    }

    let box_class = box_classes.join(" ");

    let icon = if indeterminate {
        rsx! {
            svg {
                view_box: "0 0 16 16",
                width: "12",
                height: "12",
                fill: "var(--ctrl-text-on-primary)",
                rect { x: "3", y: "7", width: "10", height: "2", rx: "1" }
            }
        }
    } else if checked {
        rsx! {
            svg {
                view_box: "0 0 16 16",
                width: "12",
                height: "12",
                fill: "none",
                stroke: "var(--ctrl-text-on-primary)",
                stroke_width: "2",
                path { d: "M3 8l3.5 3.5L13 5" }
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        div {
            class: "ctrl-transfer__select-all",
            onclick: move |_| { on_toggle.call(()); },
            div {
                class: "{box_class}",
                {icon}
            }
            span { class: "ctrl-transfer__select-all-label", "{label}" }
        }
    }
}

/// 渲染搜索框
fn render_search(
    value: &str,
    placeholder: &str,
    oninput: EventHandler<String>,
) -> Element {
    let placeholder = placeholder.to_string();
    let value = value.to_string();

    rsx! {
        div {
            class: "ctrl-transfer__search",
            input {
                class: "ctrl-transfer__search-input",
                r#type: "text",
                value: "{value}",
                placeholder: "{placeholder}",
                oninput: move |evt: FormEvent| {
                    oninput.call(evt.value().clone());
                },
            }
            svg {
                class: "ctrl-transfer__search-icon",
                view_box: "0 0 24 24",
                width: "14",
                height: "14",
                fill: "none",
                stroke: "var(--ctrl-text-secondary, #999)",
                stroke_width: "2",
                path { d: "M11 19a8 8 0 100-16 8 8 0 000 16zM21 21l-4.35-4.35" }
            }
        }
    }
}

/// Transfer 穿梭框组件
#[allow(non_snake_case)]
pub fn Transfer(props: TransferProps) -> Element {
    const CSS: &str = include_str!("../../assets/transfer.css");

    let mut left_search = use_signal(|| String::new());
    let mut right_search = use_signal(|| String::new());
    let mut left_checked_keys = use_signal(|| Vec::<String>::new());
    let mut right_checked_keys = use_signal(|| Vec::<String>::new());

    // Self-managing target_keys: init from props, then update internally.
     // Sync from props when they change (supports controlled mode).
     let mut internal_target_keys = use_signal(|| props.target_keys.clone());
     {
         let ext = props.target_keys.clone();
         use_effect(move || {
             internal_target_keys.set(ext.clone());
         });
     }

    let transfer_class = {
        let mut c = String::from("ctrl-transfer");
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    // 计算左侧（source）和右侧（target）数据
    let source_items: Vec<TransferItem> = props
        .data_source
        .iter()
        .filter(|item| !internal_target_keys().contains(&item.key))
        .cloned()
        .collect();

    let target_items: Vec<TransferItem> = props
        .data_source
        .iter()
        .filter(|item| internal_target_keys().contains(&item.key))
        .cloned()
        .collect();

    // 根据搜索过滤左侧数据
    let filtered_source: Vec<TransferItem> = if left_search().is_empty() {
        source_items.clone()
    } else {
        let q = left_search().to_lowercase();
        source_items
            .iter()
            .filter(|item| item.label.to_lowercase().contains(&q))
            .cloned()
            .collect()
    };

    // 根据搜索过滤右侧数据
    let filtered_target: Vec<TransferItem> = if right_search().is_empty() {
        target_items.clone()
    } else {
        let q = right_search().to_lowercase();
        target_items
            .iter()
            .filter(|item| item.label.to_lowercase().contains(&q))
            .cloned()
            .collect()
    };

    // 左侧可选择（非禁用）的项数量
    let left_selectable: Vec<&TransferItem> = filtered_source.iter().filter(|i| !i.disabled).collect();
    let left_selectable_keys: Vec<String> = left_selectable.iter().map(|i| i.key.clone()).collect();

    // 右侧可选择（非禁用）的项数量
    let right_selectable: Vec<&TransferItem> = filtered_target.iter().filter(|i| !i.disabled).collect();
    let right_selectable_keys: Vec<String> = right_selectable.iter().map(|i| i.key.clone()).collect();

    // 左侧全选状态
    let left_checked_count = left_checked_keys().iter().filter(|k| left_selectable_keys.contains(k)).count();
    let left_all_checked = !left_selectable.is_empty() && left_checked_count == left_selectable.len();
    let left_indeterminate = left_checked_count > 0 && left_checked_count < left_selectable.len();

    // 右侧全选状态
    let right_checked_count = right_checked_keys().iter().filter(|k| right_selectable_keys.contains(k)).count();
    let right_all_checked = !right_selectable.is_empty() && right_checked_count == right_selectable.len();
    let right_indeterminate = right_checked_count > 0 && right_checked_count < right_selectable.len();

    // 统计数据
    let left_stat = format!("{}/{}", left_checked_keys().len(), source_items.len());
    let right_stat = format!("{}/{}", right_checked_keys().len(), target_items.len());

    let (left_title, right_title) = match &props.titles {
        Some((l, r)) => (l.clone(), r.clone()),
        None => ("Source".to_string(), "Target".to_string()),
    };

    // transfer right
    let on_transfer_right = {
        let onchange = props.onchange.clone();
        let mut tk = internal_target_keys;
        move |_| {
            let keys_to_move: Vec<String> = left_checked_keys()
                .iter()
                .filter(|k| source_items.iter().any(|si| si.key == **k && !si.disabled))
                .cloned()
                .collect();
            if keys_to_move.is_empty() {
                return;
            }
            let current = tk();
            let mut new_target: Vec<String> = current.clone();
            for k in &keys_to_move {
                if !new_target.contains(k) {
                    new_target.push(k.clone());
                }
            }
            left_checked_keys.set(Vec::new());
            tk.set(new_target.clone());
            if let Some(ref handler) = onchange {
                handler.call(new_target);
            }
        }
    };

    // transfer left
    let on_transfer_left = {
        let onchange = props.onchange.clone();
        let mut tk = internal_target_keys;
        move |_| {
            let keys_to_move: Vec<String> = right_checked_keys()
                .iter()
                .filter(|k| target_items.iter().any(|ti| ti.key == **k && !ti.disabled))
                .cloned()
                .collect();
            if keys_to_move.is_empty() {
                return;
            }
            let current = tk();
            let new_target: Vec<String> = current
                .iter()
                .filter(|k| !keys_to_move.contains(k))
                .cloned()
                .collect();
            right_checked_keys.set(Vec::new());
            tk.set(new_target.clone());
            if let Some(ref handler) = onchange {
                handler.call(new_target);
            }
        }
    };

    // 左侧全选点击
    let left_select_all_onclick = EventHandler::new({
        let mut left_checked_keys = left_checked_keys.clone();
        let left_selectable_keys = left_selectable_keys.clone();
        move |_| {
            if left_all_checked || left_indeterminate {
                left_checked_keys.set(Vec::new());
            } else {
                left_checked_keys.set(left_selectable_keys.clone());
            }
        }
    });

    // 右侧全选点击
    let right_select_all_onclick = EventHandler::new({
        let mut right_checked_keys = right_checked_keys.clone();
        let right_selectable_keys = right_selectable_keys.clone();
        move |_| {
            if right_all_checked || right_indeterminate {
                right_checked_keys.set(Vec::new());
            } else {
                right_checked_keys.set(right_selectable_keys.clone());
            }
        }
    });

    // 左侧单项切换
    let left_toggle_factory = |key: String| {
        let mut left_checked_keys = left_checked_keys.clone();
        move |_| {
            let mut current = left_checked_keys();
            if current.contains(&key) {
                current.retain(|k| k != &key);
            } else {
                current.push(key.clone());
            }
            left_checked_keys.set(current);
        }
    };

    // 右侧单项切换
    let right_toggle_factory = |key: String| {
        let mut right_checked_keys = right_checked_keys.clone();
        move |_| {
            let mut current = right_checked_keys();
            if current.contains(&key) {
                current.retain(|k| k != &key);
            } else {
                current.push(key.clone());
            }
            right_checked_keys.set(current);
        }
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{transfer_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },

            // 左侧面板
            div {
                class: "ctrl-transfer__panel",
                div {
                    class: "ctrl-transfer__panel-header",
                    span { class: "ctrl-transfer__panel-title", "{left_title}" }
                    span { class: "ctrl-transfer__panel-stat", "{left_stat}" }
                }
                if props.show_search {
                    {
                        let search_placeholder = props.search_placeholder.clone();
                        render_search(
                            &left_search(),
                            &search_placeholder,
                            EventHandler::new(move |v: String| {
                                left_search.set(v);
                            }),
                        )
                    }
                }
                div {
                    class: "ctrl-transfer__panel-body",
                    {
                        let left_all = left_all_checked;
                        let left_indet = left_indeterminate;
                        let on_click = left_select_all_onclick.clone();
                        render_select_all(
                            left_all,
                            left_indet,
                            "Select All",
                            on_click,
                        )
                    }
                    div {
                        class: "ctrl-transfer__list",
                        if filtered_source.is_empty() {
                            div {
                                class: "ctrl-transfer__empty",
                                "No data"
                            }
                        } else {
                            for item in &filtered_source {
                                {
                                    let checked = left_checked_keys().contains(&item.key);
                                    let key = item.key.clone();
                                    let handler = EventHandler::new(left_toggle_factory(key));
                                    render_item(item, checked, false, handler)
                                }
                            }
                        }
                    }
                }
            }

            // 中间操作按钮区
            div {
                class: "ctrl-transfer__actions",
                button {
                    class: "ctrl-transfer__action-btn",
                    r#type: "button",
                    disabled: left_checked_keys().is_empty(),
                    onclick: on_transfer_right,
                    svg {
                        view_box: "0 0 24 24",
                        width: "16",
                        height: "16",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M9 18l6-6-6-6" }
                    }
                }
                button {
                    class: "ctrl-transfer__action-btn",
                    r#type: "button",
                    disabled: right_checked_keys().is_empty(),
                    onclick: on_transfer_left,
                    svg {
                        view_box: "0 0 24 24",
                        width: "16",
                        height: "16",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M15 18l-6-6 6-6" }
                    }
                }
            }

            // 右侧面板
            div {
                class: "ctrl-transfer__panel",
                div {
                    class: "ctrl-transfer__panel-header",
                    span { class: "ctrl-transfer__panel-title", "{right_title}" }
                    span { class: "ctrl-transfer__panel-stat", "{right_stat}" }
                }
                if props.show_search {
                    {
                        let search_placeholder = props.search_placeholder.clone();
                        render_search(
                            &right_search(),
                            &search_placeholder,
                            EventHandler::new(move |v: String| {
                                right_search.set(v);
                            }),
                        )
                    }
                }
                div {
                    class: "ctrl-transfer__panel-body",
                    {
                        let right_all = right_all_checked;
                        let right_indet = right_indeterminate;
                        let on_click = right_select_all_onclick.clone();
                        render_select_all(
                            right_all,
                            right_indet,
                            "Select All",
                            on_click,
                        )
                    }
                    div {
                        class: "ctrl-transfer__list",
                        if filtered_target.is_empty() {
                            div {
                                class: "ctrl-transfer__empty",
                                "No data"
                            }
                        } else {
                            for item in &filtered_target {
                                {
                                    let checked = right_checked_keys().contains(&item.key);
                                    let key = item.key.clone();
                                    let handler = EventHandler::new(right_toggle_factory(key));
                                    render_item(item, checked, false, handler)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
