use std::sync::atomic::{AtomicU32, Ordering};

use dioxus::prelude::*;

/// 全局递增计数器，用于生成 duration 的 ±500ms 随机抖动（WASM 兼容）
fn next_jitter() -> i64 {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    (COUNTER.fetch_add(1, Ordering::Relaxed) % 1000) as i64 - 500
}

/// Message 组件类型
#[derive(PartialEq, Clone, Default)]
pub enum MessageType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// Message 弹出位置
#[derive(PartialEq, Clone, Default)]
pub enum MessagePlacement {
    /// 顶部居中（默认）
    #[default]
    Top,
    /// 顶部靠右
    TopRight,
    /// 顶部靠左
    TopLeft,
    /// 底部居中
    Bottom,
}

/// Message 容器组件属性
#[derive(Props, PartialEq, Clone)]
pub struct MessageContainerProps {
    /// 弹出位置
    #[props(default = MessagePlacement::default())]
    pub placement: MessagePlacement,

    /// 最大显示数量，超出部分会被隐藏（先进先出）
    #[props(default = 5)]
    pub max_count: usize,

    /// 子元素（多条 Message）
    pub children: Element,
}

/// Message 容器组件 —— 提供 fixed 定位，多条消息自动堆叠
///
/// 组件内嵌 CSS 样式（include_str!），用户无需手动加载样式文件。
#[allow(non_snake_case)]
pub fn MessageContainer(props: MessageContainerProps) -> Element {
    const CSS: &str = include_str!("../../assets/message.css");

    let placement_class = match props.placement {
        MessagePlacement::Top => "ctrl-message-container--top",
        MessagePlacement::TopRight => "ctrl-message-container--top-right",
        MessagePlacement::TopLeft => "ctrl-message-container--top-left",
        MessagePlacement::Bottom => "ctrl-message-container--bottom",
    };

    // 限制显示数量：只显示最近的 max_count 条
    let children: Vec<VNode> = props.children.into_iter().collect();
    let total = children.len();
    let skip = if total > props.max_count {
        total - props.max_count
    } else {
        0
    };

    rsx! {
        style { {CSS} }
        div {
            class: "ctrl-message-container {placement_class}",
            for child in children.into_iter().skip(skip) {
                {child}
            }
        }
    }
}

/// 单条 Message 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct MessageProps {
    /// 消息类型
    #[props(default = MessageType::default())]
    pub r#type: MessageType,

    /// 消息文字
    #[props(default = "".to_string())]
    pub content: String,

    /// 自动消失时间（毫秒），0 表示不自动消失
    #[props(default = 3000)]
    pub duration: u64,

    /// 外部关闭信号：设为 true 时触发退出动画，动画结束后 onclose 回调
    #[props(default = false)]
    pub closing: bool,

    /// 消息关闭时回调（自动消失或点击关闭按钮时触发）
    pub onclose: Option<EventHandler<()>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// 单条 Message 组件 —— 需放在 MessageContainer 内使用
#[allow(non_snake_case)]
pub fn Message(props: MessageProps) -> Element {
    // 退出动画播放完成后 调用 onclose 通知父组件移除
    let mut leaving = use_signal(|| false);

    // 关闭流程（自动消失、点击关闭、或 closing 外部信号）
    let trigger_close = {
        let mut l = leaving.clone();
        let onclose = props.onclose.clone();
        move || {
            if l() {
                return; // 防止重复触发
            }
            l.set(true);
            let oc = onclose.clone();
            spawn(async move {
                use gloo_timers::future::TimeoutFuture;
                TimeoutFuture::new(400).await; // 等待退出动画播放完毕（CSS 过渡 0.3s）
                if let Some(ref handler) = oc {
                    handler.call(());
                }
            });
        }
    };

    // 自动消失逻辑（duration > 0 时内部计时器自动触发关闭）
    if props.duration > 0 {
        let mut start_close = trigger_close.clone();
        let dur = props.duration;
        spawn(async move {
            // 添加 ±500ms 抖动，避免多条同时创建的消息在同一时刻消失
            let jitter = next_jitter();
            let actual_dur = (dur as i64 + jitter).max(500) as u32;
            use gloo_timers::future::TimeoutFuture;
            TimeoutFuture::new(actual_dur).await;
            start_close();
        });
    }

    // 外部 closing 信号：父组件标记超限时触发退出动画
    if props.closing && !leaving() {
        leaving.set(true);
        let oc = props.onclose.clone();
        spawn(async move {
            use gloo_timers::future::TimeoutFuture;
            TimeoutFuture::new(400).await;
            if let Some(ref handler) = oc {
                handler.call(());
            }
        });
    }

    if props.content.is_empty() {
        return rsx! {};
    }

    let type_class = match props.r#type {
        MessageType::Info => "ctrl-message--info",
        MessageType::Success => "ctrl-message--success",
        MessageType::Warning => "ctrl-message--warning",
        MessageType::Error => "ctrl-message--error",
    };

    let icon_char = match props.r#type {
        MessageType::Info => "ℹ",
        MessageType::Success => "✔",
        MessageType::Warning => "⚠",
        MessageType::Error => "✕",
    };

    let anim_class = if leaving() { " ctrl-message--leaving" } else { "" };

    let msg_class = if props.class.is_empty() {
        format!("ctrl-message {}{}", type_class, anim_class)
    } else {
        format!("ctrl-message {} {}{}", type_class, props.class, anim_class)
    };

    let mut close = trigger_close.clone();

    rsx! {
        div {
            class: "{msg_class}",
            span {
                class: "ctrl-message__icon",
                "{icon_char}"
            }
            span {
                class: "ctrl-message__text",
                "{props.content}"
            }
            button {
                class: "ctrl-message__close",
                onclick: move |_| close(),
                "✕"
            }
        }
    }
}
