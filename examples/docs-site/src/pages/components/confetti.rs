use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn ConfettiPage() -> Element {
    rsx! {
div { id: "confetti", style: "margin-top: 64px;",
            h1 {
                "Confetti 彩带庆祝"
            }
            p {
                "每当 trigger 变化时，从容器中心喷射一轮彩带粒子，常用于支付成功、任务完成、订阅升级等庆祝场景。需放在 position:relative 的容器内。支持 prefers-reduced-motion 下停用。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("点击按钮，让计数自增以触发彩带。".to_string()),
                demo: rsx! {
                    ConfettiBasicDemo {}
                },
                code: "let mut trigger = use_signal(|| 0u32);\n\ndiv { style: \"position:relative;\",\n    Confetti { trigger: trigger() }\n    Button { onclick: move |_| trigger += 1, \"庆祝一下\" }\n}".to_string(),
            }

            DemoBox {
                title: "自定义粒子".to_string(),
                description: Some("通过 count 控制数量，colors 自定义配色。".to_string()),
                demo: rsx! {
                    ConfettiCustomDemo {}
                },
                code: "Confetti {\n    trigger: trigger(),\n    count: 100,\n    colors: vec![\"#6366f1\".into(), \"#a855f7\".into()],\n}".to_string(),
            }

            DemoBox {
                title: "自定义形状".to_string(),
                description: Some("通过 shapes 配置粒子形状。可选 Rect（矩形）、Circle（圆形）、Square（正方形）、Strip（长条），支持混搭。".to_string()),
                demo: rsx! {
                    ConfettiShapeDemo {}
                },
                code: "Confetti {\n    trigger: trigger(),\n    shapes: vec![\n        ConfettiShape::Circle,\n        ConfettiShape::Strip,\n    ],\n}".to_string(),
            }

            DemoBox {
                title: "蓄力模式".to_string(),
                description: Some("启用 charge 后，长按容器可蓄力。蓄力越久彩带数量越多、爆开范围越大（有上限）。松开时喷射。".to_string()),
                demo: rsx! {
                    ConfettiChargeDemo {}
                },
                code: "Confetti {\n    charge: true,\n    charge_min: 0.3,\n    charge_max: 3.0,\n    charge_duration: 2000,\n}".to_string(),
            }

            h2 { "Confetti Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("trigger", "u32", "0", "触发器：每次变化喷射一轮（0 时不喷射）"),
                ("count", "usize", "60", "彩带粒子数量"),
                ("colors", "Vec<String>", "默认多彩组合", "粒子颜色列表"),
                ("shapes", "Vec<ConfettiShape>", "默认混合 Rect + Circle", "粒子形状列表：Rect / Circle / Square / Strip"),
                ("duration", "u32", "2500", "单轮动画时长（毫秒）"),
                ("charge", "bool", "false", "是否启用蓄力模式"),
                ("charge_min", "f64", "0.3", "蓄力最小倍率（瞬时点击强度）"),
                ("charge_max", "f64", "3.0", "蓄力最大倍率（蓄满时强度上限）"),
                ("charge_duration", "u32", "2000", "蓄满所需时长（毫秒）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn ConfettiBasicDemo() -> Element {
    let mut trigger = use_signal(|| 0u32);

    rsx! {
        div { style: "position:relative; height:200px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); background:var(--ctrl-bg-secondary); display:flex; align-items:center; justify-content:center;",
            Confetti { trigger: trigger() }
            Button {
                variant: Variant::Primary,
                onclick: move |_| trigger += 1,
                "🎉 庆祝一下"
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn ConfettiCustomDemo() -> Element {
    let mut trigger = use_signal(|| 0u32);

    rsx! {
        div { style: "position:relative; height:200px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); background:var(--ctrl-bg-secondary); display:flex; align-items:center; justify-content:center;",
            Confetti {
                trigger: trigger(),
                count: 100,
                colors: vec!["#6366f1".to_string(), "#a855f7".to_string(), "#06b6d4".to_string()],
            }
            Button {
                variant: Variant::Secondary,
                onclick: move |_| trigger += 1,
                "撒 100 片彩带"
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn ConfettiShapeDemo() -> Element {
    let mut trigger = use_signal(|| 0u32);

    rsx! {
        div { style: "display:flex; gap:12px;",
            div { style: "position:relative; flex:1; height:180px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); background:var(--ctrl-bg-secondary); display:flex; align-items:center; justify-content:center;",
                Confetti {
                    trigger: trigger(),
                    count: 50,
                    shapes: vec![ConfettiShape::Circle],
                    colors: vec!["#f59e0b".to_string(), "#ef4444".to_string()],
                }
                Button {
                    size: Size::Sm,
                    variant: Variant::Secondary,
                    onclick: move |_| trigger += 1,
                    "圆形"
                }
            }
            div { style: "position:relative; flex:1; height:180px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); background:var(--ctrl-bg-secondary); display:flex; align-items:center; justify-content:center;",
                Confetti {
                    trigger: trigger(),
                    count: 50,
                    shapes: vec![ConfettiShape::Strip],
                    colors: vec!["#6366f1".to_string(), "#a855f7".to_string()],
                }
                Button {
                    size: Size::Sm,
                    variant: Variant::Secondary,
                    onclick: move |_| trigger += 1,
                    "长条"
                }
            }
            div { style: "position:relative; flex:1; height:180px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); background:var(--ctrl-bg-secondary); display:flex; align-items:center; justify-content:center;",
                Confetti {
                    trigger: trigger(),
                    count: 50,
                    shapes: vec![ConfettiShape::Square],
                    colors: vec!["#10b981".to_string(), "#06b6d4".to_string()],
                }
                Button {
                    size: Size::Sm,
                    variant: Variant::Secondary,
                    onclick: move |_| trigger += 1,
                    "正方形"
                }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn ConfettiChargeDemo() -> Element {
    rsx! {
        div { style: "position:relative; height:220px; border-radius:var(--ctrl-radius-lg); border:1px solid var(--ctrl-border); background:var(--ctrl-bg-secondary); display:flex; align-items:center; justify-content:center; user-select:none;",
            Confetti {
                charge: true,
                charge_min: 0.3,
                charge_max: 3.0,
                charge_duration: 2000,
                count: 80,
            }
            div { style: "text-align:center; pointer-events:none;",
                div { style: "font-size:32px;", "🎆" }
                div { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm); margin-top:4px;", "长按蓄力 · 松手绽放" }
            }
        }
    }
}
