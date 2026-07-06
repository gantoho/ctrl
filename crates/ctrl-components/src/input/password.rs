use dioxus::prelude::*;
use ctrl_core::types::Size;

/// InputPassword 密码输入框组件属性
#[derive(Props, PartialEq, Clone)]
pub struct InputPasswordProps {
    /// 输入框尺寸（默认 md）
    #[props(default = Size::default())]
    pub size: Size,

    /// 当前值
    #[props(default = "".to_string())]
    pub value: String,

    /// 占位文本
    #[props(default = "".to_string())]
    pub placeholder: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,

    /// 是否显示错误状态
    #[props(default = false)]
    pub error: bool,

    /// 是否显示清除按钮（value 非空时）
    #[props(default = false)]
    pub allow_clear: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 输入事件
    pub oninput: Option<EventHandler<String>>,

    /// 清除事件
    pub onclear: Option<EventHandler<()>>,

    /// 获得焦点事件
    pub onfocus: Option<EventHandler<FocusEvent>>,

    /// 失去焦点事件
    pub onblur: Option<EventHandler<FocusEvent>>,
}

/// InputPassword 密码输入框组件
///
/// 内置密码显隐切换开关（眼睛图标），通过 suffix 向基础 Input 注入切换按钮。
///
/// # 示例
///
/// ```rust
/// let mut password = use_signal(|| String::new());
/// InputPassword {
///     placeholder: "请输入密码",
///     value: password(),
///     oninput: move |v| password.set(v),
/// }
/// ```
#[allow(non_snake_case)]
pub fn InputPassword(props: InputPasswordProps) -> Element {
    let visible = use_signal(|| false);

    let toggle_icon = {
        let mut vis = visible;
        move |_| {
            vis.set(!vis());
        }
    };

    let icon_class = "ctrl-input__suffix-btn";
    let icon = if visible() {
        // 眼睛睁开图标（显示密码时的状态）
        rsx! {
            span {
                class: "{icon_class}",
                title: "隐藏密码",
                onclick: toggle_icon,
                // ▼ 睁开眼
                svg {
                    width: "16",
                    height: "16",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" }
                    circle { cx: "12", cy: "12", r: "3" }
                }
            }
        }
    } else {
        // 眼睛闭上图标（隐藏密码时的状态）
        rsx! {
            span {
                class: "{icon_class}",
                title: "显示密码",
                onclick: toggle_icon,
                // 闭眼
                svg {
                    width: "16",
                    height: "16",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24" }
                    line { x1: "1", y1: "1", x2: "23", y2: "23" }
                }
            }
        }
    };

    rsx! {
        // 使用基础 Input 组件，通过 suffix 注入密码显隐切换
        super::input::Input {
            size: props.size,
            value: props.value.clone(),
            placeholder: props.placeholder.clone(),
            disabled: props.disabled,
            readonly: props.readonly,
            error: props.error,
            r#type: if visible() { "text".to_string() } else { "password".to_string() },
            class: props.class.clone(),
            style: props.style.clone(),
            allow_clear: props.allow_clear,
            suffix: rsx! { {icon} },
            oninput: props.oninput.clone(),
            onclear: props.onclear.clone(),
            onfocus: props.onfocus.clone(),
            onblur: props.onblur.clone(),
        }
    }
}
