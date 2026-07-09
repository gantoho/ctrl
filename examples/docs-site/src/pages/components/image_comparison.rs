use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn ImageComparisonPage() -> Element {
    let before = "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=pencil%20sketch%20line%20art%20of%20a%20mountain%20lake%20landscape%2C%20monochrome%2C%20unfinished%20drawing&image_size=landscape_16_9";
    let after = "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=vivid%20colorful%20photograph%20of%20a%20mountain%20lake%20landscape%2C%20golden%20hour%2C%20highly%20detailed%2C%20realistic&image_size=landscape_16_9";

    rsx! {
div { id: "image-comparison", style: "margin-top: 64px;",
            h1 {
                "ImageComparison 图片对比"
            }
            p {
                "通过可拖拽的分隔滑块对比前后两张图片，在容器内按下并拖动即可改变分隔位置。常用于修图前后、设计稿对比、AI 上色等场景。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("按住容器左右拖动滑块，对比 before / after 两张图。".to_string()),
                demo: rsx! {
                    div { style: "max-width:560px; margin:0 auto;",
                        ImageComparison {
                            before: before.to_string(),
                            after: after.to_string(),
                            before_alt: "线稿".to_string(),
                            after_alt: "成品".to_string(),
                            height: "320px".to_string(),
                        }
                    }
                },
                code: "ImageComparison {\n    before: before_url,\n    after: after_url,\n    height: \"320px\".to_string(),\n}".to_string(),
            }

            DemoBox {
                title: "初始位置".to_string(),
                description: Some("initial 设置初始分隔位置（0~100）。".to_string()),
                demo: rsx! {
                    div { style: "max-width:560px; margin:0 auto;",
                        ImageComparison {
                            before: before.to_string(),
                            after: after.to_string(),
                            initial: 25.0,
                            height: "260px".to_string(),
                        }
                    }
                },
                code: "ImageComparison { before, after, initial: 25.0, height: \"260px\".to_string() }".to_string(),
            }

            h2 { "ImageComparison Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("before", "String", "—", "前景（左侧）图片地址"),
                ("after", "String", "—", "背景（右侧）图片地址"),
                ("before_alt", "String", "\"before\"", "before 替代文本"),
                ("after_alt", "String", "\"after\"", "after 替代文本"),
                ("initial", "f64", "50.0", "初始分隔位置（0~100）"),
                ("height", "String", "\"320px\"", "容器高度"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}
