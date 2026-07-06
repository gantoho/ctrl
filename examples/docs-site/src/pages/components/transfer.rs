use dioxus::prelude::*;
use ctrl::prelude::*;
use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TransferPage() -> Element {
    rsx! {
        h1 { "Transfer 穿梭框" }
        p { "双栏穿梭选择框，用于在多选项之间进行移动操作。常用于权限分配、分类归类等场景。" }

        h2 { "基本用法" }

        DemoBox {
            title: "基础穿梭".to_string(),
            description: Some("左侧展示可选项，右侧展示已选项，通过中间按钮进行移动。支持搜索过滤。".to_string()),
            demo: rsx! {
                Transfer {
                    data_source: vec![
                        TransferItem { key: "1".to_string(), label: "选项 1".to_string(), disabled: false },
                        TransferItem { key: "2".to_string(), label: "选项 2".to_string(), disabled: false },
                        TransferItem { key: "3".to_string(), label: "选项 3".to_string(), disabled: false },
                        TransferItem { key: "4".to_string(), label: "选项 4".to_string(), disabled: false },
                        TransferItem { key: "5".to_string(), label: "选项 5".to_string(), disabled: true },
                        TransferItem { key: "6".to_string(), label: "选项 6".to_string(), disabled: false },
                        TransferItem { key: "7".to_string(), label: "选项 7".to_string(), disabled: false },
                    ],
                    target_keys: vec!["2".to_string(), "4".to_string()],
                }
            },
            code: "Transfer {\n    data_source: vec![\n        TransferItem { key: \"1\".to_string(), label: \"选项 1\".to_string(), disabled: false },\n        TransferItem { key: \"2\".to_string(), label: \"选项 2\".to_string(), disabled: false },\n    ],\n    target_keys: vec![\"2\".to_string()],\n}".to_string(),
        }

        DemoBox {
            title: "带搜索".to_string(),
            description: Some("设置 show_search 为 true 启用搜索过滤。".to_string()),
            demo: rsx! {
                Transfer {
                    show_search: true,
                    search_placeholder: "搜索选项".to_string(),
                    data_source: vec![
                        TransferItem { key: "apple".to_string(), label: "Apple".to_string(), disabled: false },
                        TransferItem { key: "banana".to_string(), label: "Banana".to_string(), disabled: false },
                        TransferItem { key: "cherry".to_string(), label: "Cherry".to_string(), disabled: false },
                        TransferItem { key: "date".to_string(), label: "Date".to_string(), disabled: false },
                    ],
                    target_keys: vec![],
                }
            },
            code: "Transfer {\n    show_search: true,\n    search_placeholder: \"搜索选项\".to_string(),\n    data_source: vec![...],\n    target_keys: vec![],\n}".to_string(),
        }

        h2 { "Transfer Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("data_source", "Vec<TransferItem>", "vec![]", "所有的数据项"),
            ("target_keys", "Vec<String>", "vec![]", "右侧已选中的 key 列表"),
            ("titles", "Option<(String, String)>", "None", "左右栏标题"),
            ("show_search", "bool", "false", "是否显示搜索框"),
            ("search_placeholder", "String", "\"\"", "搜索占位文本"),
            ("class", "String", "\"\"", "自定义 CSS 类名"),
            ("style", "String", "\"\"", "自定义内联样式"),
            ("onchange", "Option<EventHandler<Vec<String>>>", "None", "target_keys 变化回调"),
        ]}
    }
}
