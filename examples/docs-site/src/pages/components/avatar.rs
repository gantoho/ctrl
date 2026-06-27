use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn AvatarPage() -> Element {
    rsx! {
div { id: "avatar", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Avatar 头像"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "头像用于表示用户或实体，支持图片和文字 fallback，圆形和方形两种形状。"
            }

            DemoBox {
                title: "尺寸".to_string(),
                description: Some("Sm 28px / Md 36px / Lg 48px".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Avatar { size: Size::Sm, "A" }
                        Avatar { size: Size::Md, "B" }
                        Avatar { size: Size::Lg, "C" }
                    }
                },
                code: "Avatar { size: Size::Sm, \"A\" }\nAvatar { size: Size::Md, \"B\" }\nAvatar { size: Size::Lg, \"C\" }".to_string(),
            }

            DemoBox {
                title: "形状".to_string(),
                description: Some("默认圆形，设置 shape=\"square\" 为方形。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Avatar { size: Size::Lg, "圆" }
                        Avatar { size: Size::Lg, shape: "square".to_string(), "方" }
                    }
                },
                code: "Avatar { size: Size::Lg, \"圆\" }\nAvatar { size: Size::Lg, shape: \"square\", \"方\" }".to_string(),
            }

            DemoBox {
                title: "图片头像".to_string(),
                description: Some("通过 src 传入图片地址，图片自动裁切填满。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Avatar { size: Size::Lg, src: "https://i.pravatar.cc/96?img=1".to_string(), alt: "用户头像".to_string(), "" }
                        Avatar { size: Size::Lg, shape: "square".to_string(), src: "https://i.pravatar.cc/96?img=2".to_string(), alt: "方形头像".to_string(), "" }
                    }
                },
                code: "Avatar { size: Size::Lg, src: \"https://...\", alt: \"用户头像\" }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Avatar Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("src", "String", "\"\"", "图片地址"),
                ("alt", "String", "\"\"", "替代文字"),
                ("size", "Size", "Md", "头像尺寸"),
                ("shape", "String", "\"circle\"", "形状（circle / square）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "文字 fallback 内容"),
            ]}
        }
    }
}
