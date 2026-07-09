use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn LensPage() -> Element {
    let img = "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=highly%20detailed%20mechanical%20watch%20macro%20photography%2C%20intricate%20gears%2C%20luxury%20product%20shot&image_size=landscape_16_9";

    rsx! {
div { id: "lens", style: "margin-top: 64px;",
            h1 { "Lens 放大镜" }
            p {
                "鼠标悬停在图片上时，浮动圆形镜片显示局部高倍放大的画面，光标跟随移动。适合产品细节查看、电商图片放大等场景。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("将图片地址传入 src，鼠标在图上移动即可放大局部。".to_string()),
                demo: rsx! {
                    div { style: "max-width:560px; margin:0 auto;",
                        Lens {
                            src: img.to_string(),
                            alt: "机械表细节".to_string(),
                            height: "320px".to_string(),
                        }
                    }
                },
                code: "Lens {\n    src: img_url,\n    height: \"320px\".to_string(),\n}".to_string(),
            }

            DemoBox {
                title: "倍率与镜片大小".to_string(),
                description: Some("zoom 调整放大倍率，size 调整镜片直径。".to_string()),
                demo: rsx! {
                    div { style: "max-width:560px; margin:0 auto;",
                        Lens {
                            src: img.to_string(),
                            zoom: 4.0,
                            size: 120.0,
                            height: "280px".to_string(),
                        }
                    }
                },
                code: "Lens { src: img_url, zoom: 4.0, size: 120.0, height: \"280px\".to_string() }".to_string(),
            }

            h2 { "Lens Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("src", "String", "—", "被放大的图片地址"),
                ("alt", "String", "\"\"", "图片替代文本"),
                ("zoom", "f64", "2.5", "放大倍率"),
                ("size", "f64", "160.0", "镜片直径 px"),
                ("height", "String", "\"320px\"", "容器高度"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}
