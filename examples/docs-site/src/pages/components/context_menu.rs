use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn ContextMenuPage() -> Element {
    let mut last_action = use_signal(|| String::new());

    let items = vec![
        ContextMenuItem::new("复制", "copy").icon("📋"),
        ContextMenuItem::new("粘贴", "paste").icon("📌"),
        ContextMenuItem::new("剪切", "cut"),
        ContextMenuItem::new("", "").divider(),
        ContextMenuItem::new("删除", "delete").icon("🗑️"),
        ContextMenuItem::new("重命名", "rename").disabled(),
    ];

    let action_text = last_action();

    rsx! {
div { id: "context-menu", style: "margin-top: 64px;",
            h1 { "ContextMenu 右键菜单" }
            p {
                "在目标区域内右键弹出操作菜单，点击菜单项执行对应操作并关闭。适合表格行、文件列表、画布等需要右键操作的场景。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("右键下方区域弹出菜单，选择操作后显示结果。".to_string()),
                demo: rsx! {
                    div { style: "display:flex; flex-direction:column; align-items:center; gap:12px;",
                        ContextMenu {
                            items: items.clone(),
                            onselect: move |id: String| last_action.set(format!("点击了: {id}")),
                            div {
                                style: "width:100%; max-width:320px; padding:48px; border:2px dashed var(--ctrl-border); border-radius:var(--ctrl-radius-lg); text-align:center; color:var(--ctrl-text-secondary); cursor:context-menu; user-select:none;",
                                {if action_text.is_empty() { "右键此处".to_string() } else { action_text.clone() }}
                            }
                        }
                    }
                },
                code: "ContextMenu {\n    items: vec![\n        ContextMenuItem::new(\"复制\", \"copy\").icon(\"📋\"),\n        ...\n    ],\n    onselect: |id| { ... },\n    div { \"右键此处\" }\n}".to_string(),
            }

            h2 { "ContextMenuProps" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("items", "Vec<ContextMenuItem>", "—", "菜单项列表"),
                ("onselect", "EventHandler<String>", "—", "选中菜单项回调（参数为 item.id）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "触发区域内容"),
            ]}

            h2 { "ContextMenuItem" }
            PropsTable { headers: vec!["方法 / 字段".to_string(), "类型".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("ContextMenuItem::new(label, id)", "fn", "创建菜单项", ""),
                (".icon(icon)", "fn", "左侧图标", ""),
                (".divider()", "fn", "设为分隔线", ""),
                (".disabled()", "fn", "禁用该项", ""),
                ("label", "String", "显示文本", ""),
                ("id", "String", "选中回调透传标识", ""),
            ]}
        }
    }
}
