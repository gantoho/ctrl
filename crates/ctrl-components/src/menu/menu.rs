use dioxus::prelude::*;
use ctrl_core::types::Direction;

/// Menu 导航菜单组件属性
#[derive(Props, PartialEq, Clone)]
pub struct MenuProps {
    /// 菜单方向
    #[props(default = Direction::Vertical)]
    pub direction: Direction,

    /// 当前激活项 key
    #[props(default = "".to_string())]
    pub active: String,

    /// 切换回调
    pub onchange: Option<EventHandler<String>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（MenuItem / Submenu）
    pub children: Element,
}

/// 菜单项属性
#[derive(Props, PartialEq, Clone)]
pub struct MenuItemProps {
    /// 唯一标识
    #[props(default = "".to_string())]
    pub item_key: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 点击事件
    pub onclick: Option<EventHandler<()>>,

    /// 图标占位
    pub icon: Option<Element>,

    /// 子元素（菜单文字）
    pub children: Element,
}

/// Menu 组件（简化的上下文提供者，MenuItem 需要访问 active/onchange）
#[allow(non_snake_case)]
pub fn Menu(props: MenuProps) -> Element {
    const CSS: &str = include_str!("../../assets/menu.css");

    let dir_class = if props.direction == Direction::Horizontal {
        "ctrl-menu--horizontal"
    } else {
        ""
    };

    let wrapper_class = if props.class.is_empty() {
        format!("ctrl-menu {}", dir_class)
    } else {
        format!("ctrl-menu {} {}", dir_class, props.class)
    };

    let _active = props.active.clone();
    let _onchange = props.onchange.clone();

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            {props.children}
        }
    }
}

/// MenuItem 导航菜单项
#[allow(non_snake_case)]
pub fn MenuItem(props: MenuItemProps) -> Element {
    let mut item_classes = vec!["ctrl-menu__item".to_string()];
    if props.disabled {
        item_classes.push("ctrl-menu__item--disabled".into());
    }
    if !props.class.is_empty() {
        item_classes.push(props.class.clone());
    }
    let class_str = item_classes.join(" ");

    let onclick = props.onclick.clone();
    let disabled = props.disabled;

    rsx! {
        div {
            class: "{class_str}",
            onclick: move |_| {
                if !disabled {
                    if let Some(ref handler) = onclick {
                        handler.call(());
                    }
                }
            },
            if let Some(icon) = props.icon.clone() {
                span { class: "ctrl-menu__icon", {icon} }
            }
            {props.children}
        }
    }
}
