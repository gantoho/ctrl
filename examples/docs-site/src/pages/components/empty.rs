use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn EmptyPage() -> Element {
    rsx! {
        h1 { "Empty 空状态" }
        p { "无数据时的占位组件，支持尺寸变体、自定义图片和操作区。" }

        DemoBox { title: "基本用法".to_string(), description: None,
            demo: rsx! { Empty { description: "暂无数据".to_string() } },
            code: "Empty { description: \"暂无数据\".to_string() }".to_string(),
        }

        DemoBox { title: "尺寸变体".to_string(), description: Some("通过 size 控制尺寸：small / default / large。".to_string()),
            demo: rsx! {
                Space { gap: "lg".to_string(), direction: Direction::Vertical, style: "width:100%;".to_string(),
                    Empty { description: "小尺寸".to_string(), size: "small".to_string() }
                    Empty { description: "默认尺寸".to_string() }
                    Empty { description: "大尺寸".to_string(), size: "large".to_string() }
                }
            },
            code: "Empty { description: \"小尺寸\", size: \"small\" }\nEmpty { description: \"默认尺寸\" }\nEmpty { description: \"大尺寸\", size: \"large\" }".to_string(),
        }

        DemoBox { title: "自定义操作".to_string(), description: None,
            demo: rsx! {
                Empty { description: "还没有内容，快来添加吧".to_string(),
                    Button { variant: Variant::Primary, size: Size::Sm, "添加内容" }
                }
            },
            code: "Empty { description: \"还没有内容\",\n    Button { variant: Variant::Primary, size: Size::Sm, \"添加内容\" }\n}".to_string(),
        }

        DemoBox { title: "自定义图片".to_string(), description: Some("通过 image 传入自定义图片 URL。".to_string()),
            demo: rsx! {
                Empty { description: "图片自定义".to_string(), image: "https://via.placeholder.com/80".to_string() }
            },
            code: "Empty { description: \"图片自定义\", image: \"https://via.placeholder.com/80\" }".to_string(),
        }

        h2 { "Empty Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("description", "String", "\"暂无数据\"", "描述文字"),
            ("image", "String", "\"\"", "自定义图片 URL"),
            ("size", "String", "\"default\"", "尺寸：small / default / large"),
            ("image_size", "String", "\"\"", "自定义图片尺寸（如 \"120px\"）"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
            ("children", "Element", "—", "底部操作区"),
        ]}
    }
}
