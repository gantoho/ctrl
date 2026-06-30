use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn CollapsePage() -> Element {
    rsx! {
div { id: "collapse", style: "margin-top: 64px;",
            h1 { "Collapse 折叠面板" }
            p { "可以折叠/展开的内容区域，用于展示 FAQ、表单分组等。" }
            DemoBox { title: "基本用法".to_string(), description: Some("点击标题展开/收起内容，动画基于 CSS grid-template-rows 实现平滑高度过渡。".to_string()),
                demo: rsx! {
                    Collapse {
                        CollapseItem { title: "标题一".to_string(), expanded: true,
                            p { "标题一展开的内容。" }
                        }
                        CollapseItem { title: "标题二".to_string(),
                            p { "标题二的内容。" }
                        }
                        CollapseItem { title: "禁用项".to_string(), disabled: true,
                            p { "这项已禁用。" }
                        }
                    }
                },
                code: "Collapse {\n    CollapseItem { title: \"标题一\".to_string(), expanded: true,\n        p { \"内容\" }\n    }\n    CollapseItem { title: \"标题二\".to_string(),\n        p { \"内容\" }\n    }\n}".to_string(),
            }
            DemoBox { title: "无动画".to_string(), description: Some("设置 animated=false 可关闭展开/收起动画。".to_string()),
                demo: rsx! {
                    Collapse {
                        CollapseItem { title: "无动画项".to_string(), animated: false,
                            p { "展开和收起没有过渡效果，直接显示/隐藏。" }
                        }
                        CollapseItem { title: "另一个无动画项".to_string(), animated: false, expanded: true,
                            p { "这一项默认展开，切换时也是瞬间切换。" }
                        }
                    }
                },
                code: "CollapseItem { title: \"标题\".to_string(), animated: false,\n    p { \"内容\" }\n}".to_string(),
            }
            DemoBox { title: "手风琴模式".to_string(), description: Some("设置 accordion=true，展开一个面板时其他面板自动收起。".to_string()),
                demo: rsx! {
                    Collapse {
                        accordion: true,
                        CollapseItem { title: "面板一".to_string(), expanded: true,
                            p { "这是第一个面板的内容，默认展开。" }
                        }
                        CollapseItem { title: "面板二".to_string(),
                            p { "这是第二个面板的内容。" }
                        }
                        CollapseItem { title: "面板三".to_string(),
                            p { "这是第三个面板的内容。" }
                        }
                    }
                },
                code: "Collapse {\n    accordion: true,\n    CollapseItem { title: \"面板一\".to_string(), expanded: true,\n        p { \"内容\" }\n    }\n    CollapseItem { title: \"面板二\".to_string(),\n        p { \"内容\" }\n    }\n    CollapseItem { title: \"面板三\".to_string(),\n        p { \"内容\" }\n    }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("borderless", "bool", "false", "是否无边框（Collapse）"),
                ("accordion", "bool", "false", "是否启用手风琴模式（同时只有一个展开项，Collapse）"),
                ("class", "String", "\"\"", "自定义 CSS 类 — Collapse"),
                ("children", "Element", "—", "子元素（CollapseItem）— Collapse"),
            ]}
            h2 { "CollapseItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "标题"),
                ("expanded", "bool", "false", "是否展开"),
                ("disabled", "bool", "false", "是否禁用"),
                ("show_arrow", "bool", "true", "是否显示箭头"),
                ("animated", "bool", "true", "是否启用展开/收起动画"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "面板内容"),
            ]}
        }
    }
}
