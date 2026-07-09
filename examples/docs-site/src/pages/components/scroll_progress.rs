use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn ScrollProgressPage() -> Element {
    rsx! {
div { id: "scroll-progress", style: "margin-top: 64px;",
            h1 {
                "ScrollProgress 滚动进度"
            }
            p {
                "监听窗口滚动，在顶部显示页面阅读进度。常用于文章、文档等长页面。将组件放在页面任意位置即可，默认以 fixed 固定在视口顶部。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("本页顶部已挂载一条进度条 —— 上下滚动页面即可看到它随阅读进度填充。".to_string()),
                demo: rsx! {
                    ScrollProgress { target: ".docs-scroll-container".to_string() }
                    div { style: "padding:24px; border-radius:var(--ctrl-radius-md); background:var(--ctrl-bg-secondary); border:1px solid var(--ctrl-border); color:var(--ctrl-text-secondary);",
                        "↑ 查看页面最顶部的进度条，滚动本页观察其变化。"
                    }
                },
                code: "// 页面滚动发生在窗口时：\nScrollProgress {}\n\n// 滚动发生在内部容器时，指定其选择器：\nScrollProgress { target: \".docs-scroll-container\".to_string() }".to_string(),
            }

            DemoBox {
                title: "自定义外观".to_string(),
                description: Some("height 控制粗细，color 自定义颜色（支持渐变）。".to_string()),
                demo: rsx! {
                    ScrollProgress {
                        height: 6,
                        target: ".docs-scroll-container".to_string(),
                        color: "linear-gradient(90deg, #06b6d4, #a855f7, #ef4444)".to_string(),
                    }
                    div { style: "padding:24px; border-radius:var(--ctrl-radius-md); background:var(--ctrl-bg-secondary); border:1px solid var(--ctrl-border); color:var(--ctrl-text-secondary);",
                        "此示例把进度条加粗为 6px 并使用三色渐变。"
                    }
                },
                code: "ScrollProgress {\n    height: 6,\n    color: \"linear-gradient(90deg, #06b6d4, #a855f7, #ef4444)\".to_string(),\n}".to_string(),
            }

            h2 { "ScrollProgress Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("height", "u32", "3", "进度条高度 px"),
                ("color", "String", "主题主色渐变", "进度条颜色（支持渐变）"),
                ("target", "String", "\"\"", "滚动容器 CSS 选择器，留空监听窗口"),
                ("fixed", "bool", "true", "true 固定视口顶部；false 贴附父容器顶部"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}
