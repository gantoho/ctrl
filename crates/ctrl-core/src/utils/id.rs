use std::sync::atomic::{AtomicU32, Ordering};

/// 全局唯一 ID 计数器（用于生成组件内唯一 ID）
static GLOBAL_ID: AtomicU32 = AtomicU32::new(1);

/// 生成一个全局唯一的标识符
///
/// # 参数
/// - `prefix: &str` - ID 前缀（如 "ctrl-popover", "ctrl-dropdown" 等）
///
/// # 返回
/// 格式为 `{prefix}-{num}` 的唯一 ID 字符串
///
/// # 示例
/// ```
/// let id = unique_id("ctrl-popover");
/// // => "ctrl-popover-1"
/// let id2 = unique_id("ctrl-popover");
/// // => "ctrl-popover-2"
/// ```
pub fn unique_id(prefix: &str) -> String {
    let num = GLOBAL_ID.fetch_add(1, Ordering::Relaxed);
    format!("{}-{}", prefix, num)
}