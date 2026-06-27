use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn CarouselPage() -> Element {
    rsx! {
div { id: "carousel", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Carousel 走马灯" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "走马灯用于在有限空间内循环展示内容。支持自动播放、箭头导航和指示器。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("使用 Carousel 包裹子元素作为轮播项。".to_string()),
                demo: rsx! {
                    Carousel { height: "200px".to_string(),
                        div { style: "background: #4a90d9; color: #fff; display: flex; align-items: center; justify-content: center; font-size: 24px; width: 100%; height: 100%;", "Slide 1" }
                        div { style: "background: #27ae60; color: #fff; display: flex; align-items: center; justify-content: center; font-size: 24px; width: 100%; height: 100%;", "Slide 2" }
                        div { style: "background: #e74c3c; color: #fff; display: flex; align-items: center; justify-content: center; font-size: 24px; width: 100%; height: 100%;", "Slide 3" }
                    }
                },
                code: "Carousel { height: \"200px\".to_string(),\n    div { \"Slide 1\" }\n    div { \"Slide 2\" }\n    div { \"Slide 3\" }\n}".to_string(),
            }

            DemoBox {
                title: "不显示箭头和指示器".to_string(),
                description: Some("设置 arrows 和 dots 为 false。".to_string()),
                demo: rsx! {
                    Carousel { height: "200px".to_string(), arrows: false, dots: false,
                        div { style: "background: #4a90d9; color: #fff; display: flex; align-items: center; justify-content: center; font-size: 24px; width: 100%; height: 100%;", "Slide 1" }
                        div { style: "background: #27ae60; color: #fff; display: flex; align-items: center; justify-content: center; font-size: 24px; width: 100%; height: 100%;", "Slide 2" }
                    }
                },
                code: "Carousel { height: \"200px\".to_string(), arrows: false, dots: false, ... }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Carousel Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("autoplay", "bool", "true", "是否自动播放"),
                ("interval", "u64", "3000", "自动播放间隔（毫秒）"),
                ("arrows", "bool", "true", "是否显示箭头"),
                ("dots", "bool", "true", "是否显示指示器"),
                ("effect", "String", "\"slide\"", "过渡效果：slide / fade"),
                ("height", "String", "\"300px\"", "容器高度"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "轮播项"),
            ]}
        }
    }
}
