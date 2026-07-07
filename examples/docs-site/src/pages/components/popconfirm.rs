use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn PopconfirmPage() -> Element {
    rsx! {
        div { id: "popconfirm", style: "margin-top: 64px;",
            h1 { "Popconfirm 气泡确认框" }
            p { "点击触发元素弹出确认气泡，用于需要用户二次确认的场景（如删除操作）。" }

            DemoBox {
                title: "基础用法".to_string(),
                description: Some("点击按钮弹出确认框，支持 confirm / cancel 回调。".to_string()),
                demo: rsx! {
                    Space { gap: "md".to_string(),
                        Popconfirm {
                            title: "确认删除".to_string(),
                            description: "删除后数据不可恢复，确定要删除吗？".to_string(),
                            Button { variant: Variant::Outline, size: Size::Sm, "删除" }
                        }
                        Popconfirm {
                            title: "确认发布".to_string(),
                            description: "确定要发布这篇文章吗？".to_string(),
                            confirm_text: "发布".to_string(),
                            cancel_text: "再想想".to_string(),
                            placement: Placement::Right,
                            Button { variant: Variant::Primary, size: Size::Sm, "发布" }
                        }
                    }
                },
                code: r#"Popconfirm {
    title: "确认删除".into(),
    description: "删除后数据不可恢复，确定要删除吗？".into(),
    onconfirm: move |_| { /* do delete */ },
    oncancel: move |_| { /* do nothing */ },
    Button { variant: Variant::Danger, "删除" }
}"#.to_string(),
            }

            DemoBox {
                title: "不同弹出位置".to_string(),
                description: Some("通过 placement 设置确认框弹出的方向。".to_string()),
                demo: rsx! {
                    Space { wrap: true, gap: "md".to_string(),
                        for p in [Placement::Top, Placement::Bottom, Placement::Left, Placement::Right] {
                            Popconfirm {
                                key: "{p:?}",
                                title: "确认操作".to_string(),
                                placement: p,
                                Button { variant: Variant::Outline, size: Size::Sm, "{p:?}" }
                            }
                        }
                    }
                },
                code: "Popconfirm { placement: Placement::Top, title: \"确认操作\".into(), Button { \"Top\" } }".to_string(),
            }

            DemoBox {
                title: "自定义按钮文字".to_string(),
                description: Some("通过 confirm_text / cancel_text 自定义按钮文案。".to_string()),
                demo: rsx! {
                    Popconfirm {
                        title: "保存修改".to_string(),
                        description: "确定要保存当前编辑的内容吗？".to_string(),
                        confirm_text: "保存".to_string(),
                        cancel_text: "放弃".to_string(),
                        Button { variant: Variant::Primary, size: Size::Sm, "保存" }
                    }
                },
                code: r#"Popconfirm {
    title: "保存修改".into(),
    confirm_text: "保存".into(),
    cancel_text: "放弃".into(),
    Button { "保存" }
}"#.to_string(),
            }

            h2 { "Popconfirm Props" }
            PropsTable {
                headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()],
                rows: vec![
                    ("title", "String", "\"确认操作\"", "确认框标题"),
                    ("description", "String", "\"\"", "确认框描述文案"),
                    ("confirm_text", "String", "\"确定\"", "确认按钮文字"),
                    ("cancel_text", "String", "\"取消\"", "取消按钮文字"),
                    ("placement", "Placement", "Top", "弹出位置：Top / Bottom / Left / Right"),
                    ("class", "String", "\"\"", "自定义类名"),
                    ("onconfirm", "EventHandler<()>", "None", "点击确认按钮回调"),
                    ("oncancel", "EventHandler<()>", "None", "点击取消按钮（或点外部关闭）回调"),
                    ("children", "Element", "—", "触发元素"),
                ],
            }
        }
    }
}
