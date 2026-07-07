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
            .docs-sidebar-menu.ctrl-menu { width:auto; border-right:none; background:transparent; padding-bottom:32px; }
            /* 侧边栏分类标题：弱化、字号更小、留白更紧凑 */
            .docs-sidebar-menu .ctrl-divider { margin:16px 0 4px; }
            .docs-sidebar-menu .ctrl-divider__text { font-size:12px; color:var(--ctrl-text-secondary); font-weight:600; letter-spacing:0.04em; }
            .docs-sidebar-menu .ctrl-menu__item { border-radius:var(--ctrl-radius-md); margin:1px 8px; padding-top:7px; padding-bottom:7px; }
            /* 文档内容区：各板块显式间距。
               注意：Card 会在 .ctrl-card 前渲染一个 <style> 兄弟节点，
               因此不能用相邻兄弟选择器，改为对每类板块单独设置外边距。 */
            .docs-content > div > h1 { margin:0 0 4px; }
            .docs-content > div > p { margin:0 0 24px; color:var(--ctrl-text-secondary); }
            .docs-content > div > h2 { margin:40px 0 16px; }
            .docs-content > div > h3 { margin:32px 0 12px; }
            .docs-content > div > .ctrl-card { margin:0 0 24px; }
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
