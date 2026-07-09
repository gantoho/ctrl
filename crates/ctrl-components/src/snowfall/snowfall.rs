use dioxus::prelude::*;

/// 一个简单的 32-bit 确定性伪随机（雪崩哈希）
fn hash32(seed: u32) -> u32 {
    let mut x = seed;
    x = x.wrapping_mul(0x9E3779B1);
    x = x ^ (x >> 16);
    x = x.wrapping_mul(0x85EBCA6B);
    x = x ^ (x >> 13);
    x
}

fn rand01(seed: u32) -> f64 {
    (hash32(seed) >> 8) as f64 / 0x00FFFFFFu32 as f64
}

/// Snowfall 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SnowfallProps {
    /// 雪花数量
    #[props(default = 60)]
    pub count: u32,

    /// 雪花颜色（CSS 颜色值）
    #[props(default = "#ffffff".to_string())]
    pub color: String,

    /// 最小尺寸（px）
    #[props(default = 2.0)]
    pub min_size: f64,

    /// 最大尺寸（px）
    #[props(default = 6.0)]
    pub max_size: f64,

    /// 最短落到底部时间（秒）
    #[props(default = 6.0)]
    pub min_duration: f64,

    /// 最长落到底部时间（秒）
    #[props(default = 14.0)]
    pub max_duration: f64,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 覆盖在雪花之上的内容
    pub children: Element,
}

/// Snowfall 飘雪组件
///
/// 雪花从顶部飘落到底部，每片大小、速度、水平漂移各不相同。
/// 常用于节日 / 冬季主题的背景装饰。`children` 展示在雪花之上。
#[allow(non_snake_case)]
pub fn Snowfall(props: SnowfallProps) -> Element {
    const CSS: &str = include_str!("../../assets/snowfall.css");

    let container_class = if props.class.is_empty() {
        "ctrl-snowfall".to_string()
    } else {
        format!("ctrl-snowfall {}", props.class)
    };

    // 生成每片雪花的 CSS 变量（索引确定性伪随机）
    let count = props.count;
    if count == 0 {
        return rsx! { style { {CSS} } div { class: "{container_class}", style: "{props.style}", {props.children} } };
    }

    let flakes: Vec<String> = (0..count)
        .map(|i| {
            let base = i.wrapping_mul(0x9E3779B1);
            // 水平起始位置 0%~100%
            let left = rand01(base ^ 0x5A1C8E) * 100.0;
            // 水平摆动幅度 20~100px
            let sway = rand01(base ^ 0x3F7D2B) * 80.0 + 20.0;
            // 尺寸
            let size = rand01(base ^ 0x71E44A) * (props.max_size - props.min_size) + props.min_size;
            // 下落时长
            let dur = rand01(base ^ 0xB28D16) * (props.max_duration - props.min_duration) + props.min_duration;
            // 延迟（避免所有片同时开始）
            let delay = rand01(base ^ 0xD435F9) * props.max_duration;
            // 透明度 0.3~0.9
            let opacity = rand01(base ^ 0xE76C83) * 0.6 + 0.3;
            format!(
                "--ctrl-snow-left:{left:.1}%;--ctrl-snow-sway:{sway:.0}px;--ctrl-snow-size:{size:.1}px;--ctrl-snow-dur:{dur:.2}s;--ctrl-snow-delay:{delay:.2}s;--ctrl-snow-opacity:{opacity:.2};",
                left = left,
                sway = sway,
                size = size,
                dur = dur,
                delay = delay,
                opacity = opacity,
            )
        })
        .collect();

    let color = props.color.clone();

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: " {props.style}",
            div { class: "ctrl-snowfall__layer",
                for flake in flakes.iter() {
                    div {
                        class: "ctrl-snowfall__flake",
                        style: "{flake}",
                        span { class: "ctrl-snowfall__dot", style: "color:{color};" }
                    }
                }
            }
            div { class: "ctrl-snowfall__content", {props.children} }
        }
    }
}
