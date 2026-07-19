use dioxus::prelude::*;

/// WordRotate 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct WordRotateProps {
    /// 循环轮播的单词列表
    pub words: Vec<String>,

    /// 每个单词显示时长（毫秒）
    #[props(default = 2000)]
    pub duration: u32,

    /// 颜色（CSS 值，留空走主题主色）
    #[props(default = "".to_string())]
    pub color: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// WordRotate 单词轮播组件
///
/// 标题中某个词平滑地垂直翻转切换，每个单词停留 duration 毫秒后翻入下一个。
/// 常用于 hero 标题中的动态关键词展示（"Build [fast / beautiful / secure] apps"）。
#[allow(non_snake_case)]
pub fn WordRotate(props: WordRotateProps) -> Element {
    const CSS: &str = include_str!("../../assets/word_rotate.css");

    let idx = use_signal(|| 0usize);
    let words = props.words.clone();
    let duration = props.duration;

    let _interval = use_signal(move || {
        let mut idx_ref = idx;
        let len = words.len();
        gloo_timers::callback::Interval::new(duration, move || {
            if len == 0 {
                return;
            }
            let mut v = idx_ref.peek().clone();
            v = (v + 1) % len;
            idx_ref.set(v);
        })
    });

    let container_class = if props.class.is_empty() {
        "ctrl-word-rotate".to_string()
    } else {
        format!("ctrl-word-rotate {}", props.class)
    };

    let color = if props.color.is_empty() {
        "var(--ctrl-primary)".to_string()
    } else {
        props.color.clone()
    };

    let current = props.words.get(idx()).map(|w| w.as_str()).unwrap_or("");

    rsx! {
        style { {CSS} }
        span {
            class: "{container_class}",
            style: "color:{color}; {props.style}",
            span { class: "ctrl-word-rotate__inner",
                for k in [idx()] {
                    span { key: "{k}", class: "ctrl-word-rotate__word", "{current}" }
                }
            }
        }
    }
}
