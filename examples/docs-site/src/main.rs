use dioxus::prelude::*;
use ctrl::prelude::*;

mod components;
mod pages;

use components::layout::DocsLayout;
use pages::home::HomePage;
use pages::components::{ComponentPage, ComponentsIndex};
use pages::theme::ThemePage;

/// 所有组件 CSS 样式已由各自组件内嵌（include_str!），
/// 用户无需手动加载任何样式文件即可使用所有组件。
fn main() {
    dioxus::launch(App);
}

/// 路由 —— 组件独立路由 /components/xxx
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(DocsLayout)]
    #[route("/")]
    HomePage {},
    #[route("/components")]
    ComponentsIndex {},
    #[route("/components/:name")]
    ComponentPage { name: String },
    #[route("/theme")]
    ThemePage {},
}

/// 全局活动章节上下文 —— 用于侧边栏滚动监听高亮
#[derive(Clone, Copy)]
pub struct ActiveSection(pub Signal<Option<String>>);

/// 从 localStorage 读取主题偏好
fn read_theme_preference() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        let val = js_sys::eval("localStorage.getItem('ctrl-theme')")
            .ok()
            .and_then(|v| v.as_string());
        return val.map(|s| s == "dark").unwrap_or(false);
    }
    #[cfg(not(target_arch = "wasm32"))]
    false
}

/// 持久化主题偏好并设置 data-theme 属性
pub fn apply_theme(is_dark: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        let theme = if is_dark { "dark" } else { "light" };
        let _ = js_sys::eval(&format!("localStorage.setItem('ctrl-theme', '{theme}')"));
        let attr = if is_dark { "dark" } else { "" };
        let _ = js_sys::eval(&format!("document.documentElement.setAttribute('data-theme', '{attr}')"));
    }
}

#[allow(non_snake_case)]
fn App() -> Element {
    let is_dark = use_signal(read_theme_preference);
    use_context_provider(|| is_dark);

    // 初始化 data-theme
    use_effect(move || apply_theme(is_dark()));

    // 初始化活动章节信号
    let active = Signal::new(None::<String>);
    use_context_provider(|| ActiveSection(active));

    rsx! {
        style { {r#"
            .docs-sidebar-menu.ctrl-menu { width:auto; border-right:none; background:transparent; }
        "#} }
        ThemeProvider {
            NotificationProvider {
                placement: ctrl::prelude::NotificationPlacement::TopRight,
                ImagePreviewProvider {
                    Router::<Route> {}
                }
            }
        }
    }
}
