use dioxus::prelude::*;

// ═══════════════════════════════════════════════════════════
// ImagePreviewAPI —— 全局图片预览触发器
// ═══════════════════════════════════════════════════════════

/// 图片预览 API，通过 use_image_preview() 获取
#[derive(Clone, Copy)]
pub struct ImagePreviewAPI {
    pub(crate) visible: Signal<bool>,
    pub(crate) urls: Signal<Vec<String>>,
    pub(crate) index: Signal<usize>,
}

/// 获取图片预览 API
pub fn use_image_preview() -> ImagePreviewAPI {
    use_context::<ImagePreviewAPI>()
}

impl ImagePreviewAPI {
    /// 打开图片预览
    /// - `urls`: 预览图片列表
    /// - `index`: 初始显示第几张（从 0 开始）
    pub fn open(&mut self, urls: Vec<String>, index: usize) {
        self.urls.set(urls);
        self.index.set(index);
        self.visible.set(true);
    }

    /// 关闭图片预览
    pub fn close(&mut self) {
        self.visible.set(false);
    }
}

// ═══════════════════════════════════════════════════════════
// ImagePreviewProvider
// ═══════════════════════════════════════════════════════════

/// ImagePreviewProvider 属性
#[derive(Props, PartialEq, Clone)]
pub struct ImagePreviewProviderProps {
    /// 子元素
    pub children: Element,
}

/// 图片预览容器组件，放在应用根节点
/// 通过 use_image_preview() 获取 API 触发预览
#[allow(non_snake_case)]
pub fn ImagePreviewProvider(props: ImagePreviewProviderProps) -> Element {
    let visible = use_signal(|| false);
    let urls = use_signal(Vec::<String>::new);
    let index = use_signal(|| 0usize);

    let api = ImagePreviewAPI {
        visible: visible.clone(),
        urls: urls.clone(),
        index: index.clone(),
    };
    use_context_provider(|| api);

    rsx! {
        {props.children}
        if visible() {
            {
                let preview_urls = urls.read().clone();
                let total = preview_urls.len();
                let mut idx = index;
                let mut vis = visible;
                let mut urls_signal = urls.clone();

                let mut close = move || { vis.set(false); };

                rsx! {
                    div {
                        class: "ctrl-image__preview-mask",
                        onclick: move |_| close(),
                        onkeydown: move |evt: KeyboardEvent| {
                            use dioxus::html::Key;
                            match evt.key() {
                                Key::Escape => close(),
                                Key::ArrowLeft if total > 1 => {
                                    let next = if idx() == 0 { total - 1 } else { idx() - 1 };
                                    idx.set(next);
                                }
                                Key::ArrowRight if total > 1 => {
                                    let next = (idx() + 1) % total;
                                    idx.set(next);
                                }
                                _ => {}
                            }
                        },
                        tabindex: "0",

                        // 上一张（仅多图时显示）
                        if total > 1 {
                            button {
                                class: "ctrl-image__preview-arrow ctrl-image__preview-arrow--prev",
                                onclick: move |evt| {
                                    evt.stop_propagation();
                                    let next = if idx() == 0 { total - 1 } else { idx() - 1 };
                                    idx.set(next);
                                },
                                "‹"
                            }
                        }

                        img {
                            class: "ctrl-image__preview-img",
                            src: "{preview_urls.get(idx()).cloned().unwrap_or_default()}",
                            onclick: move |evt: MouseEvent| evt.stop_propagation(),
                        }

                        // 下一张（仅多图时显示）
                        if total > 1 {
                            button {
                                class: "ctrl-image__preview-arrow ctrl-image__preview-arrow--next",
                                onclick: move |evt| {
                                    evt.stop_propagation();
                                    let next = (idx() + 1) % total;
                                    idx.set(next);
                                },
                                "›"
                            }
                        }

                        // 多图计数器
                        if total > 1 {
                            div { class: "ctrl-image__preview-counter", "{idx() + 1} / {total}" }
                        }

                        button {
                            class: "ctrl-image__preview-close",
                            onclick: move |_| close(),
                            "✕"
                        }
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════
// Image 组件
// ═══════════════════════════════════════════════════════════

/// Image 图片组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ImageProps {
    /// 图片地址（封面 / 缩略图）
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

    /// 多图预览列表（传入后预览时可左右切换）
    #[props(default = Vec::new())]
    pub preview_urls: Vec<String>,

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
    let mut preview_index = use_signal(|| 0usize);

    // src 为空时，回退到 preview_urls 的第一张
    let display_src = if props.src.is_empty() && !props.preview_urls.is_empty() {
        props.preview_urls[0].clone()
    } else {
        props.src.clone()
    };

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
                    src: "{display_src}",
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
                    // 多图模式：使用 preview_urls；单图模式：使用 src
                    let has_multi = !props.preview_urls.is_empty();
                    let urls = if has_multi {
                        props.preview_urls.clone()
                    } else {
                        vec![display_src.clone()]
                    };
                    let total = urls.len();
                    let idx = preview_index() % total;

                    // 阻止图片点击关闭遮罩
                    let stop_propagation = move |evt: MouseEvent| evt.stop_propagation();

                    rsx! {
                        div {
                            class: "ctrl-image__preview-mask",
                            onclick: move |_| preview_visible.set(false),
                            onkeydown: move |evt: KeyboardEvent| {
                                use dioxus::html::Key;
                                match evt.key() {
                                    Key::Escape => preview_visible.set(false),
                                    Key::ArrowLeft if has_multi && total > 1 => {
                                        let next = if preview_index() == 0 { total - 1 } else { preview_index() - 1 };
                                        preview_index.set(next);
                                    }
                                    Key::ArrowRight if has_multi && total > 1 => {
                                        let next = (preview_index() + 1) % total;
                                        preview_index.set(next);
                                    }
                                    _ => {}
                                }
                            },
                            tabindex: "0",

                            // 上一张（仅多图时显示）
                            if has_multi && total > 1 {
                                button {
                                    class: "ctrl-image__preview-arrow ctrl-image__preview-arrow--prev",
                                    onclick: move |evt| {
                                        evt.stop_propagation();
                                        let next = if preview_index() == 0 { total - 1 } else { preview_index() - 1 };
                                        preview_index.set(next);
                                    },
                                    "‹"
                                }
                            }

                            img {
                                class: "ctrl-image__preview-img",
                                src: "{urls[idx]}",
                                alt: "{props.alt}",
                                onclick: stop_propagation,
                            }

                            // 下一张（仅多图时显示）
                            if has_multi && total > 1 {
                                button {
                                    class: "ctrl-image__preview-arrow ctrl-image__preview-arrow--next",
                                    onclick: move |evt| {
                                        evt.stop_propagation();
                                        let next = (preview_index() + 1) % total;
                                        preview_index.set(next);
                                    },
                                    "›"
                                }
                            }

                            // 多图计数器
                            if has_multi && total > 1 {
                                div { class: "ctrl-image__preview-counter", "{idx + 1} / {total}" }
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
