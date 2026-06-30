use dioxus::prelude::*;
use ctrl::prelude::*;
use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn ButtonPage() -> Element {
    rsx! {
        h1 { "Button 按钮" }
        p { "按钮用于触发操作。Ctrl UI 提供了四种语义变体和三种尺寸，支持禁用、块级等状态。" }

        h2 { "变体 Variant" }

        DemoBox {
            title: "Primary - 主要按钮".to_string(),
            description: Some("最重要的操作按钮，用于提交、确认等场景。".to_string()),
            demo: rsx! {
                Button { variant: Variant::Primary, "Primary" }
                Button { variant: Variant::Primary, disabled: true, "禁用" }
            },
            code: "Button { variant: Variant::Primary, \"Primary\" }\nButton { variant: Variant::Primary, disabled: true, \"禁用\" }".to_string(),
        }

        DemoBox {
            title: "Secondary - 次要按钮".to_string(),
            description: Some("用于取消、返回等次要操作。".to_string()),
            demo: rsx! {
                Button { variant: Variant::Secondary, "Secondary" }
                Button { variant: Variant::Secondary, disabled: true, "禁用" }
            },
            code: "Button { variant: Variant::Secondary, \"Secondary\" }".to_string(),
        }

        DemoBox {
            title: "Outline - 描边按钮".to_string(),
            description: Some("中等强调，适合编辑、查看等操作。".to_string()),
            demo: rsx! {
                Button { variant: Variant::Outline, "Outline" }
                Button { variant: Variant::Outline, disabled: true, "禁用" }
            },
            code: "Button { variant: Variant::Outline, \"Outline\" }".to_string(),
        }

        DemoBox {
            title: "Ghost - 幽灵按钮".to_string(),
            description: Some("低强调，常用于列表中的操作链接。".to_string()),
            demo: rsx! {
                Button { variant: Variant::Ghost, "Ghost" }
                Button { variant: Variant::Ghost, disabled: true, "禁用" }
            },
            code: "Button { variant: Variant::Ghost, \"Ghost\" }".to_string(),
        }

        h2 { "尺寸 Size" }

        DemoBox {
            title: "三种尺寸".to_string(),
            description: Some("Sm 28px / Md 36px / Lg 44px".to_string()),
            demo: rsx! {
                Button { variant: Variant::Primary, size: Size::Sm, "Small" }
                Button { variant: Variant::Primary, size: Size::Md, "Medium" }
                Button { variant: Variant::Primary, size: Size::Lg, "Large" }
            },
            code: "Button { size: Size::Sm, \"Small\" }\nButton { size: Size::Md, \"Medium\" }\nButton { size: Size::Lg, \"Large\" }".to_string(),
        }

        DemoBox {
            title: "块级按钮".to_string(),
            description: Some("block 为 true 时按钮宽度撑满父容器。".to_string()),
            demo: rsx! {
                Button { variant: Variant::Primary, block: true, "块级按钮" }
                Button { variant: Variant::Outline, block: true, "块级描边" }
            },
            code: "Button { variant: Variant::Primary, block: true, \"块级按钮\" }".to_string(),
        }

        DemoBox {
            title: "交互示例".to_string(),
            description: Some("点击按钮实时更新计数，展示事件绑定和禁用切换。".to_string()),
            demo: rsx! { ButtonInteract {} },
            code: "let mut count = use_signal(|| 0);\nlet mut disabled = use_signal(|| false);\n\nButton {\n    disabled: disabled(),\n    onclick: move |_| count.set(count() + 1),\n    \"点击次数: {count()}\"\n}\nButton {\n    onclick: move |_| disabled.set(!disabled()),\n    if disabled() { \"恢复\" } else { \"禁用\" }\n}".to_string(),
        }

        h2 { "Button Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("variant", "Variant", "Primary", "按钮语义变体"),
            ("size", "Size", "Md", "按钮尺寸"),
            ("disabled", "bool", "false", "是否禁用"),
            ("loading", "bool", "false", "是否加载中"),
            ("block", "bool", "false", "是否块级"),
            ("r#type", "String", "\"button\"", "原生 button type 属性"),
            ("class", "String", "\"\"", "自定义 CSS 类名"),
            ("style", "String", "\"\"", "自定义内联样式"),
            ("onclick", "Option<EventHandler>", "None", "点击事件"),
            ("children", "Element", "—", "子元素"),
        ]}
    }
}
