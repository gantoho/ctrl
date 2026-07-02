use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn FloatButtonPage() -> Element {
    rsx! {
        h1 { "FloatButton 浮动按钮" }
        p { "固定在屏幕边缘的浮动操作按钮，常用于快捷操作入口。" }

        DemoBox { title: "基础用法".to_string(), description: Some("默认悬浮在右下角。".to_string()),
            demo: rsx! {
                FloatButton {
                    onclick: move |_| { /* 点击 */ },
                }
            },
            code: "FloatButton { onclick: move |_| ... }".to_string(),
        }

        DemoBox { title: "不同位置".to_string(), description: Some("通过 position 设置按钮位置。".to_string()),
            demo: rsx! {
                div { style: "height:200px;position:relative;background:var(--ctrl-bg-secondary);border-radius:8px;",
                    FloatButton { position: FloatButtonPosition::BottomRight, offset: 12,
                        icon: r#"<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="2"/><text x="12" y="16" text-anchor="middle" font-size="14" fill="currentColor">BR</text></svg>"#.to_string(),
                    }
                    FloatButton { position: FloatButtonPosition::TopRight, offset: 12,
                        icon: r#"<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="2"/><text x="12" y="16" text-anchor="middle" font-size="14" fill="currentColor">TR</text></svg>"#.to_string(),
                    }
                    FloatButton { position: FloatButtonPosition::BottomLeft, offset: 12,
                        icon: r#"<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="2"/><text x="12" y="16" text-anchor="middle" font-size="14" fill="currentColor">BL</text></svg>"#.to_string(),
                    }
                    FloatButton { position: FloatButtonPosition::TopLeft, offset: 12,
                        icon: r#"<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="2"/><text x="12" y="16" text-anchor="middle" font-size="14" fill="currentColor">TL</text></svg>"#.to_string(),
                    }
                }
            },
            code: r#"FloatButton { position: FloatButtonPosition::BottomRight, ... }
FloatButton { position: FloatButtonPosition::TopLeft, offset: 24, ... }"#.to_string(),
        }

        DemoBox { title: "外形与类型".to_string(), description: Some("支持圆形/方形，以及 default/primary 类型。".to_string()),
            demo: rsx! {
                Space { direction: Direction::Horizontal, gap: "md".to_string(),
                    FloatButton { shape: "circle".to_string(), btn_type: "default".to_string(), label: "圆形".to_string() }
                    FloatButton { shape: "square".to_string(), btn_type: "default".to_string(), label: "方形".to_string() }
                    FloatButton { shape: "circle".to_string(), btn_type: "primary".to_string(), label: "主色".to_string() }
                }
            },
            code: r#"FloatButton { shape: "circle", btn_type: "default", label: "圆形" }
FloatButton { shape: "square", btn_type: "primary", label: "主色" }"#.to_string(),
        }

        DemoBox { title: "徽标".to_string(), description: Some("支持小圆点和数字徽标。".to_string()),
            demo: rsx! {
                Space { direction: Direction::Horizontal, gap: "md".to_string(),
                    FloatButton { dot: true, label: "小红点".to_string() }
                    FloatButton { badge: 5, label: "数量".to_string() }
                    FloatButton { badge: 120, label: "99+".to_string() }
                }
            },
            code: r#"FloatButton { dot: true, ... }
FloatButton { badge: 5, ... }
FloatButton { badge: 120, ... }  // 显示 "99+""#.to_string(),
        }

        DemoBox { title: "按钮组".to_string(), description: Some("将多个浮动按钮放在同一组中，统一位置。".to_string()),
            demo: rsx! {
                FloatButtonGroup { position: FloatButtonPosition::BottomRight, offset: 24,
                    FloatButton {
                        btn_type: "primary".to_string(),
                        icon: r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 5v14M5 12h14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>"#.to_string(),
                        tooltip: "新建".to_string(),
                    }
                    FloatButton {
                        icon: r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 16l4-4-4-4M8 12h8" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>"#.to_string(),
                        tooltip: "编辑".to_string(),
                    }
                    FloatButton {
                        icon: r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M3 6h18M3 12h18M3 18h18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>"#.to_string(),
                        tooltip: "更多".to_string(),
                    }
                }
            },
            code: r#"FloatButtonGroup { position: FloatButtonPosition::BottomRight, offset: 24,
    FloatButton { btn_type: "primary", label: "新建" }
    FloatButton { label: "编辑" }
}"#.to_string(),
        }

        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("position", "FloatButtonPosition", "BottomRight", "位置"),
            ("offset", "u32", "24", "距边缘偏移（px）"),
            ("label", "Option<String>", "None", "按钮文案"),
            ("icon", "Option<String>", "None", "图标 SVG"),
            ("onclick", "Option<EventHandler>", "None", "点击回调"),
            ("dot", "bool", "false", "是否显示红点徽标"),
            ("badge", "Option<u32>", "None", "徽标数字"),
            ("tooltip", "Option<String>", "None", "提示文字"),
            ("shape", "String", "\"circle\"", "形状：circle / square"),
            ("btn_type", "String", "\"default\"", "类型：default / primary"),
            ("class", "String", "\"\"", "自定义类名"),
            ("style", "String", "\"\"", "自定义样式"),
        ] }
    }
}
