use dioxus::prelude::*;

/// 整数哈希（murmur3 finalizer 变体），具备雪崩效应：
/// 相邻输入也能得到充分打散的输出，避免规律性轨迹。
fn hash32(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x
}

/// 由种子取 [0, 1) 归一化随机数
fn rand01(seed: u32) -> f32 {
    (hash32(seed) % 100_000) as f32 / 100_000.0
}

/// Confetti 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ConfettiProps {
    /// 触发器：每次该值变化即喷射一轮彩带（通常传入一个自增计数）
    #[props(default = 0)]
    pub trigger: u32,

    /// 彩带粒子数量
    #[props(default = 60)]
    pub count: usize,

    /// 粒子颜色列表（留空使用默认多彩组合）
    #[props(default = Vec::new())]
    pub colors: Vec<String>,

    /// 单轮动画时长，单位毫秒
    #[props(default = 2500)]
    pub duration: u32,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// Confetti 彩带庆祝组件
///
/// 每当 `trigger` 变化时，从容器中心喷射一轮彩带粒子，常用于支付成功、任务完成等庆祝场景。
/// 需放在 `position: relative` 的容器内，覆盖满父容器。
#[allow(non_snake_case)]
pub fn Confetti(props: ConfettiProps) -> Element {
    const CSS: &str = include_str!("../../assets/confetti.css");

    let container_class = if props.class.is_empty() {
        "ctrl-confetti".to_string()
    } else {
        format!("ctrl-confetti {}", props.class)
    };

    let colors = if props.colors.is_empty() {
        vec![
            "#6366f1".to_string(),
            "#a855f7".to_string(),
            "#06b6d4".to_string(),
            "#22c55e".to_string(),
            "#f59e0b".to_string(),
            "#ef4444".to_string(),
        ]
    } else {
        props.colors.clone()
    };

    let duration = props.duration;
    let count = props.count;

    // trigger 为非 0 时渲染，绑 key = trigger；trigger 变化时 Dioxus diff 发现 key 变了
    // 会销毁旧 div 并全新挂载 → 所有粒子的 CSS animation 从 0 帧重播
    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{props.style}",
            if props.trigger > 0 {
                div { key: "{props.trigger}", class: "ctrl-confetti__burst",
                    for i in 0..count {
                        {
                            // 每个参数用不同 salt 的哈希，保证相邻粒子的每项属性都充分打散
                            let base = (i as u32)
                                .wrapping_mul(0x9E37_79B1)
                                .wrapping_add(props.trigger.wrapping_mul(0x85EB_CA6B));
                            let f_tx = rand01(base ^ 0xA511_E9B3);
                            let f_peak = rand01(base ^ 0x1B87_3593);
                            let f_fall = rand01(base ^ 0xCC9E_2D51);
                            let f_rs = rand01(base ^ 0x27D4_EB2F);
                            let f_dir = rand01(base ^ 0x1656_67B1);
                            let f_re = rand01(base ^ 0xD6E8_FEB8);
                            let f_dur = rand01(base ^ 0x1657_4E35);
                            let f_delay = rand01(base ^ 0x3B97_0D99);
                            let f_shape = rand01(base ^ 0x2545_F491);

                            // 水平散开：-200 .. 200 px（惯性衰减）
                            let tx = f_tx * 400.0 - 200.0;
                            // 上升峰值：-60 .. -220 px（幅度差异大）
                            let peak = -(60.0 + f_peak * 160.0);
                            // 下落终点：180 .. 460 px（掉出容器）
                            let fall = 180.0 + f_fall * 280.0;
                            // 旋转起止（含正反向）
                            let rs = f_rs * 360.0;
                            let dir = if f_dir < 0.5 { 1.0 } else { -1.0 };
                            let re = rs + dir * (360.0 + f_re * 900.0);
                            // 每片独立时长：60% .. 130% 基准，打散节奏
                            let dur = duration as f32 * (0.6 + f_dur * 0.7);
                            // 每片独立延迟：0 .. 220ms
                            let delay = f_delay * 220.0;

                            let color = &colors[i % colors.len()];
                            let w = 5 + (f_shape * 5.0) as u32;
                            let h = 8 + (f_peak * 8.0) as u32;
                            let round = if f_shape < 0.33 { "50%" } else { "2px" };

                            let layer_vars = format!(
                                "--ctrl-c-tx:{tx:.0}px; --ctrl-c-peak:{peak:.0}px; --ctrl-c-fall:{fall:.0}px; --ctrl-c-dur:{dur:.0}ms; --ctrl-c-delay:{delay:.0}ms;"
                            );
                            let chip_style = format!(
                                "--ctrl-c-rs:{rs:.0}deg; --ctrl-c-re:{re:.0}deg; background:{color}; width:{w}px; height:{h}px; border-radius:{round};"
                            );
                            rsx! {
                                span { key: "{i}", class: "ctrl-confetti__x", style: "{layer_vars}",
                                    span { class: "ctrl-confetti__y",
                                        span { class: "ctrl-confetti__chip", style: "{chip_style}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
