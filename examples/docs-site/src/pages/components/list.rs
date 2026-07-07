use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn ListPage() -> Element {
    rsx! {
div { id: "list", style: "margin-top: 64px;",
            h1 {
                "List 列表"
            }
            p {
                "通用列表组件，支持头像、标题、描述、操作区等结构化内容展示。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("最简单的列表项结构，包含标题与描述。".to_string()),
                demo: rsx! {
                    div { style: "max-width:480px;",
                        List {
                            ListItem {
                                div {
                                    div { style: "color:var(--ctrl-text); line-height:1.5;", "列表标题" }
                                    div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary); margin-top:2px;", "列表描述文字" }
                                }
                            }
                            ListItem {
                                div {
                                    div { style: "color:var(--ctrl-text); line-height:1.5;", "第二条标题" }
                                    div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary); margin-top:2px;", "这是第二项的描述信息" }
                                }
                            }
                            ListItem {
                                div {
                                    div { style: "color:var(--ctrl-text); line-height:1.5;", "第三条标题" }
                                    div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary); margin-top:2px;", "第三项的描述文字" }
                                }
                            }
                        }
                    }
                },
                code: "List {\n    ListItem { div { \"列表标题\" } }\n    ListItem { div { \"第二条标题\" } }\n}".to_string(),
            }

            DemoBox {
                title: "带头像".to_string(),
                description: Some("在列表项中添加头像区域。".to_string()),
                demo: rsx! {
                    div { style: "max-width:480px;",
                        List {
                            ListItem {
                                div { style: "display:flex; align-items:center; gap:12px;",
                                    Avatar { size: Size::Md, "A" }
                                    div {
                                        div { style: "color:var(--ctrl-text); line-height:1.5;", "Alice Chen" }
                                        div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary); margin-top:2px;", "前端工程师" }
                                    }
                                }
                            }
                            ListItem {
                                div { style: "display:flex; align-items:center; gap:12px;",
                                    Avatar { size: Size::Md, shape: Shape::Square, src: "https://i.pravatar.cc/48?img=5".to_string(), alt: "头像".to_string(), "" }
                                    div {
                                        div { style: "color:var(--ctrl-text); line-height:1.5;", "Bob Wang" }
                                        div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary); margin-top:2px;", "后端开发" }
                                    }
                                }
                            }
                        }
                    }
                },
                code: "ListItem {\n    div { style: \"display:flex; align-items:center; gap:12px;\",\n        Avatar { size: Size::Md, \"A\" }\n        div { div { \"Alice Chen\" } div { \"前端工程师\" } }\n    }\n}".to_string(),
            }

            DemoBox {
                title: "带操作区".to_string(),
                description: Some("通过 extra 属性在列表项右侧放置操作按钮或状态信息。".to_string()),
                demo: rsx! {
                    div { style: "max-width:480px;",
                        List {
                            ListItem {
                                extra: rsx! { Button { variant: Variant::Ghost, size: Size::Sm, "管理" } },
                                div {
                                    div { style: "color:var(--ctrl-text); line-height:1.5;", "通知设置" }
                                    div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary); margin-top:2px;", "管理你的通知偏好" }
                                }
                            }
                            ListItem {
                                extra: rsx! { Button { variant: Variant::Outline, size: Size::Sm, "设置" } },
                                div {
                                    div { style: "color:var(--ctrl-text); line-height:1.5;", "隐私设置" }
                                    div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary); margin-top:2px;", "控制谁可以看到你的信息" }
                                }
                            }
                            ListItem {
                                extra: rsx! { span { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary);", "已启用" } },
                                div {
                                    div { style: "color:var(--ctrl-text); line-height:1.5;", "账号安全" }
                                    div { style: "font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary); margin-top:2px;", "保护你的账户安全" }
                                }
                            }
                        }
                    }
                },
                code: "ListItem {\n    extra: rsx! { Button { variant: Variant::Ghost, size: Size::Sm, \"管理\" } },\n    div { div { \"通知设置\" } div { \"管理你的通知偏好\" } }\n}".to_string(),
            }

            DemoBox {
                title: "无边框无分隔".to_string(),
                description: Some("通过 bordered / split 控制外边框与分隔线。".to_string()),
                demo: rsx! {
                    div { style: "max-width:480px;",
                        List { bordered: false, split: false,
                            ListItem { div { style: "color:var(--ctrl-text);", "项目一" } }
                            ListItem { div { style: "color:var(--ctrl-text);", "项目二" } }
                            ListItem { div { style: "color:var(--ctrl-text);", "项目三" } }
                        }
                    }
                },
                code: "List { bordered: false, split: false,\n    ListItem { \"项目一\" }\n    ListItem { \"项目二\" }\n}".to_string(),
            }

            h2 { "List Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("bordered", "bool", "true", "是否显示外边框"),
                ("split", "bool", "true", "是否显示列表项分隔线"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "列表项内容"),
            ]}

            h2 { "ListItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("extra", "Option<Element>", "None", "右侧额外内容（操作按钮等）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "列表项主体内容"),
            ]}
        }
    }
}
