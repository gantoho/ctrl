use dioxus::prelude::*;
use ctrl_core::types::{Size, Variant};
use crate::button::Button;

/// 复制文本到剪贴板
#[allow(unused_variables)]
fn copy_text(text: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let clipboard = window.navigator().clipboard();
            let _ = clipboard.write_text(text);
        }
    }
}

/// CopyButton 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct CopyButtonProps {
    /// 待复制的文本内容
    pub text: String,

    /// 默认文案
    #[props(default = "复制".to_string())]
    pub label: String,

    /// 复制成功后的文案
    #[props(default = "已复制".to_string())]
    pub copied_label: String,

    /// 是否显示图标
    #[props(default = true)]
    pub show_icon: bool,

    /// 成功态持续时间（毫秒）
    #[props(default = 2000)]
    pub duration: u32,

    /// 按钮变体
    #[props(default = Variant::Outline)]
    pub variant: Variant,

    /// 按钮尺寸
    #[props(default = Size::default())]
    pub size: Size,

    /// 复制成功回调（返回被复制的文本）
    pub oncopy: Option<EventHandler<String>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// CopyButton 复制按钮组件
///
/// 点击将 `text` 复制到剪贴板，并在 `duration` 毫秒内展示成功态（文案与图标切换）。
/// 复用 [`Button`] 保持视觉一致。
#[allow(non_snake_case)]
pub fn CopyButton(props: CopyButtonProps) -> Element {
    const CSS: &str = include_str!("../../assets/copy_button.css");
    let mut copied = use_signal(|| false);

    let text = props.text.clone();
    let oncopy = props.oncopy;
    let duration = props.duration;

    let (icon, current_label) = if copied() {
        ("✓", props.copied_label.clone())
    } else {
        ("📋", props.label.clone())
    };

    let btn_class = if props.class.is_empty() {
        "ctrl-copy-button".to_string()
    } else {
        format!("ctrl-copy-button {}", props.class)
    };
    let btn_class = if copied() {
        format!("{} ctrl-copy-button--copied", btn_class)
    } else {
        btn_class
    };

    rsx! {
        style { {CSS} }
        Button {
            variant: props.variant,
            size: props.size,
            class: btn_class,
            style: props.style.clone(),
            onclick: move |_| {
                copy_text(&text);
                copied.set(true);
                if let Some(ref handler) = oncopy {
                    handler.call(text.clone());
                }
                let mut c = copied;
                gloo_timers::callback::Timeout::new(duration, move || {
                    c.set(false);
                }).forget();
            },
            span { class: "ctrl-copy-button__inner",
                if props.show_icon {
                    span { class: "ctrl-copy-button__icon", "{icon}" }
                }
                span { class: "ctrl-copy-button__label", "{current_label}" }
            }
        }
    }
}
