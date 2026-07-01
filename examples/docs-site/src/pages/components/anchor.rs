use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn AnchorPage() -> Element {
    let items = vec![
        AnchorItem { id: "basic".to_string(), title: "基本用法".to_string(), href: "#anchor-basic".to_string() },
        AnchorItem { id: "offset".to_string(), title: "滚动偏移".to_string(), href: "#anchor-offset".to_string() },
        AnchorItem { id: "props".to_string(), title: "Anchor Props".to_string(), href: "#anchor-props".to_string() },
    ];

    rsx! {
        div { id: "anchor-basic",
            h1 { "Anchor 锚点导航" }
            p { "用于页面内锚点导航，点击平滑滚动到对应位置，当前激活项高亮。" }
        }

        DemoBox { title: "基本用法".to_string(), description: Some("传入锚点列表，组件自动渲染导航并处理点击滚动。".to_string()),
            demo: rsx! {
                Row { gutter: 24,
                    Col { span: 6,
                        Anchor { items: items.clone(), style: "position:sticky; top:0;".to_string() }
                    }
                    Col { span: 18,
                        div { style: "height:1px;",
                            div { id: "anchor-basic", }
                        }
                        h3 { "基本用法" }
                        p { "这是一个锚点导航的基本用法示例。点击左侧导航项可平滑滚动到页面内对应位置。" }
                        div { style: "height:1px;",
                            div { id: "anchor-offset", }
                        }
                        h3 { "滚动偏移" }
                        p { "通过 offset_top 设置滚动偏移量，适配固定顶部导航栏。" }
                        div { style: "height:1px;",
                            div { id: "anchor-props", }
                        }
                        h3 { "Anchor Props" }
                        p { "见下方 Props 表格。" }
                    }
                }
            },
            code: "Anchor { items: vec![\n    AnchorItem { id: \"basic\", title: \"基本用法\", href: \"#basic\" },\n    AnchorItem { id: \"offset\", title: \"滚动偏移\", href: \"#offset\" },\n] }".to_string(),
        }

        h2 { "Anchor Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("items", "Vec<AnchorItem>", "—", "锚点项列表"),
            ("offset_top", "u32", "0", "滚动偏移量（px）"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}

        h2 { "AnchorItem" }
        PropsTable { headers: vec!["字段".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("id", "String", "—", "唯一标识"),
            ("title", "String", "—", "显示文本"),
            ("href", "String", "—", "锚点选择器（如 #section1）"),
        ]}
    }
}
