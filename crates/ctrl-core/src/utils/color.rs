//! 颜色工具函数 —— 用于深色模式自动推算

/// 解析十六进制颜色字符串 `#RRGGBB` → (R, G, B)
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some((r, g, b))
    } else if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
        let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
        let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
        // 3-digit hex → 6-digit: repeat each digit
        Some((r * 17, g * 17, b * 17))
    } else {
        None
    }
}

/// RGB → hex 字符串
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{r:02X}{g:02X}{b:02X}")
}

/// 相对亮度 (sRGB)，用于判断颜色是"亮"还是"暗"
/// 返回值 ≈0..1，越大越亮
pub fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
    let linearize = |c: u8| {
        let v = c as f64 / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linearize(r) + 0.7152 * linearize(g) + 0.0722 * linearize(b)
}

/// 亮化颜色（向白色混合 factor 比例，0=不变，1=完全变白）
pub fn lighten((r, g, b): (u8, u8, u8), factor: f64) -> (u8, u8, u8) {
    let blend = |c: u8| {
        let v = c as f64 + (255.0 - c as f64) * factor;
        v.clamp(0.0, 255.0) as u8
    };
    (blend(r), blend(g), blend(b))
}

/// 暗化颜色（向黑色混合 factor 比例，0=不变，1=完全变黑）
pub fn darken((r, g, b): (u8, u8, u8), factor: f64) -> (u8, u8, u8) {
    let blend = |c: u8| {
        let v = c as f64 * (1.0 - factor);
        v.clamp(0.0, 255.0) as u8
    };
    (blend(r), blend(g), blend(b))
}
