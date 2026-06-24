use dioxus::prelude::*;
use ctrl_core::types::Size;

/// Switch 组件注入的 CSS 样式
const SWITCH_CSS: &str = r#"
/* ── 开关轨道 ── */
.ctrl-switch {
    position: relative;
    display: inline-flex;
    align-items: center;
    border-radius: 999px;
    transition: all var(--ctrl-transition);
    outline: none;
    border: none;
    cursor: pointer;
}

/* ── 尺寸 ── */
.ctrl-switch--sm { width: 32px; height: 18px; }
.ctrl-switch--md { width: 40px; height: 22px; }
.ctrl-switch--lg { width: 48px; height: 26px; }

/* ── 轨道颜色 ── */
.ctrl-switch--unchecked { background: var(--ctrl-border-hover); }
.ctrl-switch--checked { background: var(--ctrl-primary); }

/* ── 禁用 ── */
.ctrl-switch--disabled {
    cursor: not-allowed;
}
.ctrl-switch--disabled.ctrl-switch--unchecked { background: #D1D5DB; }
.ctrl-switch--disabled.ctrl-switch--checked { background: #A5B4FC; }

/* ── 滑块 ── */
.ctrl-switch__thumb {
    border-radius: 50%;
    position: absolute;
    top: 50%;
    transform: translateY(-50%);
    transition: all var(--ctrl-transition);
    background: white;
    box-shadow: 0 1px 3px rgba(0,0,0,0.15);
}
.ctrl-switch--disabled .ctrl-switch__thumb {
    background: var(--ctrl-bg-disabled);
    box-shadow: none;
}

/* ── 滑块位置和尺寸 ── */
.ctrl-switch--sm .ctrl-switch__thumb { width: 14px; height: 14px; }
.ctrl-switch--md .ctrl-switch__thumb { width: 18px; height: 18px; }
.ctrl-switch--lg .ctrl-switch__thumb { width: 22px; height: 22px; }

.ctrl-switch--sm.ctrl-switch--unchecked .ctrl-switch__thumb { left: 2px; }
.ctrl-switch--sm.ctrl-switch--checked .ctrl-switch__thumb { left: 16px; }
.ctrl-switch--md.ctrl-switch--unchecked .ctrl-switch__thumb { left: 2px; }
.ctrl-switch--md.ctrl-switch--checked .ctrl-switch__thumb { left: 20px; }
.ctrl-switch--lg.ctrl-switch--unchecked .ctrl-switch__thumb { left: 2px; }
.ctrl-switch--lg.ctrl-switch--checked .ctrl-switch__thumb { left: 24px; }
"#;

/// 构建开关 class 列表
fn build_switch_class(size: Size, checked: bool, disabled: bool) -> String {
    let mut classes = vec!["ctrl-switch".to_string()];

    match size {
        Size::Sm => classes.push("ctrl-switch--sm".into()),
        Size::Md => classes.push("ctrl-switch--md".into()),
        Size::Lg => classes.push("ctrl-switch--lg".into()),
    }

    if checked {
        classes.push("ctrl-switch--checked".into());
    } else {
        classes.push("ctrl-switch--unchecked".into());
    }

    if disabled {
        classes.push("ctrl-switch--disabled".into());
    }

    classes.join(" ")
}

/// Switch 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SwitchProps {
    /// 是否选中
    #[props(default = false)]
    pub checked: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 尺寸
    #[props(default = Size::default())]
    pub size: Size,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 状态变化事件
    pub onchange: Option<EventHandler<bool>>,
}

/// Switch 开关组件
#[allow(non_snake_case)]
pub fn Switch(props: SwitchProps) -> Element {
    let switch_class = build_switch_class(props.size, props.checked, props.disabled);

    let user_class = if props.class.is_empty() {
        switch_class
    } else {
        format!("{} {}", switch_class, props.class)
    };

    let onchange = props.onchange.clone();
    let checked = props.checked;
    let disabled = props.disabled;

    rsx! {
        style { {SWITCH_CSS} }
        div {
            class: "{user_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            role: "switch",
            aria_checked: if checked { "true" } else { "false" },
            onclick: move |_| {
                if !disabled {
                    if let Some(ref handler) = onchange {
                        handler.call(!checked);
                    }
                }
            },
            span { class: "ctrl-switch__thumb" }
        }
    }
}
