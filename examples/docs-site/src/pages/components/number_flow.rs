use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn NumberFlowPage() -> Element {
    rsx! {
div { id: "number-flow", style: "margin-top: 64px;",
            h1 {
                "NumberFlow 数字动画"
            }
            p {
                "数值展示组件，每个数字位通过 0-9 纵向滚动实现变化动画，适合统计数据、金额等动态数值。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("常见的数值展示，带统计标签。".to_string()),
                demo: rsx! {
                    Row { gutter: 16,
                        Col { span: 8,
                            div { style: "text-align:center;",
                                NumberFlow { value: "12,580".to_string(), style: "font-size:32px; color:var(--ctrl-primary);".to_string() }
                                div { style: "margin-top:8px; font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary);", "用户总数" }
                            }
                        }
                        Col { span: 8,
                            div { style: "text-align:center;",
                                NumberFlow { value: "98.6".to_string(), suffix: "%".to_string(), style: "font-size:32px; color:var(--ctrl-success);".to_string() }
                                div { style: "margin-top:8px; font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary);", "成功率" }
                            }
                        }
                        Col { span: 8,
                            div { style: "text-align:center;",
                                NumberFlow { value: "52,390".to_string(), prefix: "¥".to_string(), style: "font-size:32px; color:var(--ctrl-warning);".to_string() }
                                div { style: "margin-top:8px; font-size:var(--ctrl-font-size-sm); color:var(--ctrl-text-secondary);", "总收入" }
                            }
                        }
                    }
                },
                code: "NumberFlow { value: \"12,580\".to_string() }\nNumberFlow { value: \"98.6\".to_string(), suffix: \"%\".to_string() }\nNumberFlow { value: \"52,390\".to_string(), prefix: \"¥\".to_string() }".to_string(),
            }

            DemoBox {
                title: "动态数值".to_string(),
                description: Some("value 变化时数字位自动滚动过渡，点击按钮体验动画。".to_string()),
                demo: rsx! {
                    NumberFlowDemo {}
                },
                code: "let mut n = use_signal(|| 1200);\nNumberFlow { value: n().to_string() }\nButton { onclick: move |_| n.set(random()), \"随机更新\" }".to_string(),
            }

            DemoBox {
                title: "前缀后缀".to_string(),
                description: Some("通过 prefix / suffix 添加货币符号、单位等辅助信息。".to_string()),
                demo: rsx! {
                    Space { gap: "xl".to_string(),
                        NumberFlow { value: "45,200".to_string(), prefix: "$".to_string(), suffix: "USD".to_string(), style: "font-size:36px;".to_string() }
                        NumberFlow { value: "99.99".to_string(), suffix: "%".to_string(), style: "font-size:36px;".to_string() }
                    }
                },
                code: "NumberFlow { value: \"45,200\".to_string(), prefix: \"$\".to_string(), suffix: \"USD\".to_string() }".to_string(),
            }

            h2 { "NumberFlow Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("value", "String", "—", "显示的数值（支持千分位、小数点）"),
                ("prefix", "String", "\"\"", "前缀（如 $、¥）"),
                ("suffix", "String", "\"\"", "后缀（如 %、K）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn NumberFlowDemo() -> Element {
    let mut value = use_signal(|| 1200u32);
    rsx! {
        div { style: "display:flex; align-items:center; gap:24px;",
            NumberFlow { value: "{value}", style: "font-size:40px; color:var(--ctrl-primary);".to_string() }
            Button {
                variant: Variant::Primary,
                onclick: move |_| {
                    let next = (value() * 7 + 13) % 90000 + 100;
                    value.set(next);
                },
                "随机更新"
            }
        }
    }
}
