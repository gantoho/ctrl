use dioxus::prelude::*;
use ctrl::prelude::*;
use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TourPage() -> Element {
    rsx! {
        h1 { "Tour 引导漫游" }
        p { "用于分步引导用户熟悉页面功能，提供聚光灯高亮和提示卡片。支持声明式和命令式两种 API，内置 8 种弹出位置和键盘导航。" }

        h2 { "重要说明" }

        div { style: "padding: 16px; background: var(--ctrl-primary-light, #e6f7ff); border: 1px solid var(--ctrl-primary, #1890ff); border-radius: var(--ctrl-radius-md); margin-bottom: 24px;",
            p { style: "font-weight: 600; margin-bottom: 8px; color: var(--ctrl-text);", "Tour 组件需要一个存在的 DOM 目标元素" }
            p { style: "color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm); line-height: 1.8;",
                "Tour 通过 CSS 选择器（如 #id 或 .class）定位目标元素。"
            }
            p { style: "color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm); line-height: 1.8;",
                "如果目标元素不存在，提示卡片会显示警告信息。请确保目标元素在 Tour 渲染时已挂载到 DOM。"
            }
        }

        h2 { "基本用法" }

        DemoBox {
            title: "声明式 API（多步骤）".to_string(),
            description: Some("通过 open 状态控制引导开关，steps 传入多个步骤。点击「下一步」逐步引导，点击「Skip」或关闭按钮取消引导。".to_string()),
            demo: rsx! {
                DeclarativeTourDemo {}
            },
            code: "let mut open = use_signal(|| false);\n\nrsx! {\n    div { id: \"step-1\", \"功能 A\" }\n    div { id: \"step-2\", \"功能 B\" }\n    button { onclick: move |_| open.set(true), \"开始引导\" }\n\n    Tour {\n        steps: vec![\n            TourStep { target: \"#step-1\".to_string(), title: \"第一步\".to_string(), description: \"...\".to_string(), placement: Placement::Bottom },\n            TourStep { target: \"#step-2\".to_string(), title: \"第二步\".to_string(), description: \"...\".to_string(), placement: Placement::Bottom },\n        ],\n        open: open(),\n        onclose: move |_| open.set(false),\n        onfinish: move |_| open.set(false),\n    }\n}".to_string(),
        }

        DemoBox {
            title: "不同弹出位置".to_string(),
            description: Some("支持 Top / Bottom / Left / Right 四种基本位置及各带 Start / End 变体，共 8 种。".to_string()),
            demo: rsx! {
                PlacementVariantsDemo {}
            },
            code: "TourStep {\n    target: \"#target\".to_string(),\n    title: \"标题\".to_string(),\n    description: \"...\".to_string(),\n    placement: Placement::Top, // Bottom / Left / Right / TopStart / ...\n}".to_string(),
        }

        DemoBox {
            title: "单步骤引导".to_string(),
            description: Some("仅一个步骤时，按钮区域只显示「Skip」和「Finish」，不会出现无意义的「Next」。".to_string()),
            demo: rsx! {
                SingleStepTourDemo {}
            },
            code: "Tour {\n    steps: vec![\n        TourStep {\n            target: \"#target\".to_string(),\n            title: \"功能介绍\".to_string(),\n            description: \"这是单步引导。\".to_string(),\n            placement: Placement::Bottom,\n        },\n    ],\n    open: open(),\n    onfinish: move |_| {\n        open.set(false);\n        // 执行完成后的回调\n    },\n}".to_string(),
        }

        DemoBox {
            title: "onfinish / onclose 回调".to_string(),
            description: Some("通过 onfinish 监听用户完成引导，通过 onclose 监听取消或关闭。常用于埋点统计或保存引导状态。".to_string()),
            demo: rsx! {
                CallbackTourDemo {}
            },
            code: "let mut completed = use_signal(|| false);\n\nTour {\n    steps: vec![...],\n    open: open(),\n    onclose: move |_| open.set(false),\n    onfinish: move |_| {\n        open.set(false);\n        completed.set(true);\n    },\n}".to_string(),
        }

        DemoBox {
            title: "命令式 API (TourProvider)".to_string(),
            description: Some("在应用根节点使用 TourProvider 包裹，任意子组件通过 use_tour() 获取 API 来命令式地启动、关闭、跳转引导。".to_string()),
            demo: rsx! {
                TourProvider {
                    div {
                        id: "tour-cmd-target",
                        style: "padding: 12px 20px; background: var(--ctrl-bg); border: 2px solid var(--ctrl-success, #52c41a); border-radius: var(--ctrl-radius-md); display: inline-block; margin-bottom: 12px;",
                        p { style: "font-weight: 600;", "命令式 API 演示" }
                    }
                    div {
                        // 启动按钮
                        DemoStartButton {}
                    }
                    // Tour 会通过 TourProvider 内部渲染
                }
            },
            code: "// 应用根节点\nrsx! {\n    TourProvider {\n        // 你的应用内容\n    }\n}\n\n// 任意子组件中\nlet mut tour = use_tour();\ntour.start(vec![\n    TourStep {\n        target: \"#step1\".to_string(),\n        title: \"欢迎\".to_string(),\n        description: \"引导说明\".to_string(),\n        placement: Placement::Bottom,\n    },\n]);\n\n// 其他 API\ntour.next();    // 下一步\ntour.prev();    // 上一步\ntour.close();   // 关闭\ntour.finish();  // 完成".to_string(),
        }

        DemoBox {
            title: "复杂布局引导".to_string(),
            description: Some("模拟真实后台管理场景：顶栏、侧边栏、数据卡片、表格操作列。引导可精准穿透复杂的 Flex/Grid 布局定位到每一个目标元素。".to_string()),
            demo: rsx! {
                ComplexLayoutTourDemo {}
            },
            code: "// 复杂页面布局\nrsx! {\n    div { id: \"app-layout\",\n        header { id: \"header-nav\", \"顶栏导航\" }\n        div { style: \"display:flex\",\n            aside { id: \"sidebar-menu\", \"侧边菜单\" }\n            main {\n                div { id: \"stat-card\", \"统计卡片\" }\n                table { id: \"data-table\", \"数据表格\" }\n            }\n        }\n    }\n    Tour { steps: vec![...], open, ... }\n}".to_string(),
        }

        h2 { "TourStep 步骤字段" }
        PropsTable { headers: vec!["字段".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("target", "String", "\"\"", "目标元素的 CSS 选择器，如 \"#my-id\"、\".my-class\""),
            ("title", "String", "\"\"", "步骤标题"),
            ("description", "String", "\"\"", "步骤详细描述文案"),
            ("placement", "Placement", "Placement::Bottom", "提示卡片相对目标的位置（8 种：Top/Bottom/Left/Right 及 Start/End 变体）"),
        ]}

        h2 { "Tour Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("steps", "Vec<TourStep>", "vec![]", "引导步骤列表"),
            ("current", "usize", "0", "当前步骤索引"),
            ("open", "bool", "false", "是否打开引导"),
            ("class", "String", "\"\"", "自定义 CSS 类名"),
            ("style", "String", "\"\"", "自定义内联样式"),
            ("onclose", "Option<EventHandler<()>>", "None", "关闭事件（点击遮罩/Escape/跳过按钮）"),
            ("onfinish", "Option<EventHandler<()>>", "None", "完成事件（点击 Finish 按钮）"),
        ]}

        h2 { "TourAPI 方法" }
        p { "通过 use_tour() 获取的 API 对象提供以下方法：" }
        PropsTable { headers: vec!["方法".to_string(), "参数".to_string(), "说明".to_string(), "".to_string()], rows: vec![
            ("start", "steps: Vec<TourStep>", "从第 0 步开始引导", ""),
            ("close", "()", "关闭引导", ""),
            ("next", "()", "跳转到下一步", ""),
            ("prev", "()", "跳转到上一步", ""),
            ("finish", "()", "完成引导并触发 onfinish", ""),
        ]}
    }
}

/// 声明式多步骤引导演示
#[component]
#[allow(non_snake_case)]
fn DeclarativeTourDemo() -> Element {
    let mut open = use_signal(|| false);

    let target_style = "padding: 12px 20px; background: var(--ctrl-bg); border: 2px solid var(--ctrl-primary); border-radius: var(--ctrl-radius-md);";

    rsx! {
        div {
            style: "display: flex; gap: 16px; flex-wrap: wrap; margin-bottom: 16px;",
            div { id: "tour-decl-1", style: "{target_style}",
                p { style: "font-weight: 600;", "功能 A" }
            }
            div { id: "tour-decl-2", style: "{target_style}",
                p { style: "font-weight: 600;", "功能 B" }
            }
            div { id: "tour-decl-3", style: "{target_style}",
                p { style: "font-weight: 600;", "功能 C" }
            }
        }
        button {
            style: "padding: 8px 16px; background: var(--ctrl-primary); color: #fff; border: none; border-radius: var(--ctrl-radius-sm); cursor: pointer; font-family: inherit; font-size: var(--ctrl-font-size-sm);",
            onclick: move |_| open.set(true),
            "开始引导"
        }
        Tour {
            steps: vec![
                TourStep {
                    target: "#tour-decl-1".to_string(),
                    title: "第一步：功能 A".to_string(),
                    description: "这是引导的第一步。点击「Next」进入下一步，点击「Skip」或右上角关闭按钮取消引导。".to_string(),
                    placement: Placement::Bottom,
                },
                TourStep {
                    target: "#tour-decl-2".to_string(),
                    title: "第二步：功能 B".to_string(),
                    description: "聚光灯已移动到功能 B。也可以使用键盘方向键（← →）切换步骤。".to_string(),
                    placement: Placement::Bottom,
                },
                TourStep {
                    target: "#tour-decl-3".to_string(),
                    title: "第三步：功能 C".to_string(),
                    description: "这是最后一步，点击「Finish」完成引导。".to_string(),
                    placement: Placement::Top,
                },
            ],
            open: open(),
            onclose: move |_| open.set(false),
            onfinish: move |_| open.set(false),
        }
    }
}

/// 演示命令式 API 启动按钮
#[component]
fn DemoStartButton() -> Element {
    let mut tour = use_tour();

    rsx! {
        button {
            style: "padding: 8px 16px; background: var(--ctrl-primary); color: #fff; border: none; border-radius: var(--ctrl-radius-sm); cursor: pointer; font-family: inherit; font-size: var(--ctrl-font-size-sm);",
            onclick: move |_| {
                tour.start(vec![
                    TourStep {
                        target: "#tour-cmd-target".to_string(),
                        title: "命令式引导".to_string(),
                        description: "这是通过 use_tour().start() 命令式启动的引导。点击「Next」进入下一步。".to_string(),
                        placement: Placement::Bottom,
                    },
                    TourStep {
                        target: "#tour-cmd-target".to_string(),
                        title: "继续引导".to_string(),
                        description: "命令式 API 同样支持多步骤。点击「Finish」完成，或「Skip」取消。".to_string(),
                        placement: Placement::Bottom,
                    },
                ]);
            },
            "点击启动引导演示"
        }
    }
}

/// 放置位置变体演示（Top / Bottom / Left / Right）
#[component]
#[allow(non_snake_case)]
fn PlacementVariantsDemo() -> Element {
    let mut open = use_signal(|| false);
    let placement = use_signal(|| Placement::Bottom);

    let box_style = "padding: 10px 16px; background: var(--ctrl-bg); border: 2px solid var(--ctrl-primary); border-radius: var(--ctrl-radius-md); display: inline-block;";

    rsx! {
        div {
            style: "display: flex; flex-wrap: wrap; gap: 12px; justify-content: center; padding: 12px 0;",
            div { id: "tour-plc-mid", style: "{box_style}",
                p { style: "font-weight: 600; margin: 0;", "目标元素" }
            }
        }
        div {
            style: "display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 16px;",
            for place in [Placement::Top, Placement::Bottom, Placement::Left, Placement::Right,
                          Placement::TopStart, Placement::BottomStart,
                          Placement::TopEnd, Placement::BottomEnd] {
                button {
                    key: "{place:?}",
                    style: "padding: 4px 10px; background: var(--ctrl-bg-secondary); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-sm); cursor: pointer; font-family: inherit; font-size: var(--ctrl-font-size-xs);",
                    onclick: {
                        let mut pl = placement;
                        move |_| { pl.set(place); open.set(true); }
                    },
                    "{place:?}"
                }
            }
        }
        Tour {
            steps: vec![
                TourStep {
                    target: "#tour-plc-mid".to_string(),
                    title: format!("{:?} 位置", placement()),
                    description: "提示卡片显示在目标元素的此方向。\\n可点击上方按钮切换 8 种位置查看效果。".to_string(),
                    placement: placement(),
                },
            ],
            open: open(),
            onclose: move |_| open.set(false),
        }
    }
}

/// 单步骤引导演示（无 Next 按钮，直接 Finish）
#[component]
#[allow(non_snake_case)]
fn SingleStepTourDemo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div {
            id: "tour-single-target",
            style: "padding: 12px 20px; background: var(--ctrl-bg); border: 2px solid var(--ctrl-warning, #faad14); border-radius: var(--ctrl-radius-md); display: inline-block; margin-bottom: 16px;",
            p { style: "font-weight: 600; margin: 0;", "单步骤目标" }
        }
        button {
            style: "padding: 8px 16px; background: var(--ctrl-primary); color: #fff; border: none; border-radius: var(--ctrl-radius-sm); cursor: pointer; font-family: inherit; font-size: var(--ctrl-font-size-sm);",
            onclick: move |_| open.set(true),
            "开始单步引导"
        }
        Tour {
            steps: vec![
                TourStep {
                    target: "#tour-single-target".to_string(),
                    title: "功能介绍".to_string(),
                    description: "仅一个步骤时，只显示「Skip」和「Finish」按钮，不会出现无意义的「Next」。".to_string(),
                    placement: Placement::Bottom,
                },
            ],
            open: open(),
            onclose: move |_| open.set(false),
            onfinish: move |_| open.set(false),
        }
    }
}

/// onfinish / onclose 回调演示
#[component]
#[allow(non_snake_case)]
fn CallbackTourDemo() -> Element {
    let mut open = use_signal(|| false);
    let mut status = use_signal(|| "等待引导...".to_string());

    rsx! {
        div {
            id: "tour-cb-target",
            style: "padding: 12px 20px; background: var(--ctrl-bg); border: 2px solid var(--ctrl-primary); border-radius: var(--ctrl-radius-md); display: inline-block; margin-bottom: 16px;",
            p { style: "font-weight: 600; margin: 0;", "回到这里" }
        }
        div {
            style: "display: flex; gap: 12px; align-items: center;",
            button {
                style: "padding: 8px 16px; background: var(--ctrl-primary); color: #fff; border: none; border-radius: var(--ctrl-radius-sm); cursor: pointer; font-family: inherit; font-size: var(--ctrl-font-size-sm);",
                onclick: move |_| open.set(true),
                "开始引导"
            }
            span {
                style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "{status}"
            }
        }
        Tour {
            steps: vec![
                TourStep {
                    target: "#tour-cb-target".to_string(),
                    title: "完成 vs 取消".to_string(),
                    description: "点击「Finish」触发 onfinish（显示已完成），点击「Skip」或关闭按钮触发 onclose（显示已取消）。".to_string(),
                    placement: Placement::Bottom,
                },
            ],
            open: open(),
            onclose: move |_| {
                status.set("已取消".to_string());
                open.set(false);
            },
            onfinish: move |_| {
                status.set("已完成".to_string());
                open.set(false);
            },
        }
    }
}

/// 复杂布局引导演示（模拟后台管理页面）
#[component]
#[allow(non_snake_case)]
fn ComplexLayoutTourDemo() -> Element {
    let mut open = use_signal(|| false);

    let header_style = "height: 48px; background: var(--ctrl-primary); color: #fff; display: flex; align-items: center; padding: 0 20px; font-weight: 600; border-radius: var(--ctrl-radius-md) var(--ctrl-radius-md) 0 0;";
    let sidebar_style = "width: 160px; min-height: 240px; background: var(--ctrl-bg); border-right: 1px solid var(--ctrl-border); padding: 16px; flex-shrink: 0;";
    let main_style = "flex:1; padding: 16px; min-width: 0;";
    let card_style = "padding: 16px; background: var(--ctrl-bg); border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); margin-bottom: 12px;";
    let row_style = "display: flex; align-items: center; gap: 8px;";

    rsx! {
        div {
            style: "border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); overflow: hidden; margin-bottom: 16px;",

            // 顶栏
            header {
                id: "tour-cx-header",
                style: "{header_style}",
                "Ctrl Admin 后台管理"
            }

            // 侧边栏 + 内容区
            div {
                style: "display: flex;",

                aside {
                    id: "tour-cx-sidebar",
                    style: "{sidebar_style}",
                    div { style: "font-weight: 600; margin-bottom: 12px; color: var(--ctrl-text);", "导航菜单" }
                    div { style: "padding: 6px 8px; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "仪表盘" }
                    div { id: "tour-cx-menu-item", style: "padding: 6px 8px; font-size: var(--ctrl-font-size-sm); background: var(--ctrl-primary-light, #e6f7ff); color: var(--ctrl-primary); border-radius: var(--ctrl-radius-sm);", "数据管理" }
                    div { style: "padding: 6px 8px; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "系统设置" }
                }

                main {
                    style: "{main_style}",

                    // 统计卡片
                    div {
                        id: "tour-cx-cards",
                        style: "{row_style}",
                        for (label, num) in [("今日访问", "1,234"), ("活跃用户", "567"), ("订单量", "89")] {
                            div {
                                key: "{label}",
                                style: "{card_style}",
                                p { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin-bottom: 4px;", "{label}" }
                                p { style: "font-size: 1.25rem; font-weight: 700; color: var(--ctrl-text); margin: 0;", "{num}" }
                            }
                        }
                    }

                    // 模拟数据表格
                    div {
                        id: "tour-cx-table",
                        style: "margin-top: 12px; border: 1px solid var(--ctrl-border); border-radius: var(--ctrl-radius-md); overflow: hidden;",
                        div { style: "display: flex; padding: 10px 16px; background: var(--ctrl-bg-secondary); font-weight: 600; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);",
                            span { style: "flex:2;", "名称" }
                            span { style: "flex:1;", "状态" }
                            span { style: "flex:1;", "操作" }
                        }
                        for (name, status) in [("订单 #1001", "已完成"), ("订单 #1002", "处理中"), ("订单 #1003", "待审核")] {
                            div {
                                key: "{name}",
                                style: "display: flex; padding: 8px 16px; border-top: 1px solid var(--ctrl-border); align-items: center; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);",
                                span { style: "flex:2;", "{name}" }
                                span {
                                    style: "flex:1; color: if status == \"已完成\" {{ \"var(--ctrl-success)\" }} else if status == \"处理中\" {{ \"var(--ctrl-primary)\" }} else {{ \"var(--ctrl-warning)\" }};",
                                    "{status}"
                                }
                                span {
                                    id: if name == "订单 #1002" { "tour-cx-action" } else { "" },
                                    style: "flex:1; display: flex; gap: 4px;",
                                    span { style: "color: var(--ctrl-primary); cursor: pointer;", "编辑" }
                                    span { style: "color: var(--ctrl-text-secondary); cursor: pointer;", "删除" }
                                }
                            }
                        }
                    }
                }
            }
        }

        button {
            style: "padding: 8px 16px; background: var(--ctrl-primary); color: #fff; border: none; border-radius: var(--ctrl-radius-sm); cursor: pointer; font-family: inherit; font-size: var(--ctrl-font-size-sm);",
            onclick: move |_| open.set(true),
            "开始页面导览"
        }

        Tour {
            steps: vec![
                TourStep {
                    target: "#tour-cx-header".to_string(),
                    title: "顶栏导航".to_string(),
                    description: "顶部导航栏，包含系统名称与全局操作入口。".to_string(),
                    placement: Placement::Bottom,
                },
                TourStep {
                    target: "#tour-cx-sidebar".to_string(),
                    title: "侧边菜单".to_string(),
                    description: "左侧菜单提供模块切换——仪表盘、数据管理、系统设置。高亮项表示当前所在模块。".to_string(),
                    placement: Placement::Right,
                },
                TourStep {
                    target: "#tour-cx-cards".to_string(),
                    title: "统计卡片".to_string(),
                    description: "顶部概览区展示核心指标：今日访问量、活跃用户数、订单量。一目了然。".to_string(),
                    placement: Placement::Bottom,
                },
                TourStep {
                    target: "#tour-cx-table".to_string(),
                    title: "数据表格".to_string(),
                    description: "表格展示业务数据列表。每行有对应状态标签和操作按钮。".to_string(),
                    placement: Placement::Top,
                },
                TourStep {
                    target: "#tour-cx-action".to_string(),
                    title: "行操作".to_string(),
                    description: "每一行数据都提供编辑和快捷操作入口，支持行级交互。".to_string(),
                    placement: Placement::Left,
                },
            ],
            open: open(),
            onclose: move |_| open.set(false),
            onfinish: move |_| open.set(false),
        }
    }
}
