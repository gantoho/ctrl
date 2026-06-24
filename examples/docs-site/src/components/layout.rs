use dioxus::prelude::*;

use crate::{Route, ActiveSection};

/// 文档页面通用布局 —— 固定侧边栏 + 顶栏 + 居中内容区
#[component]
#[allow(non_snake_case)]
pub fn DocsLayout() -> Element {
    let mut is_dark = use_context::<Signal<bool>>();
    let ActiveSection(active_section) = use_context::<ActiveSection>();

    // 滚动监听：轮询检测当前可见的 section 并更新信号
    use_effect(move || {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = active_section;
        }
        #[cfg(target_arch = "wasm32")]
        {
            js_sys::eval(r#"
                if (!window.__scrollSpyInited) {
                    window.__scrollSpyInited = true;
                    window.__activeSection = '';
                    window.__scrollSpySections = ['button', 'input', 'switch', 'checkbox', 'radio', 'select', 'tag', 'card', 'dialog', 'table', 'badge', 'avatar', 'progress', 'tooltip', 'tabs', 'alert', 'breadcrumb', 'pagination', 'message'];
                    window.__updateActiveSection = function() {
                        for (const id of window.__scrollSpySections) {
                            const el = document.getElementById(id);
                            if (el) {
                                const rect = el.getBoundingClientRect();
                                if (rect.top <= 100) {
                                    window.__activeSection = id;
                                }
                            }
                        }
                    };
                    window.addEventListener('scroll', window.__updateActiveSection, { passive: true });
                }
            "#).ok();

            let mut active = active_section.clone();
            spawn(async move {
                use gloo_timers::future::TimeoutFuture;
                loop {
                    TimeoutFuture::new(200).await;
                    let js = "window.__activeSection || ''";
                    if let Ok(val) = js_sys::eval(js) {
                        let s = val.as_string().unwrap_or_default();
                        let new_val = if s.is_empty() { None } else { Some(s) };
                        if active.peek().as_deref() != new_val.as_deref() {
                            active.set(new_val);
                        }
                    }
                }
            });
        }
    });

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
                        onclick: move |_| is_dark.set(!is_dark()),
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
    let ActiveSection(active_section) = use_context::<ActiveSection>();

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
            NavItem { label: "介绍".to_string(), target: Route::HomePage {}, current: current_route.clone(), hash: None::<String>, active_section: active_section }

            GroupTitle { title: "组件".to_string() }
            NavItem { label: "Button 按钮".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("button".to_string()), active_section: active_section }
            NavItem { label: "Input 输入框".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("input".to_string()), active_section: active_section }
            NavItem { label: "Switch 开关".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("switch".to_string()), active_section: active_section }
            NavItem { label: "Checkbox 复选框".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("checkbox".to_string()), active_section: active_section }
            NavItem { label: "Radio 单选框".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("radio".to_string()), active_section: active_section }
            NavItem { label: "Select 下拉选择".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("select".to_string()), active_section: active_section }
            NavItem { label: "Tag 标签".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("tag".to_string()), active_section: active_section }
            NavItem { label: "Card 卡片".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("card".to_string()), active_section: active_section }
            NavItem { label: "Dialog 对话框".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("dialog".to_string()), active_section: active_section }
            NavItem { label: "Table 表格".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("table".to_string()), active_section: active_section }
            NavItem { label: "Badge 徽标".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("badge".to_string()), active_section: active_section }
            NavItem { label: "Avatar 头像".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("avatar".to_string()), active_section: active_section }
            NavItem { label: "Progress 进度条".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("progress".to_string()), active_section: active_section }
            NavItem { label: "Tooltip 气泡提示".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("tooltip".to_string()), active_section: active_section }
            NavItem { label: "Tabs 标签页".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("tabs".to_string()), active_section: active_section }
            NavItem { label: "Alert 警告提示".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("alert".to_string()), active_section: active_section }
            NavItem { label: "Breadcrumb 面包屑".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("breadcrumb".to_string()), active_section: active_section }
            NavItem { label: "Pagination 分页".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("pagination".to_string()), active_section: active_section }
            NavItem { label: "Message 全局提示".to_string(), target: Route::ComponentsPage {}, current: current_route.clone(), hash: Some("message".to_string()), active_section: active_section }

            GroupTitle { title: "指南".to_string() }
            NavItem { label: "主题定制".to_string(), target: Route::ThemePage {}, current: current_route.clone(), hash: None::<String>, active_section: active_section }
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
    hash: Option<String>,
    active_section: Signal<Option<String>>,
) -> Element {
    let navigator = use_navigator();

    let is_active = if let Some(ref section_id) = hash {
        active_section().as_deref() == Some(section_id)
    } else {
        current == target
    };

    let bg = if is_active { "var(--ctrl-primary-light)" } else { "transparent" };
    let color = if is_active { "var(--ctrl-primary)" } else { "var(--ctrl-text-secondary)" };
    let weight = if is_active { "600" } else { "400" };

    let t = target;
    let h = hash.clone();

    rsx! {
        button {
            style: "width: 100%; text-align: left; padding: 8px 12px; background: {bg}; border: none; border-radius: var(--ctrl-radius-sm); cursor: pointer; font-size: var(--ctrl-font-size-md); font-weight: {weight}; color: {color}; font-family: inherit; transition: background 0.15s ease; outline: none; appearance: none; -webkit-appearance: none;",
            onclick: move |_| {
                let _ = navigator.push(t.clone());
                if let Some(ref h) = h {
                    js_scroll_to(h.clone());
                }
            },
            "{label}"
        }
    }
}

/// 滚动到指定元素
fn js_scroll_to(hash: String) {
    #[cfg(target_arch = "wasm32")]
    {
        let mut h = hash;
        if !h.starts_with('#') {
            h = format!("#{}", h);
        }
        let js_str = format!(
            "document.querySelector('{}')?.scrollIntoView({{ behavior: 'smooth', block: 'start' }});",
            h
        );
        let _ = js_sys::eval(&js_str);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = hash;
    }
}
