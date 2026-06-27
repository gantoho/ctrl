use dioxus::prelude::*;
use std::rc::Rc;

/// Form 表单组件属性
#[derive(Props, PartialEq, Clone)]
pub struct FormProps {
    /// 布局方式："vertical" | "horizontal" | "inline"
    #[props(default = "vertical".to_string())]
    pub layout: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 提交回调
    pub onsubmit: Option<EventHandler<Rc<FormData>>>,

    /// 子元素
    pub children: Element,
}

/// FormItem 表单项属性
#[derive(Props, PartialEq, Clone)]
pub struct FormItemProps {
    /// 标签文本
    #[props(default = "".to_string())]
    pub label: String,

    /// 是否必填
    #[props(default = false)]
    pub required: bool,

    /// 帮助文本
    #[props(default = "".to_string())]
    pub help: String,

    /// 错误信息
    #[props(default = "".to_string())]
    pub error: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（表单控件）
    pub children: Element,
}

/// Form 表单组件
#[allow(non_snake_case)]
pub fn Form(props: FormProps) -> Element {
    const CSS: &str = include_str!("../../assets/form.css");

    let form_class = {
        let mut c = String::from("ctrl-form");
        if props.layout == "horizontal" {
            c.push_str(" ctrl-form--horizontal");
        } else if props.layout == "inline" {
            c.push_str(" ctrl-form--inline");
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        if let Some(ref cb) = props.onsubmit {
            cb.call(evt.data());
        }
    };

    rsx! {
        style { {CSS} }
        form {
            class: "{form_class}",
            style: "{props.style}",
            onsubmit: handle_submit,
            {props.children}
        }
    }
}

/// FormItem 表单项组件
#[allow(non_snake_case)]
pub fn FormItem(props: FormItemProps) -> Element {
    let label_class = {
        if props.required {
            "ctrl-form__label ctrl-form__label--required"
        } else {
            "ctrl-form__label"
        }
    };

    let item_class = {
        let mut c = String::from("ctrl-form__item");
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    rsx! {
        div { class: "{item_class}",
            if !props.label.is_empty() {
                label { class: "{label_class}", "{props.label}" }
            }
            div { class: "ctrl-form__control",
                {props.children}
            }
            if !props.error.is_empty() {
                div { class: "ctrl-form__error", "{props.error}" }
            } else if !props.help.is_empty() {
                div { class: "ctrl-form__help", "{props.help}" }
            }
        }
    }
}