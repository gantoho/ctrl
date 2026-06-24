use dioxus::prelude::*;

/// Tag 组件注入的 CSS 样式
const TAG_CSS: &str = r#"
/* ── 标签 ── */
.ctrl-tag {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px 10px;
    border-radius: var(--ctrl-radius-sm);
    font-size: var(--ctrl-font-size-sm);
    line-height: 1.5;
    font-weight: 500;
    white-space: nowrap;
}

/* ── 关闭按钮 ── */
.ctrl-tag__close {
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    margin-left: 2px;
    opacity: 0.7;
    transition: opacity 0.15s;
    background: none;
    border: none;
    outline: none;
    padding: 0;
}
.ctrl-tag__close:hover {
    opacity: 1;
}
"#;

/// Tag 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TagProps {
    /// 标签颜色，直接传入 CSS 颜色值
    #[props(default = "var(--ctrl-primary)".to_string())]
    pub color: String,

    /// 是否可关闭
    #[props(default = false)]
    pub closable: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 关闭事件
    pub onclose: Option<EventHandler<()>>,

    /// 子元素
    pub children: Element,
}

/// Tag 标签组件
#[allow(non_snake_case)]
pub fn Tag(props: TagProps) -> Element {
    let mut visible = use_signal(|| true);
    let onclose = props.onclose.clone();

    if !visible() {
        return rsx! {};
    }

    let tag_class = if props.class.is_empty() {
        "ctrl-tag".to_string()
    } else {
        format!("ctrl-tag {}", props.class)
    };

    // 使用 CSS 变量传递颜色（保持不变，不随信号变化）
    let color_style = format!(
        "color: {c}; background: {c}14; border: 1px solid {c}30; {extra}",
        c = props.color,
        extra = props.style,
    );

    rsx! {
        style { {TAG_CSS} }
        span {
            class: "{tag_class}",
            style: "{color_style}",
            {props.children}
            if props.closable {
                button {
                    class: "ctrl-tag__close",
                    onclick: move |_| {
                        visible.set(false);
                        if let Some(ref handler) = onclose {
                            handler.call(());
                        }
                    },
                    svg {
                        width: "10",
                        height: "10",
                        view_box: "0 0 10 10",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        path { d: "M2 2l6 6M8 2l-6 6" }
                    }
                }
            }
        }
    }
}
