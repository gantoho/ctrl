use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn DrawerPage() -> Element {
    rsx! {
div { id: "drawer", style: "margin-top: 64px;",
            h1 { "Drawer 抽屉" }
            p { "从屏幕边缘滑出的浮层面板，适用于表单、详情等场景。支持声明式和命令式两种用法。" }

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
    content: rsx! { p { "这里是抽屉的内容区域" } },
    placement: Placement::Right,
    ..Default::default()
});"#.to_string(),
            }

            h2 { "DrawerProvider —— 命令式容器" }
            p {
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

            h2 { "use_drawer() API" }
            PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("api.open(config)", "DrawerConfig", "打开抽屉", ""),
                ("api.close()", "—", "关闭抽屉", ""),
            ]}

            h2 { "DrawerConfig 字段" }
            PropsTable { headers: vec!["字段".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "抽屉标题"),
                ("content", "Element", "rsx!{}", "抽屉内容（Element）"),
                ("placement", "Placement", "Right", "位置：Left / Right / Top / Bottom"),
                ("size", "String", "\"380px\"", "宽度或高度"),
                ("show_close", "bool", "true", "是否显示关闭按钮"),
                ("onclose", "Option<EventHandler>", "None", "关闭时回调"),
            ]}

            h2 { "Drawer Props（声明式）" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("visible", "bool", "false", "是否打开"),
                ("title", "String", "\"\"", "抽屉标题"),
                ("placement", "Placement", "Right", "位置：Left / Right / Top / Bottom"),
                ("size", "String", "\"380px\"", "宽度或高度"),
                ("show_close", "bool", "true", "是否显示关闭按钮"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭回调"),
                ("footer", "Option<Element>", "None", "底部操作区"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "抽屉内容"),
            ]}
        }
    }
}
