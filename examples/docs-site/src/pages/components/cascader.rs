use dioxus::prelude::*;
use ctrl::prelude::*;
use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn CascaderPage() -> Element {
    rsx! {
        h1 { "Cascader 级联选择器" }
        p { "用于从树形层级数据中选择一个值，如省市区选择、组织架构选择等场景。点击选项展开下一级，选择叶子节点确认。" }

        h2 { "基本用法" }

        DemoBox {
            title: "基础级联".to_string(),
            description: Some("点击选项逐级展开，选择叶子节点触发 onchange 回调。".to_string()),
            demo: rsx! {
                div { style: "width: 300px;",
                    Cascader {
                        placeholder: "请选择地区".to_string(),
                        options: vec![
                            CascaderOption {
                                value: "zhejiang".to_string(), label: "浙江".to_string(), disabled: false,
                                children: vec![
                                    CascaderOption { value: "hangzhou".to_string(), label: "杭州".to_string(), disabled: false, children: vec![] },
                                    CascaderOption { value: "ningbo".to_string(), label: "宁波".to_string(), disabled: false, children: vec![] },
                                ],
                            },
                            CascaderOption {
                                value: "jiangsu".to_string(), label: "江苏".to_string(), disabled: false,
                                children: vec![
                                    CascaderOption { value: "nanjing".to_string(), label: "南京".to_string(), disabled: false, children: vec![] },
                                    CascaderOption { value: "suzhou".to_string(), label: "苏州".to_string(), disabled: false, children: vec![] },
                                ],
                            },
                        ],
                    }
                }
            },
            code: "Cascader {\n    placeholder: \"请选择地区\".to_string(),\n    options: vec![\n        CascaderOption {\n            value: \"zhejiang\", label: \"浙江\",\n            children: vec![\n                CascaderOption { value: \"hangzhou\", label: \"杭州\", children: vec![] },\n            ],\n        },\n    ],\n}".to_string(),
        }

        DemoBox {
            title: "可清空".to_string(),
            description: Some("设置 clearable 为 true 显示清空按钮。".to_string()),
            demo: rsx! {
                div { style: "width: 300px;",
                    Cascader {
                        clearable: true,
                        placeholder: "可清空选择".to_string(),
                        options: vec![
                            CascaderOption {
                                value: "beijing".to_string(), label: "北京".to_string(), disabled: false,
                                children: vec![
                                    CascaderOption { value: "chaoyang".to_string(), label: "朝阳区".to_string(), disabled: false, children: vec![] },
                                ],
                            },
                        ],
                    }
                }
            },
            code: "Cascader {\n    clearable: true,\n    placeholder: \"可清空选择\".to_string(),\n    options: vec![...],\n}".to_string(),
        }

        DemoBox {
            title: "悬停展开".to_string(),
            description: Some("设置 expand_on_hover 为 true，鼠标悬停父级即展开下一级，选择叶子节点仍需点击。".to_string()),
            demo: rsx! {
                div { style: "width: 300px;",
                    Cascader {
                        expand_on_hover: true,
                        placeholder: "悬停展开地区".to_string(),
                        options: vec![
                            CascaderOption {
                                value: "zhejiang".to_string(), label: "浙江".to_string(), disabled: false,
                                children: vec![
                                    CascaderOption { value: "hangzhou".to_string(), label: "杭州".to_string(), disabled: false, children: vec![] },
                                    CascaderOption { value: "ningbo".to_string(), label: "宁波".to_string(), disabled: false, children: vec![] },
                                ],
                            },
                            CascaderOption {
                                value: "jiangsu".to_string(), label: "江苏".to_string(), disabled: false,
                                children: vec![
                                    CascaderOption { value: "nanjing".to_string(), label: "南京".to_string(), disabled: false, children: vec![] },
                                    CascaderOption { value: "suzhou".to_string(), label: "苏州".to_string(), disabled: false, children: vec![] },
                                ],
                            },
                        ],
                    }
                }
            },
            code: "Cascader {\n    expand_on_hover: true,\n    placeholder: \"悬停展开地区\".to_string(),\n    options: vec![...],\n}".to_string(),
        }

        h2 { "Cascader Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("value", "Option<Vec<String>>", "None", "当前选中路径"),
            ("options", "Vec<CascaderOption>", "vec![]", "级联选项树"),
            ("placeholder", "String", "\"\"", "占位文本"),
            ("disabled", "bool", "false", "是否禁用"),
            ("size", "Size", "Md", "尺寸"),
            ("clearable", "bool", "false", "是否可清空"),
            ("expand_on_hover", "bool", "false", "是否悬停展开下一级"),
            ("class", "String", "\"\"", "自定义 CSS 类名"),
            ("style", "String", "\"\"", "自定义内联样式"),
            ("onchange", "Option<EventHandler<Vec<String>>>", "None", "选中值变化回调"),
        ]}
    }
}
