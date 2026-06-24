use dioxus::prelude::*;

/// Breadcrumb 面包屑组件属性
#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbProps {
    /// 分隔符
    #[props(default = "/".to_string())]
    pub separator: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（BreadcrumbItem）
    pub children: Element,
}

/// BreadcrumbItem 属性
#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbItemProps {
    /// 链接地址（为空则不可点击）
    #[props(default = "".to_string())]
    pub href: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素
    pub children: Element,
}

/// Breadcrumb 面包屑组件
#[allow(non_snake_case)]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let nav_class = if props.class.is_empty() {
        "ctrl-breadcrumb".to_string()
    } else {
        format!("ctrl-breadcrumb {}", props.class)
    };

    rsx! {
        nav {
            class: "{nav_class}",
            {props.children}
        }
    }
}

/// BreadcrumbItem 面包屑项
#[allow(non_snake_case)]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    let item_class = if props.class.is_empty() {
        "ctrl-breadcrumb__item".to_string()
    } else {
        format!("ctrl-breadcrumb__item {}", props.class)
    };

    if !props.href.is_empty() {
        rsx! {
            span {
                class: "{item_class}",
                a {
                    class: "ctrl-breadcrumb__link",
                    href: "{props.href}",
                    {props.children}
                }
            }
        }
    } else {
        rsx! {
            span {
                class: "{item_class} ctrl-breadcrumb__item--active",
                {props.children}
            }
        }
    }
}
