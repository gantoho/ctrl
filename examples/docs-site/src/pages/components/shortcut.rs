use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn ShortcutPage() -> Element {
    rsx! {
div { id: "shortcut", style: "margin-top: 64px;",
            h1 {
                "Shortcut 快捷键系统"
            }
            p {
                "一套完整的键盘快捷键能力：既可用 use_shortcut 独立注册单个快捷键，也可用 ShortcutProvider + use_register_shortcut 构建集中管理的快捷键系统，并通过 use_shortcut_list 生成快捷键帮助面板。ShortcutKeys 组件用于跨平台展示按键。"
            }

            DemoBox {
                title: "use_shortcut 独立使用".to_string(),
                description: Some("最简单的用法，无需 Provider。按 ⌘K / Ctrl+K 触发计数。".to_string()),
                demo: rsx! {
                    ShortcutBasicDemo {}
                },
                code: "let mut count = use_signal(|| 0);\nuse_shortcut(\"mod+k\", move || count += 1);".to_string(),
            }

            DemoBox {
                title: "ShortcutKeys 展示组件".to_string(),
                description: Some("将组合键渲染为跨平台按键：macOS 显示 ⌘/⌥/⇧，其他平台显示 Ctrl/Alt/Shift。".to_string()),
                demo: rsx! {
                    Space { gap: "lg".to_string(), direction: Direction::Vertical,
                        div { style: "display:flex; align-items:center; gap:12px;",
                            ShortcutKeys { combo: "mod+k".to_string() }
                            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "打开命令面板" }
                        }
                        div { style: "display:flex; align-items:center; gap:12px;",
                            ShortcutKeys { combo: "mod+shift+p".to_string(), separator: true }
                            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "命令菜单（带 + 连接符）" }
                        }
                        div { style: "display:flex; align-items:center; gap:12px;",
                            ShortcutKeys { combo: "ctrl+alt+delete".to_string(), size: Size::Md }
                            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "中号按键" }
                        }
                    }
                },
                code: "ShortcutKeys { combo: \"mod+k\".to_string() }\nShortcutKeys { combo: \"mod+shift+p\".to_string(), separator: true }".to_string(),
            }

            DemoBox {
                title: "快捷键系统与帮助面板".to_string(),
                description: Some("用 ShortcutProvider 包裹，多个组件各自 use_register_shortcut 注册；use_shortcut_list 读取注册表生成帮助面板。按 ? 查看全部快捷键。".to_string()),
                demo: rsx! {
                    ShortcutProvider {
                        ShortcutSystemDemo {}
                    }
                },
                code: "ShortcutProvider {\n    // 子组件内注册\n    use_register_shortcut(\"mod+s\", \"保存\", move || { /* ... */ });\n    use_register_shortcut(\"mod+n\", \"新建\", move || { /* ... */ });\n    // 读取注册表生成帮助面板\n    let list = use_shortcut_list();\n}".to_string(),
            }

            h2 { "组合键语法" }
            PropsTable { headers: vec!["记号".to_string(), "含义".to_string(), "示例".to_string(), "".to_string()], rows: vec![
                ("mod", "跨平台修饰键：macOS ⌘ / 其他 Ctrl", "mod+k", ""),
                ("ctrl / control", "Control 键", "ctrl+c", ""),
                ("alt / option", "Alt / Option 键", "alt+enter", ""),
                ("shift", "Shift 键", "mod+shift+p", ""),
                ("meta / cmd / win", "Meta / ⌘ / Windows 键", "meta+s", ""),
                ("主键", "取 KeyboardEvent.key，支持 escape/enter/arrowup 等别名", "escape", ""),
            ]}

            h2 { "use_shortcut" }
            p { "独立注册单个快捷键，无需 Provider。组件卸载自动解绑。" }
            PropsTable { headers: vec!["参数".to_string(), "类型".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("combo", "impl Into<String>", "组合键，如 \"mod+k\"", ""),
                ("handler", "impl FnMut() + 'static", "命中时触发的回调", ""),
            ]}

            h2 { "ShortcutProvider" }
            p { "快捷键系统上下文。内部安装单个 document 监听器集中调度，并维护可查询的注册表。" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("children", "Element", "被快捷键系统覆盖的子树", ""),
            ]}

            h2 { "use_register_shortcut" }
            p { "向系统注册带描述的快捷键（需在 ShortcutProvider 内）。描述进入注册表，可用于帮助面板。另有 use_register_shortcut_grouped 支持指定分组。" }
            PropsTable { headers: vec!["参数".to_string(), "类型".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("combo", "impl Into<String>", "组合键", ""),
                ("description", "impl Into<String>", "功能描述", ""),
                ("handler", "impl FnMut() + 'static", "命中时触发的回调", ""),
            ]}

            h2 { "use_shortcut_list" }
            p { "读取当前已注册的所有快捷键信息（响应式），返回 Vec<ShortcutInfo>，用于生成帮助面板。" }

            h2 { "ShortcutKeys Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("combo", "String", "—", "组合键字符串"),
                ("size", "Size", "Sm", "按键尺寸"),
                ("separator", "bool", "false", "按键间是否显示 + 连接符"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn ShortcutBasicDemo() -> Element {
    let mut count = use_signal(|| 0);
    use_shortcut("mod+k", move || count += 1);

    rsx! {
        div { style: "display:flex; align-items:center; gap:12px;",
            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "按 " }
            ShortcutKeys { combo: "mod+k".to_string() }
            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", " 已触发 " }
            span { style: "color:var(--ctrl-primary); font-weight:700; font-size:20px;", "{count}" }
            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", " 次" }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn ShortcutSystemDemo() -> Element {
    let mut log = use_signal(|| "（等待操作）".to_string());
    let mut help_open = use_signal(|| false);

    use_register_shortcut("mod+s", "保存当前文件", move || log.set("已保存 ✓".to_string()));
    use_register_shortcut("mod+n", "新建文件", move || log.set("已新建文件 📄".to_string()));
    use_register_shortcut("mod+shift+f", "全局搜索", move || log.set("打开全局搜索 🔍".to_string()));
    use_register_shortcut("shift+/", "显示快捷键帮助", move || help_open.set(!help_open()));

    let list = use_shortcut_list();

    rsx! {
        div { style: "display:flex; align-items:center; gap:16px; flex-wrap:wrap;",
            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);",
                "试试 "
                ShortcutKeys { combo: "mod+s".to_string() }
                "、"
                ShortcutKeys { combo: "mod+n".to_string() }
                "、"
                ShortcutKeys { combo: "mod+shift+f".to_string() }
                "，或 "
                ShortcutKeys { combo: "shift+/".to_string() }
                " 查看帮助"
            }
            Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| help_open.set(!help_open()), "快捷键帮助" }
        }
        div { style: "margin-top:12px; color:var(--ctrl-text);",
            "最近操作："
            span { style: "color:var(--ctrl-primary); font-weight:600;", "{log}" }
        }
        if help_open() {
            div { style: "margin-top:16px; border:1px solid var(--ctrl-border); border-radius:var(--ctrl-radius-md); overflow:hidden; max-width:420px;",
                div { style: "padding:10px 16px; background:var(--ctrl-bg-secondary); font-weight:600; color:var(--ctrl-text); font-size:var(--ctrl-font-size-sm);", "键盘快捷键" }
                for info in list.iter() {
                    div { key: "{info.combo}",
                        style: "display:flex; align-items:center; justify-content:space-between; gap:16px; padding:10px 16px; border-top:1px solid var(--ctrl-border);",
                        span { style: "color:var(--ctrl-text); font-size:var(--ctrl-font-size-sm);", "{info.description}" }
                        ShortcutKeys { combo: "{info.combo}" }
                    }
                }
            }
        }
    }
}
