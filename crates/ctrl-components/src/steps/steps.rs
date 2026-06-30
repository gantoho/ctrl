use dioxus::prelude::*;
use ctrl_core::types::Direction;
use std::rc::Rc;
use std::cell::Cell;

// ── 步骤上下文：Steps 通过 Context 向每个 Step 传递 current 和位置计数器 ──
#[derive(Clone)]
struct StepCtx {
    /// 当前激活步骤索引
    current: i32,
    /// 步骤计数器（每个 Step 渲染时递增分配索引）
    counter: Rc<Cell<i32>>,
}

/// Steps 步骤条组件属性
#[derive(Props, PartialEq, Clone)]
pub struct StepsProps {
    /// 当前进度索引（从 0 开始），-1 表示未开始
    #[props(default = -1)]
    pub current: i32,

    /// 排列方向
    #[props(default = Direction::Horizontal)]
    pub direction: Direction,

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

    let dir_class = if props.direction == Direction::Vertical {
        "ctrl-steps--vertical"
    } else {
        ""
    };

    let wrapper_class = if props.class.is_empty() {
        format!("ctrl-steps {}", dir_class)
    } else {
        format!("ctrl-steps {} {}", dir_class, props.class)
    };

    // 通过 Context 将 current 索引和共享计数器传递给所有 Step 子组件
    use_context_provider(|| StepCtx {
        current: props.current,
        counter: Rc::new(Cell::new(0)),
    });

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
    // 从 Steps 上下文获取 current 索引，并通过共享计数器分配自身位置
    let ctx = try_use_context::<StepCtx>();

    let my_index = if let Some(ref c) = ctx {
        let idx = c.counter.get();
        c.counter.set(idx + 1);
        idx
    } else {
        -1 // 无上下文时回退（独立使用 Step）
    };

    let status = if my_index < 0 {
        "wait"
    } else if let Some(ref c) = ctx {
        if my_index < c.current {
            "finish"
        } else if my_index == c.current {
            "process"
        } else {
            "wait"
        }
    } else {
        "wait"
    };

    let item_class = {
        let mut c = String::from("ctrl-steps__item");
        match status {
            "finish" => c.push_str(" ctrl-steps__item--finish"),
            "process" => c.push_str(" ctrl-steps__item--process"),
            _ => {}
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    // 圆圈内容：完成显示 ✓，进行中显示索引+1，待处理显示索引+1
    let circle_content = match status {
        "finish" => "✓".to_string(),
        _ => format!("{}", my_index.max(0) as usize + 1),
    };

    rsx! {
        div {
            class: "{item_class}",
            div {
                class: "ctrl-steps__head",
                span { class: "ctrl-steps__circle", "{circle_content}" }
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
