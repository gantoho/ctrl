use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn NotificationPage() -> Element {
    rsx! {
div { id: "notification", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Notification 通知提醒" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "全局通知提醒，支持四种类型和自动关闭。推荐使用 NotificationProvider + useNotification 上下文 API。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    NotificationDocs {}
                },
                code: "let mut notif = use_notification();\nnotif.info(\"通知\".to_string(), \"这是一条通知\".to_string());\nnotif.success(\"成功\".to_string(), \"操作成功\".to_string());\nnotif.warning(\"警告\".to_string(), \"请注意\".to_string());\nnotif.error(\"错误\".to_string(), \"操作失败\".to_string());".to_string(),
            }
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "NotificationProvider Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("placement", "NotificationPlacement", "TopRight", "弹出位置（TopRight / TopLeft / BottomRight / BottomLeft）"),
                ("max_count", "usize", "5", "最大显示数量，超出部分隐藏（先进先出）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "子元素"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "NotificationProps（单条通知）" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("r#type", "NotificationType", "Info", "通知类型（Info / Success / Warning / Error）"),
                ("title", "String", "\"\"", "标题"),
                ("content", "String", "\"\"", "内容"),
                ("duration", "u64", "4500", "自动关闭时间(ms)，0 则不自动关闭"),
                ("closing", "bool", "false", "外部关闭信号"),
                ("onclose", "Option<EventHandler>", "None", "关闭回调"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "useNotification API" }
            PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "返回值".to_string(), "说明".to_string()], rows: vec![
                ("info(title, content)", "String, String", "—", "打开一条信息通知"),
                ("success(title, content)", "String, String", "—", "打开一条成功通知"),
                ("warning(title, content)", "String, String", "—", "打开一条警告通知"),
                ("error(title, content)", "String, String", "—", "打开一条错误通知"),
                ("open(type, title, content)", "NotificationType, String, String", "—", "打开一条通知（默认 duration 4500ms）"),
                ("open_with_duration(type, title, content, duration)", "NotificationType, String, String, u64", "—", "打开一条通知（自定义 duration）"),
                ("remove(id)", "u32", "—", "移除指定通知"),
                ("clear()", "—", "—", "清除所有通知"),
            ]}
        }
    }
}
