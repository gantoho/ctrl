use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn SkeletonPage() -> Element {
    rsx! {
div { id: "skeleton", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Skeleton 骨架屏" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "内容加载时的占位动画，支持文字、头像、图片、按钮等多种变体，以及列表、卡片等复合骨架。" }

            // 基本变体
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin-bottom: 20px;", "Skeleton - 基础变体" }
            DemoBox { title: "形状控制".to_string(), description: Some("通过 shape 属性控制骨架块的圆角样式。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: flex-end;",
                        div { style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                            Skeleton { variant: "rect".to_string(), width: "64px".to_string(), height: "64px".to_string() }
                            span { style: "font-size: var(--ctrl-font-size-xs); color: var(--ctrl-text-secondary);", "default" }
                        }
                        div { style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                            Skeleton { variant: "rect".to_string(), shape: Shape::Rounded, width: "64px".to_string(), height: "64px".to_string() }
                            span { style: "font-size: var(--ctrl-font-size-xs); color: var(--ctrl-text-secondary);", "round" }
                        }
                        div { style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                            Skeleton { variant: "rect".to_string(), shape: Shape::Circle, width: "64px".to_string(), height: "64px".to_string() }
                            span { style: "font-size: var(--ctrl-font-size-xs); color: var(--ctrl-text-secondary);", "circle" }
                        }
                    }
                },
                code: "Skeleton { variant: \"rect\", shape: \"default\", width: \"64px\", height: \"64px\" }\nSkeleton { variant: \"rect\", shape: \"round\", width: \"64px\", height: \"64px\" }\nSkeleton { variant: \"rect\", shape: \"circle\", width: \"64px\", height: \"64px\" }".to_string(),
            }
            DemoBox { title: "静止态 vs 动画".to_string(), description: Some("animated: false 时停止闪烁，仅显示灰色占位。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        Skeleton { variant: "text".to_string(), rows: 2, animated: true, width: "200px".to_string() }
                        Skeleton { variant: "text".to_string(), rows: 2, animated: false, width: "200px".to_string() }
                    }
                },
                code: "Skeleton { variant: \"text\", rows: 2, animated: true }\nSkeleton { variant: \"text\", rows: 2, animated: false }".to_string(),
            }
            DemoBox { title: "文本多行".to_string(), description: None,
                demo: rsx! { Skeleton { variant: "text".to_string(), rows: 3 } },
                code: "Skeleton { variant: \"text\".to_string(), rows: 3 }".to_string(),
            }
            DemoBox { title: "重复列表".to_string(), description: Some("count 属性生成重复骨架块，适用于占位列表场景。".to_string()),
                demo: rsx! {
                    Skeleton { variant: "text".to_string(), count: 4, gap: "12px".to_string(), width: "100%".to_string() }
                },
                code: "Skeleton { variant: \"text\", count: 4, gap: \"12px\" }".to_string(),
            }
            DemoBox { title: "圆形头像组".to_string(), description: None,
                demo: rsx! {
                    div { style: "display: flex; gap: 12px;",
                        Skeleton { variant: "avatar".to_string(), shape: Shape::Circle, width: "40px".to_string(), height: "40px".to_string(), count: 3 }
                    }
                },
                code: "Skeleton { variant: \"avatar\", shape: \"circle\", width: \"40px\", height: \"40px\", count: 3 }".to_string(),
            }

            // 复合骨架
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonCard - 卡片骨架" }
            DemoBox { title: "卡片骨架".to_string(), description: Some("图片 + 标题 + 文字行的复合卡片骨架。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; gap: 16px;",
                        SkeletonCard { width: "240px".to_string(), rows: 2 }
                        SkeletonCard { width: "240px".to_string(), rows: 1 }
                    }
                },
                code: "SkeletonCard { width: \"240px\".to_string(), rows: 2 }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonList - 列表骨架" }
            DemoBox { title: "列表骨架".to_string(), description: Some("头像 + 文字组合的列表骨架，适用于好友列表、通知列表等场景。".to_string()),
                demo: rsx! {
                    SkeletonList { count: 3, rows: 2 }
                },
                code: "SkeletonList { count: 3, rows: 2 }".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonRow - 行骨架" }
            DemoBox { title: "头像 + 文字行".to_string(), description: Some("可配置头像尺寸和是否显示头像。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        SkeletonRow { rows: 2 }
                        SkeletonRow { rows: 2, avatar: false }
                        SkeletonRow { rows: 1, avatar_size: "32px".to_string() }
                    }
                },
                code: "SkeletonRow { rows: 2 }\nSkeletonRow { rows: 2, avatar: false }\nSkeletonRow { rows: 1, avatar_size: \"32px\".to_string() }".to_string(),
            }

            // Skeleton Props
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Skeleton Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("variant", "String", "\"text\"", "变体（text / title / avatar / image / button / rect）"),
                ("rows", "usize", "3", "行数（仅 text 有效）"),
                ("shape", "String", "\"default\"", "形状（default / circle / round）"),
                ("count", "usize", "0", "重复次数（>0 时生成列表，忽略 rows）"),
                ("gap", "String", "\"8px\"", "count > 1 时的项间距"),
                ("width", "String", "\"\"", "宽度"),
                ("height", "String", "\"\"", "高度"),
                ("animated", "bool", "true", "是否显示闪烁动画"),
                ("loading", "Option<bool>", "None", "None 始终显示骨架；Some(false) 显示 children"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "加载完成后显示的内容"),
            ]}

            // SkeletonCard Props
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonCard Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("rows", "usize", "2", "文字行数"),
                ("image_height", "String", "\"180px\"", "图片区高度"),
                ("width", "String", "\"\"", "卡片宽度"),
                ("animated", "bool", "true", "是否显示动画"),
                ("loading", "Option<bool>", "None", "加载控制"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "加载完成后显示的内容"),
            ]}

            // SkeletonList Props
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonList Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("count", "usize", "3", "列表条目数"),
                ("avatar", "bool", "true", "是否显示头像"),
                ("avatar_size", "String", "\"40px\"", "头像尺寸"),
                ("rows", "usize", "2", "每项文字行数"),
                ("animated", "bool", "true", "是否显示动画"),
                ("loading", "Option<bool>", "None", "加载控制"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "加载完成后显示的内容"),
            ]}

            // SkeletonRow Props
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "SkeletonRow Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("rows", "usize", "3", "文字行数"),
                ("avatar", "bool", "true", "是否显示头像"),
                ("avatar_size", "String", "\"48px\"", "头像尺寸（宽高相同）"),
                ("animated", "bool", "true", "是否显示动画"),
                ("loading", "Option<bool>", "None", "加载控制"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "加载完成后显示的内容"),
            ]}
        }
    }
}
