use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn AlertPage() -> Element {
    rsx! {
div { id: "alert", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Alert 警告提示"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "用于展示重要的提示信息，支持四种类型、可关闭功能。提供两种模式：内联（Inline）嵌入文档流，全局横幅（Banner）fixed 定位浮在页面顶部。"
            }

            DemoBox {
                title: "全局顶部横幅（点击触发）".to_string(),
                description: Some("设置 mode: Banner，Alert 会以 fixed 定位浮在页面最顶部。配合 closable 和 duration 实现自动消失。".to_string()),
                demo: rsx! { AlertBannerDocsDemo {} },
                code: "let mut show = use_signal(|| false);\n\nif show() {\n    Alert {\n        r#type: AlertType::Info,\n        title: \"系统维护通知\",\n        description: \"今晚 22:00 - 23:00 将进行系统升级\",\n        mode: AlertMode::Banner,\n        closable: true,\n        duration: 5000,\n    }\n}\n\nButton { onclick: move |_| show.set(true), \"显示横幅\" }".to_string(),
            }

            DemoBox {
                title: "内联提示（始终显示）".to_string(),
                description: Some("默认模式，嵌入文档流，适合表单提示、卡片内警告。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 480px;",
                        Alert { r#type: AlertType::Info, title: "系统通知".to_string(), description: "新版本已发布，建议立即更新。".to_string() }
                        Alert { r#type: AlertType::Success, title: "保存成功".to_string(), description: "您的数据已成功保存到服务器。".to_string(), closable: true }
                        Alert { r#type: AlertType::Warning, description: "当前网络不稳定，部分功能可能受限。".to_string() }
                        Alert { r#type: AlertType::Error, title: "加载失败".to_string(), description: "无法加载用户列表，请刷新页面重试。".to_string(), closable: true }
                    }
                },
                code: "Alert { r#type: AlertType::Info, title: \"系统通知\", description: \"...\" }\nAlert { r#type: AlertType::Success, title: \"保存成功\", closable: true }\nAlert { r#type: AlertType::Warning, description: \"网络不稳定\" }\nAlert { r#type: AlertType::Error, title: \"加载失败\", closable: true }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "AlertBannerContainer Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("max_count", "usize", "5", "最大显示数量，超出部分隐藏（先进先出）"),
                ("children", "Element", "—", "子元素（多条 Banner Alert）"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Alert Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("type", "AlertType", "Info", "提示类型（Info / Success / Warning / Error）"),
                ("title", "String", "\"\"", "标题"),
                ("description", "String", "\"\"", "描述内容"),
                ("closable", "bool", "false", "是否可关闭"),
                ("show_icon", "bool", "true", "是否显示图标"),
                ("mode", "AlertMode", "Inline", "显示模式（Inline 内联 / Banner 全局横幅）"),
                ("duration", "u64", "0", "自动关闭时间(ms)，仅 Banner 模式有效，0 表示不自动关闭"),
                ("closing", "bool", "false", "外部关闭信号，设为 true 触发退出动画"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭回调"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}
