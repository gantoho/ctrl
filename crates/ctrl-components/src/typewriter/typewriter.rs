use dioxus::prelude::*;

/// 打字机内部状态
#[derive(Clone, Copy, PartialEq)]
struct TypeState {
    word_idx: usize,
    len: usize,
    deleting: bool,
    acc: f64,
}

/// Typewriter 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TypewriterProps {
    /// 循环播放的文本列表
    pub words: Vec<String>,

    /// 打字速度（毫秒/字）
    #[props(default = 130.0)]
    pub type_speed: f64,

    /// 删除速度（毫秒/字）
    #[props(default = 60.0)]
    pub delete_speed: f64,

    /// 完整单词停留时长（毫秒）
    #[props(default = 1600.0)]
    pub pause: f64,

    /// 是否显示光标
    #[props(default = true)]
    pub cursor: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

const TICK_MS: f64 = 40.0;

/// Typewriter 打字机组件
///
/// 循环地逐字输入、停留、删除文本列表，营造打字机动画效果。
#[allow(non_snake_case)]
pub fn Typewriter(props: TypewriterProps) -> Element {
    const CSS: &str = include_str!("../../assets/typewriter.css");

    let mut state = use_signal(|| TypeState { word_idx: 0, len: 0, deleting: false, acc: 0.0 });

    let words = props.words.clone();
    let type_speed = props.type_speed;
    let delete_speed = props.delete_speed;
    let pause = props.pause;

    // 固定 tick 驱动的状态机，组件销毁时 Interval 随 Signal 一起 drop
    let _interval = use_signal(move || {
        let mut st = state;
        let words = words.clone();
        gloo_timers::callback::Interval::new(TICK_MS as u32, move || {
            if words.is_empty() {
                return;
            }
            let mut s = st.peek().clone();
            let word_len = words[s.word_idx].chars().count();

            let threshold = if !s.deleting && s.len >= word_len {
                pause
            } else if s.deleting {
                delete_speed
            } else {
                type_speed
            };

            s.acc += TICK_MS;
            if s.acc < threshold {
                st.set(s);
                return;
            }
            s.acc = 0.0;

            if !s.deleting {
                if s.len < word_len {
                    s.len += 1;
                } else {
                    s.deleting = true;
                }
            } else if s.len > 0 {
                s.len -= 1;
            } else {
                s.deleting = false;
                s.word_idx = (s.word_idx + 1) % words.len();
            }
            st.set(s);
        })
    });

    let s = state();
    let visible: String = props
        .words
        .get(s.word_idx)
        .map(|w| w.chars().take(s.len).collect())
        .unwrap_or_default();

    let tw_class = if props.class.is_empty() {
        "ctrl-typewriter".to_string()
    } else {
        format!("ctrl-typewriter {}", props.class)
    };

    rsx! {
        style { {CSS} }
        span { class: "{tw_class}", style: "{props.style}",
            span { class: "ctrl-typewriter__text", "{visible}" }
            if props.cursor {
                span { class: "ctrl-typewriter__cursor", "|" }
            }
        }
    }
}
