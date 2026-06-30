use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn ResultPage() -> Element {
    rsx! {
div { id: "result", style: "margin-top: 64px;",
            h1 {
                "Result 结果页"
            }
            p {
                "用于展示操作结果反馈。支持 4 种状态（Info / Success / Warning / Error），可自定义图标、标题、副标题和底部操作区。"
            }

            DemoBox {
                title: "四种状态".to_string(),
                description: Some("通过 status 控制状态，内置 success / info / warning / error 四种。".to_string()),
                demo: rsx! {
                    div { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
                        Result {
                            status: ResultStatus::Success,
                            title: "操作成功".to_string(),
                        }
                        Result {
                            status: ResultStatus::Info,
                            title: "提示信息".to_string(),
                        }
                        Result {
                            status: ResultStatus::Warning,
                            title: "注意警告".to_string(),
                        }
                        Result {
                            status: ResultStatus::Error,
                            title: "操作失败".to_string(),
                        }
                    }
                },
                code: r#"Result { status: ResultStatus::Success, title: "操作成功".to_string() }
Result { status: ResultStatus::Info,    title: "提示信息".to_string() }
Result { status: ResultStatus::Warning, title: "注意警告".to_string() }
Result { status: ResultStatus::Error,   title: "操作失败".to_string() }"#.to_string(),
            }

            DemoBox {
                title: "带副标题和操作".to_string(),
                description: Some("通过 subtitle 补充说明，children 传入底部操作按钮。".to_string()),
                demo: rsx! {
                    Result {
                        status: ResultStatus::Success,
                        title: "提交成功".to_string(),
                        subtitle: "您的申请已提交，预计 3 个工作日内完成审核。审核结果将以短信通知。".to_string(),
                        Button { variant: Variant::Primary, "返回首页" }
                        Button { variant: Variant::Outline, "查看详情" }
                    }
                },
                code: r#"Result {
    status: ResultStatus::Success,
    title: "提交成功".to_string(),
    subtitle: "您的申请已提交，预计 3 个工作日内审核。".to_string(),
    Button { variant: Variant::Primary, "返回首页" }
    Button { variant: Variant::Outline, "查看详情" }
}"#.to_string(),
            }

            DemoBox {
                title: "自定义图标".to_string(),
                description: Some("通过 icon 属性传入自定义图标内容，覆盖默认 emoji。".to_string()),
                demo: rsx! {
                    Result {
                        status: ResultStatus::Info,
                        title: "正在处理中".to_string(),
                        subtitle: "请稍候，系统正在处理您的请求...".to_string(),
                        icon: rsx! {
                            div { style: "width: 72px; height: 72px; display: flex; align-items: center; justify-content: center; margin: 0 auto;", Loading { size: Size::Lg } }
                        }
                    }
                },
                code: r#"Result {
    status: ResultStatus::Info,
    title: "正在处理中".to_string(),
    subtitle: "请稍候...".to_string(),
    icon: rsx! { div { Loading { size: Size::Lg } } }
}"#.to_string(),
            }

            h2 { "Result Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("status", "ResultStatus", "Info", "状态：Success / Info / Warning / Error"),
                ("title", "String", "\"\"", "标题"),
                ("subtitle", "String", "\"\"", "副标题 / 描述"),
                ("icon", "Option<Element>", "None", "自定义图标（覆盖默认 emoji）"),
                ("class", "String", "\"\"", "自定义 CSS 类名"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "底部操作区（按钮等）"),
            ]}

            h2 { "ResultStatus 枚举" }
            PropsTable { headers: vec!["值".to_string(), "默认图标".to_string(), "默认颜色".to_string(), "".to_string()], rows: vec![
                ("Info (默认)", "ℹ️", "var(--ctrl-info)", ""),
                ("Success", "✅", "var(--ctrl-success)", ""),
                ("Warning", "⚠️", "var(--ctrl-warning)", ""),
                ("Error", "❌", "var(--ctrl-danger)", ""),
            ]}
        }
    }
}
