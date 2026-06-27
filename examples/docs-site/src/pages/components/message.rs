use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn MessagePage() -> Element {
    rsx! {
div { id: "message", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Message 全局提示"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "轻量级全局提示，以 fixed 定位弹出在页面指定位置，展示操作结果或系统通知。支持自动消失和多位置弹出。"
            }

            DemoBox {
                title: "点击触发（不同类型）".to_string(),
                description: Some("点击按钮后，Message 会以 fixed 定位弹出在页面顶部居中，3 秒后自动消失。".to_string()),
                demo: rsx! { MessageTriggerDocsDemo {} },
                code: "let mut trigger = use_signal(|| 0u32);\nlet mut msg_type = use_signal(|| MessageType::Info);\nlet mut msg_content = use_signal(|| String::new());\n\n// 点击时设置类型和内容，trigger +1 触发重新渲染\nButton { onclick: move |_| {\n    msg_type.set(MessageType::Success);\n    msg_content.set(\"操作成功！\".into());\n    trigger.set(trigger() + 1);\n}, \"保存\" }\n\nif trigger() > 0 {\n    Message {\n        r#type: msg_type(),\n        content: msg_content(),\n        duration: 3000,\n    }\n}".to_string(),
            }

            DemoBox {
                title: "不同位置".to_string(),
                description: Some("通过 placement 控制弹出位置：Top（顶部居中）、TopRight（顶部靠右）、TopLeft（顶部靠左）、Bottom（底部居中）。".to_string()),
                demo: rsx! { MessagePositionDocsDemo {} },
                code: "let mut placement = use_signal(|| MessagePlacement::Top);\n\nButton { onclick: move |_| placement.set(MessagePlacement::TopRight), \"右上角\" }\n\nMessage { r#type: MessageType::Success, content: \"已复制\", placement: placement(), duration: 3000 }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "MessageContainer Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("placement", "MessagePlacement", "Top", "弹出位置（Top / TopRight / TopLeft / Bottom）"),
                ("max_count", "usize", "5", "最大显示数量，超出部分隐藏（先进先出）"),
                ("children", "Element", "—", "子元素（多条 Message）"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Message Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("type", "MessageType", "Info", "消息类型（Info / Success / Warning / Error）"),
                ("content", "String", "\"\"", "消息文字"),
                ("duration", "u64", "3000", "自动消失时间(ms)，0 表示不自动消失"),
                ("closing", "bool", "false", "外部关闭信号，设为 true 触发退出动画"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭回调"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }
    }
}
