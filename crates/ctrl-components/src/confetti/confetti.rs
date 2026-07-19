use dioxus::prelude::*;
use gloo_timers::callback::Timeout;

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

/// 彩带粒子形状
#[derive(Clone, PartialEq)]
pub enum ConfettiShape {
    /// 矩形（圆角小矩形）
    Rect,
    /// 圆形
    Circle,
    /// 正方形
    Square,
    /// 长条（细长纸带）
    Strip,
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

    /// 粒子形状列表（随机分配，留空默认混合 Rect + Circle）
    #[props(default = Vec::new())]
    pub shapes: Vec<ConfettiShape>,

    /// 单轮动画时长，单位毫秒
    #[props(default = 2500)]
    pub duration: u32,

    /// 是否启用蓄力模式：长按鼠标蓄力，松开时按蓄力程度喷射
    #[props(default = false)]
    pub charge: bool,

    /// 蓄力最小倍率（瞬时点击时的强度）
    #[props(default = 0.3)]
    pub charge_min: f64,

    /// 蓄力最大倍率（蓄满时的强度）
    #[props(default = 3.0)]
    pub charge_max: f64,

    /// 蓄力满所需时长（毫秒）
    #[props(default = 2000)]
    pub charge_duration: u32,

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
///
/// 启用 `charge` 后，长按容器将进入蓄力状态，松开时彩带强度随蓄力时长递增。
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

    let shapes = if props.shapes.is_empty() {
        vec![ConfettiShape::Rect, ConfettiShape::Circle]
    } else {
        props.shapes.clone()
    };

    let duration = props.duration;
    let count = props.count;

    // ── 蓄力状态 ──
    let charge_level = use_signal(|| 0.0f64);
    let is_charging = use_signal(|| false);
    let charge_trigger = use_signal(|| 0u32);
    let charge_intensity = use_signal(|| 1.0f64);

    let charge_enabled = props.charge;
    let charge_min = props.charge_min;
    let charge_max = props.charge_max;
    let charge_dur = props.charge_duration;

    // 蓄力 tick：递归 timeout，每 ~16ms 更新一次蓄力等级
    let start_charge = {
        let mut level = charge_level;
        let mut charging = is_charging;
        let ch_dur = charge_dur;
        move || {
            charging.set(true);
            level.set(0.0);
            let tick_step = 16.0 / ch_dur as f64;
            let lv = level;
            let chg = charging;
            fn tick(
                mut lv: Signal<f64>,
                chg: Signal<bool>,
                step: f64,
            ) {
                if !chg() {
                    return;
                }
                let new_level = (lv() + step).min(1.0);
                lv.set(new_level);
                if new_level < 1.0 {
                    let timeout = Timeout::new(16, move || {
                        tick(lv, chg, step);
                    });
                    timeout.forget();
                }
            }
            tick(lv, chg, tick_step);
        }
    };

    let fire_charge = {
        let mut level = charge_level;
        let mut charging = is_charging;
        let mut c_trigger = charge_trigger;
        let mut intensity = charge_intensity;
        move || {
            if !charging() {
                return;
            }
            charging.set(false);
            let lv = level();
            let it = charge_min + lv * (charge_max - charge_min);
            intensity.set(it);
            level.set(0.0);
            c_trigger += 1;
        }
    };

    // 对外部 trigger 和蓄力 trigger 做合并：取最大的 trigger 变化
    let effective_trigger = if charge_enabled && charge_trigger() > 0 {
        charge_trigger()
    } else {
        props.trigger
    };
    let effective_intensity = if charge_enabled {
        charge_intensity()
    } else {
        1.0
    };
    let effective_count = ((count as f64) * effective_intensity).max(5.0) as usize;

    // ── 蓄力指示器 ──
    let charge_indicator = if charge_enabled && is_charging() {
        let lv = charge_level();
        let pct = (lv * 100.0) as u32;
        let ring_size = 60.0 + lv * 80.0; // 60→140px 环
        let ring_opacity = 0.5 + lv * 0.5;
        rsx! {
            div { class: "ctrl-confetti__charge-overlay",
                div {
                    class: "ctrl-confetti__charge-ring",
                    style: "width:{ring_size}px;height:{ring_size}px;opacity:{ring_opacity};",
                }
                div { class: "ctrl-confetti__charge-text",
                    span { class: "ctrl-confetti__charge-label", "蓄力中" }
                    span { class: "ctrl-confetti__charge-pct", "{pct}%" }
                }
            }
        }
    } else {
        rsx! {}
    };

    // charge 模式下容器接收指针事件
    let charge_class = if charge_enabled { " ctrl-confetti--charge" } else { "" };
    let container_full_class = format!("{container_class}{charge_class}");

    rsx! {
        style { {CSS} }
        div {
            class: "{container_full_class}",
            style: "{props.style}",
            onmousedown: {
                let mut sc = start_charge.clone();
                move |_| { if charge_enabled { sc(); } }
            },
            onmouseup: {
                let mut fc = fire_charge.clone();
                move |_| { if charge_enabled { fc(); } }
            },
            onmouseleave: {
                let chg = is_charging;
                let mut fc = fire_charge.clone();
                move |_| { if charge_enabled && chg() { fc(); } }
            },
            ontouchstart: {
                let mut sc = start_charge.clone();
                move |e: TouchEvent| {
                    if charge_enabled {
                        e.prevent_default();
                        sc();
                    }
                }
            },
            ontouchend: {
                let mut fc = fire_charge.clone();
                move |_| { if charge_enabled { fc(); } }
            },
            {charge_indicator}
            if effective_trigger > 0 {
                {
                    let it = effective_intensity;
                    rsx! {
                        div { key: "{effective_trigger}", class: "ctrl-confetti__burst",
                            for i in 0..effective_count {
                                {
                                    // 每个参数用不同 salt 的哈希
                                    let base = (i as u32)
                                        .wrapping_mul(0x9E37_79B1)
                                        .wrapping_add(effective_trigger.wrapping_mul(0x85EB_CA6B));
                                    let f_tx = rand01(base ^ 0xA511_E9B3);
                                    let f_peak = rand01(base ^ 0x1B87_3593);
                                    let f_fall = rand01(base ^ 0xCC9E_2D51);
                                    let f_rs = rand01(base ^ 0x27D4_EB2F);
                                    let f_dir = rand01(base ^ 0x1656_67B1);
                                    let f_re = rand01(base ^ 0xD6E8_FEB8);
                                    let f_dur = rand01(base ^ 0x1657_4E35);
                                    let f_delay = rand01(base ^ 0x3B97_0D99);
                                    let f_shape = rand01(base ^ 0x2545_F491);

                                    // 水平散开随强度增加
                                    let tx = (f_tx * 400.0 - 200.0) * it as f32;
                                    // 上升峰值随强度增加
                                    let peak = -(60.0 + f_peak * 160.0 * it as f32);
                                    // 下落终点随强度增加
                                    let fall = (180.0 + f_fall * 280.0) * it as f32;
                                    let rs = f_rs * 360.0;
                                    let dir = if f_dir < 0.5 { 1.0 } else { -1.0 };
                                    let re = rs + dir * (360.0 + f_re * 900.0);
                                    let dur = duration as f32 * (0.6 + f_dur * 0.7);
                                    let delay = f_delay * 220.0;

                                    let color = &colors[i % colors.len()];
                                    let shape = &shapes[i % shapes.len()];
                                    let (w, h, round) = match shape {
                                        ConfettiShape::Rect => {
                                            let w = 5 + (f_shape * 5.0) as u32;
                                            let h = 8 + (f_peak * 8.0) as u32;
                                            (w, h, "2px")
                                        }
                                        ConfettiShape::Circle => {
                                            let s = 6 + (f_shape * 8.0) as u32;
                                            (s, s, "50%")
                                        }
                                        ConfettiShape::Square => {
                                            let s = 5 + (f_shape * 6.0) as u32;
                                            (s, s, "2px")
                                        }
                                        ConfettiShape::Strip => {
                                            let w = 3 + (f_shape * 3.0) as u32;
                                            let h = 12 + (f_peak * 14.0) as u32;
                                            (w, h, "1px")
                                        }
                                    };

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
    }
}
