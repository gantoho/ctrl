use dioxus::prelude::*;

use crate::{Route, apply_theme};

/// 文档页面通用布局 —— 固定侧边栏 + 顶栏 + 居中内容区
#[component]
#[allow(non_snake_case)]
pub fn DocsLayout() -> Element {
    let mut is_dark = use_context::<Signal<bool>>();

    rsx! {
        div {
            style: "display: flex; min-height: 100vh; background: var(--ctrl-bg); color: var(--ctrl-text);",

            // ── 固定侧边栏（sticky）──
            nav {
                style: "width: 260px; flex-shrink: 0; border-right: 1px solid var(--ctrl-border); position: sticky; top: 0; height: 100vh; display: flex; flex-direction: column; overflow-y: auto;",
                SidebarContent { }
            }

            // ── 右侧区域 ──
            div {
                style: "flex: 1; display: flex; flex-direction: column; min-width: 0;",

                header {
                    style: "height: 56px; border-bottom: 1px solid var(--ctrl-border); display: flex; align-items: center; justify-content: flex-end; padding: 0 24px; flex-shrink: 0;",
                    button {
                        style: "display: inline-flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: none; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-sm); cursor: pointer; font-size: 1rem; color: var(--ctrl-text); outline: none; appearance: none; -webkit-appearance: none;",
                        onclick: move |_| {
                            let next = !is_dark();
                            is_dark.set(next);
                            apply_theme(next);
                        },
                        if is_dark() { "☀️" } else { "🌙" }
                    }
                }

                div {
                    style: "flex: 1; display: flex; justify-content: center; overflow-y: auto;",
                    main {
                        style: "width: 100%; max-width: 960px; padding: 32px 24px;",
                        Outlet::<Route> {}
                    }
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn SidebarContent() -> Element {
    let navigator = use_navigator();
    let current_route = use_route::<Route>();

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 4px; padding: 0 12px;",
            div { style: "padding: 8px 12px; margin-bottom: 16px;",
                h2 { style: "font-size: 1.25rem; font-weight: 700; color: var(--ctrl-text); cursor: pointer;",
                    onclick: move |_| { let _ = navigator.push(Route::HomePage {}); },
                    "Ctrl UI"
                }
                p { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin: 4px 0 0 0;", "Dioxus 组件库" }
            }

            GroupTitle { title: "开始".to_string() }
            NavItem { label: "介绍".to_string(), target: Route::HomePage {}, current: current_route.clone() }

            GroupTitle { title: "组件".to_string() }
            NavItem { label: "组件总览".to_string(), target: Route::ComponentsIndex {}, current: current_route.clone() }
            NavItem { label: "Button 按钮".to_string(), target: Route::ComponentPage { name: "button".to_string() }, current: current_route.clone() }
            NavItem { label: "Input 输入框".to_string(), target: Route::ComponentPage { name: "input".to_string() }, current: current_route.clone() }
            NavItem { label: "Textarea 多行输入".to_string(), target: Route::ComponentPage { name: "textarea".to_string() }, current: current_route.clone() }
            NavItem { label: "Result 结果页".to_string(), target: Route::ComponentPage { name: "result".to_string() }, current: current_route.clone() }
            NavItem { label: "Statistic 统计数值".to_string(), target: Route::ComponentPage { name: "statistic".to_string() }, current: current_route.clone() }
            NavItem { label: "Descriptions 描述列表".to_string(), target: Route::ComponentPage { name: "descriptions".to_string() }, current: current_route.clone() }
            NavItem { label: "Grid 栅格布局".to_string(), target: Route::ComponentPage { name: "grid".to_string() }, current: current_route.clone() }
            NavItem { label: "Switch 开关".to_string(), target: Route::ComponentPage { name: "switch".to_string() }, current: current_route.clone() }
            NavItem { label: "Checkbox 复选框".to_string(), target: Route::ComponentPage { name: "checkbox".to_string() }, current: current_route.clone() }
            NavItem { label: "Radio 单选框".to_string(), target: Route::ComponentPage { name: "radio".to_string() }, current: current_route.clone() }
            NavItem { label: "Select 下拉选择".to_string(), target: Route::ComponentPage { name: "select".to_string() }, current: current_route.clone() }
            NavItem { label: "Tag 标签".to_string(), target: Route::ComponentPage { name: "tag".to_string() }, current: current_route.clone() }
            NavItem { label: "Card 卡片".to_string(), target: Route::ComponentPage { name: "card".to_string() }, current: current_route.clone() }
            NavItem { label: "Dialog 对话框".to_string(), target: Route::ComponentPage { name: "dialog".to_string() }, current: current_route.clone() }
            NavItem { label: "Table 表格".to_string(), target: Route::ComponentPage { name: "table".to_string() }, current: current_route.clone() }
            NavItem { label: "Badge 徽标".to_string(), target: Route::ComponentPage { name: "badge".to_string() }, current: current_route.clone() }
            NavItem { label: "Avatar 头像".to_string(), target: Route::ComponentPage { name: "avatar".to_string() }, current: current_route.clone() }
            NavItem { label: "Progress 进度条".to_string(), target: Route::ComponentPage { name: "progress".to_string() }, current: current_route.clone() }
            NavItem { label: "Tooltip 气泡提示".to_string(), target: Route::ComponentPage { name: "tooltip".to_string() }, current: current_route.clone() }
            NavItem { label: "Tabs 标签页".to_string(), target: Route::ComponentPage { name: "tabs".to_string() }, current: current_route.clone() }
            NavItem { label: "Alert 警告提示".to_string(), target: Route::ComponentPage { name: "alert".to_string() }, current: current_route.clone() }
            NavItem { label: "Breadcrumb 面包屑".to_string(), target: Route::ComponentPage { name: "breadcrumb".to_string() }, current: current_route.clone() }
            NavItem { label: "Pagination 分页".to_string(), target: Route::ComponentPage { name: "pagination".to_string() }, current: current_route.clone() }
            NavItem { label: "Message 全局提示".to_string(), target: Route::ComponentPage { name: "message".to_string() }, current: current_route.clone() }
            NavItem { label: "Divider 分割线".to_string(), target: Route::ComponentPage { name: "divider".to_string() }, current: current_route.clone() }
            NavItem { label: "Loading 加载中".to_string(), target: Route::ComponentPage { name: "loading".to_string() }, current: current_route.clone() }
            NavItem { label: "Empty 空状态".to_string(), target: Route::ComponentPage { name: "empty".to_string() }, current: current_route.clone() }
            NavItem { label: "Skeleton 骨架屏".to_string(), target: Route::ComponentPage { name: "skeleton".to_string() }, current: current_route.clone() }
            NavItem { label: "Backtop 回到顶部".to_string(), target: Route::ComponentPage { name: "backtop".to_string() }, current: current_route.clone() }
            NavItem { label: "Collapse 折叠面板".to_string(), target: Route::ComponentPage { name: "collapse".to_string() }, current: current_route.clone() }
            NavItem { label: "Popover 气泡卡片".to_string(), target: Route::ComponentPage { name: "popover".to_string() }, current: current_route.clone() }
            NavItem { label: "Drawer 抽屉".to_string(), target: Route::ComponentPage { name: "drawer".to_string() }, current: current_route.clone() }
            NavItem { label: "Notification 通知".to_string(), target: Route::ComponentPage { name: "notification".to_string() }, current: current_route.clone() }
            NavItem { label: "Dropdown 下拉菜单".to_string(), target: Route::ComponentPage { name: "dropdown".to_string() }, current: current_route.clone() }
            NavItem { label: "Menu 导航菜单".to_string(), target: Route::ComponentPage { name: "menu".to_string() }, current: current_route.clone() }
            NavItem { label: "Steps 步骤条".to_string(), target: Route::ComponentPage { name: "steps".to_string() }, current: current_route.clone() }
            NavItem { label: "Timeline 时间线".to_string(), target: Route::ComponentPage { name: "timeline".to_string() }, current: current_route.clone() }
            NavItem { label: "Slider 滑块".to_string(), target: Route::ComponentPage { name: "slider".to_string() }, current: current_route.clone() }
            NavItem { label: "Rate 评分".to_string(), target: Route::ComponentPage { name: "rate".to_string() }, current: current_route.clone() }
            NavItem { label: "Image 图片".to_string(), target: Route::ComponentPage { name: "image".to_string() }, current: current_route.clone() }
            NavItem { label: "Space 间距".to_string(), target: Route::ComponentPage { name: "space".to_string() }, current: current_route.clone() }
            NavItem { label: "Segmented 分段".to_string(), target: Route::ComponentPage { name: "segmented".to_string() }, current: current_route.clone() }
            NavItem { label: "InputNumber 数字输入".to_string(), target: Route::ComponentPage { name: "input_number".to_string() }, current: current_route.clone() }
            NavItem { label: "Upload 上传".to_string(), target: Route::ComponentPage { name: "upload".to_string() }, current: current_route.clone() }
            NavItem { label: "Carousel 走马灯".to_string(), target: Route::ComponentPage { name: "carousel".to_string() }, current: current_route.clone() }
            NavItem { label: "Form 表单".to_string(), target: Route::ComponentPage { name: "form".to_string() }, current: current_route.clone() }
            NavItem { label: "Tree 树形控件".to_string(), target: Route::ComponentPage { name: "tree".to_string() }, current: current_route.clone() }
            NavItem { label: "DatePicker 日期选择".to_string(), target: Route::ComponentPage { name: "date_picker".to_string() }, current: current_route.clone() }

            GroupTitle { title: "指南".to_string() }
            NavItem { label: "主题定制".to_string(), target: Route::ThemePage {}, current: current_route.clone() }
        }
    }
}

#[component]
fn GroupTitle(title: String) -> Element {
    rsx! {
        div { style: "padding: 12px 12px 6px; font-size: var(--ctrl-font-size-sm); font-weight: 600; color: var(--ctrl-text-disabled); text-transform: uppercase; letter-spacing: 0.05em;", "{title}" }
    }
}

#[component]
fn NavItem(
    label: String,
    target: Route,
    current: Route,
) -> Element {
    let navigator = use_navigator();

    let is_active = current == target;

    let bg = if is_active { "var(--ctrl-primary-light)" } else { "transparent" };
    let color = if is_active { "var(--ctrl-primary)" } else { "var(--ctrl-text-secondary)" };
    let weight = if is_active { "600" } else { "400" };

    let t = target;

    rsx! {
        button {
            style: "width: 100%; text-align: left; padding: 8px 12px; background: {bg}; border: none; border-radius: var(--ctrl-radius-sm); cursor: pointer; font-size: var(--ctrl-font-size-md); font-weight: {weight}; color: {color}; font-family: inherit; transition: background 0.15s ease; outline: none; appearance: none; -webkit-appearance: none;",
            onclick: move |_| {
                let _ = navigator.push(t.clone());
            },
            "{label}"
        }
    }
}
