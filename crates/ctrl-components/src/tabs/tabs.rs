use dioxus::prelude::*;

/// Tabs 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TabsProps {
    /// 当前激活的 tab key
    #[props(default = "0".to_string())]
    pub active: String,

    /// tab 切换回调
    pub onchange: Option<EventHandler<String>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（Tab 组件）
    pub children: Element,
}

/// Tab 单个面板属性
#[derive(Props, PartialEq, Clone)]
pub struct TabProps {
    /// tab 唯一标识（不能使用 key，避免与 Dioxus 内置 key 冲突）
    #[props(default = "".to_string())]
    pub tab_key: String,

    /// tab 标题文字
    #[props(default = "".to_string())]
    pub title: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 子元素（面板内容）
    pub children: Element,
}

/// Tabs 标签页组件
#[allow(non_snake_case)]
pub fn Tabs(props: TabsProps) -> Element {
    const CSS: &str = include_str!("../../assets/tabs.css");
    let wrapper_class = if props.class.is_empty() {
        "ctrl-tabs".to_string()
    } else {
        format!("ctrl-tabs {}", props.class)
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            {props.children}
        }
    }
}

/// Tab 面板组件
#[allow(non_snake_case)]
pub fn Tab(props: TabProps) -> Element {
    rsx! {
        div {
            class: "ctrl-tabs__pane",
            {props.children}
        }
    }
}

/// TabNav 标签导航栏组件
#[derive(Props, PartialEq, Clone)]
pub struct TabNavProps {
    /// 标签项列表: vec![(key, title, disabled)]
    #[props(default = vec![])]
    pub items: Vec<(String, String, bool)>,

    /// 当前激活的 tab key
    #[props(default = "0".to_string())]
    pub active: String,

    /// tab 切换回调
    pub onchange: Option<EventHandler<String>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// TabNav 标签导航栏
#[allow(non_snake_case)]
pub fn TabNav(props: TabNavProps) -> Element {
    let nav_class = if props.class.is_empty() {
        "ctrl-tabs__nav".to_string()
    } else {
        format!("ctrl-tabs__nav {}", props.class)
    };

    let active = props.active.clone();
    let items = props.items.clone();
    let onchange = props.onchange.clone();

    rsx! {
        div {
            class: "{nav_class}",
            for (key, title, disabled) in items.iter() {
                {
                    let is_active = *key == active;
                    let mut tab_class = "ctrl-tabs__tab".to_string();
                    if is_active { tab_class.push_str(" ctrl-tabs__tab--active"); }
                    if *disabled { tab_class.push_str(" ctrl-tabs__tab--disabled"); }
                    let key_clone = key.clone();
                    let title_clone = title.clone();
                    let oc = onchange.clone();
                    let dis = *disabled;
                    rsx! {
                        div {
                            class: "{tab_class}",
                            onclick: move |_| {
                                if !dis {
                                    if let Some(ref handler) = oc {
                                        handler.call(key_clone.clone());
                                    }
                                }
                            },
                            "{title_clone}"
                        }
                    }
                }
            }
        }
    }
}

/// TabContent 标签内容区组件
#[derive(Props, PartialEq, Clone)]
pub struct TabContentProps {
    pub children: Element,
}

#[allow(non_snake_case)]
pub fn TabContent(props: TabContentProps) -> Element {
    rsx! {
        div {
            class: "ctrl-tabs__content",
            {props.children}
        }
    }
}
