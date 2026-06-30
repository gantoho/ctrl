use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn StepsPage() -> Element {
    rsx! {
div { id: "steps", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Steps 步骤条" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "用于展示流程进度，支持水平和垂直排列。Steps 通过 Context 向每个 Step 注入 current 状态，Step 自动根据自身位置计算完成/进行中/待处理状态。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("水平步骤条，current = 0 表示第一步正在进行中，之前无已完成步骤。".to_string()),
                demo: rsx! {
                    Steps { current: 0,
                        Step { title: "步骤一".to_string(), description: "第一步描述".to_string() }
                        Step { title: "步骤二".to_string(), description: "第二步描述".to_string() }
                        Step { title: "步骤三".to_string(), description: "最后一步".to_string() }
                    }
                },
                code: "Steps { current: 0,\n    Step { title: \"步骤一\", description: \"描述\" }\n    Step { title: \"步骤二\" }\n    Step { title: \"步骤三\" }\n}".to_string(),
            }

            DemoBox {
                title: "指定当前进度".to_string(),
                description: Some("current = 1 表示第二步正在进行，第一步已完成（显示 ✓）。".to_string()),
                demo: rsx! {
                    Steps { current: 1,
                        Step { title: "已完成".to_string(), description: "上一步".to_string() }
                        Step { title: "进行中".to_string(), description: "当前步骤".to_string() }
                        Step { title: "待处理".to_string(), description: "下一步".to_string() }
                    }
                },
                code: "Steps { current: 1, /* current = 1，对应第二步 */\n    Step { title: \"已完成\" }\n    Step { title: \"进行中\" }\n    Step { title: \"待处理\" }\n}".to_string(),
            }

            DemoBox {
                title: "垂直排列".to_string(),
                description: Some("设置 direction = Vertical。".to_string()),
                demo: rsx! {
                    Steps { current: 1, direction: Direction::Vertical,
                        Step { title: "已完成".to_string(), description: "上一步".to_string() }
                        Step { title: "进行中".to_string(), description: "当前步骤".to_string() }
                        Step { title: "待处理".to_string(), description: "下一步".to_string() }
                    }
                },
                code: "Steps { current: 1, direction: Direction::Vertical,\n    Step { title: \"已完成\", description: \"上一步\" }\n    Step { title: \"进行中\", description: \"当前\" }\n    Step { title: \"待处理\", description: \"下一步\" }\n}".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Steps Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("current", "i32", "-1", "当前步骤索引，0=第一步进行中，n=第n步进行中，-1=未开始"),
                ("direction", "Direction", "Horizontal", "排列方向：Horizontal / Vertical"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "子元素（Step）"),
            ]}

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Step Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "步骤标题"),
                ("description", "String", "\"\"", "步骤描述"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Step 渲染状态" }
            p { style: "font-size: var(--ctrl-font-size-md); color: var(--ctrl-text-secondary); margin-bottom: 12px;",
                "Step 通过 Steps 的 Context (StepCtx) 自动获取 current 索引和自身位置，根据对比计算状态："
            }
            PropsTable { headers: vec!["状态".to_string(), "条件".to_string(), "圆圈显示".to_string(), "".to_string()], rows: vec![
                ("finish (已完成)", "step_index < current", "✓ 对勾", ""),
                ("process (进行中)", "step_index == current", "序号（从 1 开始）", ""),
                ("wait (待处理)", "step_index > current", "序号（从 1 开始）", ""),
            ]}
        }
    }
}
