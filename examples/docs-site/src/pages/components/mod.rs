pub mod shared;

mod button;
mod copy_button;
mod calendar;
mod input;
mod input_tag;
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
mod avatar_group;
mod progress;
mod circular_progress;
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
mod command;
mod shortcut;
mod backtop;
mod collapse;
mod popconfirm;
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
mod typography;
mod color_picker;
mod anchor;
mod watermark;
mod code_block;
mod countdown;
mod spin;
mod float_button;
mod time_picker;
mod virtual_list;
mod infinite_scroll;
mod affix;
mod auto_complete;
mod cascader;
mod config_provider;
mod splitter;
mod tour;
mod transfer;

mod list;
mod aspect_ratio;
mod kbd;
mod scroll_area;
mod number_flow;
mod mask;
mod hover_card;
mod marquee;
mod gradient_text;
mod spotlight_card;
mod border_beam;
mod meteors;
mod shimmer_button;
mod flip_card;
mod ripple;
mod aurora_background;
mod confetti;
mod tilt;
mod dot_pattern;
mod scroll_progress;
mod magnetic;
mod animated_list;
mod text_reveal;
mod orbiting_circles;
mod retro_grid;
mod image_comparison;
mod lens;
mod bento_grid;
mod word_rotate;
mod snowfall;
mod color_swatch;
mod context_menu;
mod masonry;
mod typewriter;

// Demo components shared across pages
pub mod _demos;

use dioxus::prelude::*;
use ctrl::prelude::*;

/// 组件分类目录 —— 侧边栏与组件总览页共用的单一数据源。
/// 返回：Vec<(分类名, Vec<(路由名, 标题, 描述)>)>
pub fn component_catalog() -> Vec<(&'static str, Vec<(&'static str, &'static str, &'static str)>)> {
    vec![
        ("通用", vec![
            ("button", "Button 按钮", "触发操作的按钮"),
            ("copy_button", "CopyButton 复制按钮", "一键复制到剪贴板"),
            ("shimmer_button", "ShimmerButton 微光按钮", "高光扫过的强调按钮"),
            ("typography", "Typography 排版", "标题、文本、链接"),
            ("color_picker", "ColorPicker 颜色选择器", "颜色选择"),
            ("gradient_text", "GradientText 流光文字", "渐变动画文字"),
            ("typewriter", "Typewriter 打字机", "逐字打印动画"),
            ("text_reveal", "TextReveal 文字揭示", "文字逐词揭示"),
            ("word_rotate", "WordRotate 单词轮播", "关键词垂直翻转轮播"),
            ("shortcut", "Shortcut 快捷键", "键盘快捷键系统与 Hook"),
            ("context_menu", "ContextMenu 右键菜单", "右键弹出操作菜单"),
            ("masonry", "Masonry 瀑布流", "瀑布流布局"),
        ]),
        ("布局", vec![
            ("grid", "Grid 栅格", "24 栅格布局"),
            ("space", "Space 间距", "控制元素间距"),
            ("divider", "Divider 分割线", "区隔内容的分割线"),
            ("splitter", "Splitter 分隔面板", "可拖拽调整的分隔面板"),
            ("scroll_area", "ScrollArea 滚动区域", "自定义滚动条容器"),
            ("aspect_ratio", "AspectRatio 宽高比", "固定宽高比容器"),
            ("affix", "Affix 固钉", "将元素固定在可视区"),
            ("watermark", "Watermark 水印", "页面水印"),
        ]),
        ("导航", vec![
            ("menu", "Menu 菜单", "导航菜单"),
            ("tabs", "Tabs 标签页", "选项卡切换"),
            ("breadcrumb", "Breadcrumb 面包屑", "面包屑导航"),
            ("pagination", "Pagination 分页", "分页器"),
            ("steps", "Steps 步骤条", "引导流程步骤"),
            ("dropdown", "Dropdown 下拉菜单", "下拉菜单"),
            ("command", "Command 命令面板", "⌘K 可搜索命令菜单"),
            ("anchor", "Anchor 锚点导航", "页面内锚点导航"),
            ("backtop", "Backtop 回到顶部", "回到页面顶部"),
        ]),
        ("数据录入", vec![
            ("input", "Input 输入框", "文本输入"),
            ("input_number", "InputNumber 数字输入框", "数字输入"),
            ("input_tag", "InputTag 标签输入", "输入文本生成标签"),
            ("textarea", "Textarea 多行输入", "多行文本输入"),
            ("select", "Select 选择器", "下拉选择"),
            ("checkbox", "Checkbox 复选框", "多选场景"),
            ("radio", "Radio 单选框", "互斥选项选择"),
            ("switch", "Switch 开关", "两种状态切换"),
            ("slider", "Slider 滑块", "数值范围选择"),
            ("rate", "Rate 评分", "评价打分"),
            ("date_picker", "DatePicker 日期选择器", "日期选择"),
            ("time_picker", "TimePicker 时间选择", "时间选择"),
            ("calendar", "Calendar 日历", "月历视图与日期选择"),
            ("upload", "Upload 上传", "文件上传"),
            ("form", "Form 表单", "表单容器与校验"),
            ("cascader", "Cascader 级联选择", "多级数据选择"),
            ("auto_complete", "AutoComplete 自动补全", "输入时自动匹配候选项"),
            ("transfer", "Transfer 穿梭框", "双栏选择移动"),
            ("segmented", "Segmented 分段控制器", "分段选择"),
        ]),
        ("数据展示", vec![
            ("table", "Table 表格", "数据表格"),
            ("list", "List 列表", "通用结构化列表"),
            ("animated_list", "AnimatedList 动画列表", "交错入场的列表"),
            ("card", "Card 卡片", "卡片容器"),
            ("spotlight_card", "SpotlightCard 聚光卡片", "光标跟随光晕卡片"),
            ("tilt", "Tilt 3D 倾斜", "鼠标驱动的透视倾斜卡片"),
            ("magnetic", "Magnetic 磁吸", "光标磁吸位移"),
            ("border_beam", "BorderBeam 边框光束", "沿边框流动的光束"),
            ("meteors", "Meteors 流星", "流星雨背景装饰"),
            ("ripple", "Ripple 涟漪背景", "同心圆扩散背景"),
            ("dot_pattern", "DotPattern 点阵背景", "点阵 / 网格装饰背景"),
            ("aurora_background", "AuroraBackground 极光背景", "多色渐变流动背景"),
            ("retro_grid", "RetroGrid 复古网格", "透视流动网格背景"),
            ("orbiting_circles", "OrbitingCircles 环绕轨道", "元素绕中心环绕"),
            ("image_comparison", "ImageComparison 图片对比", "拖拽滑块对比前后图"),
            ("lens", "Lens 放大镜", "悬停局部放大图片"),
            ("bento_grid", "BentoGrid 便当网格", "大小不一的特性网格"),
            ("snowfall", "Snowfall 飘雪", "雪花飘落背景装饰"),
            ("color_swatch", "ColorSwatch 色卡", "颜色样本网格"),
            ("confetti", "Confetti 彩带庆祝", "喷射彩带庆祝动效"),
            ("scroll_progress", "ScrollProgress 滚动进度", "页面阅读进度条"),
            ("flip_card", "FlipCard 翻转卡片", "3D 翻转正反面"),
            ("tag", "Tag 标签", "标签"),
            ("badge", "Badge 徽标", "徽标数字"),
            ("avatar", "Avatar 头像", "头像"),
            ("avatar_group", "AvatarGroup 头像组", "堆叠头像与 +N 溢出"),
            ("tooltip", "Tooltip 文字提示", "文字提示"),
            ("popover", "Popover 气泡卡片", "气泡卡片"),
            ("hover_card", "HoverCard 悬停卡片", "悬停弹出信息卡片"),
            ("collapse", "Collapse 折叠面板", "折叠面板"),
            ("timeline", "Timeline 时间轴", "时间轴"),
            ("descriptions", "Descriptions 描述列表", "标签-值信息展示"),
            ("statistic", "Statistic 统计数值", "统计数据展示"),
            ("number_flow", "NumberFlow 数字动画", "数值滚动变化动画"),
            ("image", "Image 图片", "图片展示与预览"),
            ("carousel", "Carousel 走马灯", "轮播图"),
            ("marquee", "Marquee 跑马灯", "无缝滚动文字/内容"),
            ("tree", "Tree 树", "树形控件"),
            ("code_block", "CodeBlock 代码块", "代码展示"),
            ("kbd", "Kbd 键盘按键", "键盘快捷键展示"),
            ("countdown", "Countdown 倒计时", "倒计时"),
            ("virtual_list", "VirtualList 虚拟列表", "长列表虚拟滚动"),
            ("infinite_scroll", "InfiniteScroll 无限滚动", "滚动加载更多"),
        ]),
        ("反馈", vec![
            ("alert", "Alert 警告提示", "警告提示"),
            ("dialog", "Dialog 对话框", "模态对话框"),
            ("drawer", "Drawer 抽屉", "抽屉面板"),
            ("mask", "Mask 遮罩层", "半透明遮罩"),
            ("message", "Message 全局提示", "全局消息提示"),
            ("notification", "Notification 通知", "通知提醒"),
            ("progress", "Progress 进度条", "进度条"),
            ("circular_progress", "CircularProgress 环形进度", "环形/仪表盘进度"),
            ("loading", "Loading 加载", "加载中"),
            ("spin", "Spin 加载中", "加载状态"),
            ("skeleton", "Skeleton 骨架屏", "骨架屏占位"),
            ("empty", "Empty 空状态", "空状态"),
            ("result", "Result 结果页", "操作结果反馈"),
            ("popconfirm", "Popconfirm 气泡确认框", "二次确认气泡弹窗"),
            ("tour", "Tour 引导漫游", "分步引导用户操作"),
            ("float_button", "FloatButton 浮动按钮", "浮动操作按钮"),
        ]),
        ("其他", vec![
            ("config_provider", "ConfigProvider 全局配置", "全局配置提供者"),
        ]),
    ]
}

/// Component index page
#[component]
#[allow(non_snake_case)]
pub fn ComponentsIndex() -> Element {
    let navigator = use_navigator();
    let catalog = component_catalog();
    let total: usize = catalog.iter().map(|(_, items)| items.len()).sum();

    rsx! {
        div { style: "margin-bottom: 32px;",
            h1 { style: "margin-bottom: 8px;", "组件总览" }
            p { style: "color: var(--ctrl-text-secondary); margin: 0;",
                "Ctrl UI 提供了 {total} 个高质量组件，按通用、布局、导航、数据录入、数据展示、反馈等场景分类。"
            }
        }

        for (category, items) in catalog {
            div { key: "{category}", style: "margin-bottom: 40px;",
                div { style: "display:flex; align-items:baseline; gap:12px; margin-bottom:16px;",
                    h2 { style: "margin:0;", "{category}" }
                    span { style: "color:var(--ctrl-text-secondary); font-size:var(--ctrl-font-size-sm);",
                        "{items.len()} 个组件"
                    }
                }
                Row { gutter: 16,
                    for (name, title, desc) in items {
                        Col { span: 24, sm: Some(12), md: Some(8), lg: Some(6),
                            div {
                                key: "{name}",
                                style: "cursor:pointer; margin-bottom:16px;",
                                onclick: {
                                    let nav = navigator.clone();
                                    move |_| { let _ = nav.push(crate::Route::ComponentPage { name: name.to_string() }); }
                                },
                                Card {
                                    title: title.to_string(),
                                    p { style: "color:var(--ctrl-text-secondary); margin:0;", "{desc}" }
                                }
                            }
                        }
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
        "copy_button" => rsx! { copy_button::CopyButtonPage {} },
        "shimmer_button" => rsx! { shimmer_button::ShimmerButtonPage {} },
        "calendar" => rsx! { calendar::CalendarPage {} },
        "auto_complete" => rsx! { auto_complete::AutoCompletePage {} },
        "cascader" => rsx! { cascader::CascaderPage {} },
        "config_provider" => rsx! { config_provider::ConfigProviderPage {} },
        "splitter" => rsx! { splitter::SplitterPage {} },
        "tour" => rsx! { tour::TourPage {} },
        "transfer" => rsx! { transfer::TransferPage {} },
        "input" => rsx! { input::InputPage {} },
        "input_tag" => rsx! { input_tag::InputTagPage {} },
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
        "avatar_group" => rsx! { avatar_group::AvatarGroupPage {} },
        "progress" => rsx! { progress::ProgressPage {} },
        "circular_progress" => rsx! { circular_progress::CircularProgressPage {} },
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
        "popconfirm" => rsx! { popconfirm::PopconfirmPage {} },
        "popover" => rsx! { popover::PopoverPage {} },
        "drawer" => rsx! { drawer::DrawerPage {} },
        "notification" => rsx! { notification::NotificationPage {} },
        "dropdown" => rsx! { dropdown::DropdownPage {} },
        "command" => rsx! { command::CommandPage {} },
        "shortcut" => rsx! { shortcut::ShortcutPage {} },
        "menu" => rsx! { menu::MenuPage {} },
        "steps" => rsx! { steps::StepsPage {} },
        "timeline" => rsx! { timeline::TimelinePage {} },
        "typography" => rsx! { typography::TypographyPage {} },
        "color_picker" => rsx! { color_picker::ColorPickerPage {} },
        "anchor" => rsx! { anchor::AnchorPage {} },
        "watermark" => rsx! { watermark::WatermarkPage {} },
        "code_block" => rsx! { code_block::CodeBlockPage {} },
        "countdown" => rsx! { countdown::CountdownPage {} },
        "spin" => rsx! { spin::SpinPage {} },
        "float_button" => rsx! { float_button::FloatButtonPage {} },
        "time_picker" => rsx! { time_picker::TimePickerPage {} },
        "virtual_list" => rsx! { virtual_list::VirtualListPage {} },
        "infinite_scroll" => rsx! { infinite_scroll::InfiniteScrollPage {} },
        "affix" => rsx! { affix::AffixPage {} },
        "list" => rsx! { list::ListPage {} },
        "aspect_ratio" => rsx! { aspect_ratio::AspectRatioPage {} },
        "kbd" => rsx! { kbd::KbdPage {} },
        "scroll_area" => rsx! { scroll_area::ScrollAreaPage {} },
        "number_flow" => rsx! { number_flow::NumberFlowPage {} },
        "mask" => rsx! { mask::MaskPage {} },
        "hover_card" => rsx! { hover_card::HoverCardPage {} },
        "marquee" => rsx! { marquee::MarqueePage {} },
        "gradient_text" => rsx! { gradient_text::GradientTextPage {} },
        "spotlight_card" => rsx! { spotlight_card::SpotlightCardPage {} },
        "border_beam" => rsx! { border_beam::BorderBeamPage {} },
        "meteors" => rsx! { meteors::MeteorsPage {} },
        "ripple" => rsx! { ripple::RipplePage {} },
        "aurora_background" => rsx! { aurora_background::AuroraBackgroundPage {} },
        "confetti" => rsx! { confetti::ConfettiPage {} },
        "tilt" => rsx! { tilt::TiltPage {} },
        "magnetic" => rsx! { magnetic::MagneticPage {} },
        "animated_list" => rsx! { animated_list::AnimatedListPage {} },
        "text_reveal" => rsx! { text_reveal::TextRevealPage {} },
        "orbiting_circles" => rsx! { orbiting_circles::OrbitingCirclesPage {} },
        "retro_grid" => rsx! { retro_grid::RetroGridPage {} },
        "image_comparison" => rsx! { image_comparison::ImageComparisonPage {} },
        "lens" => rsx! { lens::LensPage {} },
        "bento_grid" => rsx! { bento_grid::BentoGridPage {} },
        "word_rotate" => rsx! { word_rotate::WordRotatePage {} },
        "snowfall" => rsx! { snowfall::SnowfallPage {} },
        "color_swatch" => rsx! { color_swatch::ColorSwatchPage {} },
        "context_menu" => rsx! { context_menu::ContextMenuPage {} },
        "masonry" => rsx! { masonry::MasonryPage {} },
        "dot_pattern" => rsx! { dot_pattern::DotPatternPage {} },
        "scroll_progress" => rsx! { scroll_progress::ScrollProgressPage {} },
        "flip_card" => rsx! { flip_card::FlipCardPage {} },
        "typewriter" => rsx! { typewriter::TypewriterPage {} },
        _ => rsx! {
            div { style: "padding:40px; text-align:center;",
                h1 { "组件未找到" }
                p { "找不到名为 \"{name}\" 的组件" }
            }
        }
    }
}