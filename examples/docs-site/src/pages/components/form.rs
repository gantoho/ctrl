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
            h1 { "Form 表单" }
            p { "表单组件用于数据录入，支持垂直、水平、内联三种布局，提供 FormItem 管理标签、帮助文本和校验信息。" }

            DemoBox {
                title: "垂直布局".to_string(),
                description: Some("默认 layout 为 vertical。".to_string()),
                demo: rsx! { BasicFormDemo {} },
                code: "Form {\n    FormItem { name: \"username\".to_string(), label: \"用户名\".to_string(), required: true,\n        Input { placeholder: \"请输入用户名\" }\n    }\n    FormItem { name: \"password\".to_string(), label: \"密码\".to_string(), required: true,\n        Input { r#type: \"password\", placeholder: \"请输入密码\" }\n    }\n    FormItem { name: \"submit\".to_string(),\n        Button { variant: Variant::Primary, \"提交\" }\n    }\n}".to_string(),
            }

            DemoBox {
                title: "水平布局".to_string(),
                description: Some("设置 layout 为 horizontal，标签在左侧。".to_string()),
                demo: rsx! {
                    Form { layout: Layout::Horizontal,
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
                    Space { direction: Direction::Vertical, gap: "md".to_string(),
                        FormItem { label: "用户名".to_string(), help: "请输入 4-16 个字符".to_string(), Input { placeholder: "请输入用户名" } }
                        FormItem { label: "邮箱".to_string(), error: "邮箱格式不正确".to_string(), Input { placeholder: "请输入邮箱" } }
                    }
                },
                code: "FormItem { name: \"...\".to_string(), label: \"...\".to_string(), help: \"帮助文本\".to_string(), ... }\nFormItem { name: \"...\".to_string(), label: \"...\".to_string(), error: \"错误信息\".to_string(), ... }".to_string(),
            }

            DemoBox {
                title: "表单验证".to_string(),
                description: Some("FormItem 内置验证规则系统，支持 required、min_length、email、pattern 及自定义验证函数。通过 validate_trigger 控制校验时机（Submit/Blur/Change）。".to_string()),
                demo: rsx! { FormValidationDemo {} },
                code: "Form {\n    validate_trigger: ValidationTrigger::Blur,\n    scroll_to_error: true,\n    onsubmit: move |_data| { submitted.set(true); },\n    FormItem {\n        name: \"username\".to_string(),\n        label: \"用户名\".to_string(),\n        required: true,\n        rules: vec![ValidationRule::min_length(3, \"用户名至少 3 个字符\")],\n        value: username(),\n        Input { ... }\n    }\n    FormItem {\n        name: \"confirm\".to_string(),\n        label: \"确认密码\".to_string(),\n        rules: vec![ValidationRule::custom(move |val| { ... })],\n        value: confirm(),\n        Input { ... }\n    }\n}".to_string(),
            }

            h2 { "Form Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("layout", "String", "\"vertical\"", "布局方式：vertical / horizontal / inline"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("validate_trigger", "ValidationTrigger", "Submit", "全局验证触发时机：Submit/Blur/Change"),
                ("validate_on_submit", "bool", "true", "提交时是否自动校验"),
                ("scroll_to_error", "bool", "false", "校验失败时是否滚动到第一个错误字段"),
                ("disabled", "bool", "false", "是否禁用整个表单"),
                ("onsubmit", "Option<EventHandler<Rc<FormData>>>", "None", "校验通过后的提交回调"),
                ("onvalidate", "Option<EventHandler<FormValidationResult>>", "None", "校验完成回调"),
                ("children", "Element", "—", "子元素"),
            ]}

            h3 { "FormItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("name", "String", "—（必填）", "字段唯一标识，用于注册到 FormContext"),
                ("label", "String", "\"\"", "标签文本"),
                ("rules", "Vec<ValidationRule>", "[]", "验证规则列表"),
                ("value", "String", "\"\"", "当前字段值（用于校验）"),
                ("validate_trigger", "Option<ValidationTrigger>", "None", "字段级验证触发时机，覆盖全局设置"),
                ("required", "bool", "false", "是否必填（自动添加 required 规则）"),
                ("help", "String", "\"\"", "帮助文本"),
                ("error", "String", "\"\"", "手动错误信息（优先级高于自动校验）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "表单控件"),
            ]}
        }
    }
}
