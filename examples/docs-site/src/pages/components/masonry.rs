use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

fn sample_items(cols: u32) -> Element {
    let colors = [
        ("#3b82f6", "#1d4ed8"),
        ("#10b981", "#047857"),
        ("#f59e0b", "#b45309"),
        ("#ef4444", "#991b1b"),
        ("#8b5cf6", "#5b21b6"),
        ("#ec4899", "#9d174d"),
        ("#06b6d4", "#0e7490"),
        ("#f97316", "#9a3412"),
        ("#84cc16", "#4d7c0f"),
        ("#14b8a6", "#0f766e"),
    ];

    let heights = [120, 180, 140, 200, 100, 220, 160, 130, 190, 150];

    rsx! {
        Masonry { cols: cols, gap: 12.0,
            for (i, (color, dark)) in colors.iter().enumerate() {
                {
                    let idx = i + 1;
                    let h = heights[i];
                    rsx! {
                        div {
                            key: "{i}",
                            style: "background:linear-gradient(135deg, {color}, {dark}); border-radius:var(--ctrl-radius-lg); padding:16px; color:#fff; min-height:{h}px;",
                            div { style: "font-weight:700; font-size:18px;", "#{idx}" }
                            div { style: "font-size:var(--ctrl-font-size-sm); opacity:0.85; margin-top:4px;", {format!("高度 {}px", h)} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn MasonryPage() -> Element {
    rsx! {
div { id: "masonry", style: "margin-top: 64px;",
            h1 { "Masonry 瀑布流" }
            p {
                "基于 CSS columns 实现的瀑布流布局，子元素按列自上而下排列，各列高度自动平衡。适合图片墙、卡片列表等不定高内容的展示场景。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("默认 3 列瀑布流，10 个不同高度的卡片自动排列。".to_string()),
                demo: rsx! { {sample_items(3)} },
                code: "Masonry { cols: 3, gap: 12.0,\n    // 子元素自动瀑布流布局\n    div { key: \"1\", ... }\n    div { key: \"2\", ... }\n    ...\n}".to_string(),
            }

            DemoBox {
                title: "4 列布局".to_string(),
                description: Some("设置 cols=4 切换为 4 列，gap 控制间距。".to_string()),
                demo: rsx! { {sample_items(4)} },
                code: "Masonry { cols: 4, gap: 16.0, ... }".to_string(),
            }

            DemoBox {
                title: "2 列布局".to_string(),
                description: Some("少量列更适合移动端或侧边栏场景。".to_string()),
                demo: rsx! { {sample_items(2)} },
                code: "Masonry { cols: 2, gap: 8.0, ... }".to_string(),
            }

            h2 { "MasonryProps" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("cols", "u32", "3", "列数"),
                ("gap", "f64", "16.0", "列间距与行间距（px）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "瀑布流子项内容"),
            ]}
        }
    }
}
