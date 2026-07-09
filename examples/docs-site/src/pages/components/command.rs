use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn CommandPage() -> Element {
    rsx! {
div { id: "command", style: "margin-top: 64px;",
            h1 {
                "Command 命令面板"
            }
            p {
                "可搜索的命令菜单（⌘K 风格），支持分组展示与键盘导航（↑/↓ 选择、Enter 确认、Esc 关闭），常用于全局快捷操作、快速跳转等场景。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("点击按钮或按 ⌘K / Ctrl+K 打开命令面板，输入关键词过滤，使用键盘或鼠标选择。快捷键通过 use_shortcut Hook 注册。".to_string()),
                demo: rsx! {
                    CommandDemo {}
                },
                code: "let mut open = use_signal(|| false);\n// 全局快捷键呼出：⌘K / Ctrl+K\nuse_shortcut(\"mod+k\", move || open.set(true));\n\nlet items = vec![\n    CommandItem::new(\"new\", \"新建文件\").icon(\"📄\").group(\"操作\").shortcut(\"⌘N\"),\n    CommandItem::new(\"search\", \"搜索\").icon(\"🔍\").group(\"操作\").shortcut(\"⌘F\"),\n];\nButton { onclick: move |_| open.set(true), \"打开命令面板\" }\nCommand {\n    open: open(),\n    items: items,\n    onselect: move |key: String| { /* 处理选中 */ },\n    onclose: move |_| open.set(false),\n}".to_string(),
            }

            h2 { "Command Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("open", "bool", "—", "是否打开"),
                ("items", "Vec<CommandItem>", "—", "命令项列表"),
                ("placeholder", "String", "输入命令进行搜索…", "搜索框占位文字"),
                ("empty_text", "String", "未找到匹配的命令", "无结果提示"),
                ("onselect", "EventHandler<String>", "—", "选中命令回调（返回 key）"),
                ("onclose", "EventHandler<()>", "—", "关闭回调"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}

            h2 { "CommandItem 构造" }
            PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "说明".to_string(), "".to_string()], rows: vec![
                ("new(key, label)", "into<String>", "创建命令项", ""),
                (".description(d)", "into<String>", "设置描述文字", ""),
                (".group(g)", "into<String>", "设置分组（需连续排列）", ""),
                (".icon(i)", "into<String>", "设置图标（emoji/文本）", ""),
                (".shortcut(s)", "into<String>", "设置快捷键提示", ""),
            ]}

            h2 { "快捷键呼出" }
            p {
                "命令面板通过快捷键系统的 use_shortcut 注册全局呼出快捷键：在 document 上监听 keydown，命中组合键时触发回调并阻止浏览器默认行为，组件卸载自动解绑。组合键以 + 连接，支持 ctrl / alt / shift / meta / mod（跨平台，macOS 为 ⌘、其他为 Ctrl）。若需集中管理多个快捷键并生成帮助面板，请使用 ShortcutProvider + use_register_shortcut + use_shortcut_list，详见「Shortcut 快捷键」页面。"
            }
            PropsTable { headers: vec!["参数".to_string(), "类型".to_string(), "说明".to_string(), "示例".to_string()], rows: vec![
                ("combo", "impl Into<String>", "组合键，如 \"mod+k\"、\"ctrl+shift+p\"、\"escape\"", "\"mod+k\""),
                ("handler", "impl FnMut() + 'static", "命中时触发的回调", "move || open.set(true)"),
            ]}
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn CommandDemo() -> Element {
    let mut open = use_signal(|| false);
    let mut last = use_signal(|| "（尚未选择）".to_string());

    // 快捷键呼出：⌘K（macOS）/ Ctrl+K（其他平台）
    use_shortcut("mod+k", move || open.set(true));

    let items = vec![
        CommandItem::new("new-file", "新建文件").icon("📄").group("操作").shortcut("⌘N"),
        CommandItem::new("new-folder", "新建文件夹").icon("📁").group("操作").shortcut("⌘⇧N"),
        CommandItem::new("search", "全局搜索").icon("🔍").group("操作").shortcut("⌘F").description("在所有文件中搜索内容"),
        CommandItem::new("go-home", "跳转到首页").icon("🏠").group("导航"),
        CommandItem::new("go-settings", "打开设置").icon("⚙️").group("导航").shortcut("⌘,"),
        CommandItem::new("go-profile", "个人资料").icon("👤").group("导航"),
        CommandItem::new("theme", "切换主题").icon("🌓").group("偏好").description("在浅色与深色之间切换"),
        CommandItem::new("logout", "退出登录").icon("🚪").group("偏好"),
    ];

    rsx! {
        div { style: "display:flex; align-items:center; gap:16px;",
            Button {
                variant: Variant::Primary,
                onclick: move |_| open.set(true),
                "打开命令面板"
            }
            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm); display:inline-flex; align-items:center; gap:6px;",
                "或按 "
                ShortcutKeys { combo: "mod+k".to_string() }
            }
            span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);",
                "上次选择："
                span { style: "color:var(--ctrl-text);", "{last}" }
            }
        }
        Command {
            open: open(),
            items: items,
            onselect: move |key: String| last.set(key),
            onclose: move |_| open.set(false),
        }
    }
}
