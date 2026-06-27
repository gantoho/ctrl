use dioxus::prelude::*; use ctrl_core::types::Size;

/// Loading 加载中组件属性
#[derive(Props, PartialEq, Clone)]
pub struct LoadingProps {
    /// 是否显示加载中
    #[props(default = true)]
    pub loading: bool,

    /// 加载文案
    #[props(default = "".to_string())]
    pub text: String,

    /// 尺寸
    #[props(default = Size::Md)]
    pub size: Size,

    /// 是否全屏遮罩
    #[props(default = false)]
    pub fullscreen: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// Loading 加载中组件
#[allow(non_snake_case)]
pub fn Loading(props: LoadingProps) -> Element {
    const CSS: &str = include_str!("../../assets/loading.css");
    if !props.loading {
        return rsx! {};
    }

    let size_class = match props.size {
        Size::Sm => "ctrl-loading--sm",
        Size::Md => "ctrl-loading--md",
        Size::Lg => "ctrl-loading--lg",
    };

    let mut classes = vec!["ctrl-loading".to_string(), size_class.to_string()];
    if props.fullscreen {
        classes.push("ctrl-loading--fullscreen".into());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let class_str = classes.join(" ");

    rsx! {
        style { {CSS} }
        div {
            class: "{class_str}",
            span { class: "ctrl-loading__spinner" }
            if !props.text.is_empty() {
                span {
                    class: "ctrl-loading__text",
                    "{props.text}"
                }
            }
        }
    }
}
