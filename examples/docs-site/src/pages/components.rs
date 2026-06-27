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
                ("loading", "bool", "false", "是否加载中"),
                ("block", "bool", "false", "是否块级"),
                ("r#type", "String", "\"button\"", "原生 button type 属性"),
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
                ("readonly", "bool", "false", "是否只读"),
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
        // Slider 滑块
        // ════════════════════════════════════════
        div { id: "slider", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Slider 滑块" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "滑块用于在数值范围内进行选择。支持水平/垂直方向、步长、刻度标记和禁用状态。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 onchange 管理滑块值。".to_string()),
                demo: rsx! { BasicSliderDemo {} },
                code: "let mut value = use_signal(|| 50);\n\nSlider {\n    value: value(),\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "自定义范围".to_string(),
                description: Some("设置 min、max 和 step 属性。".to_string()),
                demo: rsx! { CustomRangeSliderDemo {} },
                code: "Slider { value: 30, min: 0, max: 100, step: 5, show_label: true }\nSlider { value: 0, min: -50, max: 50, step: 10, show_label: true }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("设置 disabled 为 true 禁用滑块。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 24px; max-width: 400px;",
                        Slider { value: 30, disabled: true }
                        Slider { value: 70, disabled: true }
                    }
                },
                code: "Slider { value: 30, disabled: true }".to_string(),
            }

            DemoBox {
                title: "显示标签".to_string(),
                description: Some("设置 show_label 显示当前值和最大值。".to_string()),
                demo: rsx! {
                    div { style: "max-width: 400px;", Slider { value: 60, show_label: true } }
                },
                code: "Slider { value: 60, show_label: true }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Slider Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("value", "i32", "0", "当前值"),
                ("min", "i32", "0", "最小值"),
                ("max", "i32", "100", "最大值"),
                ("step", "i32", "1", "步长"),
                ("disabled", "bool", "false", "是否禁用"),
                ("vertical", "bool", "false", "是否垂直"),
                ("marks", "bool", "false", "是否显示刻度"),
                ("show_label", "bool", "false", "是否显示数值标签"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<i32>>", "None", "值变化回调"),
            ]}
        }

        // ════════════════════════════════════════
        // Rate 评分
        // ════════════════════════════════════════
        div { id: "rate", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Rate 评分" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "评分组件用于收集用户对内容的评价。支持半星、自定义图标和只读状态。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 onchange 管理评分。".to_string()),
                demo: rsx! { BasicRateDemo {} },
                code: "let mut value = use_signal(|| 3.0);\n\nRate {\n    value: value(),\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "半星模式".to_string(),
                description: Some("设置 allow_half 允许半星评分。鼠标悬停在星的左半区选半星，右半区选整星。".to_string()),
                demo: rsx! { HalfRateDemo {} },
                code: "let mut value = use_signal(|| 2.5);\n\nRate {\n    value: value(),\n    allow_half: true,\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "禁用/只读".to_string(),
                description: Some("禁用或只读状态下的评分。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        Rate { value: 4.0, disabled: true }
                        Rate { value: 5.0, disabled: true }
                    }
                },
                code: "Rate { value: 4, disabled: true }\nRate { value: 5, disabled: true }".to_string(),
            }

            DemoBox {
                title: "显示文本".to_string(),
                description: Some("设置 show_text 显示评论文本。".to_string()),
                demo: rsx! { Rate { value: 4.0, show_text: true } },
                code: "Rate { value: 4, show_text: true }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Rate Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("value", "f64", "0.0", "当前评分值"),
                ("count", "i32", "5", "星星数量"),
                ("allow_half", "bool", "false", "是否允许半星"),
                ("disabled", "bool", "false", "是否禁用"),
                ("show_text", "bool", "false", "是否显示评论文本"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<f64>>", "None", "值变化回调"),
            ]}
        }

        // ════════════════════════════════════════
        // Image 图片
        // ════════════════════════════════════════
        div { id: "image", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Image 图片" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "图片组件支持懒加载、占位图、加载失败回退和预览功能。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 src 属性指定图片地址。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 12px;",
                        Image { src: "https://picsum.photos/200/150".to_string(), width: "200px".to_string(), height: "150px".to_string() }
                        Image { src: "https://picsum.photos/150/150".to_string(), width: "150px".to_string(), height: "150px".to_string() }
                    }
                },
                code: "Image { src: \"https://picsum.photos/200/150\".to_string(), width: \"200px\".to_string(), height: \"150px\".to_string() }".to_string(),
            }

            DemoBox {
                title: "加载失败".to_string(),
                description: Some("设置 fallback 提供加载失败时的占位图。".to_string()),
                demo: rsx! {
                    Image { src: "https://invalid.url/image.jpg".to_string(), width: "200px".to_string(), height: "150px".to_string(), fallback: "https://picsum.photos/200/150".to_string() }
                },
                code: "Image { src: \"...\".to_string(), fallback: \"https://picsum.photos/200/150\".to_string() }".to_string(),
            }

            DemoBox {
                title: "预览模式".to_string(),
                description: Some("设置 preview 为 true 允许点击预览大图。".to_string()),
                demo: rsx! {
                    Image { src: "https://picsum.photos/200/150".to_string(), width: "200px".to_string(), height: "150px".to_string(), preview: true }
                },
                code: "Image { src: \"...\".to_string(), preview: true }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Image Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("src", "String", "\"\"", "图片地址"),
                ("alt", "String", "\"\"", "替代文本"),
                ("width", "String", "\"\"", "宽度"),
                ("height", "String", "\"\"", "高度"),
                ("fallback", "String", "\"\"", "加载失败时的占位图"),
                ("preview", "bool", "false", "是否可预览"),
                ("lazy", "bool", "true", "是否懒加载"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
            ]}
        }

        // ════════════════════════════════════════
        // Space 间距
        // ════════════════════════════════════════
        div { id: "space", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Space 间距" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "间距组件用于设置子元素之间的间距。支持水平和垂直方向，提供三种预设尺寸。" }

            DemoBox {
                title: "水平间距".to_string(),
                description: Some("默认 direction 为 horizontal。".to_string()),
                demo: rsx! {
                    Space {
                        Button { variant: Variant::Primary, "按钮 1" }
                        Button { variant: Variant::Outline, "按钮 2" }
                        Button { variant: Variant::Ghost, "按钮 3" }
                    }
                },
                code: "Space {\n    Button { variant: Variant::Primary, \"按钮 1\" }\n    Button { variant: Variant::Outline, \"按钮 2\" }\n    Button { variant: Variant::Ghost, \"按钮 3\" }\n}".to_string(),
            }

            DemoBox {
                title: "垂直间距".to_string(),
                description: Some("设置 direction 为 vertical。".to_string()),
                demo: rsx! {
                    Space { direction: "vertical".to_string(),
                        Button { variant: Variant::Primary, block: true, "按钮 1" }
                        Button { variant: Variant::Outline, block: true, "按钮 2" }
                        Button { variant: Variant::Ghost, block: true, "按钮 3" }
                    }
                },
                code: "Space { direction: \"vertical\".to_string(),\n    Button { variant: Variant::Primary, block: true, \"按钮 1\" }\n    Button { variant: Variant::Outline, block: true, \"按钮 2\" }\n}".to_string(),
            }

            DemoBox {
                title: "自定义间距".to_string(),
                description: Some("通过 gap 属性设置间距大小 (sm / md / lg)。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        Space { gap: "sm".to_string(), Tag { "小" } Tag { "间" } Tag { "距" } }
                        Space { gap: "md".to_string(), Tag { "中" } Tag { "间" } Tag { "距" } }
                        Space { gap: "lg".to_string(), Tag { "大" } Tag { "间" } Tag { "距" } }
                    }
                },
                code: "Space { gap: \"sm\".to_string(), ... }\nSpace { gap: \"md\".to_string(), ... }\nSpace { gap: \"lg\".to_string(), ... }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Space Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("direction", "String", "\"horizontal\"", "排列方向：horizontal / vertical"),
                ("size", "String", "\"md\"", "间距大小：sm / md / lg"),
                ("align", "String", "\"center\"", "对齐方式：start / center / end"),
                ("wrap", "bool", "false", "是否换行"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "子元素"),
            ]}
        }

        // ════════════════════════════════════════
        // Segmented 分段控制器
        // ════════════════════════════════════════
        div { id: "segmented", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Segmented 分段控制器" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "分段控制器用于在多个选项间切换，类似 iOS 的 Segmented Control。支持三种尺寸和禁用状态。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 onchange 管理选中项。".to_string()),
                demo: rsx! { BasicSegmentedDemo {} },
                code: "let mut value = use_signal(|| String::new());\nlet options = vec![(\"日\".to_string(), \"day\".to_string()), (\"周\".to_string(), \"week\".to_string())];\n\nSegmented {\n    options: options,\n    value: value(),\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "尺寸".to_string(),
                description: Some("Sm / Md / Lg 三种尺寸。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        Segmented { options: vec![("小".to_string(), "sm".to_string()), ("中".to_string(), "md".to_string()), ("大".to_string(), "lg".to_string())], size: Size::Sm }
                        Segmented { options: vec![("小".to_string(), "sm".to_string()), ("中".to_string(), "md".to_string()), ("大".to_string(), "lg".to_string())], size: Size::Md }
                        Segmented { options: vec![("小".to_string(), "sm".to_string()), ("中".to_string(), "md".to_string()), ("大".to_string(), "lg".to_string())], size: Size::Lg }
                    }
                },
                code: "Segmented { options: vec![...], size: Size::Sm }\nSegmented { options: vec![...], size: Size::Md }\nSegmented { options: vec![...], size: Size::Lg }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("整体禁用或禁用单个选项。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        Segmented { options: vec![("日".to_string(), "day".to_string()), ("周".to_string(), "week".to_string()), ("月".to_string(), "month".to_string())], value: "周".to_string(), disabled: true }
                    }
                },
                code: "Segmented { options: vec![...], disabled: true }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Segmented Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("options", "Vec<String>", "[]", "选项列表"),
                ("value", "String", "\"\"", "当前选中值"),
                ("size", "Size", "Md", "组件尺寸"),
                ("disabled", "bool", "false", "是否禁用"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<String>>", "None", "值变化回调"),
            ]}
        }

        // ════════════════════════════════════════
        // InputNumber 数字输入框
        // ════════════════════════════════════════
        div { id: "input_number", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "InputNumber 数字输入框" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "数字输入框用于精确数值输入，内置增减按钮，支持范围限制和步长控制。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 onchange 管理数值。".to_string()),
                demo: rsx! { BasicInputNumberDemo {} },
                code: "let mut value = use_signal(|| 0);\n\nInputNumber {\n    value: value(),\n    onchange: move |v| value.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "范围限制".to_string(),
                description: Some("设置 min、max 和 step 属性。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 200px;",
                        InputNumber { value: 5, min: 0, max: 10 }
                        InputNumber { value: 50, min: 0, max: 100, step: 5 }
                        InputNumber { value: 0, min: -10, max: 10 }
                    }
                },
                code: "InputNumber { value: 5, min: 0, max: 10 }\nInputNumber { value: 50, min: 0, max: 100, step: 5 }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("禁用时不可交互。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 200px;",
                        InputNumber { value: 42, disabled: true }
                    }
                },
                code: "InputNumber { value: 42, disabled: true }".to_string(),
            }

            DemoBox {
                title: "三种尺寸".to_string(),
                description: Some("Sm / Md / Lg。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 200px;",
                        InputNumber { value: 1, size: Size::Sm }
                        InputNumber { value: 2, size: Size::Md }
                        InputNumber { value: 3, size: Size::Lg }
                    }
                },
                code: "InputNumber { value: 1, size: Size::Sm }\nInputNumber { value: 2, size: Size::Md }\nInputNumber { value: 3, size: Size::Lg }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "InputNumber Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("value", "i32", "0", "当前值"),
                ("min", "i32", "0", "最小值"),
                ("max", "i32", "100", "最大值"),
                ("step", "i32", "1", "步长"),
                ("disabled", "bool", "false", "是否禁用"),
                ("size", "Size", "Md", "组件尺寸"),
                ("placeholder", "String", "\"\"", "占位文本"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<i32>>", "None", "值变化回调"),
            ]}
        }

        // ════════════════════════════════════════
        // Upload 上传
        // ════════════════════════════════════════
        div { id: "upload", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Upload 上传" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "上传组件用于文件上传，支持点击上传和拖拽上传两种模式，可展示文件列表。" }

            DemoBox {
                title: "点击上传".to_string(),
                description: Some("基本点击上传按钮。".to_string()),
                demo: rsx! { ClickUploadDemo {} },
                code: "let mut files = use_signal(|| Vec::new());\n\nUpload {\n    files: files(),\n    onchange: move |f| files.set(f),\n    Button { variant: Variant::Primary, \"选择文件\" }\n}".to_string(),
            }

            DemoBox {
                title: "拖拽上传".to_string(),
                description: Some("设置 drag 为 true 启用拖拽区域。".to_string()),
                demo: rsx! {
                    div { style: "max-width: 400px;",
                        Upload {
                            drag: true,
                            tip: "支持 JPG、PNG 格式，单文件不超过 5MB".to_string(),
                            div { style: "padding: 8px;", Button { variant: Variant::Primary, "选择文件" } }
                        }
                    }
                },
                code: "Upload {\n    drag: true,\n    tip: \"支持 JPG、PNG 格式，单文件不超过 5MB\".to_string(),\n    Button { variant: Variant::Primary, \"选择文件\" }\n}".to_string(),
            }

            DemoBox {
                title: "文件列表".to_string(),
                description: Some("展示已选文件，支持移除操作。".to_string()),
                demo: rsx! { FileListUploadDemo {} },
                code: "let mut files = use_signal(|| vec![\n    UploadFile { name: \"photo.jpg\".to_string(), size: 204800 },\n    UploadFile { name: \"doc.pdf\".to_string(), size: 1024000 },\n]);\n\nUpload {\n    files: files(),\n    onremove: move |i| { let mut f = files(); f.remove(i); files.set(f); },\n    Button { variant: Variant::Primary, \"选择文件\" }\n}".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Upload Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("files", "Vec<UploadFile>", "[]", "文件列表"),
                ("accept", "String", "\"\"", "接受的文件类型"),
                ("multiple", "bool", "false", "是否多选"),
                ("disabled", "bool", "false", "是否禁用"),
                ("drag", "bool", "false", "是否拖拽模式"),
                ("tip", "String", "\"\"", "提示文字"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<Vec<UploadFile>>>", "None", "文件选择回调"),
                ("onremove", "Option<EventHandler<usize>>", "None", "文件移除回调"),
                ("children", "Element", "—", "触发元素"),
            ]}
        }

        // ════════════════════════════════════════
        // Carousel 走马灯
        // ════════════════════════════════════════
        div { id: "carousel", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Carousel 走马灯" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "走马灯用于在有限空间内循环展示内容。支持自动播放、箭头导航和指示器。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("使用 Carousel 包裹子元素作为轮播项。".to_string()),
                demo: rsx! {
                    Carousel { height: "200px".to_string(),
                        div { style: "background: #4a90d9; color: #fff; display: flex; align-items: center; justify-content: center; font-size: 24px; width: 100%; height: 100%;", "Slide 1" }
                        div { style: "background: #27ae60; color: #fff; display: flex; align-items: center; justify-content: center; font-size: 24px; width: 100%; height: 100%;", "Slide 2" }
                        div { style: "background: #e74c3c; color: #fff; display: flex; align-items: center; justify-content: center; font-size: 24px; width: 100%; height: 100%;", "Slide 3" }
                    }
                },
                code: "Carousel { height: \"200px\".to_string(),\n    div { \"Slide 1\" }\n    div { \"Slide 2\" }\n    div { \"Slide 3\" }\n}".to_string(),
            }

            DemoBox {
                title: "不显示箭头和指示器".to_string(),
                description: Some("设置 arrows 和 dots 为 false。".to_string()),
                demo: rsx! {
                    Carousel { height: "200px".to_string(), arrows: false, dots: false,
                        div { style: "background: #4a90d9; color: #fff; display: flex; align-items: center; justify-content: center; font-size: 24px; width: 100%; height: 100%;", "Slide 1" }
                        div { style: "background: #27ae60; color: #fff; display: flex; align-items: center; justify-content: center; font-size: 24px; width: 100%; height: 100%;", "Slide 2" }
                    }
                },
                code: "Carousel { height: \"200px\".to_string(), arrows: false, dots: false, ... }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Carousel Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("autoplay", "bool", "true", "是否自动播放"),
                ("interval", "u64", "3000", "自动播放间隔（毫秒）"),
                ("arrows", "bool", "true", "是否显示箭头"),
                ("dots", "bool", "true", "是否显示指示器"),
                ("effect", "String", "\"slide\"", "过渡效果：slide / fade"),
                ("height", "String", "\"300px\"", "容器高度"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("children", "Element", "—", "轮播项"),
            ]}
        }

        // ════════════════════════════════════════
        // Form 表单
        // ════════════════════════════════════════
        div { id: "form", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Form 表单" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "表单组件用于数据录入，支持垂直、水平、内联三种布局，提供 FormItem 管理标签、帮助文本和校验信息。" }

            DemoBox {
                title: "垂直布局".to_string(),
                description: Some("默认 layout 为 vertical。".to_string()),
                demo: rsx! { BasicFormDemo {} },
                code: "Form {\n    FormItem { label: \"用户名\".to_string(), required: true,\n        Input { placeholder: \"请输入用户名\" }\n    }\n    FormItem { label: \"密码\".to_string(), required: true,\n        Input { r#type: \"password\", placeholder: \"请输入密码\" }\n    }\n    FormItem {\n        Button { variant: Variant::Primary, \"提交\" }\n    }\n}".to_string(),
            }

            DemoBox {
                title: "水平布局".to_string(),
                description: Some("设置 layout 为 horizontal，标签在左侧。".to_string()),
                demo: rsx! {
                    Form { layout: "horizontal".to_string(),
                        FormItem { label: "姓名".to_string(), required: true, Input { placeholder: "请输入姓名" } }
                        FormItem { label: "邮箱".to_string(), required: true, Input { placeholder: "请输入邮箱" } }
                        FormItem { label: "备注".to_string(), Input { placeholder: "选填" } }
                    }
                },
                code: "Form { layout: \"horizontal\".to_string(), ... }".to_string(),
            }

            DemoBox {
                title: "帮助文本与错误".to_string(),
                description: Some("通过 FormItem 的 help 和 error 属性展示辅助信息。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",
                        FormItem { label: "用户名".to_string(), help: "请输入 4-16 个字符".to_string(), Input { placeholder: "请输入用户名" } }
                        FormItem { label: "邮箱".to_string(), error: "邮箱格式不正确".to_string(), Input { placeholder: "请输入邮箱" } }
                    }
                },
                code: "FormItem { label: \"...\".to_string(), help: \"帮助文本\".to_string(), ... }\nFormItem { label: \"...\".to_string(), error: \"错误信息\".to_string(), ... }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Form Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("layout", "String", "\"vertical\"", "布局方式：vertical / horizontal / inline"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onsubmit", "Option<EventHandler<Rc<FormData>>>", "None", "提交回调"),
                ("children", "Element", "—", "子元素"),
            ]}

            h3 { style: "font-size: 1rem; font-weight: 600; color: var(--ctrl-text); margin: 24px 0 12px;", "FormItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("label", "String", "\"\"", "标签文本"),
                ("required", "bool", "false", "是否必填"),
                ("help", "String", "\"\"", "帮助文本"),
                ("error", "String", "\"\"", "错误信息"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "表单控件"),
            ]}
        }

        // ════════════════════════════════════════
        // Tree 树形控件
        // ════════════════════════════════════════
        div { id: "tree", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Tree 树形控件" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "树形控件用于展示层级结构数据。支持节点展开/收起、选择和复选框功能。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 data 属性传入树形数据。".to_string()),
                demo: rsx! { BasicTreeDemo {} },
                code: "let tree_data = vec![\n    TreeNode { node_key: \"1\".to_string(), title: \"节点 1\".to_string(),\n        child_nodes: vec![\n            TreeNode { node_key: \"1-1\".to_string(), title: \"节点 1-1\".to_string() },\n            TreeNode { node_key: \"1-2\".to_string(), title: \"节点 1-2\".to_string() },\n        ]\n    },\n];\n\nTree { data: tree_data, default_expand_all: true }".to_string(),
            }

            DemoBox {
                title: "可选择".to_string(),
                description: Some("通过 selected_key 和 onselect 实现节点选择。".to_string()),
                demo: rsx! { SelectableTreeDemo {} },
                code: "let mut selected = use_signal(|| String::new());\n\nTree {\n    data: tree_data,\n    selected_key: selected(),\n    default_expand_all: true,\n    onselect: move |k| selected.set(k),\n}".to_string(),
            }

            DemoBox {
                title: "带复选框".to_string(),
                description: Some("设置 checkable 为 true 启用复选框。".to_string()),
                demo: rsx! { CheckableTreeDemo {} },
                code: "let mut checked = use_signal(|| Vec::new());\n\nTree {\n    data: tree_data,\n    checkable: true,\n    checked_keys: checked(),\n    default_expand_all: true,\n    oncheck: move |keys| checked.set(keys),\n}".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Tree Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("data", "Vec<TreeNode>", "[]", "树形数据"),
                ("selected_key", "String", "\"\"", "选中节点 key"),
                ("checkable", "bool", "false", "是否显示复选框"),
                ("checked_keys", "Vec<String>", "[]", "选中节点 keys"),
                ("default_expand_all", "bool", "false", "默认展开全部"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onselect", "Option<EventHandler<String>>", "None", "选择回调"),
                ("onexpand", "Option<EventHandler<(String, bool)>>", "None", "展开/收起回调"),
                ("oncheck", "Option<EventHandler<Vec<String>>>", "None", "复选框回调"),
            ]}

            h3 { style: "font-size: 1rem; font-weight: 600; color: var(--ctrl-text); margin: 24px 0 12px;", "TreeNode Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("node_key", "String", "—", "节点唯一标识"),
                ("title", "String", "—", "节点标题"),
                ("child_nodes", "Vec<TreeNode>", "[]", "子节点"),
                ("disabled", "bool", "false", "是否禁用"),
                ("selectable", "bool", "true", "是否可选择"),
            ]}
        }

        // ════════════════════════════════════════
        // DatePicker 日期选择器
        // ════════════════════════════════════════
        div { id: "date_picker", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "DatePicker 日期选择器" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "日期选择器用于选择日期。提供日历面板、月份导航和今天快捷按钮。" }

            DemoBox {
                title: "基本用法".to_string(),
                description: Some("通过 value 和 onchange 管理日期。".to_string()),
                demo: rsx! { BasicDatePickerDemo {} },
                code: "let mut date = use_signal(|| String::new());\n\nDatePicker {\n    value: date(),\n    onchange: move |v| date.set(v),\n}".to_string(),
            }

            DemoBox {
                title: "带初始值".to_string(),
                description: Some("设置 value 属性指定初始日期。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 12px;",
                        DatePicker { value: "2024-06-15".to_string() }
                        DatePicker { value: "2024-12-25".to_string() }
                    }
                },
                code: "DatePicker { value: \"2024-06-15\".to_string() }\nDatePicker { value: \"2024-12-25\".to_string() }".to_string(),
            }

            DemoBox {
                title: "禁用状态".to_string(),
                description: Some("设置 disabled 为 true 禁用选择器。".to_string()),
                demo: rsx! { DatePicker { value: "2024-06-15".to_string(), disabled: true } },
                code: "DatePicker { value: \"2024-06-15\".to_string(), disabled: true }".to_string(),
            }

            DemoBox {
                title: "不可清除".to_string(),
                description: Some("设置 clearable 为 false 隐藏清除按钮。".to_string()),
                demo: rsx! { DatePicker { value: "2024-06-15".to_string(), clearable: false } },
                code: "DatePicker { value: \"2024-06-15\".to_string(), clearable: false }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "DatePicker Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("value", "String", "\"\"", "选中的日期（YYYY-MM-DD）"),
                ("placeholder", "String", "\"请选择日期\"", "占位文本"),
                ("disabled", "bool", "false", "是否禁用"),
                ("clearable", "bool", "true", "是否可清除"),
                ("format", "String", "\"YYYY-MM-DD\"", "日期格式"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<String>>", "None", "值变化回调"),
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

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Tabs Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("active", "String", "\"0\"", "当前激活的 tab key"),
                ("onchange", "Option<EventHandler<String>>", "None", "tab 切换回调"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "子元素（Tab）"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Tab Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("tab_key", "String", "\"\"", "唯一标识"),
                ("title", "String", "\"\"", "tab 标题文字"),
                ("disabled", "bool", "false", "是否禁用"),
                ("children", "Element", "—", "面板内容"),
            ]}
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

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "AlertBannerContainer Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("max_count", "usize", "5", "最大显示数量，超出部分隐藏（先进先出）"),
                ("children", "Element", "—", "子元素（多条 Banner Alert）"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Alert Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("type", "AlertType", "Info", "提示类型（Info / Success / Warning / Error）"),
                ("title", "String", "\"\"", "标题"),
                ("description", "String", "\"\"", "描述内容"),
                ("closable", "bool", "false", "是否可关闭"),
                ("show_icon", "bool", "true", "是否显示图标"),
                ("mode", "AlertMode", "Inline", "显示模式（Inline 内联 / Banner 全局横幅）"),
                ("duration", "u64", "0", "自动关闭时间(ms)，仅 Banner 模式有效，0 表示不自动关闭"),
                ("closing", "bool", "false", "外部关闭信号，设为 true 触发退出动画"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭回调"),
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
                ("children", "Element", "—", "子元素（BreadcrumbItem）"),
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

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "MessageContainer Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("placement", "MessagePlacement", "Top", "弹出位置（Top / TopRight / TopLeft / Bottom）"),
                ("max_count", "usize", "5", "最大显示数量，超出部分隐藏（先进先出）"),
                ("children", "Element", "—", "子元素（多条 Message）"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Message Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("type", "MessageType", "Info", "消息类型（Info / Success / Warning / Error）"),
                ("content", "String", "\"\"", "消息文字"),
                ("duration", "u64", "3000", "自动消失时间(ms)，0 表示不自动消失"),
                ("closing", "bool", "false", "外部关闭信号，设为 true 触发退出动画"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭回调"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }

        // ════════════════════════════════════════
        // Divider 分割线
        // ════════════════════════════════════════
        div { id: "divider", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Divider 分割线" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "区隔内容的分割线，支持文字、虚线和垂直等变体。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; width: 100%;",
                        span { "上方内容" }
                        Divider {}
                        span { "下方内容" }
                    }
                },
                code: "Divider {}".to_string(),
            }
            DemoBox { title: "带文字".to_string(), description: Some("文字居中显示在分割线中间。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; width: 100%;",
                        span { "内容区" }
                        Divider { content: "分割文字".to_string() }
                        span { "另一区" }
                    }
                },
                code: "Divider { content: \"分割文字\".to_string() }".to_string(),
            }
            DemoBox { title: "虚线分割线".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 12px; width: 100%;",
                        span { "上方内容" }
                        Divider { dashed: true }
                        span { "下方内容" }
                    }
                },
                code: "Divider { dashed: true }".to_string(),
            }
            DemoBox { title: "垂直分割线".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; align-items: center; gap: 8px;",
                        span { "链接" }
                        Divider { direction: "vertical".to_string() }
                        span { "菜单" }
                        Divider { direction: "vertical".to_string() }
                        span { "设置" }
                    }
                },
                code: "Divider { direction: \"vertical\".to_string() }".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("direction", "String", "\"horizontal\"", "方向（horizontal / vertical）"),
                ("content", "String", "\"\"", "中间文字，空则为纯分割线"),
                ("dashed", "bool", "false", "是否虚线"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }

        // ════════════════════════════════════════
        // Loading 加载中
        // ════════════════════════════════════════
        div { id: "loading", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Loading 加载中" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "显示加载状态，支持三种尺寸和自定义文字。" }
            DemoBox { title: "尺寸".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Loading { size: Size::Sm }
                        Loading { size: Size::Md }
                        Loading { size: Size::Lg }
                    }
                },
                code: "Loading { size: Size::Sm }\nLoading { size: Size::Md }\nLoading { size: Size::Lg }".to_string(),
            }
            DemoBox { title: "带文字".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; gap: 16px;",
                        Loading { text: "加载中...".to_string(), size: Size::Md }
                    }
                },
                code: "Loading { text: \"加载中...\".to_string() }".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("loading", "bool", "true", "是否显示加载中"),
                ("text", "String", "\"\"", "加载文案"),
                ("size", "Size", "Md", "尺寸（Sm / Md / Lg）"),
                ("fullscreen", "bool", "false", "是否全屏遮罩"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }

        // ════════════════════════════════════════
        // Empty 空状态
        // ════════════════════════════════════════
        div { id: "empty", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Empty 空状态" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "无数据时的占位组件，可自定义图片和操作。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! { Empty { description: "暂无数据".to_string() } },
                code: "Empty { description: \"暂无数据\".to_string() }".to_string(),
            }
            DemoBox { title: "自定义操作".to_string(), description: None,
                demo: rsx! {
                    Empty { description: "还没有内容，快来添加吧".to_string(),
                        Button { variant: Variant::Primary, size: Size::Sm, "添加内容" }
                    }
                },
                code: "Empty { description: \"还没有内容\".to_string(),\n    Button { variant: Variant::Primary, size: Size::Sm, \"添加内容\" }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("description", "String", "\"暂无数据\"", "描述文字"),
                ("image", "String", "\"\"", "自定义图片 URL"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "底部操作区"),
            ]}
        }

        // ════════════════════════════════════════
        // Skeleton 骨架屏
        // ════════════════════════════════════════
        div { id: "skeleton", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Skeleton 骨架屏" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "内容加载时的占位动画，支持文字、头像、图片、按钮等多种变体，以及列表、卡片等复合骨架。" }

            // 基本变体
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin-bottom: 20px;", "Skeleton - 基础变体" }
            DemoBox { title: "形状控制".to_string(), description: Some("通过 shape 属性控制骨架块的圆角样式。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: flex-end;",
                        div { style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                            Skeleton { variant: "rect".to_string(), width: "64px".to_string(), height: "64px".to_string() }
                            span { style: "font-size: var(--ctrl-font-size-xs); color: var(--ctrl-text-secondary);", "default" }
                        }
                        div { style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                            Skeleton { variant: "rect".to_string(), shape: "round".to_string(), width: "64px".to_string(), height: "64px".to_string() }
                            span { style: "font-size: var(--ctrl-font-size-xs); color: var(--ctrl-text-secondary);", "round" }
                        }
                        div { style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                            Skeleton { variant: "rect".to_string(), shape: "circle".to_string(), width: "64px".to_string(), height: "64px".to_string() }
                            span { style: "font-size: var(--ctrl-font-size-xs); color: var(--ctrl-text-secondary);", "circle" }
                        }
                    }
                },
                code: "Skeleton { variant: \"rect\", shape: \"default\", width: \"64px\", height: \"64px\" }\nSkeleton { variant: \"rect\", shape: \"round\", width: \"64px\", height: \"64px\" }\nSkeleton { variant: \"rect\", shape: \"circle\", width: \"64px\", height: \"64px\" }".to_string(),
            }
            DemoBox { title: "静止态 vs 动画".to_string(), description: Some("animated: false 时停止闪烁，仅显示灰色占位。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Skeleton { variant: "text".to_string(), rows: 2, animated: true, width: "200px".to_string() }
                        Skeleton { variant: "text".to_string(), rows: 2, animated: false, width: "200px".to_string() }
                    }
                },
                code: "Skeleton { variant: \"text\", rows: 2, animated: true }\nSkeleton { variant: \"text\", rows: 2, animated: false }".to_string(),
            }
            DemoBox { title: "文本多行".to_string(), description: None,
                demo: rsx! { Skeleton { variant: "text".to_string(), rows: 3 } },
                code: "Skeleton { variant: \"text\".to_string(), rows: 3 }".to_string(),
            }
            DemoBox { title: "重复列表".to_string(), description: Some("count 属性生成重复骨架块，适用于占位列表场景。".to_string()),
                demo: rsx! {
                    Skeleton { variant: "text".to_string(), count: 4, gap: "12px".to_string(), width: "100%".to_string() }
                },
                code: "Skeleton { variant: \"text\", count: 4, gap: \"12px\" }".to_string(),
            }
            DemoBox { title: "圆形头像组".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; gap: 12px;",
                        Skeleton { variant: "avatar".to_string(), shape: "circle".to_string(), width: "40px".to_string(), height: "40px".to_string(), count: 3 }
                    }
                },
                code: "Skeleton { variant: \"avatar\", shape: \"circle\", width: \"40px\", height: \"40px\", count: 3 }".to_string(),
            }

            // 复合骨架
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonCard - 卡片骨架" }
            DemoBox { title: "卡片骨架".to_string(), description: Some("图片 + 标题 + 文字行的复合卡片骨架。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px;",
                        SkeletonCard { width: "240px".to_string(), rows: 2 }
                        SkeletonCard { width: "240px".to_string(), rows: 1 }
                    }
                },
                code: "SkeletonCard { width: \"240px\".to_string(), rows: 2 }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonList - 列表骨架" }
            DemoBox { title: "列表骨架".to_string(), description: Some("头像 + 文字组合的列表骨架，适用于好友列表、通知列表等场景。".to_string()),
                demo: rsx! {
                    SkeletonList { count: 3, rows: 2 }
                },
                code: "SkeletonList { count: 3, rows: 2 }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonRow - 行骨架" }
            DemoBox { title: "头像 + 文字行".to_string(), description: Some("可配置头像尺寸和是否显示头像。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        SkeletonRow { rows: 2 }
                        SkeletonRow { rows: 2, avatar: false }
                        SkeletonRow { rows: 1, avatar_size: "32px".to_string() }
                    }
                },
                code: "SkeletonRow { rows: 2 }\nSkeletonRow { rows: 2, avatar: false }\nSkeletonRow { rows: 1, avatar_size: \"32px\".to_string() }".to_string(),
            }

            // Skeleton Props
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Skeleton Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("variant", "String", "\"text\"", "变体（text / title / avatar / image / button / rect）"),
                ("rows", "usize", "3", "行数（仅 text 有效）"),
                ("shape", "String", "\"default\"", "形状（default / circle / round）"),
                ("count", "usize", "0", "重复次数（>0 时生成列表，忽略 rows）"),
                ("gap", "String", "\"8px\"", "count > 1 时的项间距"),
                ("width", "String", "\"\"", "宽度"),
                ("height", "String", "\"\"", "高度"),
                ("animated", "bool", "true", "是否显示闪烁动画"),
                ("loading", "Option<bool>", "None", "None 始终显示骨架；Some(false) 显示 children"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "加载完成后显示的内容"),
            ]}

            // SkeletonCard Props
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonCard Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("rows", "usize", "2", "文字行数"),
                ("image_height", "String", "\"180px\"", "图片区高度"),
                ("width", "String", "\"\"", "卡片宽度"),
                ("animated", "bool", "true", "是否显示动画"),
                ("loading", "Option<bool>", "None", "加载控制"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "加载完成后显示的内容"),
            ]}

            // SkeletonList Props
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonList Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("count", "usize", "3", "列表条目数"),
                ("avatar", "bool", "true", "是否显示头像"),
                ("avatar_size", "String", "\"40px\"", "头像尺寸"),
                ("rows", "usize", "2", "每项文字行数"),
                ("animated", "bool", "true", "是否显示动画"),
                ("loading", "Option<bool>", "None", "加载控制"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "加载完成后显示的内容"),
            ]}

            // SkeletonRow Props
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonRow Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("rows", "usize", "3", "文字行数"),
                ("avatar", "bool", "true", "是否显示头像"),
                ("avatar_size", "String", "\"48px\"", "头像尺寸（宽高相同）"),
                ("animated", "bool", "true", "是否显示动画"),
                ("loading", "Option<bool>", "None", "加载控制"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "加载完成后显示的内容"),
            ]}
        }

        // ════════════════════════════════════════
        // Backtop 回到顶部
        // ════════════════════════════════════════
        div { id: "backtop", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Backtop 回到顶部" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "页面滚动后显示在右下角的回到顶部按钮，点击后平滑滚动到顶部。支持多种缓动效果和弹簧阻尼模式。" }

            style { {r#"
.ctrl-backtop.demo-bt-default { right: 40px; bottom: 40px; }
.ctrl-backtop.demo-bt-default::after { content: "easeOutCubic"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-expo { left: 40px; right: auto; bottom: 40px; }
.ctrl-backtop.demo-bt-expo::after { content: "easeOutExpo 500ms"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-back { right: 40px; bottom: 100px; }
.ctrl-backtop.demo-bt-back::after { content: "easeOutBack 500ms"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-elastic { left: 40px; right: auto; bottom: 100px; }
.ctrl-backtop.demo-bt-elastic::after { content: "easeOutElastic 600ms"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-damping { right: 40px; bottom: 160px; }
.ctrl-backtop.demo-bt-damping::after { content: "damping"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-auto { left: 40px; right: auto; bottom: 160px; }
.ctrl-backtop.demo-bt-auto::after { content: "auto"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-fast { right: 40px; bottom: 220px; }
.ctrl-backtop.demo-bt-fast::after { content: "200ms"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-slow { left: 40px; right: auto; bottom: 220px; }
.ctrl-backtop.demo-bt-slow::after { content: "800ms"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-red { right: 40px; bottom: 280px; background: #e74c3c; color: #fff; border-color: transparent; }
.ctrl-backtop.demo-bt-red::after { content: "custom style"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-bottom { left: 50%; right: auto; top: 40px; transform: translateX(-50%); }
.ctrl-backtop.demo-bt-bottom::after { content: "回到底部 ↓"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
"#} }

            DemoBox { title: "可交互示例（真实组件）".to_string(), description: Some("滚动页面后，下方各位置的按钮会依次出现，点击体验不同的回顶效果。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px; width: 100%;",
                        div { style: "padding: 16px; border-radius: 8px; background: var(--ctrl-bg-secondary);",
                            p { style: "margin: 0 0 8px 0; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text); font-weight: 600;", "按钮位置说明" }
                            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                                span { "[右下] easeOutCubic（默认 400ms）" }
                                span { "[左下] easeOutExpo（500ms）" }
                                span { "[右中] easeOutBack（回弹 500ms）" }
                                span { "[左中] easeOutElastic（弹性 600ms）" }
                                span { "[右上] damping（弹簧阻尼）" }
                                span { "[左上] auto（瞬间跳转）" }
                                span { "[右侧] 200ms（快速）" }
                                span { "[左侧] 800ms（舒缓）" }
                                span { "[最上] 自定义红色样式" }
                                span { "[顶部居中] 回到底部 ↓ (target_position: bottom)" }
                            }
                        }
                    }
                    // 各位置的真实 Backtop 实例
                    Backtop { class: "demo-bt-default".to_string() }
                    Backtop { class: "demo-bt-expo".to_string(), easing: "easeOutExpo".to_string(), duration: 500, visibility_height: 300 }
                    Backtop { class: "demo-bt-back".to_string(), easing: "easeOutBack".to_string(), duration: 500, visibility_height: 300 }
                    Backtop { class: "demo-bt-elastic".to_string(), easing: "easeOutElastic".to_string(), duration: 600, visibility_height: 300 }
                    Backtop { class: "demo-bt-damping".to_string(), damping: true, visibility_height: 300 }
                    Backtop { class: "demo-bt-auto".to_string(), behavior: "auto".to_string(), visibility_height: 300 }
                    Backtop { class: "demo-bt-fast".to_string(), duration: 200, visibility_height: 300 }
                    Backtop { class: "demo-bt-slow".to_string(), duration: 800, visibility_height: 300 }
                    Backtop { class: "demo-bt-red".to_string(), visibility_height: 300 }
                    Backtop { class: "demo-bt-bottom".to_string(), target_position: "bottom".to_string(), visibility_height: 100 }
                },
                code: "Backtop {}                                              // 默认 easeOutCubic 400ms\nBacktop { easing: \"easeOutExpo\".to_string(), duration: 500 }     // 指数减速\nBacktop { easing: \"easeOutBack\".to_string(), duration: 500 }     // 回弹感\nBacktop { easing: \"easeOutElastic\".to_string(), duration: 600 }  // 弹性震荡\nBacktop { damping: true }                                         // 弹簧阻尼\nBacktop { behavior: \"auto\".to_string() }                          // 瞬间跳转\nBacktop { duration: 200 }                                          // 快速\nBacktop { duration: 800 }                                          // 舒缓\nBacktop { class: \"my-backtop\".to_string() }                       // 自定义样式\nBacktop { target_position: \"bottom\".to_string() }                 // 回到底部".to_string(),
            }

            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("target", "String", "\"\"", "监听滚动的容器选择器（默认 window）"),
                ("visibility_height", "u32", "200", "显示阈值（px），bottom 模式时为距离底部的距离"),
                ("behavior", "String", "\"smooth\"", "滚动行为（smooth 平滑 / auto 瞬间）"),
                ("duration", "u32", "400", "滚动动画时长（ms），behavior=auto 时无效"),
                ("easing", "String", "\"easeOutCubic\"", "缓动函数（easeOutQuad / easeOutCubic / easeOutQuart / easeOutQuint / easeOutExpo / easeOutBack / easeOutElastic）"),
                ("damping", "bool", "false", "是否启用弹簧阻尼效果（启用后忽略 duration 和 easing）"),
                ("target_position", "String", "\"top\"", "目标位置：top（顶部）或 bottom（底部）"),
                ("onclick", "Option<EventHandler>", "None", "点击回调"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "自定义按钮内容"),
            ]}
        }

        // ════════════════════════════════════════
        // Collapse 折叠面板
        // ════════════════════════════════════════
        div { id: "collapse", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Collapse 折叠面板" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "可以折叠/展开的内容区域，用于展示 FAQ、表单分组等。" }
            DemoBox { title: "基本用法".to_string(), description: Some("点击标题展开/收起内容，动画基于 CSS grid-template-rows 实现平滑高度过渡。".to_string()),
                demo: rsx! {
                    Collapse {
                        CollapseItem { title: "标题一".to_string(), expanded: true,
                            p { style: "margin: 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);", "标题一展开的内容。" }
                        }
                        CollapseItem { title: "标题二".to_string(),
                            p { style: "margin: 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);", "标题二的内容。" }
                        }
                        CollapseItem { title: "禁用项".to_string(), disabled: true,
                            p { style: "margin: 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);", "这项已禁用。" }
                        }
                    }
                },
                code: "Collapse {\n    CollapseItem { title: \"标题一\".to_string(), expanded: true,\n        p { \"内容\" }\n    }\n    CollapseItem { title: \"标题二\".to_string(),\n        p { \"内容\" }\n    }\n}".to_string(),
            }
            DemoBox { title: "无动画".to_string(), description: Some("设置 animated=false 可关闭展开/收起动画。".to_string()),
                demo: rsx! {
                    Collapse {
                        CollapseItem { title: "无动画项".to_string(), animated: false,
                            p { style: "margin: 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);", "展开和收起没有过渡效果，直接显示/隐藏。" }
                        }
                        CollapseItem { title: "另一个无动画项".to_string(), animated: false, expanded: true,
                            p { style: "margin: 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);", "这一项默认展开，切换时也是瞬间切换。" }
                        }
                    }
                },
                code: "CollapseItem { title: \"标题\".to_string(), animated: false,\n    p { \"内容\" }\n}".to_string(),
            }
            DemoBox { title: "手风琴模式".to_string(), description: Some("设置 accordion=true，展开一个面板时其他面板自动收起。".to_string()),
                demo: rsx! {
                    Collapse {
                        accordion: true,
                        CollapseItem { title: "面板一".to_string(), expanded: true,
                            p { style: "margin: 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);", "这是第一个面板的内容，默认展开。" }
                        }
                        CollapseItem { title: "面板二".to_string(),
                            p { style: "margin: 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);", "这是第二个面板的内容。" }
                        }
                        CollapseItem { title: "面板三".to_string(),
                            p { style: "margin: 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);", "这是第三个面板的内容。" }
                        }
                    }
                },
                code: "Collapse {\n    accordion: true,\n    CollapseItem { title: \"面板一\".to_string(), expanded: true,\n        p { \"内容\" }\n    }\n    CollapseItem { title: \"面板二\".to_string(),\n        p { \"内容\" }\n    }\n    CollapseItem { title: \"面板三\".to_string(),\n        p { \"内容\" }\n    }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("borderless", "bool", "false", "是否无边框（Collapse）"),
                ("accordion", "bool", "false", "是否启用手风琴模式（同时只有一个展开项，Collapse）"),
                ("class", "String", "\"\"", "自定义 CSS 类 — Collapse"),
                ("children", "Element", "—", "子元素（CollapseItem）— Collapse"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "CollapseItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "标题"),
                ("expanded", "bool", "false", "是否展开"),
                ("disabled", "bool", "false", "是否禁用"),
                ("show_arrow", "bool", "true", "是否显示箭头"),
                ("animated", "bool", "true", "是否启用展开/收起动画"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "面板内容"),
            ]}
        }

        // ════════════════════════════════════════
        // Popover 气泡卡片
        // ════════════════════════════════════════
        div { id: "popover", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Popover 气泡卡片" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "点击/悬停触发的气泡卡片，可包含标题和内容。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; gap: 12px;",
                        Popover { placement: "top".to_string(), title: "提示".to_string(), content: rsx! { span { "向上弹出" } },
                            Button { variant: Variant::Outline, size: Size::Sm, "Top" }
                        }
                        Popover { placement: "bottom".to_string(), title: "通知".to_string(), content: rsx! { span { "向下弹出" } },
                            Button { variant: Variant::Outline, size: Size::Sm, "Bottom" }
                        }
                    }
                },
                code: "Popover { placement: \"top\".to_string(), title: \"提示\".to_string(), content: rsx! { span { \"内容\" } },\n    Button { variant: Variant::Outline, size: Size::Sm, \"Top\" }\n}".to_string(),
            }
            DemoBox { title: "overflow:hidden 容器内".to_string(), description: Some("即使祖先容器设置了 overflow: hidden，气泡卡片也不会被裁切，因为使用 position:fixed 定位。".to_string()),
                demo: rsx! {
                    div {
                        style: "overflow: hidden; border: 2px dashed var(--ctrl-border); border-radius: var(--ctrl-radius-md); padding: 20px; width: 200px; height: 80px; display: flex; align-items: center; justify-content: center;",
                        Popover { placement: "top".to_string(), title: "提示".to_string(), content: rsx! { span { "不会被 overflow:hidden 裁切" } },
                            Button { variant: Variant::Primary, size: Size::Sm, "点击弹出" }
                        }
                    }
                },
                code: "div { style: \"overflow: hidden; ...\",\n    Popover { placement: \"top\".to_string(), title: \"提示\".to_string(),\n        content: rsx! { span { \"内容\" } },\n        Button { \"点击弹出\" }\n    }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("placement", "String", "\"top\"", "弹出位置（top / bottom / left / right）"),
                ("title", "String", "\"\"", "气泡标题"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "触发元素"),
                ("content", "Element", "—", "气泡内容"),
            ]}
        }

        // ════════════════════════════════════════
        // Drawer 抽屉
        // ════════════════════════════════════════
        div { id: "drawer", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Drawer 抽屉" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "从屏幕边缘滑出的浮层面板，适用于表单、详情等场景。" }
            DemoBox { title: "基本用法".to_string(), description: Some("点击按钮打开右侧抽屉。".to_string()),
                demo: rsx! {
                    DrawerDocs {}
                },
                code: "let mut visible = use_signal(|| false);\nButton { onclick: move |_| visible.set(true), \"打开\" }\nDrawer { visible: visible(), title: \"标题\".to_string(),\n    onclose: move |_| visible.set(false),\n    p { \"内容\" }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("visible", "bool", "false", "是否打开"),
                ("title", "String", "\"\"", "抽屉标题"),
                ("placement", "String", "\"right\"", "位置（left / right / top / bottom）"),
                ("size", "String", "\"380px\"", "宽度或高度"),
                ("show_close", "bool", "true", "是否显示关闭按钮"),
                ("onclose", "Option<EventHandler<()>>", "None", "关闭回调"),
                ("footer", "Option<Element>", "None", "底部操作区"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "抽屉内容"),
            ]}
        }

        // ════════════════════════════════════════
        // Notification 通知提醒
        // ════════════════════════════════════════
        div { id: "notification", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Notification 通知提醒" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "全局通知提醒，支持四种类型和自动关闭。推荐使用 NotificationProvider + useNotification 上下文 API。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    NotificationDocs {}
                },
                code: "let mut notif = use_notification();\nnotif.info(\"通知\".to_string(), \"这是一条通知\".to_string());\nnotif.success(\"成功\".to_string(), \"操作成功\".to_string());\nnotif.warning(\"警告\".to_string(), \"请注意\".to_string());\nnotif.error(\"错误\".to_string(), \"操作失败\".to_string());".to_string(),
            }
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "NotificationProvider Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("placement", "NotificationPlacement", "TopRight", "弹出位置（TopRight / TopLeft / BottomRight / BottomLeft）"),
                ("max_count", "usize", "5", "最大显示数量，超出部分隐藏（先进先出）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "子元素"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "NotificationProps（单条通知）" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("r#type", "NotificationType", "Info", "通知类型（Info / Success / Warning / Error）"),
                ("title", "String", "\"\"", "标题"),
                ("content", "String", "\"\"", "内容"),
                ("duration", "u64", "4500", "自动关闭时间(ms)，0 则不自动关闭"),
                ("closing", "bool", "false", "外部关闭信号"),
                ("onclose", "Option<EventHandler>", "None", "关闭回调"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "useNotification API" }
            PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "返回值".to_string(), "说明".to_string()], rows: vec![
                ("info(title, content)", "String, String", "—", "打开一条信息通知"),
                ("success(title, content)", "String, String", "—", "打开一条成功通知"),
                ("warning(title, content)", "String, String", "—", "打开一条警告通知"),
                ("error(title, content)", "String, String", "—", "打开一条错误通知"),
                ("open(type, title, content)", "NotificationType, String, String", "—", "打开一条通知（默认 duration 4500ms）"),
                ("open_with_duration(type, title, content, duration)", "NotificationType, String, String, u64", "—", "打开一条通知（自定义 duration）"),
                ("remove(id)", "u32", "—", "移除指定通知"),
                ("clear()", "—", "—", "清除所有通知"),
            ]}
        }

        // ════════════════════════════════════════
        // Dropdown 下拉菜单
        // ════════════════════════════════════════
        div { id: "dropdown", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Dropdown 下拉菜单" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "点击后展开的下拉菜单，支持选项和分割线。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    DropdownDocs {}
                },
                code: "Dropdown {\n    trigger: rsx! { Button { \"操作\" } },\n    DropdownItem { \"选项一\" }\n    DropdownDivider {}\n    DropdownItem { disabled: true, \"禁用项\" }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("placement", "String", "\"bottom\"", "弹出位置（bottom / bottom-start / bottom-end / top / top-start / top-end）"),
                ("trigger", "Element", "—", "触发元素（通常是 Button）"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "菜单项（DropdownItem / DropdownDivider）"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "DropdownItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("disabled", "bool", "false", "是否禁用"),
                ("onclick", "Option<EventHandler<()>>", "None", "点击事件"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "菜单项文字"),
            ]}
        }

        // ════════════════════════════════════
        // Menu 导航菜单
        // ════════════════════════════════════════
        div { id: "menu", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Menu 导航菜单" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "垂直/水平导航菜单，适用于侧边栏或顶部导航。" }
            DemoBox { title: "垂直菜单".to_string(), description: None,
                demo: rsx! {
                    MenuDocs {}
                },
                code: "Menu {\n    MenuItem { \"首页\" }\n    MenuItem { \"组件\" }\n    MenuItem { \"文档\" }\n    MenuItem { disabled: true, \"禁用项\" }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("direction", "String", "\"vertical\"", "方向（vertical / horizontal）— Menu"),
                ("active", "String", "\"\"", "当前激活项 key — Menu"),
                ("onchange", "Option<EventHandler<String>>", "None", "切换回调 — Menu"),
                ("class", "String", "\"\"", "自定义 CSS 类 — Menu"),
                ("children", "Element", "—", "子元素（MenuItem / Submenu）— Menu"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "MenuItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("item_key", "String", "\"\"", "唯一标识"),
                ("disabled", "bool", "false", "是否禁用"),
                ("onclick", "Option<EventHandler<()>>", "None", "点击事件"),
                ("icon", "Option<Element>", "None", "图标插槽"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "菜单文字"),
            ]}
        }

        // ════════════════════════════════════════
        // Steps 步骤条
        // ════════════════════════════════════════
        div { id: "steps", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Steps 步骤条" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "用于展示流程进度，支持水平和垂直排列。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    Steps {
                        Step { title: "步骤一".to_string(), description: "第一步描述".to_string() }
                        Step { title: "步骤二".to_string(), description: "第二步描述".to_string() }
                        Step { title: "步骤三".to_string(), description: "最后一步".to_string() }
                    }
                },
                code: "Steps {\n    Step { title: \"步骤一\".to_string(), description: \"描述\".to_string() }\n    Step { title: \"步骤二\".to_string() }\n    Step { title: \"步骤三\".to_string() }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("current", "i32", "-1", "当前步骤索引（Steps），-1 表示未开始"),
                ("direction", "String", "\"horizontal\"", "方向（horizontal / vertical）— Steps"),
                ("class", "String", "\"\"", "自定义 CSS 类 — Steps"),
                ("children", "Element", "—", "子元素（Step）— Steps"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Step Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("title", "String", "\"\"", "步骤标题"),
                ("description", "String", "\"\"", "步骤描述"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
            ]}
        }

        // ════════════════════════════════════════
        // Timeline 时间线
        // ════════════════════════════════════════
        div { id: "timeline", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Timeline 时间线" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "垂直时间线组件，用于按时间顺序展示事件。" }
            DemoBox { title: "基本用法".to_string(), description: None,
                demo: rsx! {
                    Timeline {
                        TimelineItem { timestamp: "2024-01-15".to_string(), "项目立项" }
                        TimelineItem { timestamp: "2024-03-20".to_string(), color: "primary".to_string(), "需求分析完成" }
                        TimelineItem { timestamp: "2024-06-10".to_string(), color: "success".to_string(), "第一阶段上线" }
                        TimelineItem { timestamp: "2024-09-01".to_string(), "项目结项" }
                    }
                },
                code: "Timeline {\n    TimelineItem { timestamp: \"2024-01-15\".to_string(), \"事件\" }\n    TimelineItem { timestamp: \"...\".to_string(), color: \"primary\".to_string(), \"事件\" }\n}".to_string(),
            }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("class", "String", "\"\"", "自定义 CSS 类 — Timeline"),
                ("children", "Element", "—", "子元素（TimelineItem）— Timeline"),
            ]}
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "TimelineItem Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("timestamp", "String", "\"\"", "时间标签"),
                ("color", "String", "\"default\"", "圆点颜色：default / primary / success / warning / danger"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "事件内容"),
            ]}
        }
    }
}

// ── Drawer 演示 ──
#[component]
#[allow(non_snake_case)]
fn DrawerDocs() -> Element {
    let mut visible = use_signal(|| false);
    rsx! {
        div {
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| visible.set(true), "打开抽屉" }
            Drawer {
                visible: visible(),
                title: "抽屉标题".to_string(),
                onclose: move |_| visible.set(false),
                p { style: "margin: 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-md);", "抽屉内容区域，可以放表单、说明等任意内容。" }
            }
        }
    }
}

// ── Notification 演示 ──
#[component]
#[allow(non_snake_case)]
fn NotificationDocs() -> Element {
    let mut api = use_notification();
    rsx! {
        div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| api.info("信息通知".to_string(), "这是一条普通信息通知。".to_string()), "信息" }
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| api.success("操作成功".to_string(), "数据已成功保存。".to_string()), "成功" }
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| api.warning("警告".to_string(), "存储空间不足，请及时清理。".to_string()), "警告" }
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| api.error("错误".to_string(), "网络请求失败，请重试。".to_string()), "错误" }
        }
    }
}

// ── Dropdown 演示 ──
#[component]
#[allow(non_snake_case)]
fn DropdownDocs() -> Element {
    rsx! {
        Dropdown {
            trigger: rsx! { Button { variant: Variant::Primary, size: Size::Sm, "打开菜单" } },
            DropdownItem { "选项一" }
            DropdownItem { "选项二" }
            DropdownDivider {}
            DropdownItem { disabled: true, "禁用项" }
        }
    }
}

// ── Menu 演示 ──
#[component]
#[allow(non_snake_case)]
fn MenuDocs() -> Element {
    rsx! {
        Menu {
            MenuItem { "首页" }
            MenuItem { "组件" }
            MenuItem { "文档" }
            MenuItem { disabled: true, "禁用项" }
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

// ── Slider 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicSliderDemo() -> Element {
    let mut value = use_signal(|| 50);

    rsx! {
        div { style: "max-width: 400px;",
            Slider {
                value: value(),
                onchange: move |v| value.set(v),
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn CustomRangeSliderDemo() -> Element {
    let mut v1 = use_signal(|| 30);
    let mut v2 = use_signal(|| 0);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 24px; max-width: 400px;",
            Slider {
                value: v1(),
                min: 0,
                max: 100,
                step: 5,
                show_label: true,
                onchange: move |v| v1.set(v),
            }
            Slider {
                value: v2(),
                min: -50,
                max: 50,
                step: 10,
                show_label: true,
                onchange: move |v| v2.set(v),
            }
        }
    }
}

// ── Rate 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicRateDemo() -> Element {
    let mut value = use_signal(|| 3.0);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px;",
            Rate {
                value: value(),
                onchange: move |v: f64| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前评分: {value()}"
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn HalfRateDemo() -> Element {
    let mut value = use_signal(|| 2.5);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px;",
            Rate {
                value: value(),
                allow_half: true,
                onchange: move |v: f64| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前评分: {value()}"
            }
        }
    }
}

// ── Segmented 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicSegmentedDemo() -> Element {
    let mut value = use_signal(|| String::new());

    let options = vec![("日".to_string(), "day".to_string()), ("周".to_string(), "week".to_string()), ("月".to_string(), "month".to_string()), ("年".to_string(), "year".to_string())];

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 12px;",
            Segmented {
                options: options,
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

// ── InputNumber 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicInputNumberDemo() -> Element {
    let mut value = use_signal(|| 0);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 200px;",
            InputNumber {
                value: value(),
                onchange: move |v| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前值: {value()}"
            }
        }
    }
}

// ── Upload 演示 ──

#[component]
#[allow(non_snake_case)]
fn ClickUploadDemo() -> Element {
    let mut files = use_signal(|| Vec::new());

    rsx! {
        Upload {
            files: files(),
            onchange: move |f| files.set(f),
            Button { variant: Variant::Primary, "选择文件" }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn FileListUploadDemo() -> Element {
    let mut files = use_signal(|| vec![
        UploadFile { name: "photo.jpg".to_string(), size: 204800, status: UploadFileStatus::Success },
        UploadFile { name: "document.pdf".to_string(), size: 1024000, status: UploadFileStatus::Success },
    ]);

    rsx! {
        Upload {
            files: files(),
            onremove: move |i| {
                let mut f = files();
                f.remove(i);
                files.set(f);
            },
            Button { variant: Variant::Primary, "选择文件" }
        }
    }
}

// ── Form 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicFormDemo() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx! {
        Form {
            FormItem { label: "用户名".to_string(), required: true,
                Input {
                    placeholder: "请输入用户名",
                    value: username(),
                    oninput: move |evt: FormEvent| username.set(evt.value()),
                }
            }
            FormItem { label: "密码".to_string(), required: true,
                Input {
                    r#type: "password",
                    placeholder: "请输入密码",
                    value: password(),
                    oninput: move |evt: FormEvent| password.set(evt.value()),
                }
            }
            FormItem {
                Button { variant: Variant::Primary, "提交" }
            }
        }
    }
}

// ── Tree 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicTreeDemo() -> Element {
    let tree_data = vec![
        TreeNode {
            node_key: "1".to_string(),
            title: "一级节点 1".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "1-1".to_string(), title: "二级节点 1-1".to_string(), ..Default::default() },
                TreeNode {
                    node_key: "1-2".to_string(),
                    title: "二级节点 1-2".to_string(),
                    child_nodes: vec![
                        TreeNode { node_key: "1-2-1".to_string(), title: "三级节点 1-2-1".to_string(), ..Default::default() },
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        TreeNode {
            node_key: "2".to_string(),
            title: "一级节点 2".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "2-1".to_string(), title: "二级节点 2-1".to_string(), ..Default::default() },
                TreeNode { node_key: "2-2".to_string(), title: "二级节点 2-2".to_string(), ..Default::default() },
            ],
            ..Default::default()
        },
    ];

    rsx! {
        Tree { data: tree_data, default_expand_all: true }
    }
}

#[component]
#[allow(non_snake_case)]
fn SelectableTreeDemo() -> Element {
    let mut selected = use_signal(|| String::new());

    let tree_data = vec![
        TreeNode {
            node_key: "1".to_string(),
            title: "节点 1".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "1-1".to_string(), title: "节点 1-1".to_string(), ..Default::default() },
                TreeNode { node_key: "1-2".to_string(), title: "节点 1-2".to_string(), ..Default::default() },
            ],
            ..Default::default()
        },
        TreeNode {
            node_key: "2".to_string(),
            title: "节点 2".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "2-1".to_string(), title: "节点 2-1".to_string(), ..Default::default() },
            ],
            ..Default::default()
        },
    ];

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 12px;",
            Tree {
                data: tree_data,
                selected_key: selected(),
                default_expand_all: true,
                onselect: move |k| selected.set(k),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前选中: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;",
                    if selected().is_empty() { "无" } else { "{selected()}" }
                }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn CheckableTreeDemo() -> Element {
    let mut checked = use_signal(|| Vec::new());

    let tree_data = vec![
        TreeNode {
            node_key: "1".to_string(),
            title: "节点 1".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "1-1".to_string(), title: "节点 1-1".to_string(), ..Default::default() },
                TreeNode { node_key: "1-2".to_string(), title: "节点 1-2".to_string(), ..Default::default() },
            ],
            ..Default::default()
        },
        TreeNode {
            node_key: "2".to_string(),
            title: "节点 2".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "2-1".to_string(), title: "节点 2-1".to_string(), ..Default::default() },
            ],
            ..Default::default()
        },
    ];

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 12px;",
            Tree {
                data: tree_data,
                checkable: true,
                checked_keys: checked(),
                default_expand_all: true,
                oncheck: move |keys| checked.set(keys),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "选中节点: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;",
                    if checked().is_empty() { "无" } else { "{checked().join(\", \")}" }
                }
            }
        }
    }
}

// ── DatePicker 演示 ──

#[component]
#[allow(non_snake_case)]
fn BasicDatePickerDemo() -> Element {
    let mut date = use_signal(|| String::new());

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 12px;",
            DatePicker {
                value: date(),
                onchange: move |v| date.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "选中日期: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;",
                    if date().is_empty() { "未选择" } else { "{date()}" }
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
