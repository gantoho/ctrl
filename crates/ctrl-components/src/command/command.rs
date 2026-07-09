use dioxus::prelude::*;

/// 命令项
#[derive(Clone, PartialEq)]
pub struct CommandItem {
    /// 唯一标识（选中时回调返回）
    pub key: String,
    /// 显示标题
    pub label: String,
    /// 描述文字
    pub description: Option<String>,
    /// 所属分组（相同分组需在列表中连续排列）
    pub group: Option<String>,
    /// 图标（emoji 或文本）
    pub icon: Option<String>,
    /// 快捷键提示
    pub shortcut: Option<String>,
}

impl CommandItem {
    /// 创建命令项
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            description: None,
            group: None,
            icon: None,
            shortcut: None,
        }
    }

    /// 设置描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置分组
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// 设置图标
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// 设置快捷键提示
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
}

/// Command 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct CommandProps {
    /// 是否打开
    pub open: bool,

    /// 命令项列表
    pub items: Vec<CommandItem>,

    /// 搜索框占位文字
    #[props(default = "输入命令进行搜索…".to_string())]
    pub placeholder: String,

    /// 无结果时的提示文字
    #[props(default = "未找到匹配的命令".to_string())]
    pub empty_text: String,

    /// 选中命令回调（返回命令 key）
    pub onselect: EventHandler<String>,

    /// 关闭回调（点击遮罩 / Esc）
    pub onclose: EventHandler<()>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// Command 命令面板组件
///
/// 可搜索的命令菜单（⌘K 风格），支持分组展示与键盘导航（↑/↓ 选择、Enter 确认、Esc 关闭）。
#[allow(non_snake_case)]
pub fn Command(props: CommandProps) -> Element {
    const CSS: &str = include_str!("../../assets/command.css");

    let mut query = use_signal(String::new);
    let mut active = use_signal(|| 0usize);

    // 打开时重置搜索与选中项
    let open = props.open;
    use_effect(move || {
        let _ = open;
        query.set(String::new());
        active.set(0);
    });

    if !props.open {
        return rsx! {};
    }

    // 过滤
    let q = query().to_lowercase();
    let filtered: Vec<CommandItem> = props
        .items
        .iter()
        .filter(|it| {
            q.is_empty()
                || it.label.to_lowercase().contains(&q)
                || it
                    .description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&q))
                    .unwrap_or(false)
        })
        .cloned()
        .collect();

    let len = filtered.len();
    let active_idx = if len == 0 { 0 } else { active().min(len - 1) };

    let onselect = props.onselect.clone();
    let onclose = props.onclose.clone();
    let onclose_key = props.onclose.clone();
    let filtered_for_key = filtered.clone();

    let panel_class = if props.class.is_empty() {
        "ctrl-command".to_string()
    } else {
        format!("ctrl-command {}", props.class)
    };

    rsx! {
        style { {CSS} }
        div {
            class: "ctrl-command__overlay",
            onclick: move |_| onclose.call(()),
            div {
                class: "{panel_class}",
                onclick: move |e: MouseEvent| e.stop_propagation(),
                div { class: "ctrl-command__search",
                    span { class: "ctrl-command__search-icon", "⌕" }
                    input {
                        class: "ctrl-command__input",
                        r#type: "text",
                        autofocus: true,
                        placeholder: "{props.placeholder}",
                        value: "{query}",
                        oninput: move |e| {
                            query.set(e.value());
                            active.set(0);
                        },
                        onkeydown: move |e: KeyboardEvent| {
                            match e.key() {
                                Key::ArrowDown => {
                                    e.prevent_default();
                                    if len > 0 {
                                        active.set((active_idx + 1) % len);
                                    }
                                }
                                Key::ArrowUp => {
                                    e.prevent_default();
                                    if len > 0 {
                                        active.set((active_idx + len - 1) % len);
                                    }
                                }
                                Key::Enter => {
                                    e.prevent_default();
                                    if let Some(item) = filtered_for_key.get(active_idx) {
                                        onselect.call(item.key.clone());
                                        onclose_key.call(());
                                    }
                                }
                                Key::Escape => {
                                    e.prevent_default();
                                    onclose_key.call(());
                                }
                                _ => {}
                            }
                        },
                    }
                }
                div { class: "ctrl-command__list",
                    if len == 0 {
                        div { class: "ctrl-command__empty", "{props.empty_text}" }
                    }
                    for (i, item) in filtered.iter().enumerate() {
                        {
                            let show_group = i == 0 || filtered[i - 1].group != item.group;
                            let item_class = if i == active_idx {
                                "ctrl-command__item ctrl-command__item--active"
                            } else {
                                "ctrl-command__item"
                            };
                            let key = item.key.clone();
                            let onselect_item = props.onselect.clone();
                            let onclose_item = props.onclose.clone();
                            rsx! {
                                if show_group {
                                    if let Some(group) = &item.group {
                                        div { class: "ctrl-command__group", "{group}" }
                                    }
                                }
                                div {
                                    key: "{item.key}",
                                    class: "{item_class}",
                                    onmouseenter: move |_| active.set(i),
                                    onclick: move |_| {
                                        onselect_item.call(key.clone());
                                        onclose_item.call(());
                                    },
                                    if let Some(icon) = &item.icon {
                                        span { class: "ctrl-command__item-icon", "{icon}" }
                                    }
                                    div { class: "ctrl-command__item-body",
                                        span { class: "ctrl-command__item-label", "{item.label}" }
                                        if let Some(desc) = &item.description {
                                            span { class: "ctrl-command__item-desc", "{desc}" }
                                        }
                                    }
                                    if let Some(shortcut) = &item.shortcut {
                                        span { class: "ctrl-command__item-shortcut", "{shortcut}" }
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
