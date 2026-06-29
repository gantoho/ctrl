use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn SpacePage() -> Element {
    rsx! {
div { id: "space", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Space 间距" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "间距组件用于设置子元素之间的间距。支持水平和垂直方向，提供三种预设尺寸。" }

            DemoBox {
                title: "水平间距".to_string(),
                description: Some("默认 direction 为 horizontal。".to_string()),
                demo: rsx! {
                    Space {
                        Button { variant: Variant::Primary, "按钮 1" }
                        Button { variant: Variant::Outline, "按钮 2" }
                        Button { variant: Variant::Ghost, "按钮 3" }
                    }
                },
                code: "Space {\n    Button { variant: Variant::Primary, \"按钮 1\" }\n    Button { variant: Variant::Outline, \"按钮 2\" }\n    Button { variant: Variant::Ghost, \"按钮 3\" }\n}".to_string(),
            }

            DemoBox {
                title: "垂直间距".to_string(),
                description: Some("设置 direction 为 vertical。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical,
                        Button { variant: Variant::Primary, block: true, "按钮 1" }
                        Button { variant: Variant::Outline, block: true, "按钮 2" }
                        Button { variant: Variant::Ghost, block: true, "按钮 3" }
                    }
                },
                code: "Space { direction: \"vertical\".to_string(),\n    Button { variant: Variant::Primary, block: true, \"按钮 1\" }\n    Button { variant: Variant::Outline, block: true, \"按钮 2\" }\n}".to_string(),
            }

            DemoBox {
                title: "自定义间距".to_string(),
                description: Some("通过 gap 属性设置间距大小 (sm / md / lg)。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        Space { gap: "sm".to_string(), Tag { "小" } Tag { "间" } Tag { "距" } }
                        Space { gap: "md".to_string(), Tag { "中" } Tag { "间" } Tag { "距" } }
                        Space { gap: "lg".to_string(), Tag { "大" } Tag { "间" } Tag { "距" } }
                    }
                },
                code: "Space { gap: \"sm\".to_string(), ... }\nSpace { gap: \"md\".to_string(), ... }\nSpace { gap: \"lg\".to_string(), ... }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Space Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("direction", "String", "\"horizontal\"", "排列方向：horizontal / vertical"),
                ("size", "String", "\"md\"", "间距大小：sm / md / lg"),
                ("align", "String", "\"center\"", "对齐方式：start / center / end"),
                ("wrap", "bool", "false", "是否换行"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "子元素"),
            ]}
        }
    }
}
