/// 合并 class 名称，过滤掉空字符串
///
/// # 示例
///
/// ```
/// let classes = cn(&["btn", "btn-primary", ""]);
/// assert_eq!(classes, "btn btn-primary");
/// ```
pub fn cn(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|c| !c.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" ")
}