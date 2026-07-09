use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn ColorSwatchPage() -> Element {
    let blue_palette = vec![
        ColorSample::new("#eff6ff", "50"),
        ColorSample::new("#bfdbfe", "100"),
        ColorSample::new("#93c5fd", "200"),
        ColorSample::new("#60a5fa", "300"),
        ColorSample::new("#3b82f6", "400"),
        ColorSample::new("#2563eb", "500").extra("Primary"),
        ColorSample::new("#1d4ed8", "600"),
        ColorSample::new("#1e40af", "700"),
    ];

    let accent = vec![
        ColorSample::new("#ef4444", "Red"),
        ColorSample::new("#22c55e", "Green"),
        ColorSample::new("#eab308", "Yellow"),
        ColorSample::new("#a855f7", "Violet"),
    ];

    rsx! {
div { id: "color-swatch", style: "margin-top: 64px;",
            h1 { "ColorSwatch 色卡" }
            p {
                "颜色样本网格，点击任意色块复制其 CSS 颜色值到剪贴板。常用于设计系统展示或主题色概览。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("传入 colors 列表，点击色块复制 hex 值。".to_string()),
                demo: rsx! {
                    ColorSwatch { colors: blue_palette.clone() }
                },
                code: "ColorSwatch {\n    colors: vec![\n        ColorSample::new(\"#3b82f6\", \"Blue-500\").extra(\"Primary\"),\n        ...\n    ],\n}".to_string(),
            }

            DemoBox {
                title: "隐藏色值".to_string(),
                description: Some("show_value: false 时仅显示色块。".to_string()),
                demo: rsx! {
                    ColorSwatch { colors: accent.clone(), show_value: false, columns: 4 }
                },
                code: "ColorSwatch { colors: accent, show_value: false, columns: 4 }".to_string(),
            }

            h2 { "ColorSwatch Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("colors", "Vec<ColorSample>", "—", "颜色样本列表"),
                ("show_value", "bool", "true", "是否显示 hex 文本"),
                ("columns", "u32", "4", "每列数量"),
                ("swatch_size", "String", "\"64px\"", "色块宽度"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}

            h2 { "ColorSample" }
            PropsTable { headers: vec!["方法 / 字段".to_string(), "类型".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("ColorSample::new(color, label)", "fn", "创建颜色样本", ""),
                (".extra(text)", "fn", "附加说明文本", ""),
                ("color", "String", "CSS 颜色值", ""),
                ("label", "String", "标签", ""),
            ]}
        }
    }
}
