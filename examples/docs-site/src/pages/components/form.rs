use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn FormPage() -> Element {
    rsx! {
div { id: "form", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Form 表单" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "表单组件用于数据录入，支持垂直、水平、内联三种布局，提供 FormItem 管理标签、帮助文本和校验信息。" }

            DemoBox {
                title: "垂直布局".to_string(),
                description: Some("默认 layout 为 vertical。".to_string()),
                demo: rsx! { BasicFormDemo {} },
                code: "Form {\n    FormItem { label: \"用户名\".to_string(), required: true,\n        Input { placeholder: \"请输入用户名\" }\n    }\n    FormItem { label: \"密码\".to_string(), required: true,\n        Input { r#type: \"password\", placeholder: \"请输入密码\" }\n    }\n    FormItem {\n        Button { variant: Variant::Primary, \"提交\" }\n    }\n}".to_string(),
            }

            DemoBox {
                title: "水平布局".to_string(),
                description: Some("设置 layout 为 horizontal，标签在左侧。".to_string()),
                demo: rsx! {
                    Form { layout: "horizontal".to_string(),
                        FormItem { label: "姓名".to_string(), required: true, Input { placeholder: "请输入姓名" } }
                        FormItem { label: "邮箱".to_string(), required: true, Input { placeholder: "请输入邮箱" } }
                        FormItem { label: "备注".to_string(), Input { placeholder: "选填" } }
                    }
                },
                code: "Form { layout: \"horizontal\".to_string(), ... }".to_string(),
            }

            DemoBox {
                title: "帮助文本与错误".to_string(),
                description: Some("通过 FormItem 的 help 和 error 属性展示辅助信息。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",
                        FormItem { label: "用户名".to_string(), help: "请输入 4-16 个字符".to_string(), Input { placeholder: "请输入用户名" } }
                        FormItem { label: "邮箱".to_string(), error: "邮箱格式不正确".to_string(), Input { placeholder: "请输入邮箱" } }
                    }
                },
                code: "FormItem { label: \"...\".to_string(), help: \"帮助文本\".to_string(), ... }\nFormItem { label: \"...\".to_string(), error: \"错误信息\".to_string(), ... }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Form Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("layout", "String", "\"vertical\"", "布局方式：vertical / horizontal / inline"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onsubmit", "Option<EventHandler<Rc<FormData>>>", "None", "提交回调"),
                ("children", "Element", "—", "子元素"),
            ]}

            h3 { style: "font-size: 1rem; font-weight: 600; color: var(--ctrl-text); margin: 24px 0 12px;", "FormItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("label", "String", "\"\"", "标签文本"),
                ("required", "bool", "false", "是否必填"),
                ("help", "String", "\"\"", "帮助文本"),
                ("error", "String", "\"\"", "错误信息"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "表单控件"),
            ]}
        }
    }
}
