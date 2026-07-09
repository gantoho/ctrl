//! 快捷键系统
//!
//! 提供一套完整的键盘快捷键能力：
//! - [`use_shortcut`]：独立注册单个快捷键（无需 Provider）。
//! - [`ShortcutProvider`]：快捷键系统上下文，集中调度并维护注册表。
//! - [`use_register_shortcut`]：向系统注册带描述的快捷键（用于生成快捷键帮助面板）。
//! - [`use_shortcut_list`]：读取当前已注册的所有快捷键（响应式）。
//! - [`shortcut_keys`]：将组合键解析为跨平台的展示用按键序列。
//!
//! # 组合键语法
//! 以 `+` 连接修饰键与主键，忽略大小写。修饰键：`ctrl`/`control`、`alt`/`option`、
//! `shift`、`meta`/`cmd`/`command`/`super`/`win`、`mod`（跨平台：macOS 为 ⌘，其他为 Ctrl）。
//! 主键取 `KeyboardEvent.key`（如 `k`、`enter`、`escape`、`/`、`arrowup`）。

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// ──────────────────────────────────────────────
// 组合键解析与匹配
// ──────────────────────────────────────────────

/// 快捷键组合的解析结果
#[derive(Clone, PartialEq)]
struct ShortcutSpec {
    ctrl: bool,
    alt: bool,
    shift: bool,
    meta: bool,
    /// 跨平台修饰键（meta 或 ctrl 任一）
    mod_: bool,
    /// 主键（已归一化为小写）
    key: String,
}

/// 归一化主键名，统一常见别名
fn normalize_key(key: &str) -> String {
    match key {
        "esc" => "escape",
        "space" | "spacebar" => " ",
        "up" => "arrowup",
        "down" => "arrowdown",
        "left" => "arrowleft",
        "right" => "arrowright",
        "return" => "enter",
        other => other,
    }
    .to_string()
}

/// 解析组合键字符串
fn parse(combo: &str) -> ShortcutSpec {
    let mut spec = ShortcutSpec {
        ctrl: false,
        alt: false,
        shift: false,
        meta: false,
        mod_: false,
        key: String::new(),
    };
    for part in combo.split('+') {
        let p = part.trim().to_lowercase();
        match p.as_str() {
            "ctrl" | "control" => spec.ctrl = true,
            "alt" | "option" => spec.alt = true,
            "shift" => spec.shift = true,
            "meta" | "cmd" | "command" | "super" | "win" => spec.meta = true,
            "mod" => spec.mod_ = true,
            "" => {}
            other => spec.key = normalize_key(other),
        }
    }
    spec
}

/// 判断事件是否匹配组合键
fn matches(spec: &ShortcutSpec, e: &web_sys::KeyboardEvent) -> bool {
    if e.key().to_lowercase() != spec.key {
        return false;
    }
    if e.shift_key() != spec.shift {
        return false;
    }
    if e.alt_key() != spec.alt {
        return false;
    }
    if spec.mod_ {
        e.meta_key() || e.ctrl_key()
    } else {
        e.ctrl_key() == spec.ctrl && e.meta_key() == spec.meta
    }
}

/// 是否为 macOS 平台（用于展示时选择 ⌘/⌥ 等符号）
fn is_mac() -> bool {
    web_sys::window()
        .and_then(|w| w.navigator().platform().ok())
        .map(|p| p.to_lowercase().contains("mac"))
        .unwrap_or(false)
}

/// 主键的展示形式
fn display_key(key: &str) -> String {
    match key {
        " " => "Space".to_string(),
        "escape" => "Esc".to_string(),
        "enter" => "Enter".to_string(),
        "tab" => "Tab".to_string(),
        "arrowup" => "↑".to_string(),
        "arrowdown" => "↓".to_string(),
        "arrowleft" => "←".to_string(),
        "arrowright" => "→".to_string(),
        k if k.chars().count() == 1 => k.to_uppercase(),
        k => {
            let mut chars = k.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        }
    }
}

/// 将组合键解析为跨平台的展示用按键序列。
///
/// 例如在 macOS 上 `"mod+k"` → `["⌘", "K"]`，在其他平台 → `["Ctrl", "K"]`。
pub fn shortcut_keys(combo: &str) -> Vec<String> {
    let spec = parse(combo);
    let mac = is_mac();
    let mut keys = Vec::new();
    if spec.mod_ {
        keys.push(if mac { "⌘".to_string() } else { "Ctrl".to_string() });
    }
    if spec.ctrl {
        keys.push("Ctrl".to_string());
    }
    if spec.alt {
        keys.push(if mac { "⌥".to_string() } else { "Alt".to_string() });
    }
    if spec.shift {
        keys.push(if mac { "⇧".to_string() } else { "Shift".to_string() });
    }
    if spec.meta {
        keys.push(if mac { "⌘".to_string() } else { "Win".to_string() });
    }
    if !spec.key.is_empty() {
        keys.push(display_key(&spec.key));
    }
    keys
}

// ──────────────────────────────────────────────
// 独立单快捷键 Hook
// ──────────────────────────────────────────────

/// 注册一个全局键盘快捷键（独立使用，无需 [`ShortcutProvider`]）。
///
/// 组件挂载时于 document 上监听 `keydown`，命中组合键时调用 `handler`
/// 并阻止浏览器默认行为；组件卸载时自动移除监听。
pub fn use_shortcut(combo: impl Into<String>, handler: impl FnMut() + 'static) {
    let spec = parse(&combo.into());
    let handler: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(handler));
    let mut cb_store: Signal<Option<Closure<dyn FnMut(web_sys::KeyboardEvent)>>> =
        use_signal(|| None);

    use_effect(move || {
        if cb_store.peek().is_some() {
            return;
        }
        let spec = spec.clone();
        let handler = handler.clone();
        let cb = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if matches(&spec, &e) {
                e.prevent_default();
                (handler.borrow_mut())();
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb_store.set(Some(cb));
    });

    use_drop(move || {
        if let Some(cb) = cb_store.take() {
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                let _ =
                    doc.remove_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            }
        }
    });
}

// ──────────────────────────────────────────────
// 快捷键系统：Provider + 注册表
// ──────────────────────────────────────────────

/// 已注册快捷键的公开信息（用于生成帮助面板）
#[derive(Clone, PartialEq)]
pub struct ShortcutInfo {
    /// 原始组合键字符串
    pub combo: String,
    /// 跨平台展示按键序列
    pub keys: Vec<String>,
    /// 功能描述
    pub description: String,
    /// 所属分组
    pub group: Option<String>,
}

type Handler = Rc<RefCell<dyn FnMut()>>;

#[derive(Clone)]
struct Entry {
    id: u64,
    spec: ShortcutSpec,
    info: ShortcutInfo,
    handler: Handler,
}

/// 快捷键注册表上下文
#[derive(Clone, Copy)]
struct ShortcutRegistry {
    entries: Signal<Vec<Entry>>,
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// 快捷键系统上下文组件。
///
/// 在内部安装单个 document `keydown` 监听器，集中调度所有通过
/// [`use_register_shortcut`] 注册的快捷键，并维护可查询的注册表。
#[component]
#[allow(non_snake_case)]
pub fn ShortcutProvider(children: Element) -> Element {
    let entries = use_signal(Vec::<Entry>::new);
    use_context_provider(|| ShortcutRegistry { entries });

    let mut cb_store: Signal<Option<Closure<dyn FnMut(web_sys::KeyboardEvent)>>> =
        use_signal(|| None);

    use_effect(move || {
        if cb_store.peek().is_some() {
            return;
        }
        let cb = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            // 先在只读借用内找到匹配的 handler 并克隆，释放借用后再调用，
            // 避免 handler 内注册/注销快捷键时产生 RefCell 借用冲突
            let matched = {
                let list = entries.peek();
                list.iter()
                    .find(|entry| matches(&entry.spec, &e))
                    .map(|entry| entry.handler.clone())
            };
            if let Some(handler) = matched {
                e.prevent_default();
                (handler.borrow_mut())();
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb_store.set(Some(cb));
    });

    use_drop(move || {
        if let Some(cb) = cb_store.take() {
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                let _ =
                    doc.remove_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            }
        }
    });

    rsx! { {children} }
}

/// 向快捷键系统注册一个带描述的快捷键（需在 [`ShortcutProvider`] 内使用）。
///
/// 描述会进入注册表，可通过 [`use_shortcut_list`] 读取以生成帮助面板。
/// 组件卸载时自动从注册表移除。
pub fn use_register_shortcut(
    combo: impl Into<String>,
    description: impl Into<String>,
    handler: impl FnMut() + 'static,
) {
    use_register_shortcut_grouped(combo, description, None::<String>, handler);
}

/// 与 [`use_register_shortcut`] 相同，但可指定分组。
pub fn use_register_shortcut_grouped(
    combo: impl Into<String>,
    description: impl Into<String>,
    group: Option<impl Into<String>>,
    handler: impl FnMut() + 'static,
) {
    let registry = use_context::<ShortcutRegistry>();
    let combo = combo.into();
    let spec = parse(&combo);
    let info = ShortcutInfo {
        keys: shortcut_keys(&combo),
        combo: combo.clone(),
        description: description.into(),
        group: group.map(|g| g.into()),
    };
    let handler: Handler = Rc::new(RefCell::new(handler));
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));

    use_effect(move || {
        let mut entries = registry.entries;
        if entries.peek().iter().any(|e| e.id == id) {
            return;
        }
        entries.write().push(Entry {
            id,
            spec: spec.clone(),
            info: info.clone(),
            handler: handler.clone(),
        });
    });

    use_drop(move || {
        let mut entries = registry.entries;
        entries.write().retain(|e| e.id != id);
    });
}

/// 读取当前已注册的所有快捷键信息（响应式，需在 [`ShortcutProvider`] 内使用）。
pub fn use_shortcut_list() -> Vec<ShortcutInfo> {
    let registry = use_context::<ShortcutRegistry>();
    let list = registry.entries.read();
    list.iter().map(|e| e.info.clone()).collect()
}
