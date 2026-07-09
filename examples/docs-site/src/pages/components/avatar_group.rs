use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn AvatarGroupPage() -> Element {
    rsx! {
div { id: "avatar-group", style: "margin-top: 64px;",
            h1 {
                "AvatarGroup 头像组"
            }
            p {
                "将多个头像堆叠展示，超出 max 的部分自动折叠为 +N，常用于协作成员、点赞列表等场景。复用 Avatar 渲染每一项。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("传入 items 列表，头像堆叠展示，悬停上浮。".to_string()),
                demo: rsx! {
                    AvatarGroup {
                        items: vec![
                            AvatarGroupItem::image("https://i.pravatar.cc/64?img=1", "用户1"),
                            AvatarGroupItem::image("https://i.pravatar.cc/64?img=2", "用户2"),
                            AvatarGroupItem::image("https://i.pravatar.cc/64?img=3", "用户3"),
                            AvatarGroupItem::image("https://i.pravatar.cc/64?img=4", "用户4"),
                        ],
                    }
                },
                code: "AvatarGroup {\n    items: vec![\n        AvatarGroupItem::image(\"...\", \"用户1\"),\n        AvatarGroupItem::image(\"...\", \"用户2\"),\n    ],\n}".to_string(),
            }

            DemoBox {
                title: "折叠溢出".to_string(),
                description: Some("设置 max 限制显示数量，其余折叠为 +N。".to_string()),
                demo: rsx! {
                    AvatarGroup {
                        max: 3,
                        items: vec![
                            AvatarGroupItem::image("https://i.pravatar.cc/64?img=5", "用户5"),
                            AvatarGroupItem::image("https://i.pravatar.cc/64?img=6", "用户6"),
                            AvatarGroupItem::image("https://i.pravatar.cc/64?img=7", "用户7"),
                            AvatarGroupItem::image("https://i.pravatar.cc/64?img=8", "用户8"),
                            AvatarGroupItem::image("https://i.pravatar.cc/64?img=9", "用户9"),
                            AvatarGroupItem::image("https://i.pravatar.cc/64?img=10", "用户10"),
                        ],
                    }
                },
                code: "AvatarGroup {\n    max: 3,\n    items: vec![ /* 6 个头像 */ ],\n}".to_string(),
            }

            DemoBox {
                title: "文本头像".to_string(),
                description: Some("无图片时使用 text 构造 fallback 文本，并可指定背景色。".to_string()),
                demo: rsx! {
                    AvatarGroup {
                        max: 4,
                        items: vec![
                            AvatarGroupItem::text("A").color("var(--ctrl-primary)"),
                            AvatarGroupItem::text("B").color("var(--ctrl-success)"),
                            AvatarGroupItem::text("C").color("var(--ctrl-warning)"),
                            AvatarGroupItem::text("D").color("var(--ctrl-danger)"),
                            AvatarGroupItem::text("E").color("var(--ctrl-info)"),
                            AvatarGroupItem::text("F"),
                        ],
                    }
                },
                code: "AvatarGroup {\n    max: 4,\n    items: vec![\n        AvatarGroupItem::text(\"A\").color(\"var(--ctrl-primary)\"),\n        AvatarGroupItem::text(\"B\").color(\"var(--ctrl-success)\"),\n    ],\n}".to_string(),
            }

            DemoBox {
                title: "尺寸与形状".to_string(),
                description: Some("通过 size 与 shape 控制头像大小和形状。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(), direction: Direction::Vertical,
                        AvatarGroup {
                            size: Size::Sm,
                            max: 4,
                            items: vec![
                                AvatarGroupItem::image("https://i.pravatar.cc/64?img=11", "u"),
                                AvatarGroupItem::image("https://i.pravatar.cc/64?img=12", "u"),
                                AvatarGroupItem::image("https://i.pravatar.cc/64?img=13", "u"),
                                AvatarGroupItem::image("https://i.pravatar.cc/64?img=14", "u"),
                                AvatarGroupItem::image("https://i.pravatar.cc/64?img=15", "u"),
                            ],
                        }
                        AvatarGroup {
                            size: Size::Lg,
                            shape: Shape::Square,
                            max: 4,
                            items: vec![
                                AvatarGroupItem::image("https://i.pravatar.cc/64?img=16", "u"),
                                AvatarGroupItem::image("https://i.pravatar.cc/64?img=17", "u"),
                                AvatarGroupItem::image("https://i.pravatar.cc/64?img=18", "u"),
                                AvatarGroupItem::image("https://i.pravatar.cc/64?img=19", "u"),
                                AvatarGroupItem::image("https://i.pravatar.cc/64?img=20", "u"),
                            ],
                        }
                    }
                },
                code: "AvatarGroup { size: Size::Sm, max: 4, items: vec![ ... ] }\nAvatarGroup { size: Size::Lg, shape: Shape::Square, max: 4, items: vec![ ... ] }".to_string(),
            }

            h2 { "AvatarGroup Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("items", "Vec<AvatarGroupItem>", "—", "头像列表"),
                ("max", "Option<usize>", "None", "最多显示数量，超出折叠为 +N"),
                ("size", "Size", "Md", "头像尺寸：Sm / Md / Lg"),
                ("shape", "Shape", "Circle", "形状：Circle / Square"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}

            h2 { "AvatarGroupItem 构造" }
            PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("image(src, alt)", "into<String>", "图片头像", ""),
                ("text(fallback)", "into<String>", "文本头像（首字母等）", ""),
                (".color(c)", "into<String>", "设置文本头像背景色", ""),
            ]}
        }
    }
}
