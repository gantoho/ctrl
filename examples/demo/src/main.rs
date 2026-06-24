use dioxus::prelude::*;
use ctrl::prelude::*;

/// 组件 CSS 样式文件（通过 asset!() 加载，支持热重载）
static CSS_BUTTON: Asset = asset!("/assets/css/button.css");
static CSS_INPUT: Asset = asset!("/assets/css/input.css");
static CSS_SWITCH: Asset = asset!("/assets/css/switch.css");
static CSS_CHECKBOX: Asset = asset!("/assets/css/checkbox.css");
static CSS_RADIO: Asset = asset!("/assets/css/radio.css");
static CSS_SELECT: Asset = asset!("/assets/css/select.css");
static CSS_TAG: Asset = asset!("/assets/css/tag.css");
static CSS_CARD: Asset = asset!("/assets/css/card.css");
static CSS_DIALOG: Asset = asset!("/assets/css/dialog.css");
static CSS_TABLE: Asset = asset!("/assets/css/table.css");
static CSS_BADGE: Asset = asset!("/assets/css/badge.css");
static CSS_AVATAR: Asset = asset!("/assets/css/avatar.css");
static CSS_PROGRESS: Asset = asset!("/assets/css/progress.css");
static CSS_TOOLTIP: Asset = asset!("/assets/css/tooltip.css");
static CSS_TABS: Asset = asset!("/assets/css/tabs.css");
static CSS_ALERT: Asset = asset!("/assets/css/alert.css");
static CSS_BREADCRUMB: Asset = asset!("/assets/css/breadcrumb.css");
static CSS_PAGINATION: Asset = asset!("/assets/css/pagination.css");
static CSS_MESSAGE: Asset = asset!("/assets/css/message.css");

fn main() {
    dioxus::launch(App);
}

#[allow(non_snake_case)]
fn App() -> Element {
    rsx! {
        // 加载组件 CSS（Dioxus 0.7 推荐方式，支持热重载）
        // reset.css 已由 ThemeProvider 自动注入
        document::Stylesheet { href: CSS_BUTTON }
        document::Stylesheet { href: CSS_INPUT }
        document::Stylesheet { href: CSS_SWITCH }
        document::Stylesheet { href: CSS_CHECKBOX }
        document::Stylesheet { href: CSS_RADIO }
        document::Stylesheet { href: CSS_SELECT }
        document::Stylesheet { href: CSS_TAG }
        document::Stylesheet { href: CSS_CARD }
        document::Stylesheet { href: CSS_DIALOG }
        document::Stylesheet { href: CSS_TABLE }
        document::Stylesheet { href: CSS_BADGE }
        document::Stylesheet { href: CSS_AVATAR }
        document::Stylesheet { href: CSS_PROGRESS }
        document::Stylesheet { href: CSS_TOOLTIP }
        document::Stylesheet { href: CSS_TABS }
        document::Stylesheet { href: CSS_ALERT }
        document::Stylesheet { href: CSS_BREADCRUMB }
        document::Stylesheet { href: CSS_PAGINATION }
        document::Stylesheet { href: CSS_MESSAGE }

        ThemeProvider {
            div {
                style: "max-width: 720px; margin: 0 auto; padding: 40px 20px; font-family: var(--ctrl-font-family);",

                // ── 标题 ──
                h1 { style: "font-size: 1.5rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                    "Ctrl UI 组件演示"
                }
                p { style: "color: var(--ctrl-text-secondary); margin-bottom: 40px; font-size: var(--ctrl-font-size-md);",
                    "开箱即用的 Dioxus UI 组件库 —— 以下展示所有已实现的组件"
                }

                // ════════════════════════════════════
                // 1. Button
                // ════════════════════════════════════
                Section { title: "Button 按钮".to_string(),
                    Row {
                        DemoCard { title: "变体 Variant".to_string(),
                            div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                                Button { variant: Variant::Primary, "Primary" }
                                Button { variant: Variant::Secondary, "Secondary" }
                                Button { variant: Variant::Outline, "Outline" }
                                Button { variant: Variant::Ghost, "Ghost" }
                            }
                        }
                        DemoCard { title: "尺寸 Size".to_string(),
                            div { style: "display: flex; gap: 8px; align-items: center;",
                                Button { variant: Variant::Primary, size: Size::Sm, "Small" }
                                Button { variant: Variant::Primary, size: Size::Md, "Medium" }
                                Button { variant: Variant::Primary, size: Size::Lg, "Large" }
                            }
                        }
                    }
                    Row {
                        DemoCard { title: "禁用 & 块级".to_string(),
                            div { style: "display: flex; flex-direction: column; gap: 8px;",
                                Button { variant: Variant::Primary, disabled: true, "禁用按钮" }
                                Button { variant: Variant::Outline, block: true, "块级按钮" }
                            }
                        }
                        DemoCard { title: "交互计数".to_string(),
                            CounterDemo {}
                        }
                    }
                }

                // ════════════════════════════════════
                // 2. Input
                // ════════════════════════════════════
                Section { title: "Input 输入框".to_string(),
                    Row {
                        DemoCard { title: "基本用法".to_string(),
                            InputBasicDemo {}
                        }
                        DemoCard { title: "尺寸".to_string(),
                            div { style: "display: flex; flex-direction: column; gap: 8px;",
                                Input { placeholder: "Small", size: Size::Sm }
                                Input { placeholder: "Medium", size: Size::Md }
                                Input { placeholder: "Large", size: Size::Lg }
                            }
                        }
                    }
                    Row {
                        DemoCard { title: "状态".to_string(),
                            div { style: "display: flex; flex-direction: column; gap: 8px;",
                                Input { placeholder: "错误状态", error: true }
                                Input { placeholder: "禁用状态", disabled: true }
                                Input { placeholder: "只读状态", readonly: true, value: "只读内容".to_string() }
                            }
                        }
                        DemoCard { title: "表单验证".to_string(),
                            FormDemo {}
                        }
                    }
                }

                // ════════════════════════════════════
                // 3. Switch
                // ════════════════════════════════════
                Section { title: "Switch 开关".to_string(),
                    Row {
                        DemoCard { title: "基本用法".to_string(),
                            SwitchBasicDemo {}
                        }
                        DemoCard { title: "尺寸".to_string(),
                            div { style: "display: flex; gap: 12px; align-items: center;",
                                Switch { size: Size::Sm }
                                Switch { size: Size::Md, checked: true }
                                Switch { size: Size::Lg }
                            }
                        }
                    }
                    Row {
                        DemoCard { title: "禁用".to_string(),
                            div { style: "display: flex; gap: 12px; align-items: center;",
                                Switch { disabled: true }
                                Switch { disabled: true, checked: true }
                            }
                        }
                        DemoCard { title: "开关联动".to_string(),
                            SwitchLinkedDemo {}
                        }
                    }
                }

                // ════════════════════════════════════
                // 4. Checkbox
                // ════════════════════════════════════
                Section { title: "Checkbox 复选框".to_string(),
                    Row {
                        DemoCard { title: "基本用法".to_string(),
                            CheckboxBasicDemo {}
                        }
                        DemoCard { title: "状态".to_string(),
                            div { style: "display: flex; flex-direction: column; gap: 6px;",
                                Checkbox { label: "未选中".to_string() }
                                Checkbox { checked: true, label: "已选中".to_string() }
                                Checkbox { indeterminate: true, label: "半选".to_string() }
                                Checkbox { disabled: true, label: "禁用".to_string() }
                            }
                        }
                    }
                    Row {
                        DemoCard { title: "全选示例".to_string(),
                            CheckAllDemoContainer {}
                        }
                        {}
                    }
                }

                // ════════════════════════════════════
                // 5. Radio
                // ════════════════════════════════════
                Section { title: "Radio 单选框".to_string(),
                    Row {
                        DemoCard { title: "基本用法".to_string(),
                            RadioBasicDemo {}
                        }
                        DemoCard { title: "禁用".to_string(),
                            div { style: "display: flex; flex-direction: column; gap: 6px;",
                                Radio { value: "a".to_string(), label: "已禁用".to_string(), checked: true, disabled: true }
                                Radio { value: "b".to_string(), label: "禁用未选".to_string(), disabled: true }
                            }
                        }
                    }
                }

                // ════════════════════════════════════
                // 6. Select
                // ════════════════════════════════════
                Section { title: "Select 下拉选择".to_string(),
                    Row {
                        DemoCard { title: "基本用法".to_string(),
                            SelectBasicDemo {}
                        }
                        DemoCard { title: "尺寸 & 禁用".to_string(),
                            div { style: "display: flex; flex-direction: column; gap: 8px;",
                                Select {
                                    size: Size::Sm,
                                    placeholder: "Small".to_string(),
                                    options: vec![("s1".into(), "小选项 A".into(), false)],
                                }
                                Select {
                                    size: Size::Md,
                                    placeholder: "Medium".to_string(),
                                    options: vec![("m1".into(), "中选项".into(), false)],
                                }
                                Select {
                                    disabled: true,
                                    placeholder: "Disabled".to_string(),
                                    options: vec![("d1".into(), "禁用项".into(), false)],
                                }
                            }
                        }
                    }
                }

                // ════════════════════════════════════
                // 7. Tag
                // ════════════════════════════════════
                Section { title: "Tag 标签".to_string(),
                    Row {
                        DemoCard { title: "颜色".to_string(),
                            div { style: "display: flex; gap: 6px; flex-wrap: wrap;",
                                Tag { color: "var(--ctrl-primary)".to_string(), "Primary" }
                                Tag { color: "var(--ctrl-success)".to_string(), "Success" }
                                Tag { color: "var(--ctrl-warning)".to_string(), "Warning" }
                                Tag { color: "var(--ctrl-danger)".to_string(), "Danger" }
                                Tag { color: "var(--ctrl-info)".to_string(), "Info" }
                            }
                        }
                        DemoCard { title: "可关闭".to_string(),
                            div { style: "display: flex; gap: 6px; flex-wrap: wrap;",
                                Tag { color: "var(--ctrl-primary)".to_string(), closable: true, "可关闭" }
                                Tag { color: "var(--ctrl-success)".to_string(), closable: true, "成功" }
                                Tag { color: "var(--ctrl-warning)".to_string(), closable: true, "警告" }
                            }
                        }
                    }
                }

                // ════════════════════════════════════
                // 8. Card
                // ════════════════════════════════════
                Section { title: "Card 卡片".to_string(),
                    Row {
                        DemoCard { title: "基本卡片".to_string(),
                            Card { title: "卡片标题".to_string(),
                                p { style: "color: var(--ctrl-text-secondary); margin: 0; font-size: var(--ctrl-font-size-sm);",
                                    "这是卡片的内容区域，用于承载结构化信息。"
                                }
                                div { style: "margin-top: 12px;",
                                    Button { variant: Variant::Primary, size: Size::Sm, "操作" }
                                }
                            }
                        }
                        DemoCard { title: "阴影卡片".to_string(),
                            Card { shadow: true, title: "阴影效果".to_string(),
                                p { style: "color: var(--ctrl-text-secondary); margin: 0; font-size: var(--ctrl-font-size-sm);",
                                    "shadow: true 为卡片添加投影效果。"
                                }
                            }
                        }
                    }
                }

                // ════════════════════════════════════
                // 9. Dialog
                // ════════════════════════════════════
                Section { title: "Dialog 对话框".to_string(),
                    Row {
                        DemoCard { title: "基本对话框".to_string(),
                            DialogBasicDemo {}
                        }
                        DemoCard { title: "确认对话框".to_string(),
                            DialogConfirmDemo {}
                        }
                    }
                }

                // ════════════════════════════════════
                // 10. Table
                // ════════════════════════════════════
                Section { title: "Table 表格".to_string(),
                    Divider {}
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
                            vec!["onclick".into(), "Option".into(), "None".into(), "点击事件".into()],
                        ],
                    }
                    Divider {}
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
                }

                // ════════════════════════════════════
                // 11. Badge
                // ════════════════════════════════════
                Section { title: "Badge 徽标".to_string(),
                    Row {
                        DemoCard { title: "基本用法".to_string(),
                            div { style: "display: flex; gap: 24px; align-items: center;",
                                Badge { count: "5".to_string(),
                                    div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; border: 1px solid var(--ctrl-border);", "消息" }
                                }
                                Badge { count: "120".to_string(),
                                    div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; border: 1px solid var(--ctrl-border);", "通知" }
                                }
                                Badge { dot: true,
                                    div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; border: 1px solid var(--ctrl-border);", "状态" }
                                }
                            }
                        }
                        DemoCard { title: "自定义颜色".to_string(),
                            div { style: "display: flex; gap: 24px; align-items: center;",
                                Badge { count: "3".to_string(), color: "var(--ctrl-success)".to_string(),
                                    div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; border: 1px solid var(--ctrl-border);", "成功" }
                                }
                                Badge { count: "99+".to_string(), color: "var(--ctrl-warning)".to_string(),
                                    div { style: "width: 40px; height: 40px; background: var(--ctrl-bg-secondary); border-radius: var(--ctrl-radius-md); display: flex; align-items: center; justify-content: center; border: 1px solid var(--ctrl-border);", "警告" }
                                }
                            }
                        }
                    }
                }

                // ════════════════════════════════════
                // 12. Avatar
                // ════════════════════════════════════
                Section { title: "Avatar 头像".to_string(),
                    Row {
                        DemoCard { title: "尺寸".to_string(),
                            div { style: "display: flex; gap: 12px; align-items: center;",
                                Avatar { size: Size::Sm, "S" }
                                Avatar { size: Size::Md, "M" }
                                Avatar { size: Size::Lg, "L" }
                            }
                        }
                        DemoCard { title: "形状".to_string(),
                            div { style: "display: flex; gap: 12px; align-items: center;",
                                Avatar { size: Size::Lg, "圆" }
                                Avatar { size: Size::Lg, shape: "square".to_string(), "方" }
                            }
                        }
                    }
                }

                // ════════════════════════════════════
                // 13. Progress
                // ════════════════════════════════════
                Section { title: "Progress 进度条".to_string(),
                    Row {
                        DemoCard { title: "基本用法".to_string(),
                            div { style: "display: flex; flex-direction: column; gap: 12px;",
                                Progress { percent: 20.0 }
                                Progress { percent: 60.0, color: "var(--ctrl-success)".to_string() }
                                Progress { percent: 90.0, color: "var(--ctrl-warning)".to_string(), show_text: true }
                            }
                        }
                        DemoCard { title: "动态进度".to_string(),
                            ProgressDynamicDemo {}
                        }
                    }
                }

                // ════════════════════════════════════
                // 14. Tooltip
                // ════════════════════════════════════
                Section { title: "Tooltip 气泡提示".to_string(),
                    Row {
                        DemoCard { title: "位置方向".to_string(),
                            div { style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                                Tooltip { content: "顶部提示".to_string(), placement: "top".to_string(),
                                    Button { variant: Variant::Outline, size: Size::Sm, "Top" }
                                }
                                Tooltip { content: "底部提示".to_string(), placement: "bottom".to_string(),
                                    Button { variant: Variant::Outline, size: Size::Sm, "Bottom" }
                                }
                                Tooltip { content: "左侧提示".to_string(), placement: "left".to_string(),
                                    Button { variant: Variant::Outline, size: Size::Sm, "Left" }
                                }
                                Tooltip { content: "右侧提示".to_string(), placement: "right".to_string(),
                                    Button { variant: Variant::Outline, size: Size::Sm, "Right" }
                                }
                            }
                        }
                        {} // 占位
                    }
                }

                // ════════════════════════════════════
                // 15. Tabs
                // ════════════════════════════════════
                Section { title: "Tabs 标签页".to_string(),
                    Row {
                        DemoCard { title: "基本用法".to_string(),
                            TabsBasicDemo {}
                        }
                        DemoCard { title: "禁用标签".to_string(),
                            TabsDisabledDemo {}
                        }
                    }
                }

                // ════════════════════════════════════
                // 16. Alert
                // ════════════════════════════════════
                Section { title: "Alert 警告提示".to_string(),
                    Row {
                        DemoCard { title: "全局横幅（点击触发）".to_string(),
                            AlertBannerDemo {}
                        }
                        DemoCard { title: "内联提示（始终显示）".to_string(),
                            div { style: "display: flex; flex-direction: column; gap: 12px;",
                                Alert { r#type: AlertType::Info, title: "系统通知".to_string(), description: "新版本已发布，建议立即更新。".to_string() }
                                Alert { r#type: AlertType::Success, title: "保存成功".to_string(), description: "您的数据已成功保存到服务器。".to_string(), closable: true }
                                Alert { r#type: AlertType::Warning, description: "当前网络不稳定，部分功能可能受限。".to_string() }
                            }
                        }
                    }
                }

                // ════════════════════════════════════
                // 17. Breadcrumb
                // ════════════════════════════════════
                Section { title: "Breadcrumb 面包屑".to_string(),
                    Row {
                        DemoCard { title: "基本用法".to_string(),
                            Breadcrumb {
                                BreadcrumbItem { href: "#".to_string(), "首页" }
                                BreadcrumbItem { href: "#".to_string(), "组件" }
                                BreadcrumbItem { "面包屑" }
                            }
                        }
                        DemoCard { title: "自定义分隔符".to_string(),
                            Breadcrumb { separator: ">".to_string(),
                                BreadcrumbItem { href: "#".to_string(), "Home" }
                                BreadcrumbItem { href: "#".to_string(), "Library" }
                                BreadcrumbItem { "Data" }
                            }
                        }
                    }
                }

                // ════════════════════════════════════
                // 18. Pagination
                // ════════════════════════════════════
                Section { title: "Pagination 分页".to_string(),
                    Row {
                        DemoCard { title: "基本用法".to_string(),
                            PaginationBasicDemo {}
                        }
                        DemoCard { title: "大数据量".to_string(),
                            Pagination { total: 200, page_size: 10 }
                        }
                    }
                }

                // ════════════════════════════════════
                // 19. Message
                // ════════════════════════════════════
                Section { title: "Message 全局提示".to_string(),
                    Row {
                        DemoCard { title: "点击触发全局提示".to_string(),
                            MessageTriggerDemo {}
                        }
                        DemoCard { title: "不同位置".to_string(),
                            MessagePositionDemo {}
                        }
                    }
                }

                // ════════════════════════════════════
                // 主题定制说明
                // ════════════════════════════════════
                Section { title: "主题定制".to_string(),
                    Card {
                        style: "background: var(--ctrl-bg-secondary); border: none;".to_string(),
                        p { style: "color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm); margin: 0 0 8px;",
                            "Ctrl UI 通过 CSS 变量实现主题定制。直接覆盖 :root 中的 CSS 变量即可修改全局样式："
                        }
                        code { style: "display: block; background: var(--ctrl-bg); padding: 12px; border-radius: var(--ctrl-radius-sm); font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);",
                            ":root {{\n    --ctrl-primary: #FF6B35;\n    --ctrl-primary-hover: #E55A2B;\n    --ctrl-radius-md: 8px;\n    --ctrl-font-family: \"PingFang SC\", sans-serif;\n}}"
                        }
                    }
                }
            }
        }
    }
}

// ════════════════════════════════════════
// 布局组件
// ════════════════════════════════════════

#[component]
fn Section(title: String, children: Element) -> Element {
    rsx! {
        div { style: "margin-bottom: 40px;",
            h2 { style: "font-size: 1.125rem; font-weight: 600; color: var(--ctrl-text); margin-bottom: 16px; padding-bottom: 8px; border-bottom: 2px solid var(--ctrl-primary);",
                "{title}"
            }
            {children}
        }
    }
}

#[component]
fn Row(children: Element) -> Element {
    rsx! {
        div { style: "display: flex; gap: 16px; margin-bottom: 16px;",
            {children}
        }
    }
}

#[component]
fn DemoCard(title: String, children: Element) -> Element {
    rsx! {
        div { style: "flex: 1; min-width: 0; background: var(--ctrl-bg); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); padding: 16px;",
            div { style: "font-size: var(--ctrl-font-size-sm); font-weight: 600; color: var(--ctrl-text-secondary); margin-bottom: 12px;",
                "{title}"
            }
            {children}
        }
    }
}

#[component]
fn Divider() -> Element {
    rsx! {
        div { style: "height: 12px;" }
    }
}

// ════════════════════════════════════════
// 交互演示
// ════════════════════════════════════════

#[component]
fn CounterDemo() -> Element {
    let mut count = use_signal(|| 0);
    let mut disabled = use_signal(|| false);

    rsx! {
        div { style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",
            Button { variant: Variant::Primary, disabled: disabled(), onclick: move |_| count.set(count() + 1), "点击 ({count()})" }
            Button { variant: Variant::Ghost, onclick: move |_| disabled.set(!disabled()), if disabled() { "恢复" } else { "禁用" } }
            Button { variant: Variant::Outline, onclick: move |_| count.set(0), "重置" }
        }
    }
}

#[component]
fn InputBasicDemo() -> Element {
    let mut value = use_signal(|| String::new());
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px;",
            Input { placeholder: "请输入内容", value: value(), oninput: move |evt: FormEvent| value.set(evt.value()) }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "输入: {value()}" }
        }
    }
}

#[component]
fn FormDemo() -> Element {
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut errors = use_signal(|| (false, false));
    let mut submitted = use_signal(|| false);

    let validate = move |_| {
        let e = (name().trim().is_empty(), !email().contains('@'));
        errors.set(e);
        if !e.0 && !e.1 { submitted.set(true); }
    };

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px;",
            Input { placeholder: "姓名", value: name(), error: errors().0, oninput: move |evt: FormEvent| name.set(evt.value()) }
            Input { placeholder: "邮箱", value: email(), error: errors().1, oninput: move |evt: FormEvent| email.set(evt.value()) }
            Button { variant: Variant::Primary, onclick: validate, "验证" }
            if submitted() {
                span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-success);", "✓ 验证通过" }
            }
        }
    }
}

#[component]
fn SwitchBasicDemo() -> Element {
    let mut on = use_signal(|| false);
    rsx! {
        div { style: "display: flex; gap: 12px; align-items: center;",
            Switch { checked: on(), onchange: move |v| on.set(v) }
            span { style: "font-size: var(--ctrl-font-size-md);", if on() { "已开启" } else { "已关闭" } }
        }
    }
}

#[component]
fn SwitchLinkedDemo() -> Element {
    let mut power = use_signal(|| false);
    let mut light = use_signal(|| false);
    let mut fan = use_signal(|| false);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px;",
            div { style: "display: flex; gap: 12px; align-items: center;",
                Switch { checked: power(), onchange: move |v| { power.set(v); if !v { light.set(false); fan.set(false); } } }
                span { style: "font-size: var(--ctrl-font-size-md);", "总电源" }
            }
            div { style: "display: flex; gap: 12px; align-items: center;",
                Switch { disabled: !power(), checked: light(), onchange: move |v| light.set(v) }
                span { style: "font-size: var(--ctrl-font-size-md);", if !power() { "灯 (请先开电源)" } else if light() { "灯 已开" } else { "灯 已关" } }
            }
            div { style: "display: flex; gap: 12px; align-items: center;",
                Switch { disabled: !power(), checked: fan(), onchange: move |v| fan.set(v) }
                span { style: "font-size: var(--ctrl-font-size-md);", if !power() { "风扇 (请先开电源)" } else if fan() { "风扇 已开" } else { "风扇 已关" } }
            }
        }
    }
}

#[component]
fn CheckboxBasicDemo() -> Element {
    let mut checked = use_signal(|| false);
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 6px;",
            Checkbox { checked: checked(), label: "同意使用协议".to_string(), onchange: move |v| checked.set(v) }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                if checked() { "已同意" } else { "未同意" }
            }
        }
    }
}

#[component]
fn CheckAllDemoContainer() -> Element {
    let items: Vec<&'static str> = vec!["选项 A", "选项 B", "选项 C"];
    let items_len = items.len();
    let mut checked = use_signal(|| vec![false; items_len]);

    let all = checked().iter().all(|&c| c);
    let some = checked().iter().any(|&c| c);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 4px;",
            Checkbox { checked: all, indeterminate: some && !all, label: "全选".to_string(), onchange: move |v| checked.set(vec![v; items_len]) }
            div { style: "height: 1px; background: var(--ctrl-border); margin: 2px 0;" }
            {items.iter().enumerate().map(|(i, item)| {
                rsx! {
                    Checkbox { key: "{i}", checked: checked()[i], label: item.to_string(), onchange: move |v| { let mut c = checked(); c[i] = v; checked.set(c); } }
                }
            })}
        }
    }
}

#[component]
fn RadioBasicDemo() -> Element {
    let mut selected = use_signal(|| "a".to_string());
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 4px;",
            Radio { value: "a".to_string(), label: "选项 A".to_string(), checked: selected() == "a", onchange: move |v| selected.set(v) }
            Radio { value: "b".to_string(), label: "选项 B".to_string(), checked: selected() == "b", onchange: move |v| selected.set(v) }
            Radio { value: "c".to_string(), label: "选项 C".to_string(), checked: selected() == "c", onchange: move |v| selected.set(v) }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin-top: 4px;", "当前: {selected()}" }
        }
    }
}

#[component]
fn SelectBasicDemo() -> Element {
    let mut value = use_signal(|| String::new());
    let options = vec![
        ("rust".to_string(), "Rust".to_string(), false),
        ("go".to_string(), "Go".to_string(), false),
        ("js".to_string(), "JavaScript".to_string(), false),
        ("py".to_string(), "Python（禁用）".to_string(), true),
    ];
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px;",
            Select { options: options, placeholder: "选择语言".to_string(), value: value(), onchange: move |v| value.set(v) }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                if value().is_empty() { "未选择" } else { "已选择: {value()}" }
            }
        }
    }
}

#[component]
fn DialogBasicDemo() -> Element {
    let mut visible = use_signal(|| false);
    rsx! {
        div {
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| visible.set(true), "打开对话框" }
            Dialog { visible: visible(), title: "提示".to_string(), onclose: move |_| visible.set(false),
                p { style: "margin: 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);",
                    "这是一个对话框示例，点击遮罩或关闭按钮可关闭。"
                }
            }
        }
    }
}

#[component]
fn DialogConfirmDemo() -> Element {
    let mut visible = use_signal(|| false);
    let mut confirmed = use_signal(|| false);

    rsx! {
        div {
            Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| visible.set(true), "确认对话框" }
            Dialog {
                visible: visible(),
                title: "确认操作".to_string(),
                onclose: move |_| visible.set(false),
                footer: rsx! {
                    Button { variant: Variant::Ghost, size: Size::Sm, onclick: move |_| visible.set(false), "取消" }
                    Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| { visible.set(false); confirmed.set(true); }, "确定" }
                },
                p { style: "margin: 0; font-size: var(--ctrl-font-size-sm);", "确定要删除这条记录吗？此操作不可恢复。" }
            }
            if confirmed() {
                span { style: "display: block; margin-top: 8px; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-success);", "✓ 操作已确认" }
            }
        }
    }
}

#[component]
fn ProgressDynamicDemo() -> Element {
    let mut percent = use_signal(|| 30.0);
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 12px;",
            Progress { percent: percent(), color: "var(--ctrl-primary)".to_string(), show_text: true }
            div { style: "display: flex; gap: 8px;",
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| { let p = percent(); if p > 0.0 { percent.set((p - 10.0).max(0.0)); } }, "-" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| { let p = percent(); if p < 100.0 { percent.set((p + 10.0).min(100.0)); } }, "+" }
            }
        }
    }
}

#[component]
fn TabsBasicDemo() -> Element {
    let mut active = use_signal(|| "tab1".to_string());
    let items = vec![
        ("tab1".to_string(), "标签一".to_string(), false),
        ("tab2".to_string(), "标签二".to_string(), false),
        ("tab3".to_string(), "标签三".to_string(), false),
    ];
    rsx! {
        div { style: "width: 100%;",
            TabNav { items: items, active: active(), onchange: move |v| active.set(v) }
            TabContent {
                div { style: "padding: 8px 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);",
                    if active() == "tab1" { "这是标签一的内容" }
                    else if active() == "tab2" { "这是标签二的内容" }
                    else { "这是标签三的内容" }
                }
            }
        }
    }
}

#[component]
fn TabsDisabledDemo() -> Element {
    let mut active = use_signal(|| "d1".to_string());
    let items = vec![
        ("d1".to_string(), "可用".to_string(), false),
        ("d2".to_string(), "禁用".to_string(), true),
        ("d3".to_string(), "可用".to_string(), false),
    ];
    rsx! {
        div { style: "width: 100%;",
            TabNav { items: items, active: active(), onchange: move |v| active.set(v) }
            TabContent {
                div { style: "padding: 8px 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);",
                    "当前: {active()}"
                }
            }
        }
    }
}

#[component]
fn PaginationBasicDemo() -> Element {
    let mut page = use_signal(|| 1u32);
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px;",
            Pagination { current: page(), total: 50, page_size: 10, onchange: move |v| page.set(v) }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "当前第 {page()} 页" }
        }
    }
}

#[component]
fn AlertBannerDemo() -> Element {
    const MAX_ALERTS: usize = 5;

    type AlertItem = (u32, AlertType, String, String, bool); // id, type, title, desc, closing

    let mut alerts = use_signal(|| Vec::<AlertItem>::new());
    let mut next_id = use_signal(|| 0u32);

    let titles = vec!["系统维护通知", "存储空间不足", "网络连接失败"];
    let descs = vec![
        "今晚 22:00 - 23:00 将进行系统升级，届时服务将暂停。",
        "您的存储空间已使用 95%，请尽快清理。",
        "无法连接到服务器，请检查网络后重试。",
    ];
    let types = [AlertType::Info, AlertType::Warning, AlertType::Error];

    let mut add_alert = move || {
        let idx = next_id() as usize % 3;
        let id = next_id();
        next_id.set(id + 1);

        let mut list = alerts.write();
        // 只统计未在 closing 状态的有效条目
        let active_count = list.iter().filter(|(_, _, _, _, c)| !c).count();
        if active_count >= MAX_ALERTS {
            // 标记最旧的一条未 closing 条目为 closing，触发它的退出动画
            if let Some(oldest) = list.iter_mut().find(|(_, _, _, _, c)| !*c) {
                oldest.4 = true;
            }
        }
        list.push((id, types[idx].clone(), titles[idx].to_string(), descs[idx].to_string(), false));
    };

    rsx! {
        AlertBannerContainer {
            for (id, a_type, title, desc, closing) in alerts().iter() {
                {
                    let alert_id = *id;
                    let t = a_type.clone();
                    let ti = title.clone();
                    let d = desc.clone();
                    let c = *closing;
                    rsx! {
                        Alert {
                            key: "{alert_id}",
                            r#type: t,
                            title: ti,
                            description: d,
                            mode: AlertMode::Banner,
                            closable: true,
                            closing: c,
                            duration: 5000,
                            onclose: move |_| alerts.write().retain(|(aid, _, _, _, _)| *aid != alert_id),
                        }
                    }
                }
            }
        }
        div { style: "display: flex; flex-direction: column; gap: 12px;",
            p { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin: 0 0 4px;", "多次点击累加显示，最多 5 条，超出时从最旧的开始依次退出。" }
            div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| add_alert(), "触发横幅" }
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前横幅数量: {alerts().len()}（上限 {MAX_ALERTS}）"
            }
        }
    }
}

#[component]
fn MessageTriggerDemo() -> Element {
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
            p { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin: 0 0 4px;", "多次点击累加显示，最多 5 条，超出时从最旧的开始依次退出。" }
            div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| add_message(MessageType::Info, "这是一条信息提示"), "Info" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| add_message(MessageType::Success, "操作成功完成！"), "Success" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| add_message(MessageType::Warning, "请注意数据安全"), "Warning" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| add_message(MessageType::Error, "操作失败，请重试"), "Error" }
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前消息数量: {messages().len()}（上限 {MAX_MESSAGES}）"
            }
        }
    }
}

#[component]
fn MessagePositionDemo() -> Element {
    const MAX_MESSAGES: usize = 5;

    type PosMsgItem = (u32, MessagePlacement, bool); // id, placement, closing

    let mut messages = use_signal(|| Vec::<PosMsgItem>::new());
    let next_id = use_signal(|| 0u32);
    let mut pos_placement = use_signal(|| MessagePlacement::Top);

    let mut add_message = {
        let mut messages = messages.clone();
        let mut next_id = next_id.clone();
        let pos_placement = pos_placement.clone();
        move || {
            let id = next_id();
            next_id.set(id + 1);

            let mut list = messages.write();
            let active_count = list.iter().filter(|(_, _, cl)| !cl).count();
            if active_count >= MAX_MESSAGES {
                if let Some(oldest) = list.iter_mut().find(|(_, _, cl)| !*cl) {
                    oldest.2 = true;
                }
            }
            list.push((id, pos_placement(), false));
        }
    };

    rsx! {
        MessageContainer {
            placement: pos_placement(),
            for (id, _, closing) in messages().iter() {
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
                            onclose: move |_| messages.write().retain(|(mid, _, _)| *mid != msg_id),
                        }
                    }
                }
            }
        }
        div { style: "display: flex; flex-direction: column; gap: 12px;",
            p { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin: 0 0 4px;", "选择位置后点击发送，最多 5 条，超出时从最旧的开始依次退出。" }
            div { style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",
                Button { variant: if pos_placement() == MessagePlacement::Top { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::Top), "顶部居中" }
                Button { variant: if pos_placement() == MessagePlacement::TopRight { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::TopRight), "顶部靠右" }
                Button { variant: if pos_placement() == MessagePlacement::TopLeft { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::TopLeft), "顶部靠左" }
                Button { variant: if pos_placement() == MessagePlacement::Bottom { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::Bottom), "底部居中" }
            }
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| add_message(), "发送消息" }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前消息数量: {messages().len()}（上限 {MAX_MESSAGES}）"
            }
        }
    }
}
