use dioxus::prelude::*;
use ctrl::prelude::*;
use crate::Route;

#[component]
#[allow(non_snake_case)]
pub fn DocsLayout() -> Element {
    let mut is_dark = use_context::<Signal<bool>>();

    rsx! {
        Row {
            Col {
                span: 4,
                style: "height:100vh; overflow-y:auto; border-right:1px solid var(--ctrl-border);",
                SidebarContent {}
            }
            Col {
                span: 20,
                class: "docs-scroll-container".to_string(),
                style: "height:100vh; overflow-y:auto;",
                div { style: "display:flex; justify-content:flex-end; padding:12px 24px 0;",
                    Button {
                        variant: Variant::Ghost,
                        size: Size::Sm,
                        onclick: move |_| {
                            let next = !is_dark();
                            is_dark.set(next);
                            crate::apply_theme(next);
                        },
                        if is_dark() { "☀️" } else { "🌙" }
                    }
                }
                div { class: "docs-content", style: "padding:8px 40px 48px; width: 100%; max-width:1000px; margin:0 auto;",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn SidebarContent() -> Element {
    let navigator = use_navigator();
    let route = use_route::<Route>();

    rsx! {
        Space { gap: "xs".to_string(), direction: Direction::Vertical, style: "padding:20px 20px 12px;",
            h2 {
                style: "cursor:pointer; margin:0;".to_string(),
                onclick: { let nav = navigator.clone(); move |_| { let _ = nav.push(Route::HomePage {}); } },
                "Ctrl UI"
            }
            p { style: "margin:0; color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);", "Dioxus 组件库" }
        }

        Menu { direction: Direction::Vertical, class: "docs-sidebar-menu".to_string(),

            Divider { content: "开始".to_string() }
            SidebarItem { label: "介绍".to_string(), route: Route::HomePage {}, current: route.clone() }
            SidebarItem { label: "组件总览".to_string(), route: Route::ComponentsIndex {}, current: route.clone() }

            for (category, items) in crate::pages::components::component_catalog() {
                Divider { content: category.to_string() }
                for (name, title, _desc) in items {
                    SidebarItem {
                        label: title.to_string(),
                        route: Route::ComponentPage { name: name.to_string() },
                        current: route.clone(),
                    }
                }
            }

            Divider { content: "指南".to_string() }
            SidebarItem { label: "主题定制".to_string(), route: Route::ThemePage {}, current: route.clone() }
        }
    }
}

#[component]
fn SidebarItem(label: String, route: Route, current: Route) -> Element {
    let navigator = use_navigator();
    let class = if current == route { "ctrl-menu__item--active".to_string() } else { String::new() };
    rsx! {
        MenuItem {
            class,
            onclick: move |_| { let _ = navigator.push(route.clone()); },
            "{label}"
        }
    }
}
