use dioxus::prelude::*;
use ctrl_core::shortcut::shortcut_keys;
use ctrl_core::types::Size;
use crate::kbd::Kbd;

/// ShortcutKeys 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ShortcutKeysProps {
    /// 组合键字符串（如 "mod+k"、"ctrl+shift+p"）
    pub combo: String,

    /// 按键尺寸
    #[props(default = Size::Sm)]
    pub size: Size,

    /// 是否在按键间显示 "+" 连接符
    #[props(default = false)]
    pub separator: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// ShortcutKeys 快捷键展示组件
///
/// 将组合键字符串渲染为跨平台的按键序列（复用 [`Kbd`]），
/// 在 macOS 上显示 ⌘/⌥/⇧ 等符号，其他平台显示 Ctrl/Alt/Shift。
#[allow(non_snake_case)]
pub fn ShortcutKeys(props: ShortcutKeysProps) -> Element {
    const CSS: &str = include_str!("../../assets/shortcut_keys.css");
    let keys = shortcut_keys(&props.combo);
    let wrapper_class = if props.class.is_empty() {
        "ctrl-shortcut-keys".to_string()
    } else {
        format!("ctrl-shortcut-keys {}", props.class)
    };

    rsx! {
        style { {CSS} }
        span { class: "{wrapper_class}",
            for (i, key) in keys.iter().enumerate() {
                if props.separator && i > 0 {
                    span { class: "ctrl-shortcut-keys__plus", "+" }
                }
                Kbd { key: "{i}", size: props.size, "{key}" }
            }
        }
    }
}
