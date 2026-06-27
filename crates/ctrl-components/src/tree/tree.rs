use dioxus::prelude::*;

/// 树节点数据
#[derive(PartialEq, Clone)]
pub struct TreeNode {
    /// 节点唯一标识
    pub node_key: String,
    /// 节点标题
    pub title: String,
    /// 子节点
    pub child_nodes: Vec<TreeNode>,
    /// 是否禁用
    pub disabled: bool,
    /// 是否可选择
    pub selectable: bool,
}

impl Default for TreeNode {
    fn default() -> Self {
        Self {
            node_key: String::new(),
            title: String::new(),
            child_nodes: Vec::new(),
            disabled: false,
            selectable: true,
        }
    }
}

/// Tree 树形组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TreeProps {
    /// 树形数据
    #[props(default = Vec::new())]
    pub data: Vec<TreeNode>,

    /// 选中的节点 key
    #[props(default = "".to_string())]
    pub selected_key: String,

    /// 是否显示复选框
    #[props(default = false)]
    pub checkable: bool,

    /// 选中的节点 keys
    #[props(default = Vec::new())]
    pub checked_keys: Vec<String>,

    /// 默认展开所有节点
    #[props(default = false)]
    pub default_expand_all: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 节点选择回调
    pub onselect: Option<EventHandler<String>>,

    /// 节点展开/收起回调
    pub onexpand: Option<EventHandler<(String, bool)>>,

    /// 复选框变化回调
    pub oncheck: Option<EventHandler<Vec<String>>>,
}

/// Tree 树形组件
#[allow(non_snake_case)]
pub fn Tree(props: TreeProps) -> Element {
    const CSS: &str = include_str!("../../assets/tree.css");

    let tree_class = {
        let mut c = String::from("ctrl-tree");
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{tree_class}",
            style: "{props.style}",
            for node in &props.data {
                TreeNodeView {
                    node: node.clone(),
                    level: 0,
                    selected_key: props.selected_key.clone(),
                    checkable: props.checkable,
                    checked_keys: props.checked_keys.clone(),
                    default_expand_all: props.default_expand_all,
                    onselect: props.onselect.clone(),
                    onexpand: props.onexpand.clone(),
                    oncheck: props.oncheck.clone(),
                }
            }
        }
    }
}

/// 树节点视图属性
#[derive(Props, PartialEq, Clone)]
struct TreeNodeViewProps {
    node: TreeNode,
    level: usize,
    selected_key: String,
    checkable: bool,
    checked_keys: Vec<String>,
    default_expand_all: bool,
    onselect: Option<EventHandler<String>>,
    onexpand: Option<EventHandler<(String, bool)>>,
    oncheck: Option<EventHandler<Vec<String>>>,
}

/// 树节点视图
#[allow(non_snake_case)]
fn TreeNodeView(props: TreeNodeViewProps) -> Element {
    let node_key = props.node.node_key.clone();
    let node_title = props.node.title.clone();
    let node_disabled = props.node.disabled;
    let node_selectable = props.node.selectable;
    let has_child_nodes = !props.node.child_nodes.is_empty();
    let expanded = use_signal(|| props.default_expand_all);
    let is_selected = props.selected_key == node_key;
    let is_checked = props.checked_keys.contains(&node_key);

    let node_class = {
        let mut c = String::from("ctrl-tree__node");
        if is_selected {
            c.push_str(" ctrl-tree__node--selected");
        }
        if node_disabled {
            c.push_str(" ctrl-tree__node--disabled");
        }
        c
    };

    let expand_class = {
        let mut c = String::from("ctrl-tree__expand");
        if expanded() {
            c.push_str(" ctrl-tree__expand--expanded");
        }
        if !has_child_nodes {
            c.push_str(" ctrl-tree__expand--empty");
        }
        c
    };

    let child_nodes_class = {
        if expanded() {
            "ctrl-tree__child_nodes"
        } else {
            "ctrl-tree__child_nodes ctrl-tree__child_nodes--hidden"
        }
    };

    let toggle_expand = {
        let mut expanded = expanded.clone();
        let onexpand = props.onexpand.clone();
        let key = node_key.clone();
        move |_| {
            if has_child_nodes {
                let new_state = !expanded();
                expanded.set(new_state);
                if let Some(ref cb) = onexpand {
                    cb.call((key.clone(), new_state));
                }
            }
        }
    };

    let handle_select = {
        let onselect = props.onselect.clone();
        let key = node_key.clone();
        let disabled = node_disabled;
        let selectable = node_selectable;
        move |_| {
            if !disabled && selectable {
                if let Some(ref cb) = onselect {
                    cb.call(key.clone());
                }
            }
        }
    };

    let handle_check = {
        let oncheck = props.oncheck.clone();
        let key = node_key.clone();
        let checked_keys = props.checked_keys.clone();
        let disabled = node_disabled;
        move |_| {
            if !disabled {
                if let Some(ref cb) = oncheck {
                    let mut new_keys = checked_keys.clone();
                    if is_checked {
                        new_keys.retain(|k| k != &key);
                    } else {
                        new_keys.push(key.clone());
                    }
                    cb.call(new_keys);
                }
            }
        }
    };

    rsx! {
        div { class: "{node_class}",
            div { class: "ctrl-tree__node-content",
                span {
                    class: "{expand_class}",
                    onclick: toggle_expand,
                    if has_child_nodes { "▶" }
                }
                if props.checkable {
                    input {
                        class: "ctrl-tree__checkbox",
                        r#type: "checkbox",
                        checked: is_checked,
                        disabled: node_disabled,
                        onchange: handle_check,
                    }
                }
                span {
                    class: "ctrl-tree__label",
                    onclick: handle_select,
                    "{node_title}"
                }
            }
            div { class: "{child_nodes_class}",
                for child in &props.node.child_nodes {
                    TreeNodeView {
                        node: child.clone(),
                        level: props.level + 1,
                        selected_key: props.selected_key.clone(),
                        checkable: props.checkable,
                        checked_keys: props.checked_keys.clone(),
                        default_expand_all: props.default_expand_all,
                        onselect: props.onselect.clone(),
                        onexpand: props.onexpand.clone(),
                        oncheck: props.oncheck.clone(),
                    }
                }
            }
        }
    }
}