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
                style: "height:100vh; overflow-y:auto;",
                Space { gap: "md".to_string(), direction: Direction::Vertical, block: true,
                    div { style: "display:flex; justify-content:flex-end; padding:8px 16px;",
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
                    Divider { gap_top: "0".to_string() }
                    div { style: "padding:0 24px 32px; width: 100%; max-width:960px; margin:0 auto;",
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
    let route = use_route::<Route>();

    rsx! {
        Space { gap: "sm".to_string(), direction: Direction::Vertical, style: "padding:12px 16px;",
            h2 {
                style: "cursor:pointer;".to_string(),
                onclick: { let nav = navigator.clone(); move |_| { let _ = nav.push(Route::HomePage {}); } },
                "Ctrl UI"
            }
            p { "Dioxus 组件库" }
        }

        Menu { direction: Direction::Vertical, class: "docs-sidebar-menu".to_string(),

            Divider { content: "开始".to_string() }
            SidebarItem { label: "介绍".to_string(), route: Route::HomePage {}, current: route.clone() }

            Divider { content: "组件".to_string() }
            SidebarItem { label: "组件总览".to_string(), route: Route::ComponentsIndex {}, current: route.clone() }
            SidebarItem { label: "Button 按钮".to_string(), route: Route::ComponentPage { name: "button".to_string() }, current: route.clone() }
            SidebarItem { label: "AutoComplete 自动补全".to_string(), route: Route::ComponentPage { name: "auto_complete".to_string() }, current: route.clone() }
            SidebarItem { label: "ConfigProvider 全局配置".to_string(), route: Route::ComponentPage { name: "config_provider".to_string() }, current: route.clone() }
            SidebarItem { label: "Input 输入框".to_string(), route: Route::ComponentPage { name: "input".to_string() }, current: route.clone() }
            SidebarItem { label: "Textarea 多行输入".to_string(), route: Route::ComponentPage { name: "textarea".to_string() }, current: route.clone() }
            SidebarItem { label: "Result 结果页".to_string(), route: Route::ComponentPage { name: "result".to_string() }, current: route.clone() }
            SidebarItem { label: "Statistic 统计数值".to_string(), route: Route::ComponentPage { name: "statistic".to_string() }, current: route.clone() }
            SidebarItem { label: "Descriptions 描述列表".to_string(), route: Route::ComponentPage { name: "descriptions".to_string() }, current: route.clone() }
            SidebarItem { label: "Grid 栅格布局".to_string(), route: Route::ComponentPage { name: "grid".to_string() }, current: route.clone() }
            SidebarItem { label: "Switch 开关".to_string(), route: Route::ComponentPage { name: "switch".to_string() }, current: route.clone() }
            SidebarItem { label: "Checkbox 复选框".to_string(), route: Route::ComponentPage { name: "checkbox".to_string() }, current: route.clone() }
            SidebarItem { label: "Radio 单选框".to_string(), route: Route::ComponentPage { name: "radio".to_string() }, current: route.clone() }
            SidebarItem { label: "Select 下拉选择".to_string(), route: Route::ComponentPage { name: "select".to_string() }, current: route.clone() }
            SidebarItem { label: "Tag 标签".to_string(), route: Route::ComponentPage { name: "tag".to_string() }, current: route.clone() }
            SidebarItem { label: "Card 卡片".to_string(), route: Route::ComponentPage { name: "card".to_string() }, current: route.clone() }
            SidebarItem { label: "Cascader 级联选择".to_string(), route: Route::ComponentPage { name: "cascader".to_string() }, current: route.clone() }
            SidebarItem { label: "Transfer 穿梭框".to_string(), route: Route::ComponentPage { name: "transfer".to_string() }, current: route.clone() }
            SidebarItem { label: "Tour 引导漫游".to_string(), route: Route::ComponentPage { name: "tour".to_string() }, current: route.clone() }
            SidebarItem { label: "Splitter 分隔面板".to_string(), route: Route::ComponentPage { name: "splitter".to_string() }, current: route.clone() }
            SidebarItem { label: "Dialog 对话框".to_string(), route: Route::ComponentPage { name: "dialog".to_string() }, current: route.clone() }
            SidebarItem { label: "Table 表格".to_string(), route: Route::ComponentPage { name: "table".to_string() }, current: route.clone() }
            SidebarItem { label: "Badge 徽标".to_string(), route: Route::ComponentPage { name: "badge".to_string() }, current: route.clone() }
            SidebarItem { label: "Avatar 头像".to_string(), route: Route::ComponentPage { name: "avatar".to_string() }, current: route.clone() }
            SidebarItem { label: "Progress 进度条".to_string(), route: Route::ComponentPage { name: "progress".to_string() }, current: route.clone() }
            SidebarItem { label: "Tooltip 气泡提示".to_string(), route: Route::ComponentPage { name: "tooltip".to_string() }, current: route.clone() }
            SidebarItem { label: "Tabs 标签页".to_string(), route: Route::ComponentPage { name: "tabs".to_string() }, current: route.clone() }
            SidebarItem { label: "Alert 警告提示".to_string(), route: Route::ComponentPage { name: "alert".to_string() }, current: route.clone() }
            SidebarItem { label: "Breadcrumb 面包屑".to_string(), route: Route::ComponentPage { name: "breadcrumb".to_string() }, current: route.clone() }
            SidebarItem { label: "Pagination 分页".to_string(), route: Route::ComponentPage { name: "pagination".to_string() }, current: route.clone() }
            SidebarItem { label: "Message 全局提示".to_string(), route: Route::ComponentPage { name: "message".to_string() }, current: route.clone() }
            SidebarItem { label: "Divider 分割线".to_string(), route: Route::ComponentPage { name: "divider".to_string() }, current: route.clone() }
            SidebarItem { label: "Loading 加载中".to_string(), route: Route::ComponentPage { name: "loading".to_string() }, current: route.clone() }
            SidebarItem { label: "Empty 空状态".to_string(), route: Route::ComponentPage { name: "empty".to_string() }, current: route.clone() }
            SidebarItem { label: "Skeleton 骨架屏".to_string(), route: Route::ComponentPage { name: "skeleton".to_string() }, current: route.clone() }
            SidebarItem { label: "Backtop 回到顶部".to_string(), route: Route::ComponentPage { name: "backtop".to_string() }, current: route.clone() }
            SidebarItem { label: "Collapse 折叠面板".to_string(), route: Route::ComponentPage { name: "collapse".to_string() }, current: route.clone() }
            SidebarItem { label: "Popover 气泡卡片".to_string(), route: Route::ComponentPage { name: "popover".to_string() }, current: route.clone() }
            SidebarItem { label: "Drawer 抽屉".to_string(), route: Route::ComponentPage { name: "drawer".to_string() }, current: route.clone() }
            SidebarItem { label: "Notification 通知".to_string(), route: Route::ComponentPage { name: "notification".to_string() }, current: route.clone() }
            SidebarItem { label: "Dropdown 下拉菜单".to_string(), route: Route::ComponentPage { name: "dropdown".to_string() }, current: route.clone() }
            SidebarItem { label: "Menu 导航菜单".to_string(), route: Route::ComponentPage { name: "menu".to_string() }, current: route.clone() }
            SidebarItem { label: "Steps 步骤条".to_string(), route: Route::ComponentPage { name: "steps".to_string() }, current: route.clone() }
            SidebarItem { label: "Timeline 时间线".to_string(), route: Route::ComponentPage { name: "timeline".to_string() }, current: route.clone() }
            SidebarItem { label: "Typography 排版".to_string(), route: Route::ComponentPage { name: "typography".to_string() }, current: route.clone() }
            SidebarItem { label: "ColorPicker 颜色选择器".to_string(), route: Route::ComponentPage { name: "color_picker".to_string() }, current: route.clone() }
            SidebarItem { label: "Anchor 锚点导航".to_string(), route: Route::ComponentPage { name: "anchor".to_string() }, current: route.clone() }
            SidebarItem { label: "Watermark 水印".to_string(), route: Route::ComponentPage { name: "watermark".to_string() }, current: route.clone() }
            SidebarItem { label: "CodeBlock 代码块".to_string(), route: Route::ComponentPage { name: "code_block".to_string() }, current: route.clone() }
            SidebarItem { label: "Countdown 倒计时".to_string(), route: Route::ComponentPage { name: "countdown".to_string() }, current: route.clone() }
            SidebarItem { label: "Spin 加载中".to_string(), route: Route::ComponentPage { name: "spin".to_string() }, current: route.clone() }
            SidebarItem { label: "FloatButton 浮动按钮".to_string(), route: Route::ComponentPage { name: "float_button".to_string() }, current: route.clone() }
            SidebarItem { label: "TimePicker 时间选择".to_string(), route: Route::ComponentPage { name: "time_picker".to_string() }, current: route.clone() }
            SidebarItem { label: "Slider 滑块".to_string(), route: Route::ComponentPage { name: "slider".to_string() }, current: route.clone() }
            SidebarItem { label: "Rate 评分".to_string(), route: Route::ComponentPage { name: "rate".to_string() }, current: route.clone() }
            SidebarItem { label: "Image 图片".to_string(), route: Route::ComponentPage { name: "image".to_string() }, current: route.clone() }
            SidebarItem { label: "Space 间距".to_string(), route: Route::ComponentPage { name: "space".to_string() }, current: route.clone() }
            SidebarItem { label: "Segmented 分段".to_string(), route: Route::ComponentPage { name: "segmented".to_string() }, current: route.clone() }
            SidebarItem { label: "InputNumber 数字输入".to_string(), route: Route::ComponentPage { name: "input_number".to_string() }, current: route.clone() }
            SidebarItem { label: "Upload 上传".to_string(), route: Route::ComponentPage { name: "upload".to_string() }, current: route.clone() }
            SidebarItem { label: "Carousel 走马灯".to_string(), route: Route::ComponentPage { name: "carousel".to_string() }, current: route.clone() }
            SidebarItem { label: "Form 表单".to_string(), route: Route::ComponentPage { name: "form".to_string() }, current: route.clone() }
            SidebarItem { label: "Tree 树形控件".to_string(), route: Route::ComponentPage { name: "tree".to_string() }, current: route.clone() }
            SidebarItem { label: "DatePicker 日期选择".to_string(), route: Route::ComponentPage { name: "date_picker".to_string() }, current: route.clone() }
            SidebarItem { label: "VirtualList 虚拟列表".to_string(), route: Route::ComponentPage { name: "virtual_list".to_string() }, current: route.clone() }
            SidebarItem { label: "InfiniteScroll 无限滚动".to_string(), route: Route::ComponentPage { name: "infinite_scroll".to_string() }, current: route.clone() }
            SidebarItem { label: "Affix 固钉".to_string(), route: Route::ComponentPage { name: "affix".to_string() }, current: route.clone() }

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
