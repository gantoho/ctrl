use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use gloo_timers::callback::Timeout;

/// Spin 加载中组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SpinProps {
    /// 是否显示加载状态
    #[props(default = true)]
    pub spinning: bool,

    /// 加载提示文案
    pub tip: Option<String>,

    /// 尺寸：sm / md / lg
    #[props(default = "md".to_string())]
    pub size: String,

    /// 自定义颜色（CSS color 值）
    pub color: Option<String>,

    /// 延迟显示时间（毫秒），避免闪烁。0 表示不延迟
    #[props(default = 0)]
    pub delay: u32,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子内容：spinning=true 时覆盖半透明遮罩，spinning=false 时正常显示
    pub children: Element,
}


/// Spin 加载中组件
///
/// 包裹内容区域，在加载状态时显示旋转动画指示器和半透明遮罩。
///
/// # Examples
///
/// ```rust
/// Spin { spinning: true, tip: "加载中...",
///     div { "内容区域" }
/// }
/// ```
#[allow(non_snake_case)]
pub fn Spin(props: SpinProps) -> Element {
    const CSS: &str = include_str!("../../assets/spin.css");

    let size_class = match props.size.as_str() {
        "sm" => "ctrl-spin--sm",
        "lg" => "ctrl-spin--lg",
        _ => "ctrl-spin--md",
    };

    let mut classes = vec!["ctrl-spin".to_string(), size_class.to_string()];
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let class_str = classes.join(" ");

    let custom_color = if let Some(ref c) = props.color {
        format!("--ctrl-spin-color: {};", c)
    } else {
        String::new()
    };

    let mut show = use_signal(|| props.delay == 0 || !props.spinning);

    #[cfg(target_arch = "wasm32")]
    {
        let spinning = props.spinning;
        let delay = props.delay;
        use_effect(move || {
            if delay > 0 && spinning {
                let t = Timeout::new(delay, move || {
                    show.set(true);
                });
                t.forget();
            } else {
                show.set(spinning);
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = show;
        use_effect(move || {});
    }

    let visible = show();

    rsx! {
        style { {CSS} }
        div {
            class: "{class_str}",
            style: "{props.style}",
            if visible {
                div { class: "ctrl-spin__overlay",
                    span {
                        class: "ctrl-spin__dot",
                        style: "{custom_color}",
                    }
                    if let Some(ref tip) = props.tip {
                        span { class: "ctrl-spin__tip", "{tip}" }
                    }
                }
            }
            div {
                class: if visible { "ctrl-spin__content ctrl-spin__content--blur" } else { "ctrl-spin__content" },
                {props.children}
            }
        }
    }
}
