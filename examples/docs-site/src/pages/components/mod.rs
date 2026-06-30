pub mod shared;

mod button;
mod input;
mod textarea;
mod switch;
mod checkbox;
mod radio;
mod slider;
mod rate;
mod image;
mod space;
mod segmented;
mod input_number;
mod upload;
mod carousel;
mod form;
mod tree;
mod date_picker;
mod select;
mod tag;
mod card;
mod dialog;
mod table;
mod badge;
mod avatar;
mod progress;
mod tooltip;
mod tabs;
mod alert;
mod breadcrumb;
mod pagination;
mod message;
mod divider;
mod loading;
mod empty;
mod skeleton;
mod backtop;
mod collapse;
mod popover;
mod drawer;
mod notification;
mod dropdown;
mod menu;
mod result;
mod statistic;
mod descriptions;
mod grid;
mod steps;
mod timeline;

// Demo components shared across pages
pub mod _demos;

use dioxus::prelude::*;

/// Component index page - shows all available components
#[component]
#[allow(non_snake_case)]
pub fn ComponentsIndex() -> Element {
    let components = vec![
        ("button", "Button 按钮", "按钮用于触发操作"),
        ("input", "Input 输入框", "输入框用于文本输入"),
        ("textarea", "Textarea 多行输入", "多行文本输入"),
        ("result", "Result 结果页", "操作结果反馈"),
        ("statistic", "Statistic 统计数值", "统计数据展示"),
        ("descriptions", "Descriptions 描述列表", "标签-值信息展示"),
        ("grid", "Grid 栅格", "24 栅格布局"),
        ("switch", "Switch 开关", "开关用于在两种状态间切换"),
        ("checkbox", "Checkbox 复选框", "复选框用于多选场景"),
        ("radio", "Radio 单选框", "单选框用于互斥选项中选择"),
        ("slider", "Slider 滑块", "滑块用于在数值范围内选择"),
        ("rate", "Rate 评分", "评分用于评价"),
        ("image", "Image 图片", "图片展示与预览"),
        ("space", "Space 间距", "控制元素间距"),
        ("segmented", "Segmented 分段控制器", "分段控制器"),
        ("input_number", "InputNumber 数字输入框", "数字输入"),
        ("upload", "Upload 上传", "文件上传"),
        ("carousel", "Carousel 走马灯", "轮播图"),
        ("form", "Form 表单", "表单容器"),
        ("tree", "Tree 树", "树形控件"),
        ("date_picker", "DatePicker 日期选择器", "日期选择"),
        ("select", "Select 选择器", "下拉选择"),
        ("tag", "Tag 标签", "标签"),
        ("card", "Card 卡片", "卡片容器"),
        ("dialog", "Dialog 对话框", "模态对话框"),
        ("table", "Table 表格", "数据表格"),
        ("badge", "Badge 徽标", "徽标"),
        ("avatar", "Avatar 头像", "头像"),
        ("progress", "Progress 进度条", "进度条"),
        ("tooltip", "Tooltip 文字提示", "文字提示"),
        ("tabs", "Tabs 标签页", "标签页"),
        ("alert", "Alert 警告", "警告提示"),
        ("breadcrumb", "Breadcrumb 面包屑", "面包屑导航"),
        ("pagination", "Pagination 分页", "分页"),
        ("message", "Message 消息", "全局消息提示"),
        ("divider", "Divider 分割线", "分割线"),
        ("loading", "Loading 加载", "加载中"),
        ("empty", "Empty 空状态", "空状态"),
        ("skeleton", "Skeleton 骨架屏", "骨架屏"),
        ("backtop", "Backtop 回到顶部", "回到顶部"),
        ("collapse", "Collapse 折叠面板", "折叠面板"),
        ("popover", "Popover 气泡卡片", "气泡卡片"),
        ("drawer", "Drawer 抽屉", "抽屉"),
        ("notification", "Notification 通知", "通知提醒"),
        ("dropdown", "Dropdown 下拉菜单", "下拉菜单"),
        ("menu", "Menu 菜单", "导航菜单"),
        ("steps", "Steps 步骤条", "步骤条"),
        ("timeline", "Timeline 时间轴", "时间轴"),
    ];

    rsx! {
        div { style: "max-width: 1200px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                "组件总览"
            }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;",
                "Ctrl UI 提供了 47 个高质量组件，覆盖表单、数据展示、反馈、导航等场景。"
            }

            div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); gap: 16px;",
                for (name, title, desc) in components {
                    div {
                        key: "{name}",
                        style: "padding: 20px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); cursor: pointer; transition: all 0.2s;",
                        onmouseenter: move |_| {},
                        onclick: move |_| {
                            let nav = use_navigator();
                            nav.push(crate::Route::ComponentPage { name: name.to_string() });
                        },
                        h3 { style: "font-size: 1rem; font-weight: 600; color: var(--ctrl-text); margin-bottom: 4px;", "{title}" }
                        p { style: "font-size: 0.875rem; color: var(--ctrl-text-secondary);", "{desc}" }
                    }
                }
            }
        }
    }
}

/// Component page router — dispatches using per-arm rsx! invocations to force
/// unique VNode templates. This ensures Dioxus performs full template replacement
/// when switching pages (old.template != new.template), which destroys old scopes
/// and creates fresh ones — preventing the "Unable to retrieve the hook" panic.
#[component]
#[allow(non_snake_case)]
pub fn ComponentPage(name: String) -> Element {
    // Each match arm uses its own rsx! invocation, producing a unique VNode template.
    // This forces Dioxus to do a full template replacement (not a scope diff) when the
    // match arm changes, which properly destroys old scopes and creates new ones.
    match name.as_str() {
        "button" => rsx! { button::ButtonPage {} },
        "input" => rsx! { input::InputPage {} },
        "textarea" => rsx! { textarea::TextareaPage {} },
        "result" => rsx! { result::ResultPage {} },
        "statistic" => rsx! { statistic::StatisticPage {} },
        "descriptions" => rsx! { descriptions::DescriptionsPage {} },
        "grid" => rsx! { grid::GridPage {} },
        "switch" => rsx! { switch::SwitchPage {} },
        "checkbox" => rsx! { checkbox::CheckboxPage {} },
        "radio" => rsx! { radio::RadioPage {} },
        "slider" => rsx! { slider::SliderPage {} },
        "rate" => rsx! { rate::RatePage {} },
        "image" => rsx! { image::ImagePage {} },
        "space" => rsx! { space::SpacePage {} },
        "segmented" => rsx! { segmented::SegmentedPage {} },
        "input_number" => rsx! { input_number::InputNumberPage {} },
        "upload" => rsx! { upload::UploadPage {} },
        "carousel" => rsx! { carousel::CarouselPage {} },
        "form" => rsx! { form::FormPage {} },
        "tree" => rsx! { tree::TreePage {} },
        "date_picker" => rsx! { date_picker::DatePickerPage {} },
        "select" => rsx! { select::SelectPage {} },
        "tag" => rsx! { tag::TagPage {} },
        "card" => rsx! { card::CardPage {} },
        "dialog" => rsx! { dialog::DialogPage {} },
        "table" => rsx! { table::TablePage {} },
        "badge" => rsx! { badge::BadgePage {} },
        "avatar" => rsx! { avatar::AvatarPage {} },
        "progress" => rsx! { progress::ProgressPage {} },
        "tooltip" => rsx! { tooltip::TooltipPage {} },
        "tabs" => rsx! { tabs::TabsPage {} },
        "alert" => rsx! { alert::AlertPage {} },
        "breadcrumb" => rsx! { breadcrumb::BreadcrumbPage {} },
        "pagination" => rsx! { pagination::PaginationPage {} },
        "message" => rsx! { message::MessagePage {} },
        "divider" => rsx! { divider::DividerPage {} },
        "loading" => rsx! { loading::LoadingPage {} },
        "empty" => rsx! { empty::EmptyPage {} },
        "skeleton" => rsx! { skeleton::SkeletonPage {} },
        "backtop" => rsx! { backtop::BacktopPage {} },
        "collapse" => rsx! { collapse::CollapsePage {} },
        "popover" => rsx! { popover::PopoverPage {} },
        "drawer" => rsx! { drawer::DrawerPage {} },
        "notification" => rsx! { notification::NotificationPage {} },
        "dropdown" => rsx! { dropdown::DropdownPage {} },
        "menu" => rsx! { menu::MenuPage {} },
        "steps" => rsx! { steps::StepsPage {} },
        "timeline" => rsx! { timeline::TimelinePage {} },
        _ => rsx! {
            div { style: "padding: 40px; text-align: center;",
                h1 { style: "font-size: 1.5rem; color: var(--ctrl-text);", "组件未找到" }
                p { style: "color: var(--ctrl-text-secondary); margin-top: 8px;", "找不到名为 \"{name}\" 的组件" }
            }
        }
    }
}