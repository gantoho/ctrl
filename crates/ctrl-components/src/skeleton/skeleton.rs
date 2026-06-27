use dioxus::prelude::*;

/// Skeleton 骨架屏组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SkeletonProps {
    /// 变体类型：text / title / avatar / image / button / rect
    #[props(default = "text".to_string())]
    pub variant: String,

    /// 行数（仅 text 有效）
    #[props(default = 3)]
    pub rows: usize,

    /// 形状：default(小圆角) / circle(圆形) / round(大圆角)
    #[props(default = "default".to_string())]
    pub shape: String,

    /// 重复次数（列表场景），>0 时忽略 rows，每个 item 使用 variant 样式
    #[props(default = 0)]
    pub count: usize,

    /// 重复项之间的间距（count > 1 时有效）
    #[props(default = "8px".to_string())]
    pub gap: String,

    /// 宽度
    #[props(default = "".to_string())]
    pub width: String,

    /// 高度
    #[props(default = "".to_string())]
    pub height: String,

    /// 是否显示动画
    #[props(default = true)]
    pub animated: bool,

    /// 是否处于加载中：None 始终显示骨架；Some(true) 显示骨架；Some(false) 显示 children
    #[props(default)]
    pub loading: Option<bool>,

    /// 加载完成后显示的内容
    children: Element,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// 构建 CSS 类名字符串
fn build_class(variant: &str, shape: &str, animated: bool, class: &str) -> String {
    let mut parts = Vec::new();
    parts.push("ctrl-skeleton".to_string());
    parts.push(format!("ctrl-skeleton--{}", variant));
    if !animated {
        parts.push("ctrl-skeleton--static".to_string());
    }
    match shape {
        "circle" => parts.push("ctrl-skeleton--circle".to_string()),
        "round" => parts.push("ctrl-skeleton--round".to_string()),
        _ => {}
    }
    if !class.is_empty() {
        parts.push(class.to_string());
    }
    parts.join(" ")
}

/// 构建内联样式
fn build_style(width: &str, height: &str) -> String {
    let mut parts = Vec::new();
    if !width.is_empty() {
        parts.push(format!("width:{}", width));
    }
    if !height.is_empty() {
        parts.push(format!("height:{}", height));
    }
    parts.join(";")
}

/// Skeleton 骨架屏组件
#[allow(non_snake_case)]
pub fn Skeleton(props: SkeletonProps) -> Element {
    const CSS: &str = include_str!("../../assets/skeleton.css");

    // loading 控制：Some(false) 时显示真实内容
    if let Some(false) = props.loading {
        return rsx! { {props.children} };
    }

    let style_str = build_style(&props.width, &props.height);
    let gap_style = if props.count > 1 { format!("gap:{}", props.gap) } else { String::new() };

    rsx! {
        style { {CSS} }

        // count 模式：重复单个骨架项
        if props.count > 1 {
            div {
                class: "ctrl-skeleton__wrapper",
                style: if gap_style.is_empty() { "" } else { "{gap_style}" },
                for _ in 0..props.count {
                    {
                        let cls = build_class(&props.variant, &props.shape, props.animated, &props.class);
                        rsx! {
                            div {
                                class: "{cls}",
                                style: if style_str.is_empty() { "" } else { "{style_str}" },
                            }
                        }
                    }
                }
            }
        }

        // rows 模式：仅在 variant == "text" 时多行
        else if props.variant == "text" && props.rows > 1 {
            {
                let cls = build_class("text", &props.shape, props.animated, &props.class);
                rsx! {
                    div { class: "ctrl-skeleton__wrapper",
                        for i in 0..props.rows {
                            {
                                let w = if i == props.rows - 1 { "width: 60%;" } else { "" };
                                let s = if style_str.is_empty() { w.to_string() } else { format!("{} {}", style_str, w) };
                                rsx! {
                                    div {
                                        key: "{i}",
                                        class: "{cls}",
                                        style: "{s}",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 单一项
        else {
            {
                let cls = build_class(&props.variant, &props.shape, props.animated, &props.class);
                rsx! {
                    div {
                        class: "{cls}",
                        style: if style_str.is_empty() { "" } else { "{style_str}" },
                    }
                }
            }
        }
    }
}

/// SkeletonCard 卡片骨架屏属性
#[derive(Props, PartialEq, Clone)]
pub struct SkeletonCardProps {
    /// 是否显示动画
    #[props(default = true)]
    pub animated: bool,

    /// 文字行数
    #[props(default = 2)]
    pub rows: usize,

    /// 图片高度
    #[props(default = "180px".to_string())]
    pub image_height: String,

    /// 卡片宽度
    #[props(default = "".to_string())]
    pub width: String,

    /// 是否处于加载中
    #[props(default)]
    pub loading: Option<bool>,

    /// 加载完成后显示的内容
    children: Element,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// SkeletonCard 卡片骨架屏：图片 + 标题 + 文字行
#[allow(non_snake_case)]
pub fn SkeletonCard(props: SkeletonCardProps) -> Element {
    if let Some(false) = props.loading {
        return rsx! { {props.children} };
    }

    let wrapper_style = if props.width.is_empty() { String::new() } else { format!("width:{}", props.width) };
    let cls = build_class("image", "round", props.animated, "");
    let title_cls = build_class("title", "default", props.animated, "");

    rsx! {
        {
            let text_cls = build_class("text", "default", props.animated, "");
            rsx! {
                div {
                    class: "ctrl-skeleton__card",
                    style: if wrapper_style.is_empty() { "" } else { "{wrapper_style}" },
                    div {
                        class: "{cls}",
                        style: "height:{props.image_height}",
                    }
                    div {
                        class: "{title_cls}",
                    }
                    for _ in 0..props.rows {
                        div {
                            class: "{text_cls}",
                        }
                    }
                }
            }
        }
    }
}

/// SkeletonList 列表骨架屏属性
#[derive(Props, PartialEq, Clone)]
pub struct SkeletonListProps {
    /// 条目数量
    #[props(default = 3)]
    pub count: usize,

    /// 是否带头像
    #[props(default = true)]
    pub avatar: bool,

    /// 头像尺寸
    #[props(default = "40px".to_string())]
    pub avatar_size: String,

    /// 每项文字行数
    #[props(default = 2)]
    pub rows: usize,

    /// 是否显示动画
    #[props(default = true)]
    pub animated: bool,

    /// 是否处于加载中
    #[props(default)]
    pub loading: Option<bool>,

    /// 加载完成后显示的内容
    children: Element,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// SkeletonList 列表骨架屏：可重复的 avatar+text 条目
#[allow(non_snake_case)]
pub fn SkeletonList(props: SkeletonListProps) -> Element {
    if let Some(false) = props.loading {
        return rsx! { {props.children} };
    }

    let avatar_cls = build_class("avatar", "circle", props.animated, "");
    let text_cls = build_class("text", "default", props.animated, "");

    rsx! {
        div { class: "ctrl-skeleton__list",
            for _ in 0..props.count {
                div { class: "ctrl-skeleton__list-item",
                    if props.avatar {
                        div {
                            class: "{avatar_cls}",
                            style: "width:{props.avatar_size}; height:{props.avatar_size}; flex-shrink: 0;",
                        }
                    }
                    div { class: "ctrl-skeleton__lines",
                        for _ in 0..props.rows {
                            div { class: "{text_cls}" }
                        }
                    }
                }
            }
        }
    }
}

/// SkeletonRow 属性
#[derive(Props, PartialEq, Clone)]
pub struct SkeletonRowProps {
    /// 文字行数
    #[props(default = 3)]
    pub rows: usize,

    /// 是否显示头像
    #[props(default = true)]
    pub avatar: bool,

    /// 头像尺寸（宽高相同）
    #[props(default = "48px".to_string())]
    pub avatar_size: String,

    /// 是否显示动画
    #[props(default = true)]
    pub animated: bool,

    /// 是否处于加载中
    #[props(default)]
    pub loading: Option<bool>,

    /// 加载完成后显示的内容
    children: Element,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// SkeletonRow 头像 + 文字行骨架屏
#[allow(non_snake_case)]
pub fn SkeletonRow(props: SkeletonRowProps) -> Element {
    if let Some(false) = props.loading {
        return rsx! { {props.children} };
    }

    let avatar_cls = build_class("avatar", "circle", props.animated, "");
    let text_cls = build_class("text", "default", props.animated, "");

    rsx! {
        div {
            class: "ctrl-skeleton__row",
            if props.avatar {
                div {
                    class: "{avatar_cls}",
                    style: "width:{props.avatar_size}; height:{props.avatar_size}; flex-shrink: 0;",
                }
            }
            div { class: "ctrl-skeleton__lines",
                for _ in 0..props.rows {
                    div { class: "{text_cls}" }
                }
            }
        }
    }
}
