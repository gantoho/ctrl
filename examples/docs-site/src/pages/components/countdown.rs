use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn CountdownPage() -> Element {
    let deadline = js_sys::Date::now() + 60_000.0; // 1分钟后
    let far = js_sys::Date::now() + 86400_000.0 * 2.0 + 3600_000.0;

    rsx! {
        h1 { "Countdown 倒计时" }
        p { "用于显示目标时间的倒计时，支持多种显示格式和样式。" }

        DemoBox { title: "基本用法".to_string(), description: Some("默认格式 hh:mm:ss，分隔卡片样式。".to_string()),
            demo: rsx! {
                Countdown { deadline }
            },
            code: "Countdown { deadline: some_timestamp }".to_string(),
        }

        DemoBox { title: "显示天".to_string(), description: Some("格式 dd:hh:mm:ss 显示天、时、分、秒。".to_string()),
            demo: rsx! {
                Countdown { deadline: far, format: "dd:hh:mm:ss".to_string() }
            },
            code: "Countdown { deadline, format: \"dd:hh:mm:ss\" }".to_string(),
        }

        DemoBox { title: "普通数字样式".to_string(), description: Some("variant: normal 不使用分隔卡片。".to_string()),
            demo: rsx! {
                Countdown { deadline, variant: "normal".to_string() }
            },
            code: "Countdown { deadline, variant: \"normal\" }".to_string(),
        }

        DemoBox { title: "自定义格式".to_string(), description: Some("format 字符串灵活控制显示哪些单位。".to_string()),
            demo: rsx! {
                Countdown { deadline, format: "mm分 ss秒".to_string() }
            },
            code: "Countdown { deadline, format: \"mm分 ss秒\" }".to_string(),
        }

        h2 { "Countdown Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("deadline", "f64", "—", "目标时间戳（毫秒）"),
            ("format", "String", "\"hh:mm:ss\"", "显示格式（d=天, h=时, m=分, s=秒）"),
            ("finished_text", "String", "\"00:00:00\"", "倒计时结束文案"),
            ("variant", "String", "\"split\"", "数字样式：split / normal"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}
    }
}
