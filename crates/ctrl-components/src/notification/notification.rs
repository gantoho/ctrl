use dioxus::prelude::*;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};

// ═══════════════════════════════════════════════════════════
// 类型定义
// ═══════════════════════════════════════════════════════════

/// 通知类型
#[derive(PartialEq, Clone, Default)]
pub enum NotificationType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// 弹出位置
#[derive(PartialEq, Clone, Default)]
pub enum NotificationPlacement {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

/// 通知条目（内部结构）
#[derive(Clone, PartialEq)]
struct NotificationEntry {
    id: u32,
    r#type: NotificationType,
    title: String,
    content: String,
    duration: u64,
    // 出场动画中的条目标记为 leaving
    leaving: bool,
}

// ═══════════════════════════════════════════════════════════
// Context API
// ═══════════════════════════════════════════════════════════

/// 通知操作 API，通过 useNotification() 获取
#[derive(Clone, Copy)]
pub struct NotificationAPI {
    entries: Signal<VecDeque<NotificationEntry>>,
    next_id: Signal<u32>,
}

/// 获取通知 API
pub fn use_notification() -> NotificationAPI {
    use_context::<NotificationAPI>()
}

impl NotificationAPI {
    /// 打开一条通知
    pub fn open(&mut self, r#type: NotificationType, title: String, content: String) {
        let mut entries = self.entries.write();
        let mut next_id = self.next_id.write();
        let id = *next_id;
        *next_id += 1;

        entries.push_back(NotificationEntry {
            id,
            r#type,
            title,
            content,
            duration: 4500,
            leaving: false,
        });
    }

    /// 打开一条通知（自定义 duration）
    pub fn open_with_duration(
        &mut self,
        r#type: NotificationType,
        title: String,
        content: String,
        duration: u64,
    ) {
        let mut entries = self.entries.write();
        let mut next_id = self.next_id.write();
        let id = *next_id;
        *next_id += 1;

        entries.push_back(NotificationEntry {
            id,
            r#type,
            title,
            content,
            duration,
            leaving: false,
        });
    }

    /// 移除指定通知
    pub fn remove(&mut self, id: u32) {
        let mut entries = self.entries.write();
        if let Some(entry) = entries.iter_mut().find(|e| e.id == id) {
            entry.leaving = true;
        }
    }

    /// 快捷方法：info
    pub fn info(&mut self, title: String, content: String) {
        self.open(NotificationType::Info, title, content);
    }

    /// 快捷方法：success
    pub fn success(&mut self, title: String, content: String) {
        self.open(NotificationType::Success, title, content);
    }

    /// 快捷方法：warning
    pub fn warning(&mut self, title: String, content: String) {
        self.open(NotificationType::Warning, title, content);
    }

    /// 快捷方法：error
    pub fn error(&mut self, title: String, content: String) {
        self.open(NotificationType::Error, title, content);
    }
}

// ═══════════════════════════════════════════════════════════
// NotificationProvider
// ═══════════════════════════════════════════════════════════

/// NotificationProvider 属性
#[derive(Props, PartialEq, Clone)]
pub struct NotificationProviderProps {
    /// 弹出位置
    #[props(default = NotificationPlacement::default())]
    pub placement: NotificationPlacement,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 最大显示数量
    #[props(default = 5)]
    pub max_count: usize,

    /// 子元素
    pub children: Element,
}

/// 通知容器组件，放在应用根节点
#[allow(non_snake_case)]
pub fn NotificationProvider(props: NotificationProviderProps) -> Element {
    let entries = use_signal(|| VecDeque::<NotificationEntry>::new());
    let next_id = use_signal(|| 0u32);

    // 提供 context
    let api = NotificationAPI {
        entries: entries.clone(),
        next_id,
    };
    use_context_provider(|| api);

    // 定时清理已离开的条目
    {
        let mut entries_clean = entries.clone();
        use_future(move || async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(500).await;
                let mut entries = entries_clean.write();
                // 移除所有 leaving 的条目
                entries.retain(|e| !e.leaving);
            }
        });
    }

    let placement_class = match props.placement {
        NotificationPlacement::TopRight => "ctrl-notification-provider--top-right",
        NotificationPlacement::TopLeft => "ctrl-notification-provider--top-left",
        NotificationPlacement::BottomRight => "ctrl-notification-provider--bottom-right",
        NotificationPlacement::BottomLeft => "ctrl-notification-provider--bottom-left",
    };

    let provider_class = if props.class.is_empty() {
        format!("ctrl-notification-provider {}", placement_class)
    } else {
        format!(
            "ctrl-notification-provider {} {}",
            placement_class, props.class
        )
    };

    // 只显示最近的 max_count 条
    let display_entries: Vec<NotificationEntry> = {
        let all = entries.read();
        let total = all.len();
        let skip = if total > props.max_count {
            total - props.max_count
        } else {
            0
        };
        all.iter().skip(skip).cloned().collect()
    };

    rsx! {
        // 组件内嵌 CSS 样式，用户无需手动加载
        style { {include_str!("../../assets/notification.css")} }
        div {
            class: "{provider_class}",
            for entry in display_entries {
                NotificationItem {
                    key: "{entry.id}",
                    entry: entry.clone(),
                    entries: entries.clone(),
                }
            }
        }
        {props.children}
    }
}

// ═══════════════════════════════════════════════════════════
// NotificationItem（内部卡片组件）
// ═══════════════════════════════════════════════════════════

#[derive(Props, PartialEq, Clone)]
struct NotificationItemProps {
    entry: NotificationEntry,
    entries: Signal<VecDeque<NotificationEntry>>,
}

#[allow(non_snake_case)]
fn NotificationItem(props: NotificationItemProps) -> Element {
    let entry = props.entry;
    let mut entries = props.entries;
    let leaving = use_signal(|| false);

    // 自动关闭计时器
    if entry.duration > 0 {
        let dur = entry.duration;
        let id = entry.id;
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(dur as u32).await;
            let mut entries = entries.write();
            if let Some(e) = entries.iter_mut().find(|e| e.id == id) {
                e.leaving = true;
            }
        });
    }

    // 监听 entry.leaving 变化
    {
        let entry_id = entry.id;
        let mut l = leaving.clone();
        use_effect(move || {
            let entries_read = entries.read();
            if let Some(e) = entries_read.iter().find(|e| e.id == entry_id) {
                if e.leaving && !l() {
                    l.set(true);
                }
            }
        });
    }

    let type_class = match entry.r#type {
        NotificationType::Info => "ctrl-notification--info",
        NotificationType::Success => "ctrl-notification--success",
        NotificationType::Warning => "ctrl-notification--warning",
        NotificationType::Error => "ctrl-notification--error",
    };

    let icon_char = match entry.r#type {
        NotificationType::Info => "ℹ",
        NotificationType::Success => "✔",
        NotificationType::Warning => "⚠",
        NotificationType::Error => "✕",
    };

    let anim_class = if leaving() {
        " ctrl-notification--leaving"
    } else {
        " ctrl-notification--entering"
    };

    let has_title = !entry.title.is_empty();
    let entry_id = entry.id;

    let mut close = {
        let mut entries = entries.clone();
        move || {
            let mut entries = entries.write();
            if let Some(e) = entries.iter_mut().find(|e| e.id == entry_id) {
                e.leaving = true;
            }
        }
    };

    rsx! {
        div {
            class: "ctrl-notification {type_class}{anim_class}",
            span { class: "ctrl-notification__icon", "{icon_char}" }
            div {
                class: "ctrl-notification__body",
                if has_title {
                    div { class: "ctrl-notification__title", "{entry.title}" }
                }
                div { class: "ctrl-notification__content", "{entry.content}" }
            }
            button {
                class: "ctrl-notification__close",
                onclick: move |_| close(),
                "✕"
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════
// 旧版 Notification（保留向后兼容）
// ═══════════════════════════════════════════════════════════

fn next_jitter() -> i64 {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    (COUNTER.fetch_add(1, Ordering::Relaxed) % 1000) as i64 - 500
}

/// 单条通知属性（旧版兼容）
#[derive(Props, PartialEq, Clone)]
pub struct NotificationProps {
    #[props(default = NotificationType::default())]
    pub r#type: NotificationType,

    #[props(default = "".to_string())]
    pub title: String,

    #[props(default = "".to_string())]
    pub content: String,

    #[props(default = 4500)]
    pub duration: u64,

    #[props(default = false)]
    pub closing: bool,

    pub onclose: Option<EventHandler<()>>,

    #[props(default = "".to_string())]
    pub class: String,
}

/// 单条通知组件（旧版兼容，建议使用 NotificationProvider + useNotification）
#[allow(non_snake_case)]
pub fn Notification(props: NotificationProps) -> Element {
    let mut leaving = use_signal(|| false);

    let trigger_close = {
        let mut l = leaving.clone();
        let onclose = props.onclose.clone();
        move || {
            if l() {
                return;
            }
            l.set(true);
            let oc = onclose.clone();
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(400).await;
                if let Some(ref handler) = oc {
                    handler.call(());
                }
            });
        }
    };

    if props.duration > 0 {
        let mut start_close = trigger_close.clone();
        let dur = props.duration;
        spawn(async move {
            let jitter = next_jitter();
            let actual_dur = (dur as i64 + jitter).max(500) as u32;
            gloo_timers::future::TimeoutFuture::new(actual_dur).await;
            start_close();
        });
    }

    if props.closing && !leaving() {
        leaving.set(true);
        let oc = props.onclose.clone();
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(400).await;
            if let Some(ref handler) = oc {
                handler.call(());
            }
        });
    }

    let type_class = match props.r#type {
        NotificationType::Info => "ctrl-notification--info",
        NotificationType::Success => "ctrl-notification--success",
        NotificationType::Warning => "ctrl-notification--warning",
        NotificationType::Error => "ctrl-notification--error",
    };

    let icon_char = match props.r#type {
        NotificationType::Info => "ℹ",
        NotificationType::Success => "✔",
        NotificationType::Warning => "⚠",
        NotificationType::Error => "✕",
    };

    let anim_class = if leaving() {
        " ctrl-notification--leaving"
    } else {
        " ctrl-notification--entering"
    };

    let notif_class = if props.class.is_empty() {
        format!("ctrl-notification {}{}", type_class, anim_class)
    } else {
        format!(
            "ctrl-notification {} {}{}",
            type_class, props.class, anim_class
        )
    };

    let mut close = trigger_close.clone();
    let has_title = !props.title.is_empty();

    rsx! {
        div {
            class: "{notif_class}",
            span { class: "ctrl-notification__icon", "{icon_char}" }
            div {
                class: "ctrl-notification__body",
                if has_title {
                    div { class: "ctrl-notification__title", "{props.title}" }
                }
                div { class: "ctrl-notification__content", "{props.content}" }
            }
            button {
                class: "ctrl-notification__close",
                onclick: move |_| close(),
                "✕"
            }
        }
    }
}