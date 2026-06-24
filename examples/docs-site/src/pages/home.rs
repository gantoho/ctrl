use dioxus::prelude::*;
// Routable, Router, Link, Outlet 等已通过 dioxus::prelude::* 导入
use ctrl::prelude::*;
use crate::components::codeblock::CodeBlock;
use crate::Route;

/// 首页 —— 英雄区 + 快速开始
#[component]
#[allow(non_snake_case)]
pub fn HomePage() -> Element {
    let navigator = use_navigator();
    let deps_code: Vec<String> = vec![
        "[dependencies]",
        "dioxus = { version = \"0.5\", features = [\"web\"] }",
        "ctrl = \"0.1\"",
    ].into_iter().map(String::from).collect();

    let provider_code: Vec<String> = vec![
        "use dioxus::prelude::*;",
        "use ctrl::prelude::*;",
        "",
        "#[allow(non_snake_case)]",
        "fn App() -> Element {",
        "    rsx! {",
        "        ThemeProvider {",
        "            // 你的应用内容",
        "        }",
        "    }",
        "}",
    ].into_iter().map(String::from).collect();

    let usage_code: Vec<String> = vec![
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
    ].into_iter().map(String::from).collect();

    rsx! {
        div {
            style: "text-align: center; padding: 60px 20px; margin-bottom: 48px;",
            h1 { style: "font-size: 2.5rem; font-weight: 800; color: var(--ctrl-text); margin-bottom: 12px;", "Ctrl UI" }
            p { style: "font-size: 1.125rem; color: var(--ctrl-text-secondary); margin-bottom: 32px; max-width: 500px; margin-left: auto; margin-right: auto;", "开箱即用的 Dioxus Web 组件库。轻松定制主题，快速构建应用。" }
            div { style: "display: flex; gap: 12px; justify-content: center; flex-wrap: wrap;",
                Button { variant: Variant::Primary, size: Size::Lg, onclick: move |_| { let _ = navigator.push(Route::ComponentsPage {}); }, "开始使用" }
                Button { variant: Variant::Outline, size: Size::Lg, "在 GitHub 上查看" }
            }
        }

        Section { title: "核心特性" }
        div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 24px; margin-bottom: 48px;",
            FeatureCard { title: "开箱即用", desc: "只需添加一个依赖即可使用所有组件，无需额外 CSS 配置或构建步骤。" }
            FeatureCard { title: "主题可定制", desc: "通过 ThemeProvider 传入自定义主题，轻松覆盖颜色、字体、圆角等设计 Token。" }
            FeatureCard { title: "样式可覆盖", desc: "每个组件都支持 class 和 style 属性，自由调整样式以满足你的需求。" }
            FeatureCard { title: "Dioxus 原生", desc: "深度集成 Dioxus 响应式模型，使用 use_signal 管理组件状态，类型安全。" }
        }

        Section { title: "快速开始" }
        SectionHeading { title: "1. 添加依赖" }
        CodeBlockMulti { lines: deps_code }
        SectionHeading { title: "2. 包裹 ThemeProvider" }
        CodeBlockMulti { lines: provider_code }
        SectionHeading { title: "3. 使用组件" }
        CodeBlockMulti { lines: usage_code }

        Section { title: "实时效果" }
        div { style: "display: flex; flex-direction: column; gap: 16px; align-items: flex-start;",
            CounterDemo {}
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn Section(title: String) -> Element {
    rsx! {
        h2 { style: "font-size: 1.5rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 20px; padding-bottom: 8px; border-bottom: 2px solid var(--ctrl-primary);", "{title}" }
    }
}

#[component]
#[allow(non_snake_case)]
fn SectionHeading(title: String) -> Element {
    rsx! {
        h3 { style: "font-size: 1rem; font-weight: 600; color: var(--ctrl-text); margin: 24px 0 12px;", "{title}" }
    }
}

#[component]
#[allow(non_snake_case)]
fn CodeBlockMulti(lines: Vec<String>) -> Element {
    let code = lines.join("\n");
    rsx! { CodeBlock { code: code, lang: Some("rust".to_string()) } }
}

#[component]
#[allow(non_snake_case)]
fn FeatureCard(title: String, desc: String) -> Element {
    rsx! {
        div { style: "padding: 24px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-lg); background: var(--ctrl-bg-secondary);",
            h3 { style: "font-size: 1rem; font-weight: 600; color: var(--ctrl-text); margin-bottom: 8px;", "{title}" }
            p { style: "font-size: var(--ctrl-font-size-md); color: var(--ctrl-text-secondary); margin: 0;", "{desc}" }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn CounterDemo() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        div { style: "display: flex; align-items: center; gap: 16px; flex-wrap: wrap;",
            Button { variant: Variant::Primary, onclick: move |_| count.set(count() + 1), "增加" }
            Button { variant: Variant::Ghost, onclick: move |_| count.set(0), "重置" }
            span { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin-left: 8px;", "计数: {count()}" }
        }
    }
}
