use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn BentoGridPage() -> Element {
    rsx! {
div { id: "bento-grid", style: "margin-top: 64px;",
            h1 { "BentoGrid 便当网格" }
            p {
                "大小不一的卡片按网格排布，通过 BentoCell 的 col_span / row_span 控制跨列跨行，常用于产品特性、功能亮点的杂志式展示。窄屏自动退化为单列。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("用 BentoCell 包裹每块内容，col_span / row_span 控制占位。".to_string()),
                demo: rsx! {
                    BentoGrid { columns: 3, row_height: "140px".to_string(),
                        BentoCell { col_span: 2, row_span: 1,
                            div { style: "font-size:20px; font-weight:800; color:var(--ctrl-text);", "⚡ 极速渲染" }
                            div { style: "color:var(--ctrl-text-secondary); margin-top:8px;", "基于 Rust + WASM，冷启动与运行时性能俱佳。" }
                        }
                        BentoCell { col_span: 1, row_span: 2,
                            div { style: "font-size:20px; font-weight:800; color:var(--ctrl-text);", "🎨 主题系统" }
                            div { style: "color:var(--ctrl-text-secondary); margin-top:8px;", "全量 --ctrl-* 变量驱动，暗色/亮色一键切换。" }
                        }
                        BentoCell { col_span: 1, row_span: 1,
                            div { style: "font-size:20px; font-weight:800; color:var(--ctrl-text);", "🧩 组件丰富" }
                        }
                        BentoCell { col_span: 1, row_span: 1,
                            div { style: "font-size:20px; font-weight:800; color:var(--ctrl-text);", "♿ 无障碍" }
                        }
                    }
                },
                code: "BentoGrid { columns: 3,\n    BentoCell { col_span: 2, ... }\n    BentoCell { col_span: 1, row_span: 2, ... }\n}".to_string(),
            }

            h2 { "BentoGrid Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("columns", "u32", "3", "网格列数"),
                ("gap", "String", "\"16px\"", "单元间距"),
                ("row_height", "String", "\"180px\"", "行最小高度"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "BentoCell 单元"),
            ]}

            h2 { "BentoCell Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("col_span", "u32", "1", "跨列数"),
                ("row_span", "u32", "1", "跨行数"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "单元内容"),
            ]}
        }
    }
}
