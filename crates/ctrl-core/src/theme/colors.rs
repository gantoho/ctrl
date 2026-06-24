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