use dioxus::prelude::*;

/// Image 图片组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ImageProps {
    /// 图片地址
    #[props(default = "".to_string())]
    pub src: String,

    /// 替代文字
    #[props(default = "".to_string())]
    pub alt: String,

    /// 图片宽度
    #[props(default = "auto".to_string())]
    pub width: String,

    /// 图片高度
    #[props(default = "auto".to_string())]
    pub height: String,

    /// 图片填充模式：cover / contain / fill / none
    #[props(default = "cover".to_string())]
    pub fit: String,

    /// 形状：default / rounded / circle
    #[props(default = "default".to_string())]
    pub shape: String,

    /// 是否可预览
    #[props(default = false)]
    pub preview: bool,

    /// 加载失败占位文字
    #[props(default = "加载失败".to_string())]
    pub fallback: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// Image 图片组件
#[allow(non_snake_case)]
pub fn Image(props: ImageProps) -> Element {
    const CSS: &str = include_str!("../../assets/image.css");
    let mut loaded = use_signal(|| false);
    let mut error = use_signal(|| false);
    let mut preview_visible = use_signal(|| false);

    let image_class = {
        let mut c = String::from("ctrl-image");
        match props.shape.as_str() {
            "rounded" => c.push_str(" ctrl-image--rounded"),
            "circle" => c.push_str(" ctrl-image--circle"),
            _ => {}
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    let img_class = match props.fit.as_str() {
        "contain" => "ctrl-image__img ctrl-image__img--contain",
        "fill" => "ctrl-image__img ctrl-image__img--fill",
        "none" => "ctrl-image__img ctrl-image__img--none",
        _ => "ctrl-image__img",
    };

    let img_style = format!("width: {}; height: {};", props.width, props.height);
    let container_style = format!("width: {}; height: {}; {}", props.width, props.height, props.style);

    let on_load = move |_| {
        loaded.set(true);
    };

    let on_error = move |_| {
        error.set(true);
    };

    let show_preview = props.preview && loaded() && !error();

    rsx! {
        style { {CSS} }
        div {
            class: "{image_class}",
            style: "{container_style}",
            // 图片
            if !error() {
                img {
                    class: "{img_class}",
                    style: "{img_style}",
                    src: "{props.src}",
                    alt: "{props.alt}",
                    onload: on_load,
                    onerror: on_error,
                    onclick: move |_| {
                        if show_preview {
                            preview_visible.set(true);
                        }
                    },
                }
            }
            // 加载中占位
            if !loaded() && !error() {
                div { class: "ctrl-image__placeholder", "加载中..." }
            }
            // 错误占位
            if error() {
                div { class: "ctrl-image__error",
                    span { class: "ctrl-image__error-icon", "🖼" }
                    span { "{props.fallback}" }
                }
            }
            // 预览
            if preview_visible() {
                {
                    let preview_src = props.src.clone();
                    rsx! {
                        div {
                            class: "ctrl-image__preview-mask",
                            onclick: move |_| preview_visible.set(false),
                            img {
                                class: "ctrl-image__preview-img",
                                src: "{preview_src}",
                                alt: "{props.alt}",
                            }
                            button {
                                class: "ctrl-image__preview-close",
                                onclick: move |_| preview_visible.set(false),
                                "✕"
                            }
                        }
                    }
                }
            }
        }
    }
}