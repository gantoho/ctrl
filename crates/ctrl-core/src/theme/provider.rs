use dioxus::prelude::*;
use super::theme::Theme;

/// 主题提供者组件
///
/// 包裹在应用最外层，自动注入 CSS 变量并通过 Context 向下传递主题配置。
///
/// # 示例
///
/// ```rust
/// use ctrl_core::theme::{ThemeProvider, Theme};
///
/// // 使用默认主题
/// rsx! { ThemeProvider { "你的应用" } }
///
/// // 自定义主题色
/// let my_theme = Theme {
///     colors: ColorPalette {
///         primary: "#FF6B35",
///         ..Default::default()
///     },
///     ..Default::default()
/// };
/// rsx! { ThemeProvider { theme: my_theme, "你的应用" } }
/// ```
#[derive(Props, PartialEq, Clone)]
pub struct ThemeProviderProps {
    /// 自定义主题（不传则使用默认主题）
    pub theme: Option<Theme>,
    /// 子元素
    pub children: Element,
}

/// 生成 CSS 变量字符串
fn build_css_vars(theme: &Theme) -> String {
    let c = &theme.colors;
    format!(
        ":root {{
            --ctrl-primary: {primary};
            --ctrl-primary-hover: {primary_hover};
            --ctrl-primary-active: {primary_active};
            --ctrl-primary-light: {primary_light};
            --ctrl-secondary: {secondary};
            --ctrl-secondary-hover: {secondary_hover};
            --ctrl-success: {success};
            --ctrl-warning: {warning};
            --ctrl-danger: {danger};
            --ctrl-info: {info};
            --ctrl-bg: {bg};
            --ctrl-bg-secondary: {bg_secondary};
            --ctrl-bg-disabled: {bg_disabled};
            --ctrl-text: {text};
            --ctrl-text-secondary: {text_secondary};
            --ctrl-text-disabled: {text_disabled};
            --ctrl-border: {border};
            --ctrl-border-hover: {border_hover};
            --ctrl-text-on-primary: {text_on_primary};
            --ctrl-mask-bg: {mask_bg};
            --ctrl-font-family: {font_family};
            --ctrl-font-size-xs: {font_size_xs};
            --ctrl-font-size-sm: {font_size_sm};
            --ctrl-font-size-md: {font_size_md};
            --ctrl-font-size-lg: {font_size_lg};
            --ctrl-spacing-xs: {spacing_xs};
            --ctrl-spacing-sm: {spacing_sm};
            --ctrl-spacing-md: {spacing_md};
            --ctrl-spacing-lg: {spacing_lg};
            --ctrl-spacing-xl: {spacing_xl};
            --ctrl-radius-sm: {radius_sm};
            --ctrl-radius-md: {radius_md};
            --ctrl-radius-lg: {radius_lg};
            --ctrl-shadow-sm: {shadow_sm};
            --ctrl-shadow-md: {shadow_md};
            --ctrl-transition: {transition};
        }}",
        primary = c.primary,
        primary_hover = c.primary_hover,
        primary_active = c.primary_active,
        primary_light = c.primary_light,
        secondary = c.secondary,
        secondary_hover = c.secondary_hover,
        success = c.success,
        warning = c.warning,
        danger = c.danger,
        info = c.info,
        bg = c.bg,
        bg_secondary = c.bg_secondary,
        bg_disabled = c.bg_disabled,
        text = c.text,
        text_secondary = c.text_secondary,
        text_disabled = c.text_disabled,
        border = c.border,
        border_hover = c.border_hover,
        text_on_primary = c.text_on_primary,
        mask_bg = c.mask_bg,
        font_family = theme.font_family,
        font_size_xs = theme.font_size_xs,
        font_size_sm = theme.font_size_sm,
        font_size_md = theme.font_size_md,
        font_size_lg = theme.font_size_lg,
        spacing_xs = theme.spacing_xs,
        spacing_sm = theme.spacing_sm,
        spacing_md = theme.spacing_md,
        spacing_lg = theme.spacing_lg,
        spacing_xl = theme.spacing_xl,
        radius_sm = theme.radius_sm,
        radius_md = theme.radius_md,
        radius_lg = theme.radius_lg,
        shadow_sm = theme.shadow_sm,
        shadow_md = theme.shadow_md,
        transition = theme.transition,
    )
}

/// 全局重置样式
const RESET_CSS: &str = include_str!("../../assets/reset.css");

#[allow(non_snake_case)]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let theme = props.theme.unwrap_or_default();
    let css_vars = build_css_vars(&theme);

    // 通过 Context 向子组件提供主题
    use_context_provider(|| theme);

    rsx! {
        // 全局重置样式（消除浏览器默认样式干扰）
        style { {RESET_CSS} }
        // 动态 CSS 变量（因每个用户的主题配置不同）
        style { {css_vars} }
        {props.children}
    }
}