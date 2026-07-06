use dioxus::prelude::*;
use ctrl::prelude::*;
use ctrl::locale::{Lang, LocaleKey};
use ctrl::types::Size;
use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

/// 演示 use_config 的自定义组件
#[component]
fn LocaleDemo() -> Element {
    let config = use_config();
    rsx! {
        div { style: "display: flex; gap: 12px; align-items: center; flex-wrap: wrap; padding: 8px 0;",
            div {
                style: "padding: 8px 16px; background: var(--ctrl-primary); color: #fff; border-radius: var(--ctrl-radius-sm); font-size: var(--ctrl-font-size-sm);",
                "{config.locale(LocaleKey::Confirm)}"
            }
            div {
                style: "padding: 8px 16px; background: var(--ctrl-bg); color: var(--ctrl-text); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-sm); font-size: var(--ctrl-font-size-sm);",
                "{config.locale(LocaleKey::Cancel)}"
            }
            div {
                style: "padding: 8px 16px; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);",
                "{config.locale(LocaleKey::NoData)}"
            }
            div {
                style: "color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-xs);",
                "当前语言: {config.lang} | 尺寸: {config.size:?} | RTL: {config.rtl}"
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn ConfigProviderPage() -> Element {
    rsx! {
        h1 { "ConfigProvider 全局配置" }
        p { "ConfigProvider 为所有子组件提供全局配置上下文，包括语言国际化（lang）、统一组件尺寸（size）、和从右到左布局（rtl）。子组件通过 use_config() 或 use_locale() 获取这些配置。" }

        h2 { "基本用法" }

        DemoBox {
            title: "在应用根层级使用".to_string(),
            description: Some(
                "在应用最外层包裹 ConfigProvider，所有内部组件自动继承配置。ConfigProvider 可以嵌套，内层覆盖外层配置。".to_string()
            ),
            demo: rsx! {
                div { style: "padding: 20px; border: 1px dashed var(--ctrl-border); border-radius: var(--ctrl-radius-md); text-align: center;",
                    p { style: "font-weight: 600; margin-bottom: 12px; color: var(--ctrl-primary);",
                        "ConfigProvider 包裹区域"
                    }
                    p { style: "color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm); line-height: 1.8;",
                        "所有子组件通过 use_config() 获取配置"
                    }
                }
            },
            code: "// 入口文件 main.rs\nrsx! {\n    ConfigProvider {\n        lang: Lang::ZhCN,     // 中文\n        size: Size::Md,       // 中等尺寸\n        // 你的应用组件\n    }\n}".to_string(),
        }

        DemoBox {
            title: "中英文切换".to_string(),
            description: Some("通过 lang 属性切换中文/英文环境，内置 Confirm、Cancel、NoData 等 16 个常用文案 key。".to_string()),
            demo: rsx! {
                div { style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "padding: 12px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md);",
                        p { style: "font-weight: 600; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin-bottom: 8px;", "中文 (ZhCN) — 默认" }
                        ConfigProvider { lang: Lang::ZhCN,
                            LocaleDemo {}
                        }
                    }
                    div {
                        style: "padding: 12px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md);",
                        p { style: "font-weight: 600; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin-bottom: 8px;", "英文 (EnUS)" }
                        ConfigProvider { lang: Lang::EnUS,
                            LocaleDemo {}
                        }
                    }
                }
            },
            code: "// 中文（默认）\nConfigProvider { lang: Lang::ZhCN,\n    // \"确定\" \"取消\" \"暂无数据\"\n}\n\n// 英文\nConfigProvider { lang: Lang::EnUS,\n    // \"Confirm\" \"Cancel\" \"No data\"\n}\n\n// 在子组件中获取文案\nlet config = use_config();\nlet btn_text = config.locale(LocaleKey::Confirm);".to_string(),
        }

        DemoBox {
            title: "全局尺寸".to_string(),
            description: Some(
                "通过 size 属性统一设置所有支持 size 属性的组件的默认尺寸（Sm=小, Md=中, Lg=大）。各组件也可以通过自己的 size 属性单独覆盖。".to_string()
            ),
            demo: rsx! {
                div { style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "padding: 12px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md);",
                        p { style: "font-weight: 600; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin-bottom: 8px;", "Sm 小尺寸" }
                        div { style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",
                            Button { size: Size::Sm, "小按钮" }
                            Tag { "小标签" }
                            Select { size: Size::Sm, placeholder: "小选择器".to_string(), options: vec![] }
                        }
                    }
                    div {
                        style: "padding: 12px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md);",
                        p { style: "font-weight: 600; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin-bottom: 8px;", "Lg 大尺寸" }
                        div { style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",
                            Button { size: Size::Lg, "大按钮" }
                            Tag { "大标签" }
                            Select { size: Size::Lg, placeholder: "大选择器".to_string(), options: vec![] }
                        }
                    }
                }
            },
            code: "ConfigProvider { size: Size::Sm,\n    Button { \"小按钮\" }\n    Select { placeholder: \"小选择器\".to_string() }\n}\n\n// 各组件可单独覆盖\nConfigProvider { size: Size::Md,\n    Button { size: Size::Lg, \"大按钮\" }  // 单独覆盖为 Lg\n}".to_string(),
        }

        DemoBox {
            title: "RTL 从右到左布局".to_string(),
            description: Some("设置 rtl 为 true 后，容器自动添加 dir=\"rtl\" 属性，适用于阿拉伯语、希伯来语等从右到左阅读的语言。".to_string()),
            demo: rsx! {
                ConfigProvider { rtl: true,
                    div { style: "display: flex; gap: 8px; direction: rtl; padding: 12px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md);",
                        Button { "تأكيد" }
                        Button { variant: Variant::Ghost, "إلغاء" }
                    }
                }
            },
            code: "ConfigProvider { rtl: true,\n    Button { \"تأكيد\" }\n    Button { variant: Some(Variant::Dashed), \"إلغاء\" }\n}".to_string(),
        }

        h2 { "use_config() API" }

        DemoBox {
            title: "获取配置".to_string(),
            description: Some(
                "在任何 ConfigProvider 的子组件中调用 use_config() 获取 GlobalConfig 结构体。如果在 ConfigProvider 外部调用，返回默认配置。".to_string()
            ),
            code: "use ctrl::locale::{Lang, LocaleKey};\nuse ctrl::types::Size;\n\n#[component]\nfn MyComponent() -> Element {\n    let config = use_config();\n    let confirm_text = config.locale(LocaleKey::Confirm);\n\n    rsx! {\n        div {\n            p { \"语言: {config.lang}\" }\n            p { \"尺寸: {config.size:?}\" }\n            Button { \"{confirm_text}\" }\n        }\n    }\n}".to_string(),
            demo: rsx! {
                div { style: "padding: 12px; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);",
                    p { style: "font-weight: 600; margin-bottom: 8px; color: var(--ctrl-text);", "GlobalConfig 结构体" }
                    table { style: "width: 100%; border-collapse: collapse; font-size: var(--ctrl-font-size-sm);",
                        tr { style: "border-bottom: 1px solid var(--ctrl-border);",
                            th { style: "text-align: left; padding: 6px 12px;", "字段" }
                            th { style: "text-align: left; padding: 6px 12px;", "类型" }
                            th { style: "text-align: left; padding: 6px 12px;", "说明" }
                        }
                        tr { style: "border-bottom: 1px solid var(--ctrl-border-light);",
                            td { style: "padding: 6px 12px;", "lang" }
                            td { style: "padding: 6px 12px;", "Lang" }
                            td { style: "padding: 6px 12px;", "当前语言 (ZhCN / EnUS)" }
                        }
                        tr { style: "border-bottom: 1px solid var(--ctrl-border-light);",
                            td { style: "padding: 6px 12px;", "size" }
                            td { style: "padding: 6px 12px;", "Size" }
                            td { style: "padding: 6px 12px;", "全局默认尺寸" }
                        }
                        tr {
                            td { style: "padding: 6px 12px;", "rtl" }
                            td { style: "padding: 6px 12px;", "bool" }
                            td { style: "padding: 6px 12px;", "是否从右到左布局" }
                        }
                    }
                }
            },
        }

        h2 { "配置层级" }
        p { "ConfigProvider 支持嵌套。内层 ConfigProvider 会覆盖外层配置，返回外层后恢复。" }

        DemoBox {
            title: "嵌套配置".to_string(),
            demo: rsx! {
                div { style: "padding: 12px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md);",
                    p { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                        "外层: ZhCN 中文"
                    }
                    ConfigProvider { lang: Lang::EnUS,
                        div { style: "margin-top: 8px; padding: 8px; background: var(--ctrl-primary-light); border-radius: var(--ctrl-radius-sm);",
                            p { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                                "内层: EnUS 英文 (覆盖外层)"
                            }
                            LocaleDemo {}
                        }
                    }
                }
            },
            code: "ConfigProvider { lang: Lang::ZhCN,\n    // 这里 use_config().lang == ZhCN\n    ConfigProvider { lang: Lang::EnUS,\n        // 这里 use_config().lang == EnUS\n        LocaleDemo {}\n    }\n    // 这里恢复 ZhCN\n}".to_string(),
            description: Some("内层 ConfigProvider 可以临时覆盖语言/尺寸/RTL 配置。".to_string()),
        }

        h2 { "ConfigProvider Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("lang", "Lang", "Lang::ZhCN", "语言 — ZhCN 中文 / EnUS 英文"),
            ("size", "Size", "Size::Md", "统一组件尺寸 (Sm / Md / Lg)"),
            ("rtl", "bool", "false", "是否从右到左布局"),
        ]}
    }
}
