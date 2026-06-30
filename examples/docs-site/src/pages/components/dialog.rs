use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn DialogPage() -> Element {
    rsx! {
div { id: "dialog", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Dialog 对话框"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "对话框用于在当前页面之上显示重要信息或需要用户操作的场景。支持声明式和命令式两种用法。"
            }

            DemoBox {
                title: "声明式：基本用法".to_string(),
                description: Some("通过 visible 控制显示，onclose 处理关闭。点击遮罩或关闭按钮均可关闭。".to_string()),
                demo: rsx! { BasicDialogDemo {} },
                code: "let mut visible = use_signal(|| false);\n\nButton { onclick: move |_| visible.set(true), \"打开对话框\" }\n\nDialog {\n    visible: visible(),\n    title: \"提示\".to_string(),\n    onclose: move |_| visible.set(false),\n    p { \"这是一条提示信息\" }\n}".to_string(),
            }

            DemoBox {
                title: "声明式：带底部操作".to_string(),
                description: Some("通过 footer 插槽自定义底部按钮区。".to_string()),
                demo: rsx! { FooterDialogDemo {} },
                code: "Dialog {\n    visible: visible(),\n    title: \"确认操作\".to_string(),\n    footer: rsx! {\n        Button { variant: Variant::Ghost, onclick: move |_| visible.set(false), \"取消\" }\n        Button { variant: Variant::Primary, onclick: move |_| visible.set(false), \"确定\" }\n    },\n    p { \"确定要执行此操作吗？\" }\n}".to_string(),
            }

            DemoBox {
                title: "命令式：use_dialog() API".to_string(),
                description: Some("通过 use_dialog() 获取 API，无需在 rsx! 中声明 Dialog 组件，直接在回调中调用 api.open() 打开对话框，支持确认/取消回调。".to_string()),
                demo: rsx! { DialogImperativeDemo {} },
                code: r#"let mut api = use_dialog();
api.open(DialogConfig {
    title: "确认删除".into(),
    content: "删除后不可恢复，确定继续？".into(),
    confirm_text: "确定".into(),
    cancel_text: "取消".into(),
    ..Default::default()
});"#.to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "DialogProvider —— 命令式容器" }
            p { style: "font-size: var(--ctrl-font-size-md); color: var(--ctrl-text-secondary); margin-bottom: 16px;",
                "使用命令式 API 前，需在应用根节点包裹 DialogProvider："
            }
            DemoBox {
                title: String::new(),
                description: None,
                demo: rsx! { span { style: "font-family: monospace; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);", "DialogProvider {{ /* 你的应用 */ }}" } },
                code: r#"rsx! {
    DialogProvider {
        // 你的应用路由或页面
        Router::<Route> {}
    }
}"#.to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "use_dialog() API" }
            PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("api.open(config)", "DialogConfig", "打开对话框", ""),
                ("api.close()", "—", "关闭对话框", ""),
            ]}

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "DialogConfig 字段" }
            PropsTable { headers: vec!["字段".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "对话框标题"),
                ("content", "String", "\"\"", "对话框内容"),
                ("confirm_text", "String", "\"确定\"", "确认按钮文字，空字符串则不显示"),
                ("cancel_text", "String", "\"取消\"", "取消按钮文字，空字符串则不显示"),
                ("width", "String", "\"480px\"", "对话框宽度"),
                ("mask_closable", "bool", "true", "点击遮罩是否关闭"),
                ("onconfirm", "Option<EventHandler>", "None", "确认按钮回调"),
                ("oncancel", "Option<EventHandler>", "None", "取消按钮回调"),
                ("onclose", "Option<EventHandler>", "None", "关闭时回调（确认/取消/遮罩关闭均会触发）"),
            ]}

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Dialog Props（声明式）" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("visible", "bool", "false", "是否显示"),
                ("title", "String", "\"\"", "对话框标题"),
                ("width", "String", "\"480px\"", "对话框宽度"),
                ("show_close", "bool", "true", "是否显示关闭按钮"),
                ("mask_closable", "bool", "true", "点击遮罩是否关闭"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭事件"),
                ("footer", "Option<Element>", "None", "底部插槽"),
                ("children", "Element", "—", "对话框内容"),
            ]}
        }
    }
}
