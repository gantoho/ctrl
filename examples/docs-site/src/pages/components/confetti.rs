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

            h2 { "Confetti Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("trigger", "u32", "0", "触发器：每次变化喷射一轮（0 时不喷射）"),
                ("count", "usize", "60", "彩带粒子数量"),
                ("colors", "Vec<String>", "默认多彩组合", "粒子颜色列表"),
                ("duration", "u32", "2500", "单轮动画时长（毫秒）"),
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
