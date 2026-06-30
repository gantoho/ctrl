use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn EmptyPage() -> Element {
    rsx! {
div { id: "empty", style: "margin-top: 64px;",
            h1 { "Empty 空状态" }
            p { "无数据时的占位组件，可自定义图片和操作。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! { Empty { description: "暂无数据".to_string() } },
                code: "Empty { description: \"暂无数据\".to_string() }".to_string(),
            }
            DemoBox { title: "自定义操作".to_string(), description: None,
                demo: rsx! {
                    Empty { description: "还没有内容，快来添加吧".to_string(),
                        Button { variant: Variant::Primary, size: Size::Sm, "添加内容" }
                    }
                },
                code: "Empty { description: \"还没有内容\".to_string(),\n    Button { variant: Variant::Primary, size: Size::Sm, \"添加内容\" }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("description", "String", "\"暂无数据\"", "描述文字"),
                ("image", "String", "\"\"", "自定义图片 URL"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "底部操作区"),
            ]}
        }
    }
}
