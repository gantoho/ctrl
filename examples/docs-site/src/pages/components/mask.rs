use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn MaskPage() -> Element {
    let mut visible = use_signal(|| false);
    let mut color_visible = use_signal(|| false);
    let mut blur_visible = use_signal(|| false);

    rsx! {
div { id: "mask", style: "margin-top: 64px;",
            h1 {
                "Mask 遮罩层"
            }
            p {
                "通用坐标遮罩层，用于阻止用户与底层内容交互，常用于弹窗、抽屉、图片预览等组件内部。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("open 为 true 时显示，点击遮罩区域通过 onclick 回调关闭。".to_string()),
                demo: rsx! {
                    div { style: "position:relative; height:200px; background:var(--ctrl-bg-secondary); border-radius:var(--ctrl-radius-md); border:1px solid var(--ctrl-border); display:flex; align-items:center; justify-content:center; overflow:hidden;",
                        p { style: "color:var(--ctrl-text-secondary);", "被遮罩覆盖的内容区域" }
                        if visible() {
                            Mask {
                                open: true,
                                onclick: move |_| visible.set(false),
                                div {
                                    style: "background:var(--ctrl-bg); padding:24px; border-radius:var(--ctrl-radius-lg); box-shadow:var(--ctrl-shadow-lg); text-align:center;",
                                    onclick: move |e: MouseEvent| e.stop_propagation(),
                                    p { style: "margin:0 0 12px 0; color:var(--ctrl-text);", "遮罩层已激活" }
                                    Button {
                                        variant: Variant::Primary,
                                        size: Size::Sm,
                                        onclick: move |_| visible.set(false),
                                        "关闭"
                                    }
                                }
                            }
                        }
                        Button {
                            variant: Variant::Primary,
                            onclick: move |_| visible.set(true),
                            "显示遮罩"
                        }
                    }
                },
                code: "Mask { open: true, onclick: |_| set_visible(false),\n    div { /* 遮罩上的弹窗内容 */ }\n}".to_string(),
            }

            DemoBox {
                title: "自定义颜色".to_string(),
                description: Some("通过 color 属性调整遮罩颜色与透明度。".to_string()),
                demo: rsx! {
                    div { style: "position:relative; height:200px; background:linear-gradient(135deg, var(--ctrl-primary), var(--ctrl-info)); border-radius:var(--ctrl-radius-md); display:flex; align-items:center; justify-content:center; overflow:hidden;",
                        p { style: "color:rgba(255,255,255,0.7);", "彩色背景区域" }
                        if color_visible() {
                            Mask {
                                open: true,
                                color: "rgba(0, 0, 0, 0.6)".to_string(),
                                onclick: move |_| color_visible.set(false),
                                div {
                                    style: "background:var(--ctrl-bg); padding:24px; border-radius:var(--ctrl-radius-lg); box-shadow:var(--ctrl-shadow-lg); text-align:center;",
                                    onclick: move |e: MouseEvent| e.stop_propagation(),
                                    p { style: "margin:0 0 12px 0; color:var(--ctrl-text);", "深色遮罩" }
                                    Button {
                                        variant: Variant::Outline,
                                        size: Size::Sm,
                                        onclick: move |_| color_visible.set(false),
                                        "关闭"
                                    }
                                }
                            }
                        }
                        Button {
                            variant: Variant::Secondary,
                            onclick: move |_| color_visible.set(true),
                            "显示深色遮罩"
                        }
                    }
                },
                code: "Mask { open: true, color: \"rgba(0, 0, 0, 0.6)\".to_string(), onclick: |_| ...,\n    // 内容\n}".to_string(),
            }

            DemoBox {
                title: "模糊遮罩".to_string(),
                description: Some("blur 为 true 时通过 backdrop-filter 启用背景模糊效果。".to_string()),
                demo: rsx! {
                    div { style: "position:relative; height:200px; background:var(--ctrl-bg-secondary); border-radius:var(--ctrl-radius-md); border:1px solid var(--ctrl-border); display:flex; align-items:center; justify-content:center; overflow:hidden;",
                        div { style: "display:flex; gap:8px;",
                            div { style: "width:40px; height:40px; background:var(--ctrl-primary); border-radius:var(--ctrl-radius-sm); opacity:0.17;" }
                            div { style: "width:40px; height:40px; background:var(--ctrl-primary); border-radius:var(--ctrl-radius-sm); opacity:0.33;" }
                            div { style: "width:40px; height:40px; background:var(--ctrl-primary); border-radius:var(--ctrl-radius-sm); opacity:0.5;" }
                            div { style: "width:40px; height:40px; background:var(--ctrl-primary); border-radius:var(--ctrl-radius-sm); opacity:0.67;" }
                            div { style: "width:40px; height:40px; background:var(--ctrl-primary); border-radius:var(--ctrl-radius-sm); opacity:0.83;" }
                            div { style: "width:40px; height:40px; background:var(--ctrl-primary); border-radius:var(--ctrl-radius-sm); opacity:1.0;" }
                        }
                        if blur_visible() {
                            Mask {
                                open: true,
                                blur: true,
                                color: "transparent".to_string(),
                            }
                        }
                        Button {
                            variant: Variant::Ghost,
                            style: "position:absolute; z-index:1;",
                            onclick: move |_| blur_visible.set(true),
                            "模糊遮罩"
                        }
                    }
                },
                code: "Mask { open: true, blur: true, color: \"transparent\".to_string() }".to_string(),
            }

            DemoBox {
                title: "透明遮罩".to_string(),
                description: Some("创建不可见但可点击的遮罩，阻止底层交互但不遮挡视觉。".to_string()),
                demo: rsx! {
                    div { style: "position:relative; height:200px; background:var(--ctrl-bg-secondary); border-radius:var(--ctrl-radius-md); border:1px solid var(--ctrl-border); display:flex; align-items:center; justify-content:center; overflow:hidden;",
                        Space { gap: "xs".to_string(),
                            Tag { color: "var(--ctrl-primary)".to_string(), "标签 A" }
                            Tag { color: "var(--ctrl-success)".to_string(), "标签 B" }
                            Tag { color: "var(--ctrl-warning)".to_string(), "标签 C" }
                        }
                        Mask {
                            open: true,
                            color: "transparent".to_string(),
                            div {
                                style: "background:var(--ctrl-bg); padding:16px 24px; border-radius:var(--ctrl-radius-md); box-shadow:var(--ctrl-shadow-md);",
                                onclick: move |e: MouseEvent| e.stop_propagation(),
                                p { style: "margin:0; font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary);", "透明遮罩 — 底层不可交互" }
                            }
                        }
                    }
                },
                code: "Mask { open: true, color: \"transparent\".to_string(),\n    // 底层内容可见但不可交互\n}".to_string(),
            }

            h2 { "Mask Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("open", "bool", "false", "是否显示遮罩"),
                ("blur", "bool", "false", "是否启用背景模糊"),
                ("color", "String", "var(--ctrl-mask-bg)", "遮罩背景色"),
                ("z_index", "i32", "1000", "层级"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onclick", "Option<EventHandler>", "None", "点击遮罩回调"),
                ("children", "Element", "—", "遮罩层上的内容"),
            ]}
        }
    }
}
