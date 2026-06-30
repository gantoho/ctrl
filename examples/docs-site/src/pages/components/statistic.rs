use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn StatisticPage() -> Element {
    rsx! {
div { id: "statistic", style: "margin-top: 64px;",
            h1 {
                "Statistic 统计数值"
            }
            p {
                "用于突出展示统计数据。支持标题、数值、前缀/后缀、趋势箭头和精度控制。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("展示标题和数值。".to_string()),
                demo: rsx! {
                    Statistic {
                        title: "总销售额".to_string(),
                        value: "128,000".to_string(),
                    }
                },
                code: r#"Statistic {
    title: "总销售额".to_string(),
    value: "128,000".to_string(),
}"#.to_string(),
            }

            DemoBox {
                title: "趋势箭头".to_string(),
                description: Some("通过 trend 显示上升/下降箭头，颜色自动适配。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(),
                        Statistic {
                            title: "日活跃用户".to_string(),
                            value: "12,580".to_string(),
                            trend: StatisticTrend::Up,
                        }
                        Statistic {
                            title: "跳出率".to_string(),
                            value: "32.5%".to_string(),
                            trend: StatisticTrend::Down,
                        }
                        Statistic {
                            title: "转化率".to_string(),
                            value: "8.2%".to_string(),
                        }
                    }
                },
                code: r#"Statistic {
    title: "日活跃用户".to_string(),
    value: "12,580".to_string(),
    trend: StatisticTrend::Up,
}
Statistic {
    title: "跳出率".to_string(),
    value: "32.5%".to_string(),
    trend: StatisticTrend::Down,
}"#.to_string(),
            }

            DemoBox {
                title: "前缀与后缀".to_string(),
                description: Some("通过 prefix / suffix 在数值前后添加自定义元素。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(),
                        Statistic {
                            title: "总销售额".to_string(),
                            value: "568,920".to_string(),
                            prefix: rsx! { span { "¥" } },
                            trend: StatisticTrend::Up,
                        }
                        Statistic {
                            title: "用户满意度".to_string(),
                            value: "96.8".to_string(),
                            suffix: rsx! { span { "%" } },
                        }
                    }
                },
                code: r#"Statistic {
    title: "总销售额".to_string(),
    value: "568,920".to_string(),
    prefix: rsx! { span { "¥" } },
    trend: StatisticTrend::Up,
}
Statistic {
    title: "用户满意度".to_string(),
    value: "96.8".to_string(),
    suffix: rsx! { span { "%" } },
}"#.to_string(),
            }

            DemoBox {
                title: "居中布局".to_string(),
                description: Some("设置 center = true 将内容居中对齐。".to_string()),
                demo: rsx! {
                    div { style: "padding: 24px; border: 1px solid var(--ctrl-border, #d9d9d9); border-radius: var(--ctrl-radius-md, 6px);",
                        Statistic {
                            center: true,
                            title: "总交易量".to_string(),
                            value: "1,234,567".to_string(),
                            suffix: rsx! { span { "笔" } },
                            trend: StatisticTrend::Up,
                        }
                    }
                },
                code: r#"Statistic {
    center: true,
    title: "总交易量".to_string(),
    value: "1,234,567".to_string(),
    suffix: rsx! { span { "笔" } },
    trend: StatisticTrend::Up,
}"#.to_string(),
            }

            DemoBox {
                title: "精度控制".to_string(),
                description: Some("设置 precision 控制小数位数（仅当 value 可解析为 f64 时生效）。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(),
                        Statistic {
                            title: "平均分".to_string(),
                            value: "98.56789".to_string(),
                            precision: 2,
                        }
                        Statistic {
                            title: "增长率".to_string(),
                            value: "12.3456".to_string(),
                            precision: 1,
                            suffix: rsx! { span { "%" } },
                            trend: StatisticTrend::Up,
                        }
                    }
                },
                code: r#"Statistic {
    title: "平均分".to_string(),
    value: "98.56789".to_string(),
    precision: 2,   // 输出 "98.57"
}
Statistic {
    title: "增长率".to_string(),
    value: "12.3456".to_string(),
    precision: 1,   // 输出 "12.3"
    suffix: rsx! { span { "%" } },
    trend: StatisticTrend::Up,
}"#.to_string(),
            }

            h2 { "Statistic Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "标题"),
                ("value", "String", "\"\"", "数值（字符串格式，如 \"128\"、\"88.5%\"）"),
                ("precision", "usize", "0", "小数位数（>0 时自动格式化，仅 value 为纯数字时生效）"),
                ("trend", "StatisticTrend", "None", "趋势方向：Up / Down / None"),
                ("center", "bool", "false", "是否居中对齐"),
                ("prefix", "Option<Element>", "None", "数值前缀"),
                ("suffix", "Option<Element>", "None", "数值后缀"),
                ("value_style", "String", "\"\"", "数值自定义内联样式"),
                ("class", "String", "\"\"", "自定义 CSS 类名"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}

            h2 { "StatisticTrend 枚举" }
            PropsTable { headers: vec!["值".to_string(), "显示".to_string(), "颜色".to_string(), "".to_string()], rows: vec![
                ("None (默认)", "不显示", "—", ""),
                ("Up", "↑ 上升箭头", "var(--ctrl-success) 绿色", ""),
                ("Down", "↓ 下降箭头", "var(--ctrl-danger) 红色", ""),
            ]}
        }
    }
}
