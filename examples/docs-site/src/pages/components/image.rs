use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

fn api_preview_button(mut api: ImagePreviewAPI) -> Element {
    rsx! {
        Button {
            variant: Variant::Primary,
            size: Size::Sm,
            onclick: move |_| {
                api.open(vec![
                    "https://picsum.photos/id/50/800/600".to_string(),
                    "https://picsum.photos/id/60/800/600".to_string(),
                    "https://picsum.photos/id/70/800/600".to_string(),
                ], 0);
            },
            "打开图片预览"
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn ImagePage() -> Element {
    let api = use_image_preview();
    rsx! {
div { id: "image", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Image 图片" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "图片组件支持懒加载、占位图、加载失败回退和预览功能。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 src 属性指定图片地址。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 12px;",
                        Image { src: "https://picsum.photos/200/150".to_string(), width: "200px".to_string(), height: "150px".to_string() }
                        Image { src: "https://picsum.photos/150/150".to_string(), width: "150px".to_string(), height: "150px".to_string() }
                    }
                },
                code: "Image { src: \"https://picsum.photos/200/150\".to_string(), width: \"200px\".to_string(), height: \"150px\".to_string() }".to_string(),
            }

            DemoBox {
                title: "加载失败".to_string(),
                description: Some("设置 fallback 提供加载失败时的占位图。".to_string()),
                demo: rsx! {
                    Image { src: "https://invalid.url/image.jpg".to_string(), width: "200px".to_string(), height: "150px".to_string(), fallback: "https://picsum.photos/200/150".to_string() }
                },
                code: "Image { src: \"...\".to_string(), fallback: \"https://picsum.photos/200/150\".to_string() }".to_string(),
            }

            DemoBox {
                title: "预览模式".to_string(),
                description: Some("设置 preview 为 true 允许点击预览大图。".to_string()),
                demo: rsx! {
                    Image { src: "https://picsum.photos/200/150".to_string(), width: "200px".to_string(), height: "150px".to_string(), preview: true }
                },
                code: "Image { src: \"...\".to_string(), preview: true }".to_string(),
            }

            DemoBox {
                title: "多图预览".to_string(),
                description: Some("通过 preview_urls 传入多张图片，预览时可左右切换。".to_string()),
                demo: rsx! {
                    Image {
                        src: "https://picsum.photos/id/10/200/150".to_string(),
                        width: "200px".to_string(),
                        height: "150px".to_string(),
                        preview: true,
                        preview_urls: vec![
                            "https://picsum.photos/id/10/800/600".to_string(),
                            "https://picsum.photos/id/20/800/600".to_string(),
                            "https://picsum.photos/id/30/800/600".to_string(),
                            "https://picsum.photos/id/40/800/600".to_string(),
                        ],
                    }
                },
                code: r#"Image {
    src: "thumbnail.jpg",
    preview: true,
    preview_urls: vec![
        "img1.jpg".into(),
        "img2.jpg".into(),
        "img3.jpg".into(),
        "img4.jpg".into(),
    ],
}` "#.to_string(),
            }

            DemoBox {
                title: "API 触发预览".to_string(),
                description: Some("通过 use_image_preview() 获取 API，无需 Image 组件即可呼出预览。".to_string()),
                demo: api_preview_button(api),
                code: r#"let mut api = use_image_preview();
api.open(vec![
    "img1.jpg".into(),
    "img2.jpg".into(),
    "img3.jpg".into(),
], 0);"#.to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "ImagePreviewProvider —— 命令式容器" }
            p { style: "font-size: var(--ctrl-font-size-md); color: var(--ctrl-text-secondary); margin-bottom: 16px;",
                "使用 API 触发预览前，需在应用根节点包裹 ImagePreviewProvider："
            }
            DemoBox {
                title: String::new(),
                description: None,
                demo: rsx! { span { style: "font-family: monospace; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);", "ImagePreviewProvider {{ /* 你的应用 */ }}" } },
                code: r#"rsx! {
    ImagePreviewProvider {
        // 你的应用路由或页面
        Router::<Route> {}
    }
}"#.to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "use_image_preview() API" }
            PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("api.open(urls, index)", "Vec<String>, usize", "打开图片预览，urls 为预览图片列表，index 为初始索引", ""),
                ("api.close()", "—", "关闭预览", ""),
            ]}

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Image Props（声明式）" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("src", "String", "\"\"", "图片地址"),
                ("alt", "String", "\"\"", "替代文本"),
                ("width", "String", "\"\"", "宽度"),
                ("height", "String", "\"\"", "高度"),
                ("fallback", "String", "\"\"", "加载失败时的占位图"),
                ("preview", "bool", "false", "是否可预览"),
                ("preview_urls", "Vec<String>", "[]", "多图预览列表，传入后在预览中可左右切换"),
                ("lazy", "bool", "true", "是否懒加载"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}
