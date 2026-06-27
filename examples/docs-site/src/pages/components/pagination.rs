use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn PaginationPage() -> Element {
    rsx! {
div { id: "pagination", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Pagination 分页"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "分页用于长列表的数据分页导航。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 total 和 page_size 计算分页，current/onchange 控制当前页。".to_string()),
                demo: rsx! { PaginationDocsDemo {} },
                code: "let mut page = use_signal(|| 1u32);\n\nPagination {\n    current: page(),\n    total: 50,\n    page_size: 10,\n    onchange: move |v| page.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "大数据量".to_string(),
                description: Some("total 为 200 时自动计算出 20 页。".to_string()),
                demo: rsx! {
                    Pagination { total: 200, page_size: 10 }
                },
                code: "Pagination { total: 200, page_size: 10 }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Pagination Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("current", "u32", "1", "当前页码"),
                ("total", "u32", "0", "总条数"),
                ("page_size", "u32", "10", "每页条数"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("onchange", "Option<EventHandler<u32>>", "None", "页码切换回调"),
            ]}
        }
    }
}
