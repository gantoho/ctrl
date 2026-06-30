use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn MessagePage() -> Element {
    rsx! {
div { id: "message", style: "margin-top: 64px;",
            h1 {
                "Message 全局提示"
            }
            p {
                "轻量级全局提示，通过 use_message() API 触发，以 fixed 定位弹出在页面指定位置，展示操作结果或系统通知。支持自动消失和多位置弹出。支持声明式和命令式两种用法。"
            }

            DemoBox {
                title: "命令式：use_message() API 不同类型".to_string(),
                description: Some("使用 use_message() 获取 API，通过 api.info / success / warning / error 快捷方法触发不同样式的消息。".to_string()),
                demo: rsx! { MessageTriggerDocsDemo {} },
                code: r#"let mut api = use_message();
api.info("已复制到剪贴板".to_string());
api.success("保存成功！".to_string());
api.warning("文件格式不支持".to_string());
api.error("网络连接超时".to_string());"#.to_string(),
            }

            DemoBox {
                title: "不同位置".to_string(),
                description: Some("通过 MessageProvider 的 placement 属性控制弹出位置：Top（顶部居中）、TopRight（顶部靠右）、TopLeft（顶部靠左）、Bottom（底部居中）。".to_string()),
                demo: rsx! { MessagePositionDocsDemo {} },
                code: r#"rsx! {
    MessageProvider {
        placement: MessagePlacement::Top,
        // 你的应用路由或页面
        Router::<Route> {}
    }
}"#.to_string(),
            }

            h2 { "MessageProvider —— 命令式容器" }
            p {
                "使用命令式 API 前，需在应用根节点包裹 MessageProvider："
            }
            DemoBox {
                title: String::new(),
                description: None,
                demo: rsx! { span { style: "font-family: monospace; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);", "MessageProvider {{ placement: MessagePlacement::Top, /* 你的应用 */ }}" } },
                code: r#"rsx! {
    MessageProvider {
        placement: MessagePlacement::Top,
        // 你的应用路由或页面
        Router::<Route> {}
    }
}"#.to_string(),
            }

            h2 { "use_message() API" }
            PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("api.info(msg)", "String", "打开一条信息提示", ""),
                ("api.success(msg)", "String", "打开一条成功提示", ""),
                ("api.warning(msg)", "String", "打开一条警告提示", ""),
                ("api.error(msg)", "String", "打开一条错误提示", ""),
                ("api.open(type, msg)", "MessageType, String", "通用打开方法（默认 3 秒自动关闭）", ""),
                ("api.open_with_duration(type, msg, dur)", "MessageType, String, u64", "自定义时长（ms），0 表示不自动关闭", ""),
            ]}

            h2 { "MessageProvider Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("placement", "MessagePlacement", "Top", "弹出位置（Top / TopRight / TopLeft / Bottom）"),
                ("max_count", "usize", "5", "最大显示数量，超出部分隐藏（先进先出）"),
                ("children", "Element", "—", "子元素"),
            ]}

            h2 { "Message Props（声明式）" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("r#type", "MessageType", "Info", "消息类型（Info / Success / Warning / Error）"),
                ("content", "String", "\"\"", "消息内容"),
                ("duration", "u64", "3000", "自动关闭时间(ms)，0 则不自动关闭"),
                ("leaving", "bool", "false", "外部关闭信号（触发退出动画）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }
    }
}
