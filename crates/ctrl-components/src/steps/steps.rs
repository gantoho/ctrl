use dioxus::prelude::*;

/// Steps 步骤条组件属性
#[derive(Props, PartialEq, Clone)]
pub struct StepsProps {
    /// 当前进度索引（从 0 开始），-1 表示未开始
    #[props(default = -1)]
    pub current: i32,

    /// 排列方向：horizontal / vertical
    #[props(default = "horizontal".to_string())]
    pub direction: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（Step）
    pub children: Element,
}

/// 单个步骤属性
#[derive(Props, PartialEq, Clone)]
pub struct StepProps {
    /// 步骤标题
    #[props(default = "".to_string())]
    pub title: String,

    /// 步骤描述
    #[props(default = "".to_string())]
    pub description: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// Steps 步骤条组件
#[allow(non_snake_case)]
pub fn Steps(props: StepsProps) -> Element {
    const CSS: &str = include_str!("../../assets/steps.css");

    let dir_class = if props.direction == "vertical" {
        "ctrl-steps--vertical"
    } else {
        ""
    };

    let wrapper_class = if props.class.is_empty() {
        format!("ctrl-steps {}", dir_class)
    } else {
        format!("ctrl-steps {} {}", dir_class, props.class)
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            {props.children}
        }
    }
}

/// 单个步骤
#[allow(non_snake_case)]
pub fn Step(props: StepProps) -> Element {
    // Step 本身不渲染状态类名，由 Steps 通过 children 遍历时处理
    // 这里渲染一个占位结构，实际状态标记由父组件注入
    rsx! {
        div {
            class: "ctrl-steps__item",
            div {
                class: "ctrl-steps__head",
                span { class: "ctrl-steps__circle", "✓" }
                span { class: "ctrl-steps__title", "{props.title}" }
            }
            if !props.description.is_empty() {
                div {
                    class: "ctrl-steps__description",
                    "{props.description}"
                }
            }
        }
    }
}
