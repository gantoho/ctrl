use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn GridPage() -> Element {
    let col_demo_bg = "min-height: 36px; display: flex; align-items: center; justify-content: center; background: var(--ctrl-primary-light, #dbeafe); color: var(--ctrl-primary); font-size: var(--ctrl-font-size-sm, 12px); font-weight: 500; border-radius: var(--ctrl-radius-sm, 4px);";
    let col_bg_pink = "min-height: 36px; display: flex; align-items: center; justify-content: center; background: #fce4ec; color: #c62828; font-size: var(--ctrl-font-size-sm, 12px); font-weight: 500; border-radius: var(--ctrl-radius-sm, 4px);";
    let col_bg_green = "min-height: 36px; display: flex; align-items: center; justify-content: center; background: #e8f5e9; color: #2e7d32; font-size: var(--ctrl-font-size-sm, 12px); font-weight: 500; border-radius: var(--ctrl-radius-sm, 4px);";
    let col_bg_orange = "min-height: 36px; display: flex; align-items: center; justify-content: center; background: #fff3e0; color: #e65100; font-size: var(--ctrl-font-size-sm, 12px); font-weight: 500; border-radius: var(--ctrl-radius-sm, 4px);";
    let col_bg_purple = "min-height: 36px; display: flex; align-items: center; justify-content: center; background: #f3e5f5; color: #7b1fa2; font-size: var(--ctrl-font-size-sm, 12px); font-weight: 500; border-radius: var(--ctrl-radius-sm, 4px);";

    let section_title = "font-size: 14px; font-weight: 600; color: var(--ctrl-text-secondary); margin-bottom: 10px;";

    rsx! {
        div { id: "grid", style: "margin-top: 64px;",
            h1 {
                "Grid 栅格布局"
            }
            p {
                "基于 CSS Grid 的 24 列弹性栅格。Row 定义 24 列 1fr 网格，Col 通过 span / offset 控制占列和偏移。支持 5 级响应式断点、2 种沟槽模式。"
            }

            // ═══ 基础栅格 ═══
            DemoBox {
                title: "基础栅格".to_string(),
                description: Some("span 控制列宽（1-24），自动均分空间，不写死任何宽度或百分比。".to_string()),
                demo: rsx! {
                    Row {
                        Col { span: 24, div { style: "{col_demo_bg}", "col-24" } }
                    }
                    div { style: "height: 8px;" }
                    Row {
                        Col { span: 12, div { style: "{col_demo_bg}", "col-12" } }
                        Col { span: 12, div { style: "{col_bg_pink}", "col-12" } }
                    }
                    div { style: "height: 8px;" }
                    Row {
                        Col { span: 8, div { style: "{col_demo_bg}", "col-8" } }
                        Col { span: 8, div { style: "{col_bg_pink}", "col-8" } }
                        Col { span: 8, div { style: "{col_demo_bg}", "col-8" } }
                    }
                    div { style: "height: 8px;" }
                    Row {
                        Col { span: 6, div { style: "{col_demo_bg}", "col-6" } }
                        Col { span: 6, div { style: "{col_bg_pink}", "col-6" } }
                        Col { span: 6, div { style: "{col_demo_bg}", "col-6" } }
                        Col { span: 6, div { style: "{col_bg_pink}", "col-6" } }
                    }
                    div { style: "height: 8px;" }
                    Row {
                        Col { span: 4, div { style: "{col_demo_bg}", "col-4" } }
                        Col { span: 4, div { style: "{col_bg_pink}", "col-4" } }
                        Col { span: 4, div { style: "{col_demo_bg}", "col-4" } }
                        Col { span: 4, div { style: "{col_bg_pink}", "col-4" } }
                        Col { span: 4, div { style: "{col_demo_bg}", "col-4" } }
                        Col { span: 4, div { style: "{col_bg_pink}", "col-4" } }
                    }
                },
                code: r#"// 单列
Row { Col { span: 24, "col-24" } }

// 等分
Row {
    Col { span: 12, "col-12" }
    Col { span: 12, "col-12" }
}

Row {
    Col { span: 8, "col-8" }
    Col { span: 8, "col-8" }
    Col { span: 8, "col-8" }
}"#.to_string(),
            }

            // ═══ 混合栅格 ═══
            DemoBox {
                title: "混合栅格".to_string(),
                description: Some("每个 Col 独立 span，无需凑满 24，多余列自动折行。".to_string()),
                demo: rsx! {
                    Row {
                        Col { span: 16, div { style: "{col_demo_bg}", "col-16" } }
                        Col { span: 8, div { style: "{col_bg_pink}", "col-8" } }
                    }
                    div { style: "height: 8px;" }
                    Row {
                        Col { span: 6, div { style: "{col_demo_bg}", "col-6" } }
                        Col { span: 4, div { style: "{col_bg_pink}", "col-4" } }
                        Col { span: 10, div { style: "{col_bg_green}", "col-10" } }
                        Col { span: 4, div { style: "{col_bg_orange}", "col-4" } }
                    }
                    div { style: "height: 8px;" }
                    Row {
                        Col { span: 3, div { style: "{col_demo_bg}", "3" } }
                        Col { span: 5, div { style: "{col_bg_pink}", "5" } }
                        Col { span: 7, div { style: "{col_bg_green}", "7" } }
                        Col { span: 3, div { style: "{col_bg_orange}", "3" } }
                        Col { span: 6, div { style: "{col_bg_purple}", "6" } }
                    }
                },
                code: r#"Row {
    Col { span: 16, "col-16" }
    Col { span: 8,  "col-8" }
}
Row {
    Col { span: 6,  "col-6" }
    Col { span: 4,  "col-4" }
    Col { span: 10, "col-10" }
    Col { span: 4,  "col-4" }
}"#.to_string(),
            }

            // ═══ Offset 偏移 ═══
            DemoBox {
                title: "Offset 偏移".to_string(),
                description: Some("offset 从第 N+1 列开始放置，不补空白占位 Col。".to_string()),
                demo: rsx! {
                    div { style: "{section_title}", "↳ 中间偏移" }
                    Row {
                        Col { span: 8, div { style: "{col_demo_bg}", "col-8" } }
                        Col { span: 8, offset: 4, div { style: "{col_bg_pink}", "col-8 offset-4" } }
                    }
                    div { style: "height: 12px;" }
                    div { style: "{section_title}", "↳ 两端偏移" }
                    Row {
                        Col { span: 6, offset: 6, div { style: "{col_demo_bg}", "col-6 offset-6" } }
                        Col { span: 6, offset: 6, div { style: "{col_bg_pink}", "col-6 offset-6" } }
                    }
                    div { style: "height: 12px;" }
                    div { style: "{section_title}", "↳ 交错偏移" }
                    Row {
                        Col { span: 6, div { style: "{col_demo_bg}", "col-6" } }
                        Col { span: 4, offset: 2, div { style: "{col_bg_pink}", "col-4 offset-2" } }
                        Col { span: 6, offset: 2, div { style: "{col_bg_green}", "col-6 offset-2" } }
                    }
                },
                code: r#"// 中间偏移
Row {
    Col { span: 8, "col-8" }
    Col { span: 8, offset: 4, "col-8 offset-4" }
}

// 两端偏移
Row {
    Col { span: 6, offset: 6, "col-6 offset-6" }
    Col { span: 6, offset: 6, "col-6 offset-6" }
}

// 交错偏移
Row {
    Col { span: 6, "col-6" }
    Col { span: 4, offset: 2, "col-4 offset-2" }
    Col { span: 6, offset: 2, "col-6 offset-2" }
}"#.to_string(),
            }

            // ═══ 对齐方式 ═══
            DemoBox {
                title: "对齐方式".to_string(),
                description: Some("水平对齐使用 offset；垂直对齐使用 Row 的 align。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "sm".to_string(),
                        div {
                            Row {
                                Col { span: 8, div { style: "{col_demo_bg}", "居左（默认）" } }
                            }
                        }
                        div {
                            Row {
                                Col { span: 8, offset: 8, div { style: "{col_bg_pink}", "居中 offset=8" } }
                            }
                        }
                        div {
                            Row {
                                Col { span: 8, offset: 16, div { style: "{col_bg_green}", "居右 offset=16" } }
                            }
                        }
                        // 垂直对齐
                        div { style: "{section_title}", "↳ 垂直居中对齐" }
                        div {
                            Row { align: RowAlign::Middle,
                                style: "height: 56px; background: var(--ctrl-bg, #f5f5f5); border-radius: var(--ctrl-radius-sm, 4px);",
                                Col { span: 6, div { style: "{col_demo_bg} height: 40px;", "col-6" } }
                                Col { span: 6, div { style: "{col_bg_pink} height: 24px; display: flex; align-items: center; justify-content: center; border-radius: var(--ctrl-radius-sm, 4px);", "col-6" } }
                                Col { span: 6, div { style: "{col_bg_green} height: 48px; display: flex; align-items: center; justify-content: center; border-radius: var(--ctrl-radius-sm, 4px);", "col-6" } }
                            }
                        }
                        div { style: "{section_title}", "↳ 顶部对齐" }
                        div {
                            Row { align: RowAlign::Top,
                                style: "height: 56px; background: var(--ctrl-bg, #f5f5f5); border-radius: var(--ctrl-radius-sm, 4px);",
                                Col { span: 6, div { style: "{col_demo_bg} height: 40px;", "col-6" } }
                                Col { span: 6, div { style: "{col_bg_pink} height: 24px; display: flex; align-items: center; justify-content: center; border-radius: var(--ctrl-radius-sm, 4px);", "col-6" } }
                                Col { span: 6, div { style: "{col_bg_green} height: 48px; display: flex; align-items: center; justify-content: center; border-radius: var(--ctrl-radius-sm, 4px);", "col-6" } }
                            }
                        }
                        div { style: "{section_title}", "↳ 底部对齐" }
                        div {
                            Row { align: RowAlign::Bottom,
                                style: "height: 56px; background: var(--ctrl-bg, #f5f5f5); border-radius: var(--ctrl-radius-sm, 4px);",
                                Col { span: 6, div { style: "{col_demo_bg} height: 40px;", "col-6" } }
                                Col { span: 6, div { style: "{col_bg_pink} height: 24px; display: flex; align-items: center; justify-content: center; border-radius: var(--ctrl-radius-sm, 4px);", "col-6" } }
                                Col { span: 6, div { style: "{col_bg_green} height: 48px; display: flex; align-items: center; justify-content: center; border-radius: var(--ctrl-radius-sm, 4px);", "col-6" } }
                            }
                        }
                    }
                },
                code: r#"// 水平对齐（通过 offset）
Row { Col { span: 8, "居左" } }
Row { Col { span: 8, offset: 8, "居中" } }
Row { Col { span: 8, offset: 16, "居右" } }

// 垂直对齐（通过 align）
Row { align: RowAlign::Middle,
    Col { span: 6, "..." }
    Col { span: 6, "..." }
    Col { span: 6, "..." }
}"#.to_string(),
            }

            // ═══ 沟槽间隔 ═══
            DemoBox {
                title: "Gutter 沟槽间隔".to_string(),
                description: Some("Padding 模式列四周均有间距，Gap 模式仅列间有间距。".to_string()),
                demo: rsx! {
                    div { style: "{section_title}", "↳ Padding 模式（默认）— 列四周均分布" }
                    Row {
                        gutter: 20,
                        Col { span: 8, div { style: "{col_demo_bg}", "col-8" } }
                        Col { span: 8, div { style: "{col_bg_pink}", "col-8" } }
                        Col { span: 8, div { style: "{col_bg_green}", "col-8" } }
                    }
                    div { style: "height: 16px;" }
                    div { style: "{section_title}", "↳ Gap 模式 — 仅列间有间距，行首尾无空白" }
                    Row {
                        gutter: 20,
                        gutter_mode: GutterMode::Gap,
                        Col { span: 8, div { style: "{col_demo_bg}", "col-8" } }
                        Col { span: 8, div { style: "{col_bg_pink}", "col-8" } }
                        Col { span: 8, div { style: "{col_bg_green}", "col-8" } }
                    }
                    div { style: "height: 16px;" }
                    div { style: "{section_title}", "↳ Gap 模式 + 混合 span" }
                    Row {
                        gutter: 16,
                        gutter_mode: GutterMode::Gap,
                        Col { span: 6, div { style: "{col_demo_bg}", "col-6" } }
                        Col { span: 10, div { style: "{col_bg_pink}", "col-10" } }
                        Col { span: 4, offset: 4, div { style: "{col_bg_green}", "col-4 offset-4" } }
                    }
                },
                code: r#"// Padding 模式
Row {
    gutter: 20,
    gutter_mode: GutterMode::Padding, // 默认可省略
    Col { span: 8, "col-8" }
    Col { span: 8, "col-8" }
    Col { span: 8, "col-8" }
}

// Gap 模式
Row {
    gutter: 20,
    gutter_mode: GutterMode::Gap,
    Col { span: 8, "col-8" }
    Col { span: 8, "col-8" }
    Col { span: 8, "col-8" }
}"#.to_string(),
            }

            // ═══ 响应式断点 ═══
            DemoBox {
                title: "响应式断点".to_string(),
                description: Some("xs(<576px) / sm(≥576px) / md(≥768px) / lg(≥992px) / xl(≥1200px)，缩小浏览器查看变化。".to_string()),
                demo: rsx! {
                    Row {
                        Col { xs: 24, sm: 12, md: 8,
                            div { style: "{col_demo_bg}", "xs:24 sm:12 md:8" }
                        }
                        Col { xs: 24, sm: 12, md: 8,
                            div { style: "{col_bg_pink}", "xs:24 sm:12 md:8" }
                        }
                        Col { xs: 24, sm: 24, md: 8,
                            div { style: "{col_bg_green}", "xs:24 sm:24 md:8" }
                        }
                    }
                    div { style: "height: 12px;" }
                    div { style: "{section_title}", "↳ 响应式 + offset 混合" }
                    Row {
                        Col { xs: 24, sm: 12, md: 6, offset: 0,
                            div { style: "{col_demo_bg}", "col-6" }
                        }
                        Col { xs: 0, sm: 0, md: 6, offset: 0,
                            div { style: "{col_bg_pink}", "col-6" }
                        }
                        Col { xs: 0, sm: 0, md: 6, offset: 0,
                            div { style: "{col_bg_green}", "col-6" }
                        }
                        Col { xs: 0, sm: 0, md: 6, offset: 0,
                            div { style: "{col_bg_orange}", "col-6" }
                        }
                    }
                },
                code: r#"Row {
    Col { xs: 24, sm: 12, md: 8, "xs:24 sm:12 md:8" }
    Col { xs: 24, sm: 12, md: 8, "xs:24 sm:12 md:8" }
    Col { xs: 24, sm: 24, md: 8, "xs:24 sm:24 md:8" }
}

// 小屏竖排，大屏横排
Row {
    Col { xs: 24, sm: 12, md: 6, "..." }
    Col { xs: 24, sm: 12, md: 6, "..." }
    Col { xs: 24, sm: 12, md: 6, "..." }
    Col { xs: 24, sm: 12, md: 6, "..." }
}"#.to_string(),
            }

            // ═══ 复杂布局 — 类似页面结构 ═══
            DemoBox {
                title: "复杂布局示例".to_string(),
                description: Some("模拟真实页面结构：Header + Sidebar + Main + Footer。".to_string()),
                demo: rsx! {
                    Space { direction: Direction::Vertical, gap: "xs".to_string(),
                        // Header
                        Row {
                            Col { span: 24,
                                div { style: "min-height: 48px; display: flex; align-items: center; justify-content: center; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: #fff; font-size: 14px; font-weight: 600; border-radius: var(--ctrl-radius-sm, 4px);",
                                    "Header — col-24"
                                }
                            }
                        }
                        // Sidebar + Content
                        Row {
                            Col { span: 6,
                                div { style: "min-height: 120px; display: flex; align-items: center; justify-content: center; background: #e3f2fd; color: #1565c0; font-size: 13px; font-weight: 500; border-radius: var(--ctrl-radius-sm, 4px);",
                                    "Sidebar\ncol-6"
                                }
                            }
                            Col { span: 18,
                                Row {
                                    Col { span: 12,
                                        div { style: "min-height: 56px; display: flex; align-items: center; justify-content: center; background: #e8f5e9; color: #2e7d32; font-size: 13px; font-weight: 500; border-radius: var(--ctrl-radius-sm, 4px);",
                                            "Content col-12"
                                        }
                                    }
                                    Col { span: 12,
                                        div { style: "min-height: 56px; display: flex; align-items: center; justify-content: center; background: #fff3e0; color: #e65100; font-size: 13px; font-weight: 500; border-radius: var(--ctrl-radius-sm, 4px);",
                                            "Content col-12"
                                        }
                                    }
                                }
                                div { style: "height: 8px;" }
                                Row {
                                    Col { span: 8,
                                        div { style: "min-height: 56px; display: flex; align-items: center; justify-content: center; background: #fce4ec; color: #c62828; font-size: 13px; font-weight: 500; border-radius: var(--ctrl-radius-sm, 4px);",
                                            "col-8"
                                        }
                                    }
                                    Col { span: 8,
                                        div { style: "min-height: 56px; display: flex; align-items: center; justify-content: center; background: #f3e5f5; color: #7b1fa2; font-size: 13px; font-weight: 500; border-radius: var(--ctrl-radius-sm, 4px);",
                                            "col-8"
                                        }
                                    }
                                    Col { span: 8,
                                        div { style: "min-height: 56px; display: flex; align-items: center; justify-content: center; background: #e0f7fa; color: #00695c; font-size: 13px; font-weight: 500; border-radius: var(--ctrl-radius-sm, 4px);",
                                            "col-8"
                                        }
                                    }
                                }
                            }
                        }
                        // Footer
                        Row {
                            Col { span: 24,
                                div { style: "min-height: 36px; display: flex; align-items: center; justify-content: center; background: var(--ctrl-bg, #f5f5f5); color: var(--ctrl-text-secondary); font-size: 12px; border-radius: var(--ctrl-radius-sm, 4px);",
                                    "Footer — col-24"
                                }
                            }
                        }
                    }
                },
                code: r#"// Header
Row {
    Col { span: 24, "Header" }
}

// Sidebar + Content
Row {
    Col { span: 6, "Sidebar" }
    Col { span: 18,
        Row {
            Col { span: 12, "Content" }
            Col { span: 12, "Content" }
        }
        Row {
            Col { span: 8, "col-8" }
            Col { span: 8, "col-8" }
            Col { span: 8, "col-8" }
        }
    }
}

// Footer
Row {
    Col { span: 24, "Footer" }
}"#.to_string(),
            }

            // ═══ Row Props ═══
            h2 { "Row Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("align", "RowAlign", "Top", "垂直对齐：Top / Middle / Bottom"),
                ("gutter", "u32", "0", "列间距（px）"),
                ("gutter_mode", "GutterMode", "Padding", "间距模式：Padding（四周有间距）/ Gap（仅列间有间距）"),
                ("class", "String", "\"\"", "自定义 CSS 类名"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "子元素（Col）"),
            ]}

            // ═══ Col Props ═══
            h2 { "Col Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("span", "u8", "24", "列占位（1-24）"),
                ("offset", "u8", "0", "列偏移（0-23），从第 N+1 列起始"),
                ("xs", "Option<u8>", "None", "<576px 断点占位"),
                ("sm", "Option<u8>", "None", "≥576px 断点占位"),
                ("md", "Option<u8>", "None", "≥768px 断点占位"),
                ("lg", "Option<u8>", "None", "≥992px 断点占位"),
                ("xl", "Option<u8>", "None", "≥1200px 断点占位"),
                ("class", "String", "\"\"", "自定义 CSS 类名"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "子元素"),
            ]}

            // ═══ RowAlign ═══
            h2 { "RowAlign 枚举" }
            PropsTable { headers: vec!["值".to_string(), "效果".to_string(), "".to_string(), "".to_string()], rows: vec![
                ("Top (默认)", "顶部对齐", "", ""),
                ("Middle", "居中对齐", "", ""),
                ("Bottom", "底部对齐", "", ""),
            ]}

            // ═══ GutterMode ═══
            h2 { "GutterMode 枚举" }
            PropsTable { headers: vec!["值".to_string(), "效果".to_string(), "".to_string(), "".to_string()], rows: vec![
                ("Padding (默认)", "Row 负 margin + Col 左右 padding，列四周均有间距", "", ""),
                ("Gap", "Row CSS gap 属性，仅在列与列之间产生间距，首尾无空白", "", ""),
            ]}
        }
    }
}
