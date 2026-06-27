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

/// 将主题偏好写入 localStorage
fn save_theme_preference(is_dark: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        let theme = if is_dark { "dark" } else { "light" };
        let _ = js_sys::eval(&format!("localStorage.setItem('ctrl-theme', '{theme}')"));
    }
}

#[allow(non_snake_case)]
fn App() -> Element {
    let is_dark = use_signal(read_theme_preference);
    use_context_provider(|| is_dark);

    // 持久化主题偏好
    use_effect(move || save_theme_preference(is_dark()));

    // 初始化活动章节信号
    let active = Signal::new(None::<String>);
    use_context_provider(|| ActiveSection(active));

    let theme = {
        let dark = is_dark();
        if dark {
            ctrl::theme::Theme {
                colors: ctrl::theme::ColorPalette {
                    primary: "#818CF8",
                    primary_hover: "#6366F1",
                    primary_active: "#4F46E5",
                    primary_light: "#1E1B4B",
                    bg: "#0F172A",
                    bg_secondary: "#1E293B",
                    text: "#F1F5F9",
                    text_secondary: "#94A3B8",
                    text_disabled: "#475569",
                    border: "#334155",
                    border_hover: "#475569",
                    ..Default::default()
                },
                ..Default::default()
            }
        } else {
            ctrl::theme::Theme::default()
        }
    };

    rsx! {
        // 所有组件 CSS 已由组件内部 include_str! 嵌入，用户无需手动加载
        // reset.css 已由 ThemeProvider 自动注入

        ThemeProvider {
            theme: theme,
            ImagePreviewProvider {
                NotificationProvider {
                    placement: NotificationPlacement::TopRight,
                    Router::<Route> {}
                }
            }
        }
    }
}
