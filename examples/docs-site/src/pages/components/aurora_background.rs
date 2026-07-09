use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn AuroraBackgroundPage() -> Element {
    rsx! {
div { id: "aurora-background", style: "margin-top: 64px;",
            h1 {
                "AuroraBackground 极光背景"
            }
            p {
                "多色渐变缓慢流动，形成极光氛围背景，常用于登录页、Hero 区、特色板块。children 展示在极光之上。支持 prefers-reduced-motion 下停用。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("包裹内容即可获得流动极光背景。".to_string()),
                demo: rsx! {
                    AuroraBackground {
                        style: "height:260px; border-radius:var(--ctrl-radius-lg); display:flex; align-items:center; justify-content:center;".to_string(),
                        div { style: "text-align:center; position:relative;",
                            div { style: "font-size:24px; font-weight:800; color:var(--ctrl-text);", "极光背景" }
                            div { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm); margin-top:4px;", "多色渐变缓缓流动" }
                        }
                    }
                },
                code: "AuroraBackground {\n    style: \"height:260px;\".to_string(),\n    div { \"...\" }\n}".to_string(),
            }

            DemoBox {
                title: "自定义配色".to_string(),
                description: Some("通过 colors 传入渐变色列表，duration 控制流动速度，blur 控制柔和度。".to_string()),
                demo: rsx! {
                    AuroraBackground {
                        colors: vec!["#f59e0b".to_string(), "#ef4444".to_string(), "#ec4899".to_string(), "#f59e0b".to_string()],
                        duration: 8,
                        blur: 50,
                        style: "height:200px; border-radius:var(--ctrl-radius-lg); display:flex; align-items:center; justify-content:center;".to_string(),
                        span { style: "color:#fff; font-weight:700; font-size:18px; position:relative;", "暖色极光" }
                    }
                },
                code: "AuroraBackground {\n    colors: vec![\"#f59e0b\".into(), \"#ef4444\".into(), \"#ec4899\".into()],\n    duration: 8,\n    blur: 50,\n}".to_string(),
            }

            h2 { "AuroraBackground Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("colors", "Vec<String>", "默认多彩组合", "极光渐变色列表"),
                ("duration", "u32", "12", "流动动画时长（秒）"),
                ("blur", "u32", "40", "模糊强度 px"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "覆盖在极光之上的内容"),
            ]}
        }
    }
}
