use dioxus::prelude::*;
use ctrl::prelude::*;
use ctrl::types::ScrollBehavior;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn BacktopPage() -> Element {
    rsx! {
div { id: "backtop", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Backtop 回到顶部" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 40px;", "页面滚动后显示在右下角的回到顶部按钮，点击后平滑滚动到顶部。支持多种缓动效果和弹簧阻尼模式。" }

            style { {r#"
.ctrl-backtop.demo-bt-default { right: 40px; bottom: 40px; }
.ctrl-backtop.demo-bt-default::after { content: "easeOutCubic"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-expo { left: 40px; right: auto; bottom: 40px; }
.ctrl-backtop.demo-bt-expo::after { content: "easeOutExpo 500ms"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-back { right: 40px; bottom: 100px; }
.ctrl-backtop.demo-bt-back::after { content: "easeOutBack 500ms"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-elastic { left: 40px; right: auto; bottom: 100px; }
.ctrl-backtop.demo-bt-elastic::after { content: "easeOutElastic 600ms"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-damping { right: 40px; bottom: 160px; }
.ctrl-backtop.demo-bt-damping::after { content: "damping"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-auto { left: 40px; right: auto; bottom: 160px; }
.ctrl-backtop.demo-bt-auto::after { content: "auto"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-fast { right: 40px; bottom: 220px; }
.ctrl-backtop.demo-bt-fast::after { content: "200ms"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-slow { left: 40px; right: auto; bottom: 220px; }
.ctrl-backtop.demo-bt-slow::after { content: "800ms"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-red { right: 40px; bottom: 280px; background: #e74c3c; color: #fff; border-color: transparent; }
.ctrl-backtop.demo-bt-red::after { content: "custom style"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
.ctrl-backtop.demo-bt-bottom { left: 50%; right: auto; top: 40px; transform: translateX(-50%); }
.ctrl-backtop.demo-bt-bottom::after { content: "回到底部 ↓"; position: absolute; left: 50%; top: calc(100% + 4px); transform: translateX(-50%); font-size: 10px; color: var(--ctrl-text-secondary); white-space: nowrap; }
"#} }

            DemoBox { title: "可交互示例（真实组件）".to_string(), description: Some("滚动页面后，下方各位置的按钮会依次出现，点击体验不同的回顶效果。".to_string()),
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px; width: 100%;",
                        div { style: "padding: 16px; border-radius: 8px; background: var(--ctrl-bg-secondary);",
                            p { style: "margin: 0 0 8px 0; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text); font-weight: 600;", "按钮位置说明" }
                            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                                span { "[右下] easeOutCubic（默认 400ms）" }
                                span { "[左下] easeOutExpo（500ms）" }
                                span { "[右中] easeOutBack（回弹 500ms）" }
                                span { "[左中] easeOutElastic（弹性 600ms）" }
                                span { "[右上] damping（弹簧阻尼）" }
                                span { "[左上] auto（瞬间跳转）" }
                                span { "[右侧] 200ms（快速）" }
                                span { "[左侧] 800ms（舒缓）" }
                                span { "[最上] 自定义红色样式" }
                                span { "[顶部居中] 回到底部 ↓ (target_position: bottom)" }
                            }
                        }
                    }
                    // 各位置的真实 Backtop 实例
                    Backtop { class: "demo-bt-default".to_string() }
                    Backtop { class: "demo-bt-expo".to_string(), easing: Easing::EaseOutExpo, duration: 500, visibility_height: 300 }
                    Backtop { class: "demo-bt-back".to_string(), easing: Easing::EaseOutBack, duration: 500, visibility_height: 300 }
                    Backtop { class: "demo-bt-elastic".to_string(), easing: Easing::EaseOutElastic, duration: 600, visibility_height: 300 }
                    Backtop { class: "demo-bt-damping".to_string(), damping: true, visibility_height: 300 }
                    Backtop { class: "demo-bt-auto".to_string(), behavior: ScrollBehavior::Auto, visibility_height: 300 }
                    Backtop { class: "demo-bt-fast".to_string(), duration: 200, visibility_height: 300 }
                    Backtop { class: "demo-bt-slow".to_string(), duration: 800, visibility_height: 300 }
                    Backtop { class: "demo-bt-red".to_string(), visibility_height: 300 }
                    Backtop { class: "demo-bt-bottom".to_string(), target_position: Placement::Bottom, visibility_height: 100 }
                },
                code: "Backtop {}                                              // 默认 easeOutCubic 400ms\nBacktop { easing: \"easeOutExpo\".to_string(), duration: 500 }     // 指数减速\nBacktop { easing: \"easeOutBack\".to_string(), duration: 500 }     // 回弹感\nBacktop { easing: \"easeOutElastic\".to_string(), duration: 600 }  // 弹性震荡\nBacktop { damping: true }                                         // 弹簧阻尼\nBacktop { behavior: \"auto\".to_string() }                          // 瞬间跳转\nBacktop { duration: 200 }                                          // 快速\nBacktop { duration: 800 }                                          // 舒缓\nBacktop { class: \"my-backtop\".to_string() }                       // 自定义样式\nBacktop { target_position: \"bottom\".to_string() }                 // 回到底部".to_string(),
            }

            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("target", "String", "\"\"", "监听滚动的容器选择器（默认 window）"),
                ("visibility_height", "u32", "200", "显示阈值（px），bottom 模式时为距离底部的距离"),
                ("behavior", "String", "\"smooth\"", "滚动行为（smooth 平滑 / auto 瞬间）"),
                ("duration", "u32", "400", "滚动动画时长（ms），behavior=auto 时无效"),
                ("easing", "String", "\"easeOutCubic\"", "缓动函数（easeOutQuad / easeOutCubic / easeOutQuart / easeOutQuint / easeOutExpo / easeOutBack / easeOutElastic）"),
                ("damping", "bool", "false", "是否启用弹簧阻尼效果（启用后忽略 duration 和 easing）"),
                ("target_position", "String", "\"top\"", "目标位置：top（顶部）或 bottom（底部）"),
                ("onclick", "Option<EventHandler>", "None", "点击回调"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("children", "Element", "—", "自定义按钮内容"),
            ]}
        }
    }
}
