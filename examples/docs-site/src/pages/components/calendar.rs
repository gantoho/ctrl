use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn CalendarPage() -> Element {
    let mut selected = use_signal(|| String::new());

    rsx! {
        div { id: "calendar", style: "margin-top: 64px;",
            h1 { "Calendar 日历" }
            p { "月历视图组件，支持上一月/下一月切换、日期点击选中。" }

            DemoBox {
                title: "基础用法".to_string(),
                description: Some("点击选中日期，通过 onchange 回调获取选中的日期值。当前日会高亮为蓝色文字。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 20px; align-items: flex-start; flex-wrap: wrap;",
                        Calendar {
                            value: selected(),
                            onchange: move |v| selected.set(v),
                        }
                        if !selected().is_empty() {
                            div { style: "padding: 8px 16px; background: var(--ctrl-bg-secondary); border-radius: var(--ctrl-radius-md); font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);",
                                "选中日期：{selected()}"
                            }
                        }
                    }
                },
                code: r#"let mut selected = use_signal(|| String::new());

Calendar {
    value: selected(),
    onchange: move |v| selected.set(v),
}"#.to_string(),
            }

            DemoBox {
                title: "命令式 API (CalendarProvider)".to_string(),
                description: Some("用 CalendarProvider 包裹应用，子组件中通过 use_calendar() 获取 API，调用 open(config) 以浮层方式弹出日历选择日期。".to_string()),
                demo: rsx! {
                    CalendarProvider {
                        CalendarCmdDemo {}
                    }
                },
                code: r#"// 根节点包裹 Provider
CalendarProvider { /* 应用内容 */ }

// 子组件中命令式调用
let mut calendar = use_calendar();
calendar.open(CalendarConfig {
    title: "选择日期".into(),
    onchange: Some(EventHandler::new(move |date: String| {
        // 处理选中的日期
    })),
    ..Default::default()
});"#.to_string(),
            }

            h2 { "Calendar Props" }
            PropsTable {
                headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()],
                rows: vec![
                    ("value", "String", "\"\"", "选中的日期，格式 \"YYYY-MM-DD\""),
                    ("onchange", "EventHandler<String>", "None", "日期变化时回调，参数为 \"YYYY-MM-DD\""),
                    ("class", "String", "\"\"", "自定义类名"),
                ],
            }

            h2 { "命令式 API" }
            PropsTable {
                headers: vec!["成员".to_string(), "类型".to_string(), "签名".to_string(), "说明".to_string()],
                rows: vec![
                    ("use_calendar", "fn", "() -> CalendarAPI", "在 CalendarProvider 子组件中获取 API"),
                    ("open", "method", "(&mut self, config: CalendarConfig)", "以浮层方式打开日历"),
                    ("close", "method", "(&mut self)", "关闭日历浮层"),
                ],
            }

            h2 { "CalendarConfig 字段" }
            PropsTable {
                headers: vec!["字段".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()],
                rows: vec![
                    ("value", "String", "\"\"", "初始选中日期 \"YYYY-MM-DD\""),
                    ("title", "String", "\"选择日期\"", "浮层标题，为空则不显示标题栏"),
                    ("mask_closable", "bool", "true", "点击遮罩是否关闭"),
                    ("onchange", "Option<EventHandler<String>>", "None", "选中日期回调，选中后自动关闭"),
                    ("onclose", "Option<EventHandler<()>>", "None", "关闭回调"),
                ],
            }
        }
    }
}

/// 命令式 API 示例：在 CalendarProvider 子组件中通过 use_calendar() 触发浮层
#[component]
#[allow(non_snake_case)]
fn CalendarCmdDemo() -> Element {
    let mut calendar = use_calendar();
    let mut picked = use_signal(String::new);

    rsx! {
        div { style: "display: flex; gap: 20px; align-items: center; flex-wrap: wrap;",
            Button {
                variant: Variant::Primary,
                onclick: move |_| {
                    calendar.open(CalendarConfig {
                        title: "选择日期".to_string(),
                        value: picked(),
                        onchange: Some(EventHandler::new(move |date: String| {
                            picked.set(date);
                        })),
                        ..Default::default()
                    });
                },
                "打开日历"
            }
            if !picked().is_empty() {
                div { style: "padding: 8px 16px; background: var(--ctrl-bg-secondary); border-radius: var(--ctrl-radius-md); font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);",
                    "选中日期：{picked()}"
                }
            }
        }
    }
}
