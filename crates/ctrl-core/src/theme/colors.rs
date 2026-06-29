use crate::utils::color::*;

/// 主题色板 —— 所有颜色值均可被用户覆盖
#[derive(Debug, Clone, PartialEq)]
pub struct ColorPalette {
    /// 主色
    pub primary: &'static str,
    /// 主色悬停
    pub primary_hover: &'static str,
    /// 主色激活
    pub primary_active: &'static str,
    /// 主色浅底
    pub primary_light: &'static str,
    /// 次级色
    pub secondary: &'static str,
    /// 次级色悬停
    pub secondary_hover: &'static str,
    /// 成功色
    pub success: &'static str,
    /// 警告色
    pub warning: &'static str,
    /// 危险色
    pub danger: &'static str,
    /// 信息色
    pub info: &'static str,
    /// 背景色
    pub bg: &'static str,
    /// 次级背景
    pub bg_secondary: &'static str,
    /// 禁用背景
    pub bg_disabled: &'static str,
    /// 文字色
    pub text: &'static str,
    /// 次级文字
    pub text_secondary: &'static str,
    /// 禁用文字
    pub text_disabled: &'static str,
    /// 边框色
    pub border: &'static str,
    /// 边框悬停
    pub border_hover: &'static str,
    /// 主色/次级色上的文字色（如白色按钮上的文字）
    pub text_on_primary: &'static str,
    /// 遮罩层背景色
    pub mask_bg: &'static str,
}

impl ColorPalette {
    /// 从浅色主题自动推算深色色板
    ///
    /// 推算策略：
    /// - 背景色系互换：浅 bg → 深 bg，深 text → 浅 text
    /// - 品牌色（primary）保持但略微调亮以适配深色背景
    /// - 语义色（success/warning/danger/info）保持色相，调整亮度
    /// - 边框和分隔线做暗化处理
    pub fn dark(&self) -> Self {
        // 从浅色 bg 推算暗色基调
        let bg_rgb = hex_to_rgb(self.bg).unwrap_or((255, 255, 255));
        let bg_lum = relative_luminance(bg_rgb.0, bg_rgb.1, bg_rgb.2);

        // 如果当前 bg 已经是暗色，则轻微变化即可
        let (dark_bg, dark_bg_secondary, dark_bg_disabled) = if bg_lum < 0.3 {
            // 已经是暗色背景，直接延用
            (
                rgb_to_hex(bg_rgb.0, bg_rgb.1, bg_rgb.2),
                rgb_to_hex_owned(darken(bg_rgb, -0.1)),
                rgb_to_hex_owned(darken(bg_rgb, -0.2)),
            )
        } else {
            // 浅色 → 深色
            (
                "#0F172A".to_string(),
                "#1E293B".to_string(),
                "#334155".to_string(),
            )
        };

        let dark_bg_secondary = dark_bg_secondary.clone();
        let dark_bg_disabled = dark_bg_disabled.clone();

        // 文字色从深色 bg 推算
        let (dark_text, dark_text_secondary, dark_text_disabled) = if bg_lum < 0.3 {
            // 暗色 bg，文字保持亮色
            (self.text.to_string(), self.text_secondary.to_string(), self.text_disabled.to_string())
        } else {
            ("#F1F5F9".to_string(), "#94A3B8".to_string(), "#475569".to_string())
        };

        // 边框色
        let (dark_border, dark_border_hover) = if bg_lum < 0.3 {
            (self.border.to_string(), self.border_hover.to_string())
        } else {
            ("#334155".to_string(), "#475569".to_string())
        };

        // 品牌色——保持色相但略微调亮（深色背景下同样的颜色会更刺眼）
        let adjust_for_dark = |hex: &str, factor: f64| -> String {
            if let Some(rgb) = hex_to_rgb(hex) {
                let lum = relative_luminance(rgb.0, rgb.1, rgb.2);
                if lum < 0.5 && bg_lum >= 0.3 {
                    // 颜色较暗且从浅色切深色：适当亮化
                    rgb_to_hex_owned(lighten(rgb, factor))
                } else {
                    hex.to_string()
                }
            } else {
                hex.to_string()
            }
        };

        let primary = adjust_for_dark(self.primary, 0.15);
        let primary_hover = adjust_for_dark(self.primary_hover, 0.1);
        let primary_active = adjust_for_dark(self.primary_active, 0.05);

        // primary_light: 深色背景下的 "浅底" 应该是更暗的底色
        let primary_light_dark = if let Some(rgb) = hex_to_rgb(self.primary) {
            // 取主色的非常暗的版本
            let lum = relative_luminance(rgb.0, rgb.1, rgb.2);
            let factor = if lum < 0.5 { 0.85 } else { 0.92 };
            rgb_to_hex_owned(darken(rgb, factor))
        } else {
            self.primary.to_string()
        };

        // 遮罩层——深色模式下用更浅的遮罩
        let mask_bg = if bg_lum < 0.3 {
            self.mask_bg.to_string()
        } else {
            "rgba(0, 0, 0, 0.65)".to_string()
        };

        Self {
            primary: leak_str(primary),
            primary_hover: leak_str(primary_hover),
            primary_active: leak_str(primary_active),
            primary_light: leak_str(primary_light_dark),
            secondary: leak_str(self.secondary.to_string()),
            secondary_hover: leak_str(self.secondary_hover.to_string()),
            success: leak_str(self.success.to_string()),
            warning: leak_str(self.warning.to_string()),
            danger: leak_str(self.danger.to_string()),
            info: leak_str(self.info.to_string()),
            bg: leak_str(dark_bg),
            bg_secondary: leak_str(dark_bg_secondary),
            bg_disabled: leak_str(dark_bg_disabled),
            text: leak_str(dark_text),
            text_secondary: leak_str(dark_text_secondary),
            text_disabled: leak_str(dark_text_disabled),
            border: leak_str(dark_border),
            border_hover: leak_str(dark_border_hover),
            text_on_primary: self.text_on_primary,
            mask_bg: leak_str(mask_bg),
        }
    }
}

/// 将堆分配的 String 泄漏为 &'static str
fn leak_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

/// (R, G, B) → hex String（不泄漏，内部使用）
fn rgb_to_hex_owned((r, g, b): (u8, u8, u8)) -> String {
    rgb_to_hex(r, g, b)
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: "#3FC99E",
            primary_hover: "#2FB58A",
            primary_active: "#1FA376",
            primary_light: "#E6FAF5",
            secondary: "#6B7280",
            secondary_hover: "#4B5563",
            success: "#10B981",
            warning: "#F59E0B",
            danger: "#EF4444",
            info: "#3B82F6",
            bg: "#FFFFFF",
            bg_secondary: "#F9FAFB",
            bg_disabled: "#E5E7EB",
            text: "#111827",
            text_secondary: "#6B7280",
            text_disabled: "#6B7280",
            border: "#E5E7EB",
            border_hover: "#D1D5DB",
            text_on_primary: "#FFFFFF",
            mask_bg: "rgba(0, 0, 0, 0.45)",
        }
    }
}
