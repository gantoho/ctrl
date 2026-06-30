use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn DividerPage() -> Element {
    rsx! {
        h1 { "Divider 分割线" }
        p { "区隔内容的分割线，支持文字、虚线和垂直等变体，上下左右间距均可独立配置。" }

        DemoBox { title: "基本用法".to_string(), description: None,
            demo: rsx! {
                span { "上方内容" }
                Divider {}
                span { "下方内容" }
            },
            code: "Divider {}".to_string(),
        }

        DemoBox { title: "带文字".to_string(), description: Some("文字居中显示在分割线中间。".to_string()),
            demo: rsx! {
                span { "内容区" }
                Divider { content: "分割文字".to_string() }
                span { "另一区" }
            },
            code: "Divider { content: \"分割文字\".to_string() }".to_string(),
        }

        DemoBox { title: "文字位置".to_string(), description: Some("通过 orientation 控制文字位置：left / center / right。".to_string()),
            demo: rsx! {
                Space { direction: Direction::Vertical, gap: "sm".to_string(),
                    Divider { content: "左对齐".to_string(), orientation: "left".to_string() }
                    Divider { content: "居中".to_string(), orientation: "center".to_string() }
                    Divider { content: "右对齐".to_string(), orientation: "right".to_string() }
                }
            },
            code: "Divider { content: \"左对齐\".to_string(), orientation: \"left\".to_string() }\nDivider { content: \"居中\".to_string(), orientation: \"center\".to_string() }\nDivider { content: \"右对齐\".to_string(), orientation: \"right\".to_string() }".to_string(),
        }

        DemoBox { title: "间距控制".to_string(), description: Some("通过 gap_top / gap_right / gap_bottom / gap_left 独立控制四个方向的间距。".to_string()),
            demo: rsx! {
                Space { direction: Direction::Vertical, gap: "sm".to_string(),
                    span { "上下间距 sm" }
                    Divider { gap_top: "sm".to_string(), gap_bottom: "sm".to_string() }
                    span { "上下间距 lg" }
                    Divider { gap_top: "lg".to_string(), gap_bottom: "lg".to_string() }
                    span { "上 0 下 lg" }
                    Divider { gap_top: "0".to_string(), gap_bottom: "lg".to_string() }
                    span { "上 xl 下 0" }
                    Divider { gap_top: "xl".to_string(), gap_bottom: "0".to_string() }
                }
            },
            code: "Divider { gap_top: \"sm\".to_string(), gap_bottom: \"sm\".to_string() }\nDivider { gap_top: \"lg\".to_string(), gap_bottom: \"lg\".to_string() }\nDivider { gap_top: \"0\".to_string(), gap_bottom: \"lg\".to_string() }".to_string(),
        }

        DemoBox { title: "虚线分割线".to_string(), description: None,
            demo: rsx! {
                span { "上方内容" }
                Divider { dashed: true }
                span { "下方内容" }
            },
            code: "Divider { dashed: true }".to_string(),
        }

        DemoBox { title: "垂直分割线".to_string(), description: None,
            demo: rsx! {
                span { "链接" }
                Divider { direction: Direction::Vertical, gap_right: "sm".to_string(), gap_left: "sm".to_string() }
                span { "菜单" }
                Divider { direction: Direction::Vertical, gap_right: "sm".to_string(), gap_left: "sm".to_string() }
                span { "设置" }
            },
            code: "Divider { direction: Direction::Vertical, gap_right: \"sm\".to_string(), gap_left: \"sm\".to_string() }".to_string(),
        }

        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("direction", "Direction", "Horizontal", "方向（Horizontal / Vertical）"),
            ("content", "String", "\"\"", "中间文字，空则为纯分割线"),
            ("orientation", "String", "\"center\"", "文字位置（left / center / right）"),
            ("gap_top", "String", "\"md\"", "上间距（sm / md / lg / xl 或自定义 CSS 值）"),
            ("gap_right", "String", "\"md\"", "右间距（sm / md / lg / xl 或自定义 CSS 值）"),
            ("gap_bottom", "String", "\"md\"", "下间距（sm / md / lg / xl 或自定义 CSS 值）"),
            ("gap_left", "String", "\"md\"", "左间距（sm / md / lg / xl 或自定义 CSS 值）"),
            ("dashed", "bool", "false", "是否虚线"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}
    }
}