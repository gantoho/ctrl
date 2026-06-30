use dioxus::prelude::*;
use ctrl::prelude::*;
use crate::components::codeblock::CodeBlock;
use crate::Route;

#[component]
#[allow(non_snake_case)]
pub fn HomePage() -> Element {
    let navigator = use_navigator();

    let deps_code = [
        "[dependencies]",
        "dioxus = { version = \"0.7\", features = [\"web\"] }",
        "ctrl = \"0.1\"",
    ].join("\n");

    let provider_code = [
        "use dioxus::prelude::*;",
        "use ctrl::prelude::*;",
        "",
        "#[allow(non_snake_case)]",
        "fn App() -> Element {",
        "    rsx! {",
        "        ThemeProvider {",
        "            App {}",
        "        }",
        "    }",
        "}",
    ].join("\n");

    let usage_code = [
        "use dioxus::prelude::*;",
        "use ctrl::prelude::*;",
        "",
        "fn Greeting() -> Element {",
        "    let mut count = use_signal(|| 0);",
        "    rsx! {",
        "        Button {",
        "            variant: Variant::Primary,",
        "            onclick: move |_| count.set(count() + 1),",
        "            \"点击次数: {count()}\"",
        "        }",
        "    }",
        "}",
    ].join("\n");

    rsx! {
        Space { gap: "xl".to_string(), direction: Direction::Vertical,

            // 英雄区
            Space { gap: "md".to_string(), direction: Direction::Vertical, style: "text-align:center; padding:48px 0 32px;",
                h1 { "Ctrl UI" }
                p { "开箱即用的 Dioxus Web 组件库。轻松定制主题，快速构建应用。" }
                Space { gap: "md".to_string(),
                    Button { variant: Variant::Primary, size: Size::Lg, onclick: move |_| { let _ = navigator.push(Route::ComponentsIndex {}); }, "开始使用" }
                    Button { variant: Variant::Outline, size: Size::Lg, "在 GitHub 上查看" }
                }
            }

            h2 { "核心特性" }
            Row { gutter: 24,
                Col { span: 12, md: Some(8), lg: Some(6),
                    Card { title: "开箱即用".to_string(),
                        p { "只需添加一个依赖即可使用所有组件，无需额外 CSS 配置或构建步骤。" }
                    }
                }
                Col { span: 12, md: Some(8), lg: Some(6),
                    Card { title: "主题可定制".to_string(),
                        p { "通过 ThemeProvider 传入自定义主题，轻松覆盖颜色、字体、圆角等设计 Token。" }
                    }
                }
                Col { span: 12, md: Some(8), lg: Some(6),
                    Card { title: "样式可覆盖".to_string(),
                        p { "每个组件都支持 class 和 style 属性，自由调整样式以满足你的需求。" }
                    }
                }
                Col { span: 12, md: Some(8), lg: Some(6),
                    Card { title: "Dioxus 原生".to_string(),
                        p { "深度集成 Dioxus 响应式模型，使用 use_signal 管理组件状态，类型安全。" }
                    }
                }
            }

            h2 { "快速开始" }
            h3 { "1. 添加依赖" }
            CodeBlock { code: deps_code, lang: Some("rust".to_string()) }
            h3 { "2. 包裹 ThemeProvider" }
            CodeBlock { code: provider_code, lang: Some("rust".to_string()) }
            h3 { "3. 使用组件" }
            CodeBlock { code: usage_code, lang: Some("rust".to_string()) }

            h2 { "实时效果" }
            CounterDemo {}
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn CounterDemo() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        Space { gap: "md".to_string(),
            Button { variant: Variant::Primary, onclick: move |_| count.set(count() + 1), "增加" }
            Button { variant: Variant::Ghost, onclick: move |_| count.set(0), "重置" }
            span { "计数: {count()}" }
        }
    }
}
