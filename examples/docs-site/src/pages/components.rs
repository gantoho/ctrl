use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::components::codeblock::CodeBlock;

/// 组件总览页 —— 所有组件文档集中在一个页面
#[component]
#[allow(non_snake_case)]
pub fn ComponentsPage() -> Element {
    rsx! {
        // ════════════════════════════════════════
        // Button 按钮
        // ════════════════════════════════════════
        div { id: "button",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Button 按钮"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;",
                "按钮用于触发操作。Ctrl UI 提供了四种语义变体和三种尺寸，支持禁用、块级等状态。"
            }

            // 变体
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin-bottom: 20px;", "变体 Variant" }

            DemoBox {
                title: "Primary - 主要按钮".to_string(),
                description: Some("最重要的操作按钮，用于提交、确认等场景。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
                        Button { variant: Variant::Primary, "Primary" }
                        Button { variant: Variant::Primary, disabled: true, "禁用" }
                    }
                },
                code: "Button { variant: Variant::Primary, \"Primary\" }\nButton { variant: Variant::Primary, disabled: true, \"禁用\" }".to_string(),
            }

            DemoBox {
                title: "Secondary - 次要按钮".to_string(),
                description: Some("用于取消、返回等次要操作。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
                        Button { variant: Variant::Secondary, "Secondary" }
                        Button { variant: Variant::Secondary, disabled: true, "禁用" }
                    }
                },
                code: "Button { variant: Variant::Secondary, \"Secondary\" }".to_string(),
            }

            DemoBox {
                title: "Outline - 描边按钮".to_string(),
                description: Some("中等强调，适合编辑、查看等操作。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
                        Button { variant: Variant::Outline, "Outline" }
                        Button { variant: Variant::Outline, disabled: true, "禁用" }
                    }
                },
                code: "Button { variant: Variant::Outline, \"Outline\" }".to_string(),
            }

            DemoBox {
                title: "Ghost - 幽灵按钮".to_string(),
                description: Some("低强调，常用于列表中的操作链接。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
                        Button { variant: Variant::Ghost, "Ghost" }
                        Button { variant: Variant::Ghost, disabled: true, "禁用" }
                    }
                },
                code: "Button { variant: Variant::Ghost, \"Ghost\" }".to_string(),
            }

            // 尺寸
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "尺寸 Size" }

            DemoBox {
                title: "三种尺寸".to_string(),
                description: Some("Sm 28px / Md 36px / Lg 44px".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 12px; flex-wrap: wrap; align-items: flex-start;",
                        Button { variant: Variant::Primary, size: Size::Sm, "Small" }
                        Button { variant: Variant::Primary, size: Size::Md, "Medium" }
                        Button { variant: Variant::Primary, size: Size::Lg, "Large" }
                    }
                },
                code: "Button { size: Size::Sm, \"Small\" }\nButton { size: Size::Md, \"Medium\" }\nButton { size: Size::Lg, \"Large\" }".to_string(),
            }

            // 块级
            DemoBox {
                title: "块级按钮".to_string(),
                description: Some("block 为 true 时按钮宽度撑满父容器。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 400px;",
                        Button { variant: Variant::Primary, block: true, "块级按钮" }
                        Button { variant: Variant::Outline, block: true, "块级描边" }
                    }
                },
                code: "Button { variant: Variant::Primary, block: true, \"块级按钮\" }".to_string(),
            }

            // 交互
            DemoBox {
                title: "交互示例".to_string(),
                description: Some("点击按钮实时更新计数，展示事件绑定和禁用切换。".to_string()),
                demo: rsx! { ButtonInteract {} },
                code: "let mut count = use_signal(|| 0);\nlet mut disabled = use_signal(|| false);\n\nButton {\n    disabled: disabled(),\n    onclick: move |_| count.set(count() + 1),\n    \"点击次数: {count()}\"\n}\nButton {\n    onclick: move |_| disabled.set(!disabled()),\n    if disabled() { \"恢复\" } else { \"禁用\" }\n}".to_string(),
            }

            // Button Props
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Button Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("variant", "Variant", "Primary", "按钮语义变体"),
                ("size", "Size", "Md", "按钮尺寸"),
                ("disabled", "bool", "false", "是否禁用"),
                ("block", "bool", "false", "是否块级"),
                ("class", "String", "\"\"", "自定义 CSS 类名"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onclick", "Option<EventHandler>", "None", "点击事件"),
                ("children", "Element", "—", "子元素"),
            ]}
        }

        // ════════════════════════════════════════
        // Input 输入框
        // ════════════════════════════════════════
        div { id: "input", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Input 输入框"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 24px;",
                "输入框用于文本输入。采用受控组件模式，支持尺寸、状态和事件绑定。"
            }

            div {
                style: "padding: 12px 16px; background: var(--ctrl-primary-light); border-radius: var(--ctrl-radius-md); margin-bottom: 32px; display: flex; align-items: flex-start; gap: 8px; font-size: var(--ctrl-font-size-md);",
                span { "💡" }
                span { strong { "注意: " } "Input 使用受控组件模式，必须通过 value + oninput 管理值。oninput 闭包参数需显式标注为 FormEvent。" }
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 oninput 实现双向绑定。".to_string()),
                demo: rsx! { BasicInputDemo {} },
                code: "let mut value = use_signal(|| String::new());\n\nInput {\n    placeholder: \"请输入内容\",\n    value: value(),\n    oninput: move |evt: FormEvent| value.set(evt.value()),\n}".to_string(),
            }

            DemoBox {
                title: "三种尺寸".to_string(),
                description: Some("Sm 28px / Md 36px / Lg 44px".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 360px;",
                        Input { placeholder: "Small", size: Size::Sm }
                        Input { placeholder: "Medium", size: Size::Md }
                        Input { placeholder: "Large", size: Size::Lg }
                    }
                },
                code: "Input { placeholder: \"Small\", size: Size::Sm }\nInput { placeholder: \"Medium\", size: Size::Md }\nInput { placeholder: \"Large\", size: Size::Lg }".to_string(),
            }

            DemoBox {
                title: "状态".to_string(),
                description: Some("禁用、只读、错误状态。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 360px;",
                        Input { placeholder: "禁用状态", disabled: true }
                        Input { placeholder: "只读状态", readonly: true, value: "只读内容".to_string() }
                        Input { placeholder: "错误状态", error: true }
                    }
                },
                code: "Input { placeholder: \"禁用状态\", disabled: true }\nInput { placeholder: \"只读状态\", readonly: true, value: \"只读内容\" }\nInput { placeholder: \"错误状态\", error: true }".to_string(),
            }

            DemoBox {
                title: "表单验证".to_string(),
                description: Some("点击提交时验证邮箱和密码，实时清除错误状态。".to_string()),
                demo: rsx! { FormValidationDemo {} },
                code: "// 代码见页面下方".to_string(),
            }

            h3 { style: "font-size: 1rem; font-weight: 600; color: var(--ctrl-text); margin: 24px 0 12px;", "表单验证完整代码" }
            CodeBlock { code: [
                "let mut email = use_signal(|| String::new());",
                "let mut password = use_signal(|| String::new());",
                "let mut errors = use_signal(|| (false, false));",
                "let mut submitted = use_signal(|| false);",
                "",
                "let validate = move |_| {",
                "    let e = (email().trim().is_empty(), password().is_empty());",
                "    errors.set(e);",
                "    if !e.0 && !e.1 { submitted.set(true); }",
                "};",
                "",
                "rsx! {",
                "    Input { placeholder: \"邮箱\", value: email(), error: errors().0,",
                "        oninput: move |evt: FormEvent| { email.set(evt.value()); },",
                "    }",
                "    Input { r#type: \"password\", placeholder: \"密码\", value: password(), error: errors().1,",
                "        oninput: move |evt: FormEvent| { password.set(evt.value()); },",
                "    }",
                "    Button { variant: Variant::Primary, block: true, onclick: validate, \"提交\" }",
                "}",
            ].join("\n"), lang: Some("rust".to_string()) }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 48px 0 20px;", "Input Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("size", "Size", "Md", "输入框尺寸"),
                ("value", "String", "\"\"", "当前值（受控）"),
                ("placeholder", "String", "\"\"", "占位文本"),
                ("disabled", "bool", "false", "是否禁用"),
                ("error", "bool", "false", "是否错误状态"),
                ("r#type", "String", "\"text\"", "原生 input type"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("oninput", "Option<EventHandler>", "None", "输入事件"),
            ]}
        }

        // ════════════════════════════════════════
        // Switch 开关
        // ════════════════════════════════════════
        div { id: "switch", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Switch 开关"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "开关用于在两种状态间切换。支持三种尺寸和禁用状态。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 checked 和 onchange 管理开关状态。".to_string()),
                demo: rsx! { BasicSwitchDemo {} },
                code: "let mut on = use_signal(|| false);\n\nSwitch {\n    checked: on(),\n    onchange: move |v| on.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "三种尺寸".to_string(),
                description: Some("Sm / Md / Lg".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Switch { size: Size::Sm }
                        Switch { size: Size::Md, checked: true }
                        Switch { size: Size::Lg }
                    }
                },
                code: "Switch { size: Size::Sm }\nSwitch { size: Size::Md, checked: true }\nSwitch { size: Size::Lg }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("禁用时不可交互，透明度降低。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Switch { disabled: true }
                        Switch { disabled: true, checked: true }
                    }
                },
                code: "Switch { disabled: true }\nSwitch { disabled: true, checked: true }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Switch Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("checked", "bool", "false", "是否选中"),
                ("disabled", "bool", "false", "是否禁用"),
                ("size", "Size", "Md", "开关尺寸"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<bool>>", "None", "状态变化事件"),
            ]}
        }

        // ════════════════════════════════════════
        // Checkbox 复选框
        // ════════════════════════════════════════
        div { id: "checkbox", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Checkbox 复选框"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "复选框用于多选场景。支持选中、未选中、半选和禁用状态。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 checked 和 onchange 管理选中状态。".to_string()),
                demo: rsx! { BasicCheckboxDemo {} },
                code: "let mut checked = use_signal(|| false);\n\nCheckbox {\n    checked: checked(),\n    label: \"同意协议\".to_string(),\n    onchange: move |v| checked.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "状态".to_string(),
                description: Some("未选中、选中、半选、禁用。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        Checkbox { label: "未选中".to_string() }
                        Checkbox { checked: true, label: "已选中".to_string() }
                        Checkbox { indeterminate: true, label: "半选状态".to_string() }
                        Checkbox { disabled: true, label: "禁用未选中".to_string() }
                        Checkbox { checked: true, disabled: true, label: "禁用已选中".to_string() }
                    }
                },
                code: "Checkbox { label: \"未选中\" }\nCheckbox { checked: true, label: \"已选中\" }\nCheckbox { indeterminate: true, label: \"半选\" }\nCheckbox { disabled: true, label: \"禁用\" }".to_string(),
            }

            DemoBox {
                title: "全选示例".to_string(),
                description: Some("使用 Checkbox 实现全选/取消全选功能。".to_string()),
                demo: rsx! { CheckAllDemo {} },
                code: "// 全选逻辑代码见页面下方".to_string(),
            }

            h3 { style: "font-size: 1rem; font-weight: 600; color: var(--ctrl-text); margin: 24px 0 12px;", "全选示例代码" }
            CodeBlock { code: [
                "let items = vec![\"选项 A\", \"选项 B\", \"选项 C\"];",
                "let mut checked = use_signal(|| vec![false; items.len()]);",
                "",
                "// 计算全选/半选状态",
                "let all = checked().iter().all(|&c| c);",
                "let some = checked().iter().any(|&c| c);",
                "let indet = some && !all;",
                "",
                "// 全选 Checkbox",
                "Checkbox {",
                "    checked: all, indeterminate: indet, label: \"全选\".to_string(),",
                "    onchange: move |v| checked.set(vec![v; items.len()]),",
                "}",
                "// 子选项",
                "for (i, item) in items.iter().enumerate() {",
                "    Checkbox { label: item.to_string(), checked: checked()[i],",
                "        onchange: move |v| { let mut c = checked(); c[i] = v; checked.set(c); },",
                "    }",
                "}",
            ].join("\n"), lang: Some("rust".to_string()) }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Checkbox Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("checked", "bool", "false", "是否选中"),
                ("disabled", "bool", "false", "是否禁用"),
                ("indeterminate", "bool", "false", "半选状态"),
                ("label", "String", "\"\"", "标签文本"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<bool>>", "None", "状态变化事件"),
            ]}
        }

        // ════════════════════════════════════════
        // Radio 单选框
        // ════════════════════════════════════════
        div { id: "radio", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Radio 单选框"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "单选框用于在一组互斥选项中选择一项。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value + onchange 管理选中值。".to_string()),
                demo: rsx! { BasicRadioDemo {} },
                code: "let mut selected = use_signal(|| \"a\".to_string());\n\nRadio { value: \"a\", label: \"选项 A\", checked: selected() == \"a\", onchange: move |v| selected.set(v) }\nRadio { value: \"b\", label: \"选项 B\", checked: selected() == \"b\", onchange: move |v| selected.set(v) }\nRadio { value: \"c\", label: \"选项 C\", checked: selected() == \"c\", onchange: move |v| selected.set(v) }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("禁用时不可点击。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        Radio { value: "a", label: "选项 A".to_string(), checked: true, disabled: true }
                        Radio { value: "b", label: "选项 B".to_string(), disabled: true }
                    }
                },
                code: "Radio { value: \"a\", label: \"选项 A\".to_string(), checked: true, disabled: true }\nRadio { value: \"b\", label: \"选项 B\".to_string(), disabled: true }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Radio Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("checked", "bool", "false", "是否选中"),
                ("disabled", "bool", "false", "是否禁用"),
                ("value", "String", "\"\"", "单选值"),
                ("label", "String", "\"\"", "标签文本"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<String>>", "None", "选中变化事件"),
            ]}
        }

        // ════════════════════════════════════════
        // Select 下拉选择
        // ════════════════════════════════════════
        div { id: "select", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Select 下拉选择"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "下拉选择器用于从一组选项中选择一项。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 options 属性传入选项列表，onchange 获取选中值。".to_string()),
                demo: rsx! { BasicSelectDemo {} },
                code: "let options = vec![\n    (\"a\".to_string(), \"选项 A\".to_string(), false),\n    (\"b\".to_string(), \"选项 B\".to_string(), false),\n    (\"c\".to_string(), \"选项 C\".to_string(), false),\n];\n\nSelect { options, placeholder: \"请选择\", value: value(), onchange: move |v| value.set(v) }".to_string(),
            }

            DemoBox {
                title: "尺寸".to_string(),
                description: Some("Sm / Md / Lg".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 240px;",
                        Select { size: Size::Sm, options: vec![("1".into(), "小".into(), false)], placeholder: "Small".to_string() }
                        Select { size: Size::Md, options: vec![("1".into(), "中".into(), false)], placeholder: "Medium".to_string() }
                        Select { size: Size::Lg, options: vec![("1".into(), "大".into(), false)], placeholder: "Large".to_string() }
                    }
                },
                code: "Select { size: Size::Sm, options, placeholder: \"Small\" }\nSelect { size: Size::Md, options, placeholder: \"Medium\" }\nSelect { size: Size::Lg, options, placeholder: \"Large\" }".to_string(),
            }

            DemoBox {
                title: "禁用".to_string(),
                description: Some("整体禁用或单项禁用。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 240px;",
                        Select {
                            disabled: true,
                            placeholder: "整个禁用".to_string(),
                            options: vec![("1".into(), "选项".into(), false)],
                        }
                    }
                },
                code: "Select { disabled: true, options, placeholder: \"整个禁用\" }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Select Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("options", "Vec<(String,String,bool)>", "[]", "选项列表 (值,标签,禁用)"),
                ("value", "String", "\"\"", "当前选中值"),
                ("placeholder", "String", "\"请选择\"", "占位文本"),
                ("size", "Size", "Md", "选择器尺寸"),
                ("disabled", "bool", "false", "是否禁用"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<String>>", "None", "选中变化事件"),
            ]}
        }

        // ════════════════════════════════════════
        // Tag 标签
        // ════════════════════════════════════════
        div { id: "tag", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Tag 标签"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "标签用于标记和分类，支持多种颜色和可关闭模式。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 color 属性设置标签颜色，默认为主题色。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                        Tag { color: "var(--ctrl-primary)".to_string(), "Primary" }
                        Tag { color: "var(--ctrl-success)".to_string(), "Success" }
                        Tag { color: "var(--ctrl-warning)".to_string(), "Warning" }
                        Tag { color: "var(--ctrl-danger)".to_string(), "Danger" }
                        Tag { color: "var(--ctrl-info)".to_string(), "Info" }
                    }
                },
                code: "Tag { color: \"var(--ctrl-primary)\".to_string(), \"Primary\" }\nTag { color: \"var(--ctrl-success)\".to_string(), \"Success\" }\nTag { color: \"var(--ctrl-warning)\".to_string(), \"Warning\" }\nTag { color: \"var(--ctrl-danger)\".to_string(), \"Danger\" }".to_string(),
            }

            DemoBox {
                title: "可关闭".to_string(),
                description: Some("设置 closable 为 true 可以显示关闭按钮，点击后标签消失。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                        Tag { color: "var(--ctrl-primary)".to_string(), closable: true, "可关闭" }
                        Tag { color: "var(--ctrl-success)".to_string(), closable: true, "成功" }
                        Tag { color: "var(--ctrl-warning)".to_string(), closable: true, "警告" }
                    }
                },
                code: "Tag { color: \"var(--ctrl-primary)\".to_string(), closable: true, \"可关闭\" }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Tag Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("color", "String", "var(--ctrl-primary)", "标签颜色（CSS 颜色值）"),
                ("closable", "bool", "false", "是否可关闭"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭事件回调"),
                ("children", "Element", "—", "标签内容"),
            ]}
        }

        // ════════════════════════════════════════
        // Card 卡片
        // ════════════════════════════════════════
        div { id: "card", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Card 卡片"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "卡片用于承载和展示信息，支持标题、边框、阴影等样式。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("默认带边框，无阴影。".to_string()),
                demo: rsx! {
                    Card { title: "卡片标题".to_string(),
                        p { style: "color: var(--ctrl-text-secondary); margin: 0; font-size: var(--ctrl-font-size-md);", "这是卡片的内容区域，可以放置任何元素。" }
                    }
                },
                code: "Card { title: \"卡片标题\".to_string(),\n    p { \"这是卡片内容\" }\n}".to_string(),
            }

            DemoBox {
                title: "带阴影".to_string(),
                description: Some("设置 shadow 为 true 显示投影。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px;",
                        Card { style: "flex: 1;".to_string(), title: "默认卡片".to_string(),
                            p { style: "color: var(--ctrl-text-secondary); margin: 0;", "bordered 为默认值 true" }
                        }
                        Card { style: "flex: 1;".to_string(), shadow: true, title: "阴影卡片".to_string(),
                            p { style: "color: var(--ctrl-text-secondary); margin: 0;", "shadow: true" }
                        }
                    }
                },
                code: "Card { title: \"阴影卡片\".to_string(), shadow: true,\n    p { \"卡片内容\" }\n}".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Card Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "卡片标题"),
                ("bordered", "bool", "true", "是否显示边框"),
                ("shadow", "bool", "false", "是否带阴影"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("header", "Option<Element>", "None", "自定义头部插槽"),
                ("children", "Element", "—", "卡片内容"),
            ]}
        }

        // ════════════════════════════════════════
        // Dialog 对话框
        // ════════════════════════════════════════
        div { id: "dialog", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Dialog 对话框"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "对话框用于在当前页面之上显示重要信息或需要用户操作的场景。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 visible 控制显示，onclose 处理关闭。点击遮罩或关闭按钮均可关闭。".to_string()),
                demo: rsx! { BasicDialogDemo {} },
                code: "let mut visible = use_signal(|| false);\n\nButton { onclick: move |_| visible.set(true), \"打开对话框\" }\n\nDialog {\n    visible: visible(),\n    title: \"提示\".to_string(),\n    onclose: move |_| visible.set(false),\n    p { \"这是一条提示信息\" }\n}".to_string(),
            }

            DemoBox {
                title: "带底部操作".to_string(),
                description: Some("通过 footer 插槽自定义底部按钮区。".to_string()),
                demo: rsx! { FooterDialogDemo {} },
                code: "Dialog {\n    visible: visible(),\n    title: \"确认操作\".to_string(),\n    footer: rsx! {\n        Button { variant: Variant::Ghost, onclick: move |_| visible.set(false), \"取消\" }\n        Button { variant: Variant::Primary, onclick: move |_| visible.set(false), \"确定\" }\n    },\n    p { \"确定要执行此操作吗？\" }\n}".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Dialog Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("visible", "bool", "false", "是否显示"),
                ("title", "String", "\"\"", "对话框标题"),
                ("width", "String", "\"480px\"", "对话框宽度"),
                ("show_close", "bool", "true", "是否显示关闭按钮"),
                ("mask_closable", "bool", "true", "点击遮罩是否关闭"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭事件"),
                ("footer", "Option<Element>", "None", "底部插槽"),
                ("children", "Element", "—", "对话框内容"),
            ]}
        }

        // ════════════════════════════════════════
        // Table 表格
        // ════════════════════════════════════════
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

        // ════════════════════════════════════════
        // Badge 徽标
        // ════════════════════════════════════════
        div { id: "badge", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Badge 徽标"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "徽标用于在子元素右上角显示数字或圆点，表示新消息数或状态。"
            }

            DemoBox {
                title: "数字徽标".to_string(),
                description: Some("通过 count 设置显示的数字，超过 max 则显示 N+。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 32px; align-items: center;",
                        Badge { count: "5".to_string(),
                            div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "消息" }
                        }
                        Badge { count: "120".to_string(), max: 99,
                            div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "通知" }
                        }
                    }
                },
                code: "Badge { count: \"5\", div { \"消息\" } }\nBadge { count: \"120\", max: 99, div { \"通知\" } }".to_string(),
            }

            DemoBox {
                title: "圆点徽标".to_string(),
                description: Some("dot 为 true 时只显示一个小圆点，用于状态提示。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 32px; align-items: center;",
                        Badge { dot: true,
                            div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "状态" }
                        }
                    }
                },
                code: "Badge { dot: true, div { \"状态\" } }".to_string(),
            }

            DemoBox {
                title: "自定义颜色".to_string(),
                description: Some("通过 color 属性自定义徽标背景色。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 32px; align-items: center;",
                        Badge { count: "3".to_string(), color: "var(--ctrl-success)".to_string(),
                            div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "完成" }
                        }
                        Badge { count: "99+".to_string(), color: "var(--ctrl-warning)".to_string(),
                            div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "警告" }
                        }
                    }
                },
                code: "Badge { count: \"3\", color: \"var(--ctrl-success)\", div { \"完成\" } }\nBadge { count: \"99+\", color: \"var(--ctrl-warning)\", div { \"警告\" } }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Badge Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("count", "String", "\"\"", "徽标数字/文字"),
                ("dot", "bool", "false", "是否显示小圆点"),
                ("max", "u32", "99", "最大显示数字"),
                ("color", "String", "var(--ctrl-danger)", "徽标背景色"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "包裹的子元素"),
            ]}
        }

        // ════════════════════════════════════════
        // Avatar 头像
        // ════════════════════════════════════════
        div { id: "avatar", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Avatar 头像"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "头像用于表示用户或实体，支持图片和文字 fallback，圆形和方形两种形状。"
            }

            DemoBox {
                title: "尺寸".to_string(),
                description: Some("Sm 28px / Md 36px / Lg 48px".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Avatar { size: Size::Sm, "A" }
                        Avatar { size: Size::Md, "B" }
                        Avatar { size: Size::Lg, "C" }
                    }
                },
                code: "Avatar { size: Size::Sm, \"A\" }\nAvatar { size: Size::Md, \"B\" }\nAvatar { size: Size::Lg, \"C\" }".to_string(),
            }

            DemoBox {
                title: "形状".to_string(),
                description: Some("默认圆形，设置 shape=\"square\" 为方形。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Avatar { size: Size::Lg, "圆" }
                        Avatar { size: Size::Lg, shape: "square".to_string(), "方" }
                    }
                },
                code: "Avatar { size: Size::Lg, \"圆\" }\nAvatar { size: Size::Lg, shape: \"square\", \"方\" }".to_string(),
            }

            DemoBox {
                title: "图片头像".to_string(),
                description: Some("通过 src 传入图片地址，图片自动裁切填满。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Avatar { size: Size::Lg, src: "https://i.pravatar.cc/96?img=1".to_string(), alt: "用户头像".to_string(), "" }
                        Avatar { size: Size::Lg, shape: "square".to_string(), src: "https://i.pravatar.cc/96?img=2".to_string(), alt: "方形头像".to_string(), "" }
                    }
                },
                code: "Avatar { size: Size::Lg, src: \"https://...\", alt: \"用户头像\" }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Avatar Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("src", "String", "\"\"", "图片地址"),
                ("alt", "String", "\"\"", "替代文字"),
                ("size", "Size", "Md", "头像尺寸"),
                ("shape", "String", "\"circle\"", "形状（circle / square）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "文字 fallback 内容"),
            ]}
        }

        // ════════════════════════════════════════
        // Progress 进度条
        // ════════════════════════════════════════
        div { id: "progress", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Progress 进度条"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "进度条用于展示任务完成进度，支持颜色自定义和百分比文字。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 percent 设置 0~100 的进度值。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",
                        Progress { percent: 20.0 }
                        Progress { percent: 60.0, color: "var(--ctrl-success)".to_string() }
                        Progress { percent: 90.0, color: "var(--ctrl-warning)".to_string(), show_text: true }
                    }
                },
                code: "Progress { percent: 20.0 }\nProgress { percent: 60.0, color: \"var(--ctrl-success)\" }\nProgress { percent: 90.0, show_text: true }".to_string(),
            }

            DemoBox {
                title: "动态进度".to_string(),
                description: Some("点击按钮实时调整进度值。".to_string()),
                demo: rsx! { ProgressDynamicDemo {} },
                code: "let mut percent = use_signal(|| 30.0);\n\nProgress { percent: percent(), show_text: true }\nButton { onclick: move |_| percent.set((percent() + 10.0).min(100.0)), \"+10\" }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Progress Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("percent", "f64", "0.0", "进度值 0~100"),
                ("color", "String", "var(--ctrl-primary)", "进度条颜色"),
                ("show_text", "bool", "false", "是否显示百分比文字"),
                ("height", "u32", "8", "进度条高度(px)"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }

        // ════════════════════════════════════════
        // Tooltip 气泡提示
        // ════════════════════════════════════════
        div { id: "tooltip", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Tooltip 气泡提示"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "鼠标悬浮时显示简短提示文字，支持上下左右四个方向。"
            }

            DemoBox {
                title: "位置方向".to_string(),
                description: Some("通过 placement 控制气泡弹出方向。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; flex-wrap: wrap;",
                        Tooltip { content: "这是一段提示文字".to_string(), placement: "top".to_string(),
                            Button { variant: Variant::Outline, size: Size::Sm, "Top" }
                        }
                        Tooltip { content: "底部弹出的提示".to_string(), placement: "bottom".to_string(),
                            Button { variant: Variant::Outline, size: Size::Sm, "Bottom" }
                        }
                        Tooltip { content: "左侧提示".to_string(), placement: "left".to_string(),
                            Button { variant: Variant::Outline, size: Size::Sm, "Left" }
                        }
                        Tooltip { content: "右侧提示".to_string(), placement: "right".to_string(),
                            Button { variant: Variant::Outline, size: Size::Sm, "Right" }
                        }
                    }
                },
                code: "Tooltip { content: \"提示文字\".into(), placement: \"top\",\n    Button { \"Top\" }\n}".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Tooltip Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("content", "String", "\"\"", "提示文字"),
                ("placement", "String", "\"top\"", "弹出方向（top / bottom / left / right）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "触发元素"),
            ]}
        }

        // ════════════════════════════════════════
        // Tabs 标签页
        // ════════════════════════════════════════
        div { id: "tabs", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Tabs 标签页"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "标签页用于在同一区域内切换显示不同内容。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("使用 TabNav + TabContent 组合，通过 active 控制当前标签。".to_string()),
                demo: rsx! { TabsBasicDemo {} },
                code: "let mut active = use_signal(|| \"tab1\".to_string());\nlet items = vec![\n    (\"tab1\", \"标签一\", false),\n    (\"tab2\", \"标签二\", false),\n];\n\nTabNav { items, active: active(), onchange: move |v| active.set(v) }\nTabContent { div { \"当前: {active()}\" } }".to_string(),
            }

            DemoBox {
                title: "禁用标签".to_string(),
                description: Some("在 items 中设置第三个元素为 true 可禁用某标签。".to_string()),
                demo: rsx! { TabsDisabledDemo {} },
                code: "let items = vec![\n    (\"d1\", \"可用\", false),\n    (\"d2\", \"禁用\", true),\n];".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "TabNav Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("items", "Vec<(String, String, bool)>", "[]", "标签项（key, title, disabled）"),
                ("active", "String", "\"0\"", "当前激活的 key"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("onchange", "Option<EventHandler<String>>", "None", "切换回调"),
            ]}
        }

        // ════════════════════════════════════════
        // Alert 警告提示
        // ════════════════════════════════════════
        div { id: "alert", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Alert 警告提示"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "用于展示重要的提示信息，支持四种类型、可关闭功能。提供两种模式：内联（Inline）嵌入文档流，全局横幅（Banner）fixed 定位浮在页面顶部。"
            }

            DemoBox {
                title: "全局顶部横幅（点击触发）".to_string(),
                description: Some("设置 mode: Banner，Alert 会以 fixed 定位浮在页面最顶部。配合 closable 和 duration 实现自动消失。".to_string()),
                demo: rsx! { AlertBannerDocsDemo {} },
                code: "let mut show = use_signal(|| false);\n\nif show() {\n    Alert {\n        r#type: AlertType::Info,\n        title: \"系统维护通知\",\n        description: \"今晚 22:00 - 23:00 将进行系统升级\",\n        mode: AlertMode::Banner,\n        closable: true,\n        duration: 5000,\n    }\n}\n\nButton { onclick: move |_| show.set(true), \"显示横幅\" }".to_string(),
            }

            DemoBox {
                title: "内联提示（始终显示）".to_string(),
                description: Some("默认模式，嵌入文档流，适合表单提示、卡片内警告。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 480px;",
                        Alert { r#type: AlertType::Info, title: "系统通知".to_string(), description: "新版本已发布，建议立即更新。".to_string() }
                        Alert { r#type: AlertType::Success, title: "保存成功".to_string(), description: "您的数据已成功保存到服务器。".to_string(), closable: true }
                        Alert { r#type: AlertType::Warning, description: "当前网络不稳定，部分功能可能受限。".to_string() }
                        Alert { r#type: AlertType::Error, title: "加载失败".to_string(), description: "无法加载用户列表，请刷新页面重试。".to_string(), closable: true }
                    }
                },
                code: "Alert { r#type: AlertType::Info, title: \"系统通知\", description: \"...\" }\nAlert { r#type: AlertType::Success, title: \"保存成功\", closable: true }\nAlert { r#type: AlertType::Warning, description: \"网络不稳定\" }\nAlert { r#type: AlertType::Error, title: \"加载失败\", closable: true }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Alert Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("type", "AlertType", "Info", "提示类型（Info / Success / Warning / Error）"),
                ("title", "String", "\"\"", "标题"),
                ("description", "String", "\"\"", "描述内容"),
                ("closable", "bool", "false", "是否可关闭"),
                ("show_icon", "bool", "true", "是否显示图标"),
                ("mode", "AlertMode", "Inline", "显示模式（Inline 内联 / Banner 全局横幅）"),
                ("duration", "u64", "0", "自动关闭时间(ms)，仅 Banner 模式有效，0 表示不自动关闭"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }

        // ════════════════════════════════════════
        // Breadcrumb 面包屑
        // ════════════════════════════════════════
        div { id: "breadcrumb", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Breadcrumb 面包屑"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "面包屑用于展示当前页面的层级路径，帮助用户了解当前位置。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("最后一项不设 href 即为当前页（不可点击）。".to_string()),
                demo: rsx! {
                    Breadcrumb {
                        BreadcrumbItem { href: "#".to_string(), "首页" }
                        BreadcrumbItem { href: "#".to_string(), "组件" }
                        BreadcrumbItem { "面包屑" }
                    }
                },
                code: "Breadcrumb {\n    BreadcrumbItem { href: \"#\", \"首页\" }\n    BreadcrumbItem { href: \"#\", \"组件\" }\n    BreadcrumbItem { \"面包屑\" }\n}".to_string(),
            }

            DemoBox {
                title: "自定义分隔符".to_string(),
                description: Some("通过 separator 属性设置分隔符。".to_string()),
                demo: rsx! {
                    Breadcrumb { separator: ">".to_string(),
                        BreadcrumbItem { href: "#".to_string(), "Home" }
                        BreadcrumbItem { href: "#".to_string(), "Library" }
                        BreadcrumbItem { "Data" }
                    }
                },
                code: "Breadcrumb { separator: \">\", ... }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Breadcrumb Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("separator", "String", "\"/\"", "分隔符"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "BreadcrumbItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("href", "String", "\"\"", "链接地址（空则不可点击）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "子元素"),
            ]}
        }

        // ════════════════════════════════════════
        // Pagination 分页
        // ════════════════════════════════════════
        div { id: "pagination", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Pagination 分页"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "分页用于长列表的数据分页导航。"
            }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 total 和 page_size 计算分页，current/onchange 控制当前页。".to_string()),
                demo: rsx! { PaginationDocsDemo {} },
                code: "let mut page = use_signal(|| 1u32);\n\nPagination {\n    current: page(),\n    total: 50,\n    page_size: 10,\n    onchange: move |v| page.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "大数据量".to_string(),
                description: Some("total 为 200 时自动计算出 20 页。".to_string()),
                demo: rsx! {
                    Pagination { total: 200, page_size: 10 }
                },
                code: "Pagination { total: 200, page_size: 10 }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Pagination Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("current", "u32", "1", "当前页码"),
                ("total", "u32", "0", "总条数"),
                ("page_size", "u32", "10", "每页条数"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("onchange", "Option<EventHandler<u32>>", "None", "页码切换回调"),
            ]}
        }

        // ════════════════════════════════════════
        // Message 全局提示
        // ════════════════════════════════════════
        div { id: "message", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "Message 全局提示"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;",
                "轻量级全局提示，以 fixed 定位弹出在页面指定位置，展示操作结果或系统通知。支持自动消失和多位置弹出。"
            }

            DemoBox {
                title: "点击触发（不同类型）".to_string(),
                description: Some("点击按钮后，Message 会以 fixed 定位弹出在页面顶部居中，3 秒后自动消失。".to_string()),
                demo: rsx! { MessageTriggerDocsDemo {} },
                code: "let mut trigger = use_signal(|| 0u32);\nlet mut msg_type = use_signal(|| MessageType::Info);\nlet mut msg_content = use_signal(|| String::new());\n\n// 点击时设置类型和内容，trigger +1 触发重新渲染\nButton { onclick: move |_| {\n    msg_type.set(MessageType::Success);\n    msg_content.set(\"操作成功！\".into());\n    trigger.set(trigger() + 1);\n}, \"保存\" }\n\nif trigger() > 0 {\n    Message {\n        r#type: msg_type(),\n        content: msg_content(),\n        duration: 3000,\n    }\n}".to_string(),
            }

            DemoBox {
                title: "不同位置".to_string(),
                description: Some("通过 placement 控制弹出位置：Top（顶部居中）、TopRight（顶部靠右）、TopLeft（顶部靠左）、Bottom（底部居中）。".to_string()),
                demo: rsx! { MessagePositionDocsDemo {} },
                code: "let mut placement = use_signal(|| MessagePlacement::Top);\n\nButton { onclick: move |_| placement.set(MessagePlacement::TopRight), \"右上角\" }\n\nMessage { r#type: MessageType::Success, content: \"已复制\", placement: placement(), duration: 3000 }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Message Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("type", "MessageType", "Info", "消息类型（Info / Success / Warning / Error）"),
                ("content", "String", "\"\"", "消息文字"),
                ("visible", "bool", "true", "是否显示"),
                ("placement", "MessagePlacement", "Top", "弹出位置（Top / TopRight / TopLeft / Bottom）"),
                ("duration", "u64", "3000", "自动消失时间(ms)，0 表示不自动消失"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }
    }
}

// ── Button 交互演示 ──

#[component]
#[allow(non_snake_case)]
fn ButtonInteract() -> Element {
    let mut count = use_signal(|| 0);
    let mut disabled = use_signal(|| false);

    rsx! {
        div { style: "display: flex; gap: 12px; flex-wrap: wrap; align-items: center;",
            Button {
                variant: Variant::Primary,
                disabled: disabled(),
                onclick: move |_| count.set(count() + 1),
                "点击次数: {count()}"
            }
            Button {
                variant: Variant::Ghost,
                onclick: move |_| disabled.set(!disabled()),
                if disabled() { "恢复" } else { "禁用" }
            }
            Button {
                variant: Variant::Outline,
                onclick: move |_| count.set(0),
                "重置"
            }
        }
    }
}

// ── Progress 动态演示 ──

#[component]
#[allow(non_snake_case)]
fn ProgressDynamicDemo() -> Element {
    let mut percent = use_signal(|| 30.0);
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 400px;",
            Progress { percent: percent(), color: "var(--ctrl-primary)".to_string(), show_text: true }
            div { style: "display: flex; gap: 8px;",
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| { let p = percent(); if p > 0.0 { percent.set((p - 10.0).max(0.0)); } }, "-10" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| { let p = percent(); if p < 100.0 { percent.set((p + 10.0).min(100.0)); } }, "+10" }
            }
        }
    }
}

// ── Tabs 基本演示 ──

#[component]
#[allow(non_snake_case)]
fn TabsBasicDemo() -> Element {
    let mut active = use_signal(|| "tab1".to_string());
    let items = vec![
        ("tab1".to_string(), "标签一".to_string(), false),
        ("tab2".to_string(), "标签二".to_string(), false),
        ("tab3".to_string(), "标签三".to_string(), false),
    ];
    rsx! {
        div { style: "width: 100%; max-width: 480px;",
            TabNav { items: items, active: active(), onchange: move |v| active.set(v) }
            TabContent {
                div { style: "padding: 16px 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-md);",
                    if active() == "tab1" { "这是标签一的内容区域。" }
                    else if active() == "tab2" { "这是标签二的内容区域。" }
                    else { "这是标签三的内容区域。" }
                }
            }
        }
    }
}

// ── Tabs 禁用演示 ──

#[component]
#[allow(non_snake_case)]
fn TabsDisabledDemo() -> Element {
    let mut active = use_signal(|| "d1".to_string());
    let items = vec![
        ("d1".to_string(), "可用".to_string(), false),
        ("d2".to_string(), "禁用".to_string(), true),
        ("d3".to_string(), "可用".to_string(), false),
    ];
    rsx! {
        div { style: "width: 100%; max-width: 480px;",
            TabNav { items: items, active: active(), onchange: move |v| active.set(v) }
            TabContent {
                div { style: "padding: 16px 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-md);",
                    "当前选中: {active()}"
                }
            }
        }
    }
}

// ── Pagination 演示 ──

#[component]
#[allow(non_snake_case)]
fn PaginationDocsDemo() -> Element {
    let mut page = use_signal(|| 1u32);
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 12px;",
            Pagination { current: page(), total: 50, page_size: 10, onchange: move |v| page.set(v) }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "当前第 {page()} 页" }
        }
    }
}

// ── Alert 全局横幅演示 ──

#[component]
#[allow(non_snake_case)]
fn AlertBannerDocsDemo() -> Element {
    const MAX_ALERTS: usize = 5;

    let mut alerts = use_signal(|| Vec::<(u32, bool)>::new()); // (id, closing)
    let mut next_id = use_signal(|| 0u32);

    let mut add_alert = move || {
        let id = next_id();
        next_id.set(id + 1);

        let mut list = alerts.write();
        let active_count = list.iter().filter(|(_, cl)| !cl).count();
        if active_count >= MAX_ALERTS {
            if let Some(oldest) = list.iter_mut().find(|(_, cl)| !*cl) {
                oldest.1 = true;
            }
        }
        list.push((id, false));
    };

    rsx! {
        AlertBannerContainer {
            for (id, closing) in alerts().iter() {
                {
                    let alert_id = *id;
                    let cl = *closing;
                    rsx! {
                        Alert {
                            key: "{alert_id}",
                            r#type: AlertType::Warning,
                            title: "存储空间不足".to_string(),
                            description: "您的存储空间已使用 95%，请尽快清理文件。".to_string(),
                            mode: AlertMode::Banner,
                            closable: true,
                            closing: cl,
                            duration: 5000,
                            onclose: move |_| alerts.write().retain(|(aid, _)| *aid != alert_id),
                        }
                    }
                }
            }
        }
        div {
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| add_alert(), "显示全局横幅" }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin-left: 12px;",
                "最多同时显示 5 条，超出后从最旧的开始依次退出"
            }
        }
    }
}

// ── Message 点击触发演示 ──

#[component]
#[allow(non_snake_case)]
fn MessageTriggerDocsDemo() -> Element {
    const MAX_MESSAGES: usize = 5;

    type MsgItem = (u32, MessageType, String, bool); // id, type, content, closing

    let mut messages = use_signal(|| Vec::<MsgItem>::new());
    let mut next_id = use_signal(|| 0u32);

    let mut add_message = move |t: MessageType, c: &str| {
        let id = next_id();
        next_id.set(id + 1);

        let mut list = messages.write();
        let active_count = list.iter().filter(|(_, _, _, cl)| !cl).count();
        if active_count >= MAX_MESSAGES {
            if let Some(oldest) = list.iter_mut().find(|(_, _, _, cl)| !*cl) {
                oldest.3 = true;
            }
        }
        list.push((id, t, c.to_string(), false));
    };

    rsx! {
        MessageContainer {
            for (id, m_type, content, closing) in messages().iter() {
                {
                    let msg_id = *id;
                    let t = m_type.clone();
                    let c2 = content.clone();
                    let cl = *closing;
                    rsx! {
                        Message {
                            key: "{msg_id}",
                            r#type: t,
                            content: c2,
                            closing: cl,
                            duration: 3000,
                            onclose: move |_| messages.write().retain(|(mid, _, _, _)| *mid != msg_id),
                        }
                    }
                }
            }
        }
        div { style: "display: flex; flex-direction: column; gap: 12px;",
            div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| add_message(MessageType::Info, "已复制到剪贴板"), "Info" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| add_message(MessageType::Success, "保存成功！"), "Success" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| add_message(MessageType::Warning, "文件格式不支持"), "Warning" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| add_message(MessageType::Error, "网络连接超时"), "Error" }
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "当前消息数量: {messages().len()}（上限 {MAX_MESSAGES}）" }
        }
    }
}

// ── Message 位置演示 ──

#[component]
#[allow(non_snake_case)]
fn MessagePositionDocsDemo() -> Element {
    const MAX_MESSAGES: usize = 5;

    let mut messages = use_signal(|| Vec::<(u32, bool)>::new()); // (id, closing)
    let mut next_id = use_signal(|| 0u32);
    let mut pos_placement = use_signal(|| MessagePlacement::Top);

    let mut add_message = move || {
        let id = next_id();
        next_id.set(id + 1);

        let mut list = messages.write();
        let active_count = list.iter().filter(|(_, cl)| !cl).count();
        if active_count >= MAX_MESSAGES {
            if let Some(oldest) = list.iter_mut().find(|(_, cl)| !*cl) {
                oldest.1 = true;
            }
        }
        list.push((id, false));
    };

    rsx! {
        MessageContainer {
            placement: pos_placement(),
            for (id, closing) in messages().iter() {
                {
                    let msg_id = *id;
                    let cl = *closing;
                    rsx! {
                        Message {
                            key: "{msg_id}",
                            r#type: MessageType::Success,
                            content: "消息已发送".to_string(),
                            closing: cl,
                            duration: 3000,
                            onclose: move |_| messages.write().retain(|(mid, _)| *mid != msg_id),
                        }
                    }
                }
            }
        }
        div { style: "display: flex; flex-direction: column; gap: 12px;",
            div { style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",
                Button { variant: if pos_placement() == MessagePlacement::Top { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::Top), "Top" }
                Button { variant: if pos_placement() == MessagePlacement::TopRight { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::TopRight), "TopRight" }
                Button { variant: if pos_placement() == MessagePlacement::TopLeft { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::TopLeft), "TopLeft" }
                Button { variant: if pos_placement() == MessagePlacement::Bottom { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::Bottom), "Bottom" }
            }
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| add_message(), "发送消息" }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "当前消息数量: {messages().len()}（上限 {MAX_MESSAGES}）" }
        }
    }
}

// ── Input 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicInputDemo() -> Element {
    let mut value = use_signal(|| String::new());

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 360px;",
            Input {
                placeholder: "请输入内容",
                value: value(),
                oninput: move |evt: FormEvent| value.set(evt.value()),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前输入: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;", "{value()}" }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn FormValidationDemo() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut errors = use_signal(|| (false, false));
    let mut submitted = use_signal(|| false);

    let validate = move |_| {
        let e = (email().trim().is_empty(), password().is_empty());
        errors.set(e);
        if !e.0 && !e.1 { submitted.set(true); }
    };

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 16px; max-width: 360px;",
            div { style: "display: flex; flex-direction: column; gap: 4px;",
                label { style: "font-size: var(--ctrl-font-size-sm); font-weight: 500;", "邮箱" }
                Input {
                    placeholder: "请输入邮箱",
                    value: email(), error: errors().0,
                    oninput: move |evt: FormEvent| {
                        email.set(evt.value());
                        submitted.set(false);
                        if errors().0 { errors.set((false, errors().1)); }
                    },
                }
                if errors().0 { span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-danger);", "请输入邮箱" } }
            }
            div { style: "display: flex; flex-direction: column; gap: 4px;",
                label { style: "font-size: var(--ctrl-font-size-sm); font-weight: 500;", "密码" }
                Input {
                    r#type: "password".to_string(),
                    placeholder: "请输入密码",
                    value: password(), error: errors().1,
                    oninput: move |evt: FormEvent| {
                        password.set(evt.value());
                        submitted.set(false);
                        if errors().1 { errors.set((errors().0, false)); }
                    },
                }
                if errors().1 { span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-danger);", "请输入密码" } }
            }
            Button { variant: Variant::Primary, block: true, onclick: validate, "提交" }
            if submitted() {
                div {
                    style: "padding: 12px; background: var(--ctrl-primary-light); border-radius: var(--ctrl-radius-sm); font-size: var(--ctrl-font-size-sm); color: var(--ctrl-primary);",
                    "验证通过！邮箱: {email()} , 密码: {password()}"
                }
            }
        }
    }
}

// ── Switch 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicSwitchDemo() -> Element {
    let mut on = use_signal(|| false);

    rsx! {
        div { style: "display: flex; gap: 16px; align-items: center;",
            Switch {
                checked: on(),
                onchange: move |v| on.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-md); color: var(--ctrl-text);",
                if on() { "已开启" } else { "已关闭" }
            }
        }
    }
}

// ── Checkbox 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicCheckboxDemo() -> Element {
    let mut checked = use_signal(|| false);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px;",
            Checkbox {
                checked: checked(),
                label: "同意使用协议".to_string(),
                onchange: move |v| checked.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前状态: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;",
                    if checked() { "已同意" } else { "未同意" }
                }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn CheckAllDemo() -> Element {
    let items: Vec<&'static str> = vec!["选项 A", "选项 B", "选项 C"];
    let items_len = items.len();
    let mut checked = use_signal(|| vec![false; items_len]);

    let all = checked().iter().all(|&c| c);
    let some = checked().iter().any(|&c| c);
    let indet = some && !all;

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px;",
            Checkbox {
                checked: all,
                indeterminate: indet,
                label: "全选".to_string(),
                onchange: move |v| checked.set(vec![v; items_len]),
            }
            div { style: "height: 1px; background: var(--ctrl-border); margin: 4px 0; width: 100%;" }
            {items.iter().enumerate().map(|(i, item)| {
                let idx = i;
                let label = format!("{}", item);
                rsx! {
                    Checkbox {
                        key: "{idx}",
                        checked: checked()[idx],
                        label: label,
                        onchange: move |v| {
                            let mut c = checked();
                            c[idx] = v;
                            checked.set(c);
                        },
                    }
                }
            })}
        }
    }
}

// ── Radio 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicRadioDemo() -> Element {
    let mut selected = use_signal(|| "a".to_string());

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px;",
            Radio {
                value: "a".to_string(),
                label: "选项 A".to_string(),
                checked: selected() == "a",
                onchange: move |v| selected.set(v),
            }
            Radio {
                value: "b".to_string(),
                label: "选项 B".to_string(),
                checked: selected() == "b",
                onchange: move |v| selected.set(v),
            }
            Radio {
                value: "c".to_string(),
                label: "选项 C（禁用）".to_string(),
                checked: selected() == "c",
                disabled: true,
                onchange: move |_| {},
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin-top: 4px;",
                "当前选中: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;", "{selected()}" }
            }
        }
    }
}

// ── Select 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicSelectDemo() -> Element {
    let mut value = use_signal(|| String::new());

    let options = vec![
        ("a".to_string(), "选项 A".to_string(), false),
        ("b".to_string(), "选项 B".to_string(), false),
        ("c".to_string(), "选项 C".to_string(), true),
    ];

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 240px;",
            Select {
                options: options,
                placeholder: "请选择".to_string(),
                value: value(),
                onchange: move |v| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前选中: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;",
                    if value().is_empty() { "无" } else { "{value()}" }
                }
            }
        }
    }
}

// ── Dialog 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicDialogDemo() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div {
            Button { variant: Variant::Primary, onclick: move |_| visible.set(true), "打开对话框" }
            Dialog {
                visible: visible(),
                title: "提示".to_string(),
                onclose: move |_| visible.set(false),
                p { style: "margin: 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-md);", "这是一条提示信息。对话框通过 visible 控制显示/隐藏，点击遮罩或右上角关闭按钮均可关闭。" }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn FooterDialogDemo() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div {
            Button { variant: Variant::Outline, onclick: move |_| visible.set(true), "确认对话框" }
            Dialog {
                visible: visible(),
                title: "确认操作".to_string(),
                onclose: move |_| visible.set(false),
                footer: rsx! {
                    Button { variant: Variant::Ghost, onclick: move |_| visible.set(false), "取消" }
                    Button {
                        variant: Variant::Primary,
                        onclick: move |_| visible.set(false),
                        "确定"
                    }
                },
                div { style: "display: flex; flex-direction: column; gap: 8px;",
                    p { style: "margin: 0; font-size: var(--ctrl-font-size-md); color: var(--ctrl-text);", "确定要执行此操作吗？" }
                    p { style: "margin: 0; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "此操作不可撤销，请谨慎操作。" }
                }
            }
        }
    }
}

// ── Props 表格组件 ──

#[component]
#[allow(non_snake_case)]
fn PropsTable(headers: Vec<String>, rows: Vec<(&'static str, &'static str, &'static str, &'static str)>) -> Element {
    rsx! {
        div { style: "overflow-x: auto; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); margin-bottom: 40px;",
            table { style: "width: 100%; border-collapse: collapse; font-size: var(--ctrl-font-size-md);",
                thead {
                    tr { style: "background: var(--ctrl-bg-secondary);",
                        {headers.iter().map(|h| rsx! {
                            th { style: "padding: 10px 16px; text-align: left; font-weight: 600; color: var(--ctrl-text); border-bottom: 1px solid var(--ctrl-border);", "{h}" }
                        })}
                    }
                }
                tbody {
                    {rows.iter().map(|(name, type_str, default, desc)| rsx! {
                        tr { style: "border-bottom: 1px solid var(--ctrl-border);",
                            td { style: "padding: 10px 16px; color: var(--ctrl-primary); font-weight: 500; font-family: monospace;", "{name}" }
                            td { style: "padding: 10px 16px; color: var(--ctrl-text-secondary); font-family: monospace;", "{type_str}" }
                            td { style: "padding: 10px 16px; color: var(--ctrl-text-secondary); font-family: monospace;", "{default}" }
                            td { style: "padding: 10px 16px; color: var(--ctrl-text);", "{desc}" }
                        }
                    })}
                }
            }
        }
    }
}
