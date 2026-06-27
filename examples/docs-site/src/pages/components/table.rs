use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TablePage() -> Element {
    rsx! {
div { id: "table", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Table 表格"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "表格用于展示结构化数据，支持斑马纹和边框。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 columns 和 data 定义表格。".to_string()),
                demo: rsx! {
                    Table {
                        columns: vec![
                            TableColumn { title: "名称".into(), ..Default::default() },
                            TableColumn { title: "类型".into(), ..Default::default() },
                            TableColumn { title: "默认值".into(), ..Default::default() },
                            TableColumn { title: "说明".into(), ..Default::default() },
                        ],
                        data: vec![
                            vec!["variant".into(), "Variant".into(), "Primary".into(), "按钮语义变体".into()],
                            vec!["size".into(), "Size".into(), "Md".into(), "按钮尺寸".into()],
                            vec!["disabled".into(), "bool".into(), "false".into(), "是否禁用".into()],
                            vec!["onclick".into(), "Option<EventHandler>".into(), "None".into(), "点击事件".into()],
                        ],
                    }
                },
                code: "let cols = vec![TableColumn { title: \"名称\".into(), ..Default::default() }, ...];\nlet data = vec![vec![\"variant\", \"Variant\", \"Primary\", \"按钮语义变体\"], ...];\n\nTable { columns: cols, data: data }".to_string(),
            }

            DemoBox {
                title: "斑马纹".to_string(),
                description: Some("设置 striped 为 true 显示交替行背景色。".to_string()),
                demo: rsx! {
                    Table {
                        striped: true,
                        columns: vec![
                            TableColumn { title: "姓名".into(), ..Default::default() },
                            TableColumn { title: "年龄".into(), ..Default::default() },
                            TableColumn { title: "城市".into(), ..Default::default() },
                        ],
                        data: vec![
                            vec!["张三".into(), "28".into(), "北京".into()],
                            vec!["李四".into(), "32".into(), "上海".into()],
                            vec!["王五".into(), "25".into(), "广州".into()],
                            vec!["赵六".into(), "30".into(), "深圳".into()],
                        ],
                    }
                },
                code: "Table { striped: true, columns: cols, data: data }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Table Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("columns", "Vec<TableColumn>", "[]", "列定义"),
                ("data", "Vec<Vec<String>>", "[]", "行数据"),
                ("striped", "bool", "false", "是否显示斑马纹"),
                ("bordered", "bool", "true", "是否显示边框"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "TableColumn 属性" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "列标题"),
                ("width", "Option<String>", "None", "列宽"),
                ("align", "Option<String>", "None", "对齐方式"),
            ]}
        }
    }
}
