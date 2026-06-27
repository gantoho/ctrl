use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn TreePage() -> Element {
    rsx! {
div { id: "tree", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Tree 树形控件" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "树形控件用于展示层级结构数据。支持节点展开/收起、选择和复选框功能。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 data 属性传入树形数据。".to_string()),
                demo: rsx! { BasicTreeDemo {} },
                code: "let tree_data = vec![\n    TreeNode { node_key: \"1\".to_string(), title: \"节点 1\".to_string(),\n        child_nodes: vec![\n            TreeNode { node_key: \"1-1\".to_string(), title: \"节点 1-1\".to_string() },\n            TreeNode { node_key: \"1-2\".to_string(), title: \"节点 1-2\".to_string() },\n        ]\n    },\n];\n\nTree { data: tree_data, default_expand_all: true }".to_string(),
            }

            DemoBox {
                title: "可选择".to_string(),
                description: Some("通过 selected_key 和 onselect 实现节点选择。".to_string()),
                demo: rsx! { SelectableTreeDemo {} },
                code: "let mut selected = use_signal(|| String::new());\n\nTree {\n    data: tree_data,\n    selected_key: selected(),\n    default_expand_all: true,\n    onselect: move |k| selected.set(k),\n}".to_string(),
            }

            DemoBox {
                title: "带复选框".to_string(),
                description: Some("设置 checkable 为 true 启用复选框。".to_string()),
                demo: rsx! { CheckableTreeDemo {} },
                code: "let mut checked = use_signal(|| Vec::new());\n\nTree {\n    data: tree_data,\n    checkable: true,\n    checked_keys: checked(),\n    default_expand_all: true,\n    oncheck: move |keys| checked.set(keys),\n}".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Tree Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("data", "Vec<TreeNode>", "[]", "树形数据"),
                ("selected_key", "String", "\"\"", "选中节点 key"),
                ("checkable", "bool", "false", "是否显示复选框"),
                ("checked_keys", "Vec<String>", "[]", "选中节点 keys"),
                ("default_expand_all", "bool", "false", "默认展开全部"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onselect", "Option<EventHandler<String>>", "None", "选择回调"),
                ("onexpand", "Option<EventHandler<(String, bool)>>", "None", "展开/收起回调"),
                ("oncheck", "Option<EventHandler<Vec<String>>>", "None", "复选框回调"),
            ]}

            h3 { style: "font-size: 1rem; font-weight: 600; color: var(--ctrl-text); margin: 24px 0 12px;", "TreeNode Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("node_key", "String", "—", "节点唯一标识"),
                ("title", "String", "—", "节点标题"),
                ("child_nodes", "Vec<TreeNode>", "[]", "子节点"),
                ("disabled", "bool", "false", "是否禁用"),
                ("selectable", "bool", "true", "是否可选择"),
            ]}
        }
    }
}
