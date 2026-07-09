use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn WordRotatePage() -> Element {
    rsx! {
div { id: "word-rotate", style: "margin-top: 64px;",
            h1 { "WordRotate 单词轮播" }
            p {
                "标题中某个词平滑地垂直翻转切换，每个单词停留 duration 毫秒后翻入下一个。常用于 hero 标题中的动态关键词展示。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("传入 words 列表，组件自动循环轮播。".to_string()),
                demo: rsx! {
                    div { style: "font-size:30px; font-weight:800; color:var(--ctrl-text); display:flex; gap:10px; align-items:baseline; justify-content:center; padding:24px;",
                        span { "Build" }
                        WordRotate { words: vec!["fast".to_string(), "beautiful".to_string(), "secure".to_string(), "modern".to_string()] }
                        span { "apps" }
                    }
                },
                code: "WordRotate { words: vec![\"fast\".to_string(), \"beautiful\".to_string(), \"secure\".to_string()] }".to_string(),
            }

            DemoBox {
                title: "速度与颜色".to_string(),
                description: Some("duration 控制切换间隔，color 自定义颜色。".to_string()),
                demo: rsx! {
                    div { style: "font-size:26px; font-weight:700; color:var(--ctrl-text); display:flex; gap:10px; align-items:baseline; justify-content:center; padding:24px;",
                        span { "为" }
                        WordRotate {
                            words: vec!["开发者".to_string(), "设计师".to_string(), "创作者".to_string()],
                            duration: 1200,
                            color: "#ec4899".to_string(),
                        }
                        span { "而生" }
                    }
                },
                code: "WordRotate { words: vec![...], duration: 1200, color: \"#ec4899\".to_string() }".to_string(),
            }

            h2 { "WordRotate Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("words", "Vec<String>", "—", "循环轮播的单词列表"),
                ("duration", "u32", "2000", "每词显示时长（毫秒）"),
                ("color", "String", "主题主色", "文字颜色"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}
