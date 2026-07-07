use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn KbdPage() -> Element {
    rsx! {
div { id: "kbd", style: "margin-top: 64px;",
            h1 {
                "Kbd 键盘按键"
            }
            p {
                "用于展示键盘快捷键或按键组合。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("单个按键展示。".to_string()),
                demo: rsx! {
                    Space { gap: "xs".to_string(),
                        Kbd { "Ctrl" }
                        Kbd { "Shift" }
                        Kbd { "Enter" }
                        Kbd { "Esc" }
                        Kbd { "Tab" }
                    }
                },
                code: "Kbd { \"Ctrl\" }\nKbd { \"Shift\" }\nKbd { \"Enter\" }".to_string(),
            }

            DemoBox {
                title: "组合按键".to_string(),
                description: Some("多个按键组合展示快捷键。".to_string()),
                demo: rsx! {
                    Space { gap: "md".to_string(), direction: Direction::Vertical,
                        div { style: "display:flex; align-items:center; gap:6px;",
                            Kbd { "Ctrl" }
                            span { style: "color:var(--ctrl-text-secondary);", "+" }
                            Kbd { "C" }
                            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm); margin-left:8px;", "复制" }
                        }
                        div { style: "display:flex; align-items:center; gap:6px;",
                            Kbd { "Ctrl" }
                            span { style: "color:var(--ctrl-text-secondary);", "+" }
                            Kbd { "V" }
                            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm); margin-left:8px;", "粘贴" }
                        }
                        div { style: "display:flex; align-items:center; gap:6px;",
                            Kbd { "Ctrl" }
                            span { style: "color:var(--ctrl-text-secondary);", "+" }
                            Kbd { "Shift" }
                            span { style: "color:var(--ctrl-text-secondary);", "+" }
                            Kbd { "N" }
                            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm); margin-left:8px;", "新建窗口" }
                        }
                    }
                },
                code: "Kbd { \"Ctrl\" }\nspan { \"+\" }\nKbd { \"C\" }".to_string(),
            }

            DemoBox {
                title: "不同尺寸".to_string(),
                description: Some("通过 size 属性控制按键大小。".to_string()),
                demo: rsx! {
                    Space { gap: "xs".to_string(),
                        Kbd { size: Size::Sm, "Sm" }
                        Kbd { size: Size::Md, "Md" }
                        Kbd { size: Size::Lg, "Lg" }
                    }
                },
                code: "Kbd { size: Size::Sm, \"Sm\" }\nKbd { size: Size::Md, \"Md\" }\nKbd { size: Size::Lg, \"Lg\" }".to_string(),
            }

            DemoBox {
                title: "在文本中使用".to_string(),
                description: Some("在段落中展示快捷键，与文字自然对齐。".to_string()),
                demo: rsx! {
                    p { style: "color:var(--ctrl-text); line-height:2.2;",
                        "按下 "
                        Kbd { "Ctrl" }
                        " + "
                        Kbd { "K" }
                        " 打开命令面板，按 "
                        Kbd { "Ctrl" }
                        " + "
                        Kbd { "S" }
                        " 保存当前文件。"
                    }
                },
                code: "p {\n    \"按下 \"\n    Kbd { \"Ctrl\" }\n    \" + \"\n    Kbd { \"K\" }\n    \" 打开命令面板\"\n}".to_string(),
            }

            h2 { "Kbd Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("size", "Size", "Md", "尺寸：Sm / Md / Lg"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "按键文本"),
            ]}
        }
    }
}
