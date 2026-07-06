use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn InputPage() -> Element {
    rsx! {
div { id: "input", style: "margin-top: 64px;",
            h1 {
                "Input 输入框"
            }
            p {
                "输入框用于文本输入。采用受控组件模式，支持尺寸、状态、前后缀、清除按钮等。"
            }

            div {
                style: "padding: 12px 16px; background: var(--ctrl-primary-light); border-radius: var(--ctrl-radius-md); margin-bottom: 32px; display: flex; align-items: flex-start; gap: 8px; font-size: var(--ctrl-font-size-md);",
                span { "💡" }
                span { strong { "注意: " } "Input 使用受控组件模式，必须通过 value + oninput 管理值。" }
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 oninput 实现双向绑定。".to_string()),
                demo: rsx! { BasicInputDemo {} },
                code: "let mut value = use_signal(|| String::new());\n\nInput {\n    placeholder: \"请输入内容\",\n    value: value(),\n    oninput: move |v: String| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "前缀和后缀".to_string(),
                description: Some("通过 prefix / suffix 属性在输入框前后嵌入图标或文本。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "sm".to_string(),
                        Input {
                            placeholder: "搜索内容",
                            prefix: rsx! {
                                svg {
                                    width: "14", height: "14", view_box: "0 0 24 24",
                                    fill: "none", stroke: "currentColor", stroke_width: "2",
                                    circle { cx: "11", cy: "11", r: "8" }
                                    line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                                }
                            }
                        }
                        Input {
                            placeholder: "域名",
                            suffix: rsx! { span { style: "color: var(--ctrl-text-secondary);", ".com" } }
                        }
                    }
                },
                code: "Input {\n    prefix: rsx! { svg { ... } },\n    placeholder: \"搜索内容\"\n}\nInput {\n    suffix: rsx! { span { \".com\" } },\n    placeholder: \"域名\"\n}".to_string(),
            }

            DemoBox {
                title: "清除按钮".to_string(),
                description: Some("allow_clear 在 value 非空时显示清除按钮，同时触发 oninput 和 onclear。".to_string()),
                demo: rsx! { InputClearDemo {} },
                code: "let mut val = use_signal(|| String::from(\"可清除的内容\"));\nInput {\n    allow_clear: true,\n    value: val(),\n    oninput: move |v: String| val.set(v),\n    onclear: move |_| val.set(String::new()),\n}".to_string(),
            }

            DemoBox {
                title: "三种尺寸".to_string(),
                description: Some("Sm 28px / Md 36px / Lg 44px".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "sm".to_string(),
                        Input { placeholder: "Small", size: Size::Sm }
                        Input { placeholder: "Medium", size: Size::Md }
                        Input { placeholder: "Large", size: Size::Lg }
                    }
                },
                code: "Input { placeholder: \"Small\", size: Size::Sm }\nInput { placeholder: \"Medium\", size: Size::Md }\nInput { placeholder: \"Large\", size: Size::Lg }".to_string(),
            }

            DemoBox {
                title: "状态".to_string(),
                description: Some("禁用、只读、错误状态。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "sm".to_string(),
                        Input { placeholder: "禁用状态", disabled: true }
                        Input { placeholder: "只读状态", readonly: true, value: "只读内容".to_string() }
                        Input { placeholder: "错误状态", error: true }
                    }
                },
                code: "Input { placeholder: \"禁用状态\", disabled: true }\nInput { placeholder: \"只读状态\", readonly: true, value: \"只读内容\" }\nInput { placeholder: \"错误状态\", error: true }".to_string(),
            }

            h2 { "Input Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("size", "Size", "Md", "输入框尺寸"),
                ("value", "String", "\"\"", "当前值（受控）"),
                ("placeholder", "String", "\"\"", "占位文本"),
                ("disabled", "bool", "false", "是否禁用"),
                ("readonly", "bool", "false", "是否只读"),
                ("error", "bool", "false", "是否错误状态"),
                ("r#type", "String", "\"text\"", "原生 input type"),
                ("prefix", "Option<Element>", "None", "前缀内容"),
                ("suffix", "Option<Element>", "None", "后缀内容"),
                ("allow_clear", "bool", "false", "是否显示清除按钮"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("oninput", "Option<EventHandler<String>>", "None", "输入事件"),
                ("onclear", "Option<EventHandler<()>>", "None", "清除事件"),
                ("onfocus", "Option<EventHandler<FocusEvent>>", "None", "获得焦点事件"),
                ("onblur", "Option<EventHandler<FocusEvent>>", "None", "失去焦点事件"),
            ]}

            h2 { "InputPassword 密码输入框" }
            p {
                "密码输入框基于 Input 实现，内置密码显隐切换按钮。属性和 Input 基本一致，但移除了 prefix / suffix / allow_clear（由组件内部控制）。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("点击右侧眼睛图标切换密码显隐。".to_string()),
                demo: rsx! { InputPasswordDemo {} },
                code: "let mut password = use_signal(|| String::new());\nInputPassword {\n    placeholder: \"请输入密码\",\n    value: password(),\n    oninput: move |v: String| password.set(v),\n}".to_string(),
            }

            h3 { "InputPassword Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("size", "Size", "Md", "输入框尺寸"),
                ("value", "String", "\"\"", "当前值"),
                ("placeholder", "String", "\"\"", "占位文本"),
                ("disabled", "bool", "false", "是否禁用"),
                ("readonly", "bool", "false", "是否只读"),
                ("error", "bool", "false", "是否错误状态"),
                ("allow_clear", "bool", "false", "是否显示清除按钮"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("oninput", "Option<EventHandler<String>>", "None", "输入事件"),
                ("onclear", "Option<EventHandler<()>>", "None", "清除事件"),
                ("onfocus", "Option<EventHandler<FocusEvent>>", "None", "获得焦点事件"),
                ("onblur", "Option<EventHandler<FocusEvent>>", "None", "失去焦点事件"),
            ]}

            h2 { "InputOtp 验证码输入" }
            p {
                "一次性验证码输入组件，适用于短信验证码、PIN 码等场景。渲染为多个独立的单字符输入框。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("输入字符自动填入下一个格子，Backspace 回退。支持 onchange 和 oncomplete 事件。".to_string()),
                demo: rsx! { InputOtpDemo {} },
                code: "let mut code = use_signal(|| String::new());\nInputOtp {\n    value: code(),\n    onchange: move |v: String| code.set(v),\n    oncomplete: move |v: String| { /* 自动提交 */ },\n}".to_string(),
            }

            DemoBox {
                title: "密码模式".to_string(),
                description: Some("mask 属性将输入字符显示为圆点。".to_string()),
                demo: rsx! { InputOtpMaskDemo {} },
                code: "InputOtp {\n    mask: true,\n    value: code(),\n    onchange: move |v: String| code.set(v),\n}".to_string(),
            }

            h3 { "InputOtp Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("length", "usize", "6", "验证码位数"),
                ("value", "String", "\"\"", "当前值"),
                ("size", "Size", "Md", "尺寸"),
                ("disabled", "bool", "false", "是否禁用"),
                ("error", "bool", "false", "是否错误状态"),
                ("mask", "bool", "false", "是否密码模式（显示圆点）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<String>>", "None", "值变化事件"),
                ("oncomplete", "Option<EventHandler<String>>", "None", "全部输入完成时触发"),
            ]}
        }
    }
}
