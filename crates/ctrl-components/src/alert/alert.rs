use std::sync::atomic::{AtomicU32, Ordering};

use dioxus::prelude::*;

/// 全局递增计数器，用于生成 duration 的 ±500ms 随机抖动（WASM 兼容）
fn next_jitter() -> i64 {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    (COUNTER.fetch_add(1, Ordering::Relaxed) % 1000) as i64 - 500
}

/// Alert 组件类型
#[derive(PartialEq, Clone, Default)]
pub enum AlertType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// Alert 显示模式
#[derive(PartialEq, Clone, Default)]
pub enum AlertMode {
    /// 内联模式（默认，嵌入文档流）
    #[default]
    Inline,
    /// 全局顶部横幅（fixed 定位，浮在页面最上方）
    Banner,
}

/// Alert Banner 容器组件属性
#[derive(Props, PartialEq, Clone)]
pub struct AlertBannerContainerProps {
    /// 最大显示数量，超出部分会被隐藏（先进先出）
    #[props(default = 5)]
    pub max_count: usize,

    /// 子元素（多条 Banner Alert）
    pub children: Element,
}

/// Alert Banner 容器 —— 提供 fixed 定位，多条横幅自动垂直堆叠
///
/// 组件内嵌 CSS 样式（include_str!），用户无需手动加载样式文件。
#[allow(non_snake_case)]
pub fn AlertBannerContainer(props: AlertBannerContainerProps) -> Element {
    const CSS: &str = include_str!("../../assets/alert.css");

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
            class: "ctrl-alert-banner-container",
            for child in children.into_iter().skip(skip) {
                {child}
            }
        }
    }
}

/// Alert 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct AlertProps {
    /// 提示类型
    #[props(default = AlertType::default())]
    pub r#type: AlertType,

    /// 标题
    #[props(default = "".to_string())]
    pub title: String,

    /// 描述内容
    #[props(default = "".to_string())]
    pub description: String,

    /// 是否可关闭
    #[props(default = false)]
    pub closable: bool,

    /// 是否显示图标
    #[props(default = true)]
    pub show_icon: bool,

    /// 显示模式：Inline（内联）/ Banner（全局横幅）
    #[props(default = AlertMode::default())]
    pub mode: AlertMode,

    /// 自动关闭时间（毫秒），0 表示不自动关闭（仅 Banner 模式有效）
    #[props(default = 0)]
    pub duration: u64,

    /// 外部关闭信号：设为 true 时触发退出动画（仅 Banner 模式有效）
    #[props(default = false)]
    pub closing: bool,

    /// 关闭时回调（点击关闭按钮或自动关闭时触发）
    pub onclose: Option<EventHandler<()>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// Alert 警告提示组件
///
/// 支持两种模式：
/// - **Inline**：嵌入文档流，适合表单提示、卡片内警告
/// - **Banner**：全局顶部横幅，适合页面级通知（需放在 AlertBannerContainer 内）
#[allow(non_snake_case)]
pub fn Alert(props: AlertProps) -> Element {
    let is_banner = props.mode == AlertMode::Banner;

    // 退出动画播放完成后调用 onclose
    let mut leaving = use_signal(|| false);

    // 关闭流程 —— Banner 模式先播退出动画再通知父组件；Inline 模式立即通知
    let trigger_close = {
        let mut l = leaving.clone();
        let onclose = props.onclose.clone();
        move || {
            if l() {
                return;
            }
            if is_banner {
                l.set(true);
                let oc = onclose.clone();
                spawn(async move {
                    use gloo_timers::future::TimeoutFuture;
                    TimeoutFuture::new(400).await; // 等待退出动画播放完毕（CSS 过渡 0.3s）
                    if let Some(ref handler) = oc {
                        handler.call(());
                    }
                });
            } else {
                if let Some(ref handler) = onclose {
                    handler.call(());
                }
            }
        }
    };

    // 自动关闭逻辑（仅 Banner 模式）
    if is_banner && props.duration > 0 {
        let mut start_close = trigger_close.clone();
        let dur = props.duration;
        spawn(async move {
            // 添加 ±500ms 抖动，避免多条同时创建的横幅在同一时刻消失
            let jitter = next_jitter();
            let actual_dur = (dur as i64 + jitter).max(500) as u32;
            use gloo_timers::future::TimeoutFuture;
            TimeoutFuture::new(actual_dur).await;
            start_close();
        });
    }

    // 外部 closing 信号（仅 Banner 模式有效）
    if is_banner && props.closing && !leaving() {
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

    let type_class = match props.r#type {
        AlertType::Info => "ctrl-alert--info",
        AlertType::Success => "ctrl-alert--success",
        AlertType::Warning => "ctrl-alert--warning",
        AlertType::Error => "ctrl-alert--error",
    };

    let mode_class = match props.mode {
        AlertMode::Inline => "ctrl-alert--inline",
        AlertMode::Banner => "ctrl-alert--banner",
    };

    let anim_class = if leaving() { " ctrl-alert--leaving" } else { "" };

    let alert_class = if props.class.is_empty() {
        format!("ctrl-alert {} {}{}", type_class, mode_class, anim_class)
    } else {
        format!("ctrl-alert {} {} {}{}", type_class, mode_class, props.class, anim_class)
    };

    let icon_char = match props.r#type {
        AlertType::Info => "ℹ",
        AlertType::Success => "✔",
        AlertType::Warning => "⚠",
        AlertType::Error => "✕",
    };

    let has_title = !props.title.is_empty();
    let has_desc = !props.description.is_empty();

    let mut close = trigger_close.clone();

    rsx! {
        div {
            class: "{alert_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            if props.show_icon {
                span {
                    class: "ctrl-alert__icon",
                    "{icon_char}"
                }
            }
            div {
                class: "ctrl-alert__body",
                if has_title {
                    div {
                        class: "ctrl-alert__title",
                        "{props.title}"
                    }
                }
                if has_desc {
                    div {
                        class: "ctrl-alert__description",
                        "{props.description}"
                    }
                }
            }
            if props.closable {
                button {
                    class: "ctrl-alert__close",
                    onclick: move |_| close(),
                    "✕"
                }
            }
        }
    }
}
