use dioxus::prelude::*;

/// InputTag 标签输入组件属性
#[derive(Props, PartialEq, Clone)]
pub struct InputTagProps {
    /// 标签列表
    #[props(default = Vec::new())]
    pub value: Vec<String>,

    /// 标签变化回调
    pub onchange: Option<EventHandler<Vec<String>>>,

    /// 占位提示文字
    #[props(default = "输入后按回车添加".to_string())]
    pub placeholder: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 最多标签数量（None 表示不限制）
    #[props(default = None)]
    pub max: Option<usize>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// InputTag 标签输入组件
///
/// 输入文本后按回车键（或逗号）生成标签，点击标签上的 X 可删除。
#[allow(non_snake_case)]
pub fn InputTag(props: InputTagProps) -> Element {
    const CSS: &str = include_str!("../../assets/input_tag.css");
    let mut input_value = use_signal(|| String::new());
    let mut tags = use_signal(|| props.value.clone());

    let wrapper_class = {
        let base = if props.class.is_empty() {
            "ctrl-input-tag".to_string()
        } else {
            format!("ctrl-input-tag {}", props.class)
        };
        if props.disabled {
            format!("{} ctrl-input-tag--disabled", base)
        } else {
            base
        }
    };

    let max_reached = move || match props.max {
        Some(max) => tags().len() >= max,
        None => false,
    };

    let notify_change = {
        let onchange = props.onchange.clone();
        move |new_tags: Vec<String>| {
            if let Some(ref cb) = onchange {
                cb.call(new_tags);
            }
        }
    };

    let mut handle_keydown = {
        let mut input_value = input_value;
        let mut tags = tags;
        let max_reached = max_reached;
        let notify_change = notify_change.clone();
        move |evt: KeyboardEvent| {
            if props.disabled {
                return;
            }
            match evt.key() {
                Key::Enter => {
                    evt.prevent_default();
                    let trimmed = input_value().trim().to_string();
                    if !trimmed.is_empty() && !max_reached() {
                        let new: Vec<String> = trimmed.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                        if !new.is_empty() {
                            let mut cur = tags();
                            for t in new { if !cur.contains(&t) { cur.push(t); } }
                            tags.set(cur.clone());
                            input_value.set(String::new());
                            notify_change(cur);
                        }
                    }
                }
                Key::Character(ref ch) if ch == "," => {
                    evt.prevent_default();
                    let trimmed = input_value().trim().to_string();
                    if !trimmed.is_empty() && !max_reached() {
                        let new: Vec<String> = trimmed.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                        if !new.is_empty() {
                            let mut cur = tags();
                            for t in new { if !cur.contains(&t) { cur.push(t); } }
                            tags.set(cur.clone());
                            input_value.set(String::new());
                            notify_change(cur);
                        }
                    }
                }
                Key::Backspace => {
                    if input_value().is_empty() {
                        let mut cur = tags();
                        if !cur.is_empty() {
                            cur.pop();
                            tags.set(cur.clone());
                            notify_change(cur);
                        }
                    }
                }
                _ => {}
            }
        }
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            // 标签列表
            for (i, tag) in tags().iter().enumerate() {
                span {
                    key: "tag-{i}",
                    class: "ctrl-input-tag__tag",
                    span { class: "ctrl-input-tag__tag-text", "{tag}" }
                    button {
                        class: "ctrl-input-tag__close",
                        disabled: props.disabled,
                        onclick: {
                            let mut tags = tags;
                            let nc = notify_change.clone();
                            move |_| {
                                let mut cur = tags();
                                cur.remove(i);
                                tags.set(cur.clone());
                                nc(cur);
                            }
                        },
                        "×"
                    }
                }
            }
            // 输入框
            input {
                class: "ctrl-input-tag__input",
                r#type: "text",
                placeholder: {
                    if max_reached() {
                        "已达到上限".to_string()
                    } else {
                        props.placeholder.clone()
                    }
                },
                value: "{input_value}",
                disabled: props.disabled,
                readonly: max_reached(),
                oninput: move |evt| input_value.set(evt.value()),
                onkeydown: handle_keydown,
            }
        }
    }
}
