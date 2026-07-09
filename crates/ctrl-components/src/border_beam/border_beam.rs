use dioxus::prelude::*;

/// BorderBeam 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct BorderBeamProps {
    /// 边框（光束轨道）厚度，单位 px
    #[props(default = 2)]
    pub size: u32,

    /// 绕行一周的时长，单位秒
    #[props(default = 4)]
    pub duration: u32,

    /// 光束渐变起始色（留空使用主题默认）
    #[props(default = "".to_string())]
    pub color_from: String,

    /// 光束渐变结束色（留空使用主题默认）
    #[props(default = "".to_string())]
    pub color_to: String,

    /// 圆角（留空使用 --ctrl-radius-lg）
    #[props(default = "".to_string())]
    pub radius: String,

    /// 是否反向绕行
    #[props(default = false)]
    pub reverse: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素
    pub children: Element,
}

/// BorderBeam 边框光束组件
///
/// 在容器边框轨道上循环流动一束光点，常用于强调卡片、AI 输入框等。
/// 使用 `@property` 驱动 conic-gradient 角度旋转实现无断裂绕行。
#[allow(non_snake_case)]
pub fn BorderBeam(props: BorderBeamProps) -> Element {
    const CSS: &str = include_str!("../../assets/border_beam.css");

    let mut vars = format!(
        "--ctrl-border-beam-size:{}px; --ctrl-border-beam-duration:{}s;",
        props.size, props.duration
    );
    if !props.color_from.is_empty() {
        vars.push_str(&format!("--ctrl-border-beam-from:{};", props.color_from));
    }
    if !props.color_to.is_empty() {
        vars.push_str(&format!("--ctrl-border-beam-to:{};", props.color_to));
    }
    if !props.radius.is_empty() {
        vars.push_str(&format!("--ctrl-border-beam-radius:{};", props.radius));
    }
    if !props.style.is_empty() {
        vars.push(' ');
        vars.push_str(&props.style);
    }

    let mut classes = vec!["ctrl-border-beam".to_string()];
    if props.reverse {
        classes.push("ctrl-border-beam--reverse".into());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let container_class = classes.join(" ");

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{vars}",
            div { class: "ctrl-border-beam__base" }
            div { class: "ctrl-border-beam__beam" }
            div { class: "ctrl-border-beam__mask" }
            div { class: "ctrl-border-beam__content", {props.children} }
        }
    }
}
