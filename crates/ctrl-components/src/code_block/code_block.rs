use dioxus::prelude::*;

/// CodeBlock 代码块组件
#[derive(Props, PartialEq, Clone)]
pub struct CodeBlockProps {
    /// 代码内容
    pub code: String,

    /// 语言标识（显示在右上角）
    pub language: Option<String>,

    /// 是否显示行号
    #[props(default = false)]
    pub show_line_numbers: bool,

    /// 是否显示复制按钮
    #[props(default = true)]
    pub show_copy: bool,

    /// 是否紧凑模式
    #[props(default = false)]
    pub compact: bool,

    /// 行高（用于复制时被通知）
    pub oncopy: Option<EventHandler<String>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// 复制文本到剪贴板
#[allow(unused_variables)]
fn copy_code(code: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let clipboard = window.navigator().clipboard();
            let _ = clipboard.write_text(code);
        }
    }
}

#[allow(non_snake_case)]
pub fn CodeBlock(props: CodeBlockProps) -> Element {
    const CSS: &str = include_str!("../../assets/code_block.css");
    let mut copied = use_signal(|| false);
    let oncopy = props.oncopy.clone();
    let code: String = props.code.clone();
    let code_for_copy = code.clone();

    let class = if props.class.is_empty() {
        "ctrl-code-block".to_string()
    } else {
        format!("ctrl-code-block {}", props.class)
    };

    let mut wrapper_class = class.clone();
    if props.show_line_numbers {
        wrapper_class.push_str(" ctrl-code-block--with-lines");
    }
    if props.compact {
        wrapper_class.push_str(" ctrl-code-block--compact");
    }

    let lang = props.language.unwrap_or_default();
    let line_count = code.lines().count();
    let line_num_width = if line_count > 99 { "40px" } else { "32px" };
    let lines: Vec<&str> = code.lines().collect();

    rsx! {
        style { {CSS} }
        div { class: "{wrapper_class}", style: "{props.style}",
            // 顶部栏：语言 + 复制按钮
            div { class: "ctrl-code-block__header",
                span { class: "ctrl-code-block__lang", "{lang}" }
                if props.show_copy {
                    button {
                        class: "ctrl-code-block__copy",
                        onclick: move |_| {
                            copy_code(&code_for_copy);
                            copied.set(true);
                            if let Some(ref handler) = oncopy {
                                handler.call(code_for_copy.clone());
                            }
                            // 1.5 秒后重置复制状态
                            let mut c = copied;
                            gloo_timers::callback::Timeout::new(1500, move || {
                                c.set(false);
                            }).forget();
                        },
                        if copied() {
                            "已复制"
                        } else {
                            "复制"
                        }
                    }
                }
            }
            // 代码内容
            pre { class: "ctrl-code-block__pre",
                code { class: "ctrl-code-block__code",
                    if props.show_line_numbers {
                        // 带行号
                        for (i, line) in lines.iter().enumerate() {
                            span { class: "ctrl-code-block__line",
                                span {
                                    class: "ctrl-code-block__line-num",
                                    style: "min-width:{line_num_width}",
                                    "{i + 1}"
                                }
                                span { class: "ctrl-code-block__line-code",
                                    "{line}"
                                    "\n"
                                }
                            }
                        }
                    } else {
                        "{code}"
                    }
                }
            }
        }
    }
}
