use super::colors::ColorPalette;

/// 主题配置 —— 用户可自由定制
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub colors: ColorPalette,
    pub font_family: &'static str,
    pub font_size_sm: &'static str,
    pub font_size_md: &'static str,
    pub font_size_lg: &'static str,
    pub radius_sm: &'static str,
    pub radius_md: &'static str,
    pub radius_lg: &'static str,
    pub shadow_sm: &'static str,
    pub shadow_md: &'static str,
    pub transition: &'static str,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: ColorPalette::default(),
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
            font_size_sm: "0.75rem",
            font_size_md: "0.875rem",
            font_size_lg: "1rem",
            radius_sm: "0.25rem",
            radius_md: "0.375rem",
            radius_lg: "0.5rem",
            shadow_sm: "0 1px 2px 0 rgba(0, 0, 0, 0.05)",
            shadow_md: "0 4px 6px -1px rgba(0, 0, 0, 0.1)",
            transition: "0.15s ease",
        }
    }
}