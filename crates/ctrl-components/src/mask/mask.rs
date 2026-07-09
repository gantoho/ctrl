use dioxus::prelude::*;

/// Mask 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct MaskProps {
    /// 是否显示遮罩
    #[props(default = false)]
    pub open: bool,

    /// 是否启用背景模糊
    #[props(default = false)]
    pub blur: bool,

    /// 是否全屏弹窗：为 true 时无论 Mask 写在何处都以 fixed 覆盖整个视口；
    /// 为 false（默认）时以 absolute 限定于最近的定位父容器
    #[props(default = false)]
    pub fullscreen: bool,

    /// 遮罩背景色（默认主题 mask 色）
    #[props(default = "var(--ctrl-mask-bg)".to_string())]
    pub color: String,

    /// 层级
    #[props(default = 1000)]
    pub z_index: i32,

    /// 点击遮罩的回调
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 遮罩内的内容（居中显示）
    pub children: Element,
}

/// Mask 遮罩组件
///
/// 默认 `position: absolute` 限定于最近的定位父容器；
/// 设置 `fullscreen: true` 则无论写在何处都以 `position: fixed` 覆盖整个视口，
/// 适合 Dialog/Drawer 等全屏弹窗场景。
#[allow(non_snake_case)]
pub fn Mask(props: MaskProps) -> Element {
    const CSS: &str = include_str!("../../assets/mask.css");

    if !props.open {
        return rsx! {};
    }

    let mut classes = vec!["ctrl-mask".to_string()];
    if props.fullscreen {
        classes.push("ctrl-mask--fullscreen".into());
    }
    if props.blur {
        classes.push("ctrl-mask--blur".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let mask_class = classes.join(" ");

    let mask_style = format!(
        "background: {color}; z-index: {z}; {extra}",
        color = props.color,
        z = props.z_index,
        extra = props.style,
    );

    let onclick = props.onclick.clone();

    rsx! {
        style { {CSS} }
        div {
            class: "{mask_class}",
            style: "{mask_style}",
            onclick: move |e| {
                if let Some(ref handler) = onclick {
                    handler.call(e);
                }
            },
            {props.children}
        }
    }
}
