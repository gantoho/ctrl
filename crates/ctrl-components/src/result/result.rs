use dioxus::prelude::*;

/// Result 结果状态
#[derive(PartialEq, Clone, Default)]
pub enum ResultStatus {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// Result 结果页组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ResultProps {
    /// 结果状态（决定图标颜色和默认图标）
    #[props(default = ResultStatus::default())]
    pub status: ResultStatus,

    /// 标题
    #[props(default = "".to_string())]
    pub title: String,

    /// 副标题 / 描述
    #[props(default = "".to_string())]
    pub subtitle: String,

    /// 自定义图标（传入后覆盖默认图标）
    pub icon: Option<Element>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 底部操作区（按钮等）
    pub children: Element,
}

/// Result 结果页组件
///
/// 用于展示操作结果反馈，支持四种状态和自定义图标。
///
/// ### 使用示例
/// ```rust
/// Result {
///     status: ResultStatus::Success,
///     title: "操作成功".to_string(),
///     subtitle: "已成功提交表单，预计 3 个工作日内审核完成。".to_string(),
///     Button { variant: Variant::Primary, "返回首页" }
///     Button { variant: Variant::Outline, "查看详情" }
/// }
/// ```
#[allow(non_snake_case)]
pub fn Result(props: ResultProps) -> Element {
    const CSS: &str = include_str!("../../assets/result.css");

    let status_class = match props.status {
        ResultStatus::Success => "ctrl-result--success",
        ResultStatus::Info => "ctrl-result--info",
        ResultStatus::Warning => "ctrl-result--warning",
        ResultStatus::Error => "ctrl-result--error",
    };

    let wrapper_class = if props.class.is_empty() {
        format!("ctrl-result {}", status_class)
    } else {
        format!("ctrl-result {} {}", status_class, props.class)
    };

    // 默认图标
    let icon = props.icon.unwrap_or_else(|| {
        let emoji = match props.status {
            ResultStatus::Success => "✅",
            ResultStatus::Info => "ℹ️",
            ResultStatus::Warning => "⚠️",
            ResultStatus::Error => "❌",
        };
        rsx! { span { "{emoji}" } }
    });

    let has_subtitle = !props.subtitle.is_empty();

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            div { class: "ctrl-result__icon", {icon} }
            h2 { class: "ctrl-result__title", "{props.title}" }
            if has_subtitle {
                p { class: "ctrl-result__subtitle", "{props.subtitle}" }
            }
            div { class: "ctrl-result__extra", {props.children} }
        }
    }
}
