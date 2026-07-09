use dioxus::prelude::*;

/// AuroraBackground 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct AuroraBackgroundProps {
    /// 极光渐变色列表（留空使用主题默认组合）
    #[props(default = Vec::new())]
    pub colors: Vec<String>,

    /// 流动动画时长，单位秒
    #[props(default = 12)]
    pub duration: u32,

    /// 模糊强度，单位 px
    #[props(default = 40)]
    pub blur: u32,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 覆盖在极光之上的内容
    pub children: Element,
}

/// AuroraBackground 极光背景组件
///
/// 多色渐变缓慢流动，形成极光氛围背景，常用于登录页、Hero 区、特色板块。
/// `children` 展示在极光之上。
#[allow(non_snake_case)]
pub fn AuroraBackground(props: AuroraBackgroundProps) -> Element {
    const CSS: &str = include_str!("../../assets/aurora_background.css");

    let container_class = if props.class.is_empty() {
        "ctrl-aurora".to_string()
    } else {
        format!("ctrl-aurora {}", props.class)
    };

    let colors = if props.colors.is_empty() {
        vec![
            "#6366f1".to_string(),
            "#a855f7".to_string(),
            "#06b6d4".to_string(),
            "#3b82f6".to_string(),
        ]
    } else {
        props.colors.clone()
    };
    let gradient = colors.join(", ");

    let mut vars = format!(
        "--ctrl-aurora-duration:{}s; --ctrl-aurora-blur:{}px; --ctrl-aurora-gradient:{};",
        props.duration, props.blur, gradient
    );
    if !props.style.is_empty() {
        vars.push(' ');
        vars.push_str(&props.style);
    }

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{vars}",
            div { class: "ctrl-aurora__layer" }
            div { class: "ctrl-aurora__content", {props.children} }
        }
    }
}
