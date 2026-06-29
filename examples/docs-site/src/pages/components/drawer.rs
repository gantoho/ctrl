use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn DrawerPage() -> Element {
    rsx! {
div { id: "drawer", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Drawer 抽屉" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "从屏幕边缘滑出的浮层面板，适用于表单、详情等场景。支持声明式和命令式两种用法。" }

            DemoBox {
                title: "声明式：基本用法".to_string(),
                description: Some("点击按钮打开右侧抽屉，通过 visible + onclose 控制显示/隐藏。".to_string()),
                demo: rsx! { DrawerDocs {} },
                code: "let mut visible = use_signal(|| false);\nButton { onclick: move |_| visible.set(true), \"打开\" }\nDrawer { visible: visible(), title: \"标题\".to_string(),\n    onclose: move |_| visible.set(false),\n    p { \"内容\" }\n}".to_string(),
            }

            DemoBox {
                title: "命令式：use_drawer() API".to_string(),
                description: Some("通过 use_drawer() 获取 API，无需在 rsx! 中声明 Drawer 组件，直接在回调中调用 api.open() 打开抽屉。".to_string()),
                demo: rsx! { DrawerImperativeDemo {} },
                code: r#"let mut api = use_drawer();
api.open(DrawerConfig {
    title: "用户详情".into(),
    content: "这里是抽屉的内容区域。".into(),
    placement: "right".into(),
    ..Default::default()
});"#.to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "DrawerProvider —— 命令式容器" }
            p { style: "font-size: var(--ctrl-font-size-md); color: var(--ctrl-text-secondary); margin-bottom: 16px;",
                "使用命令式 API 前，需在应用根节点包裹 DrawerProvider："
            }
            DemoBox {
                title: String::new(),
                description: None,
                demo: rsx! { span { style: "font-family: monospace; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);", "DrawerProvider {{ /* 你的应用 */ }}" } },
                code: r#"rsx! {
    DrawerProvider {
        // 你的应用路由或页面
        Router::<Route> {}
    }
}"#.to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "use_drawer() API" }
            PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("api.open(config)", "DrawerConfig", "打开抽屉", ""),
                ("api.close()", "—", "关闭抽屉", ""),
            ]}

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "DrawerConfig 字段" }
            PropsTable { headers: vec!["字段".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "抽屉标题"),
                ("content", "String", "\"\"", "抽屉内容"),
                ("placement", "String", "\"right\"", "位置：left / right / top / bottom"),
                ("size", "String", "\"380px\"", "宽度或高度"),
                ("on_close", "Option<EventHandler>", "None", "关闭时回调"),
            ]}

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Drawer Props（声明式）" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("visible", "bool", "false", "是否打开"),
                ("title", "String", "\"\"", "抽屉标题"),
                ("placement", "String", "\"right\"", "位置（left / right / top / bottom）"),
                ("size", "String", "\"380px\"", "宽度或高度"),
                ("show_close", "bool", "true", "是否显示关闭按钮"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭回调"),
                ("footer", "Option<Element>", "None", "底部操作区"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "抽屉内容"),
            ]}
        }
    }
}
