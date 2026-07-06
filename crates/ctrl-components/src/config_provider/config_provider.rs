use dioxus::prelude::*;
use ctrl_core::locale::{Lang, LocaleKey, t};
use ctrl_core::types::Size;

/// 全局配置
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalConfig {
    /// 语言
    pub lang: Lang,
    /// 全局组件尺寸
    pub size: Size,
    /// 是否 RTL
    pub rtl: bool,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            lang: Lang::default(),
            size: Size::default(),
            rtl: false,
        }
    }
}

impl GlobalConfig {
    /// 获取文案
    pub fn locale(&self, key: LocaleKey) -> &'static str {
        t(key, self.lang)
    }
}

/// ConfigProvider 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ConfigProviderProps {
    /// 语言
    #[props(default = Lang::default())]
    pub lang: Lang,

    /// 全局尺寸
    #[props(default = Size::default())]
    pub size: Size,

    /// 是否 RTL
    #[props(default = false)]
    pub rtl: bool,

    /// 子元素
    pub children: Element,
}

/// ConfigProvider 全局配置提供者
///
/// 包裹在应用外层，为所有子组件提供全局配置（语言、尺寸、RTL）。
///
/// # 示例
///
/// ```rust
/// use ctrl_components::ConfigProvider;
/// use ctrl_core::locale::Lang;
///
/// rsx! {
///     ConfigProvider {
///         lang: Lang::EnUS,
///         size: Size::Lg,
///         children: rsx! { /* 你的应用 */ }
///     }
/// }
/// ```
#[allow(non_snake_case)]
pub fn ConfigProvider(props: ConfigProviderProps) -> Element {
    let config = GlobalConfig {
        lang: props.lang,
        size: props.size,
        rtl: props.rtl,
    };

    use_context_provider(|| config);

    let dir = if props.rtl { "rtl" } else { "ltr" };

    rsx! {
        div {
            dir: "{dir}",
            style: "width: 100%; height: 100%;",
            {props.children}
        }
    }
}

/// 获取全局配置
///
/// 返回 `GlobalConfig`，如果在 ConfigProvider 外部调用则返回默认值。
pub fn use_config() -> GlobalConfig {
    try_use_context::<GlobalConfig>().unwrap_or_default()
}

/// 获取当前语言配置的便捷函数
pub fn use_locale() -> GlobalConfig {
    use_config()
}
