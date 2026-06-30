use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn NotificationPage() -> Element {
    rsx! {
div { id: "notification", style: "margin-top: 64px;",
            h1 { "Notification 通知提醒" }
            p { "全局通知提醒，支持四种类型和自动关闭。支持声明式和命令式两种用法。" }

            DemoBox {
                title: "命令式：use_notification() API".to_string(),
                description: Some("通过 use_notification() 获取 API，直接在回调中调用 api.info / success / warning / error 触发通知。".to_string()),
                demo: rsx! { NotificationDocs {} },
                code: "let mut api = use_notification();\napi.info(\"通知\".to_string(), \"这是一条通知\".to_string());\napi.success(\"成功\".to_string(), \"操作成功\".to_string());\napi.warning(\"警告\".to_string(), \"请注意\".to_string());\napi.error(\"错误\".to_string(), \"操作失败\".to_string());".to_string(),
            }

            h2 { "NotificationProvider —— 命令式容器" }
            p {
                "使用命令式 API 前，需在应用根节点包裹 NotificationProvider："
            }
            DemoBox {
                title: String::new(),
                description: None,
                demo: rsx! { span { style: "font-family: monospace; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);", "NotificationProvider {{ placement: NotificationPlacement::TopRight, /* 你的应用 */ }}" } },
                code: r#"rsx! {
    NotificationProvider {
        placement: NotificationPlacement::TopRight,
        // 你的应用路由或页面
        Router::<Route> {}
    }
}"#.to_string(),
            }

            h2 { "use_notification() API" }
            PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("api.info(title, content)", "String, String", "打开一条信息通知", ""),
                ("api.success(title, content)", "String, String", "打开一条成功通知", ""),
                ("api.warning(title, content)", "String, String", "打开一条警告通知", ""),
                ("api.error(title, content)", "String, String", "打开一条错误通知", ""),
                ("api.open(type, title, content)", "NotificationType, String, String", "通用打开方法（默认 4.5 秒自动关闭）", ""),
                ("api.open_with_duration(type, title, content, dur)", "NotificationType, String, String, u64", "自定义时长（ms），0 表示不自动关闭", ""),
                ("api.remove(id)", "u32", "移除指定通知", ""),
            ]}

            h2 { "NotificationProvider Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("placement", "NotificationPlacement", "TopRight", "弹出位置（TopRight / TopLeft / BottomRight / BottomLeft）"),
                ("max_count", "usize", "5", "最大显示数量，超出部分隐藏（先进先出）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "子元素"),
            ]}

            h2 { "Notification Props（声明式）" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("r#type", "NotificationType", "Info", "通知类型（Info / Success / Warning / Error）"),
                ("title", "String", "\"\"", "标题"),
                ("content", "String", "\"\"", "内容"),
                ("duration", "u64", "4500", "自动关闭时间(ms)，0 则不自动关闭"),
                ("closing", "bool", "false", "外部关闭信号"),
                ("onclose", "Option<EventHandler>", "None", "关闭回调"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }
    }
}
