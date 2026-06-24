use dioxus::prelude::*;
use ctrl::prelude::*;

mod components;
mod pages;

use components::layout::DocsLayout;
use pages::home::HomePage;
use pages::components::ComponentsPage;
use pages::theme::ThemePage;

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

/// 路由 —— 只有 3 个页面，所有组件放在同一个 Components 页面
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(DocsLayout)]
    #[route("/")]
    HomePage {},
    #[route("/components")]
    ComponentsPage {},
    #[route("/theme")]
    ThemePage {},
}

/// 全局活动章节上下文 —— 用于侧边栏滚动监听高亮
#[derive(Clone, Copy)]
pub struct ActiveSection(pub Signal<Option<String>>);

#[allow(non_snake_case)]
fn App() -> Element {
    let is_dark = use_signal(|| false);
    use_context_provider(|| is_dark);

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
            theme: theme,
            Router::<Route> {}
        }
    }
}
