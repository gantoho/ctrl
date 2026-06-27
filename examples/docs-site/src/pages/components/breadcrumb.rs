use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn BreadcrumbPage() -> Element {
    rsx! {
div { id: "breadcrumb", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Breadcrumb 面包屑"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "面包屑用于展示当前页面的层级路径，帮助用户了解当前位置。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("最后一项不设 href 即为当前页（不可点击）。".to_string()),
                demo: rsx! {
                    Breadcrumb {
                        BreadcrumbItem { href: "#".to_string(), "首页" }
                        BreadcrumbItem { href: "#".to_string(), "组件" }
                        BreadcrumbItem { "面包屑" }
                    }
                },
                code: "Breadcrumb {\n    BreadcrumbItem { href: \"#\", \"首页\" }\n    BreadcrumbItem { href: \"#\", \"组件\" }\n    BreadcrumbItem { \"面包屑\" }\n}".to_string(),
            }

            DemoBox {
                title: "自定义分隔符".to_string(),
                description: Some("通过 separator 属性设置分隔符。".to_string()),
                demo: rsx! {
                    Breadcrumb { separator: ">".to_string(),
                        BreadcrumbItem { href: "#".to_string(), "Home" }
                        BreadcrumbItem { href: "#".to_string(), "Library" }
                        BreadcrumbItem { "Data" }
                    }
                },
                code: "Breadcrumb { separator: \">\", ... }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Breadcrumb Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("separator", "String", "\"/\"", "分隔符"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "子元素（BreadcrumbItem）"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "BreadcrumbItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("href", "String", "\"\"", "链接地址（空则不可点击）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "子元素"),
            ]}
        }
    }
}
