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
            h1 {
                "Alert 警告提示"
            }
            p {
                "用于展示重要的提示信息，支持四种类型、可关闭功能。提供两种模式：内联（Inline）嵌入文档流，全局横幅（Banner）fixed 定位浮在页面顶部。支持声明式和命令式两种用法。"
            }

            DemoBox {
                title: "命令式：use_alert_banner() API".to_string(),
                description: Some("通过 use_alert_banner() 获取 API，直接在回调中调用 api.info / success / warning / error 触发全局横幅，无需声明 Alert 组件。".to_string()),
                demo: rsx! { AlertBannerDocsDemo {} },
                code: r#"let mut api = use_alert_banner();
api.warning("存储空间不足".into(), "您的存储空间已使用 95%，请尽快清理文件。".into());
api.success("操作成功".into(), "数据已提交".into());"#.to_string(),
            }

            DemoBox {
                title: "声明式：内联提示（始终显示）".to_string(),
                description: Some("默认模式，嵌入文档流，适合表单提示、卡片内警告。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "sm".to_string(),
                        Alert { r#type: AlertType::Info, title: "系统通知".to_string(), description: "新版本已发布，建议立即更新。".to_string() }
                        Alert { r#type: AlertType::Success, title: "保存成功".to_string(), description: "您的数据已成功保存到服务器。".to_string(), closable: true }
                        Alert { r#type: AlertType::Warning, description: "当前网络不稳定，部分功能可能受限。".to_string() }
                        Alert { r#type: AlertType::Error, title: "加载失败".to_string(), description: "无法加载用户列表，请刷新页面重试。".to_string(), closable: true }
                    }
                },
                code: "Alert { r#type: AlertType::Info, title: \"系统通知\", description: \"...\" }\nAlert { r#type: AlertType::Success, title: \"保存成功\", closable: true }\nAlert { r#type: AlertType::Warning, description: \"网络不稳定\" }\nAlert { r#type: AlertType::Error, title: \"加载失败\", closable: true }".to_string(),
            }

            h2 { "AlertBannerProvider —— 命令式容器" }
            p {
                "使用命令式 API 前，需在应用根节点包裹 AlertBannerProvider："
            }
            DemoBox {
                title: String::new(),
                description: None,
                demo: rsx! { span { style: "font-family: monospace; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);", "AlertBannerProvider {{ /* 你的应用 */ }}" } },
                code: r#"rsx! {
    AlertBannerProvider {
        // 你的应用路由或页面
        Router::<Route> {}
    }
}"#.to_string(),
            }

            h2 { "use_alert_banner() API" }
            PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("api.info(title, desc)", "String, String", "打开一条信息横幅", ""),
                ("api.success(title, desc)", "String, String", "打开一条成功横幅", ""),
                ("api.warning(title, desc)", "String, String", "打开一条警告横幅", ""),
                ("api.error(title, desc)", "String, String", "打开一条错误横幅", ""),
                ("api.open(type, title, desc)", "AlertType, String, String", "通用打开方法（默认 5 秒自动关闭）", ""),
                ("api.open_with_duration(type, title, desc, dur)", "AlertType, String, String, u64", "自定义时长（ms），0 表示不自动关闭", ""),
                ("api.remove(id)", "u32", "移除指定横幅", ""),
            ]}

            h2 { "AlertBannerProvider Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("max_count", "usize", "5", "最大显示数量，超出部分隐藏（先进先出）"),
                ("children", "Element", "—", "子元素"),
            ]}

            h2 { "Alert Props（声明式）" }
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
