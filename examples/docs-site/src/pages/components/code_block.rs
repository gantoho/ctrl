use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn CodeBlockPage() -> Element {
    let sample_code = [
        "use dioxus::prelude::*;",
        "use ctrl::prelude::*;",
        "",
        "fn App() -> Element {",
        "    let mut count = use_signal(|| 0);",
        "    rsx! {",
        "        Button {",
        "            variant: Variant::Primary,",
        "            onclick: move |_| count.set(count() + 1),",
        "            \"Clicks: {count()}\"",
        "        }",
        "    }",
        "}",
    ].join("\n");

    let html_code = [
        "<!-- 按钮组件 -->",
        "<button class=\"ctrl-btn ctrl-btn--primary\">",
        "  <span>点击我</span>",
        "</button>",
    ].join("\n");

    rsx! {
        h1 { "CodeBlock 代码块" }
        p { "用于在页面中展示代码片段，支持行号、复制等功能。" }

        DemoBox { title: "基本用法".to_string(), description: None,
            demo: rsx! {
                CodeBlock { code: sample_code.clone() }
            },
            code: "CodeBlock { code: sample_code }".to_string(),
        }

        DemoBox { title: "显示语言标签".to_string(), description: Some("通过 language 属性指定代码语言，显示在顶部栏。".to_string()),
            demo: rsx! {
                CodeBlock { code: sample_code.clone(), language: Some("rust".to_string()) }
            },
            code: "CodeBlock { code: sample_code, language: Some(\"rust\") }".to_string(),
        }

        DemoBox { title: "显示行号".to_string(), description: Some("show_line_numbers: true 时每行显示行号，鼠标悬停高亮。".to_string()),
            demo: rsx! {
                CodeBlock { code: html_code.to_string(), language: Some("html".to_string()), show_line_numbers: true }
            },
            code: "CodeBlock { code, language: Some(\"html\"), show_line_numbers: true }".to_string(),
        }

        DemoBox { title: "紧凑模式".to_string(), description: Some("compact: true 时减小内边距，适合窄区域展示。".to_string()),
            demo: rsx! {
                CodeBlock { code: "let x = 42;".to_string(), language: Some("rust".to_string()), compact: true }
            },
            code: "CodeBlock { code: \"let x = 42;\", language: Some(\"rust\"), compact: true }".to_string(),
        }

        DemoBox { title: "隐藏复制按钮".to_string(), description: Some("show_copy: false 隐藏右上角复制按钮。".to_string()),
            demo: rsx! {
                CodeBlock { code: sample_code, show_copy: false, language: Some("rust".to_string()) }
            },
            code: "CodeBlock { code, show_copy: false, language: Some(\"rust\") }".to_string(),
        }

        h2 { "CodeBlock Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("code", "String", "—", "代码内容"),
            ("language", "Option<String>", "None", "代码语言（显示在顶部栏）"),
            ("show_line_numbers", "bool", "false", "是否显示行号"),
            ("show_copy", "bool", "true", "是否显示复制按钮"),
            ("compact", "bool", "false", "紧凑模式"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}
    }
}
