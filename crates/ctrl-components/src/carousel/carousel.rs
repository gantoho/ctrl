use dioxus::prelude::*;
use ctrl_core::types::Effect;
use std::cell::Cell;
use std::rc::Rc;

// ── Carousel 上下文：向 CarouselSlide 传递 active_index 和分配位置 ──
#[derive(Clone)]
struct CarouselCtx {
    active: Signal<usize>,
    counter: Rc<Cell<usize>>,
}

/// Carousel 走马灯组件属性
#[derive(Props, PartialEq, Clone)]
pub struct CarouselProps {
    /// 是否自动播放
    #[props(default = true)]
    pub autoplay: bool,

    /// 自动播放间隔（毫秒）
    #[props(default = 3000)]
    pub interval: u64,

    /// 是否循环播放
    #[props(default = true)]
    pub loop_play: bool,

    /// 是否显示箭头
    #[props(default = true)]
    pub arrows: bool,

    /// 是否显示指示器
    #[props(default = true)]
    pub dots: bool,

    /// 过渡效果
    #[props(default = Effect::default())]
    pub effect: Effect,

    /// 容器高度（如 "300px"）
    #[props(default = "300px".to_string())]
    pub height: String,

    /// 幻灯片总数（自动统计 CarouselSlide 数量时可不传）
    #[props(default = 0)]
    pub total_hint: usize,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素（CarouselSlide）
    pub children: Element,
}

/// CarouselSlide 子组件属性
#[derive(Props, PartialEq, Clone)]
pub struct CarouselSlideProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（单张幻灯片内容）
    pub children: Element,
}

/// CarouselSlide 单张幻灯片组件
///
/// 嵌套在 Carousel 中使用，自动获取当前激活状态。
/// 可通过 class 接收 `ctrl-carousel__slide--active` 实现进入/退出动画。
#[allow(non_snake_case)]
pub fn CarouselSlide(props: CarouselSlideProps) -> Element {
    let ctx = try_use_context::<CarouselCtx>();
    let my_index = if let Some(ref c) = ctx {
        let idx = c.counter.get();
        c.counter.set(idx + 1);
        idx
    } else {
        0
    };

    let is_active = ctx.as_ref().map(|c| (c.active)() == my_index).unwrap_or(false);
    let slide_class = if props.class.is_empty() {
        format!("ctrl-carousel__slide")
    } else {
        format!("ctrl-carousel__slide {}", props.class)
    };

    rsx! {
        div {
            class: "{slide_class}",
            "data-active": if is_active { "true" } else { "false" },
            "data-index": "{my_index}",
            {props.children}
        }
    }
}

/// Carousel 走马灯组件
#[allow(non_snake_case)]
pub fn Carousel(props: CarouselProps) -> Element {
    const CSS: &str = include_str!("../../assets/carousel.css");
    let active_index = use_signal(|| 0usize);
    let loop_play = props.loop_play;

    // 通过 Context 向 CarouselSlide 传递 active_index 和计数器
    use_context_provider(|| CarouselCtx {
        active: active_index.clone(),
        counter: Rc::new(Cell::new(0)),
    });

    let total = props.total_hint.max(1);

    let carousel_class = {
        let mut c = String::from("ctrl-carousel");
        if props.effect == Effect::Fade {
            c.push_str(" ctrl-carousel--fade");
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    // 自动播放（带取消机制）
    if props.autoplay {
        #[allow(unused_mut)]
        let mut index = active_index.clone();
        let interval = props.interval;
        let total = total;
        let loop_play = loop_play;
        let cancelled = Rc::new(Cell::new(false));
        let cancelled_for_effect = cancelled.clone();

        use_effect(move || {
            let cancelled = cancelled_for_effect.clone();
            #[cfg(target_arch = "wasm32")]
            {
                wasm_bindgen_futures::spawn_local(async move {
                    loop {
                        if cancelled.get() { break; }
                        gloo_timers::future::TimeoutFuture::new(interval as u32).await;
                        if cancelled.get() { break; }
                        let current = index();
                        let next = if current + 1 >= total {
                            if loop_play { 0 } else { current }
                        } else {
                            current + 1
                        };
                        index.set(next);
                    }
                });
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = index;
                let _ = cancelled;
                let _ = interval;
                let _ = total;
                let _ = loop_play;
            }
        });

        // 组件卸载时取消循环
        use_drop(move || {
            cancelled.set(true);
        });
    }

    let prev = {
        let mut index = active_index.clone();
        let total = total;
        let loop_play = loop_play;
        move |_| {
            let current = index();
            let new = if current == 0 {
                if loop_play { total - 1 } else { 0 }
            } else {
                current - 1
            };
            index.set(new);
        }
    };

    let next = {
        let mut index = active_index.clone();
        let total = total;
        let loop_play = loop_play;
        move |_| {
            let current = index();
            let new = if current + 1 >= total {
                if loop_play { 0 } else { current }
            } else {
                current + 1
            };
            index.set(new);
        }
    };

    let mut go_to = {
        let mut index = active_index.clone();
        move |i: usize| {
            index.set(i);
        }
    };

    // 渲染指示器
    let dots_ui = if props.dots && total > 1 {
        rsx! {
            div { class: "ctrl-carousel__dots",
                for i in 0..total {
                    {
                        let i = i;
                        let dot_class = if i == active_index() {
                            "ctrl-carousel__dot ctrl-carousel__dot--active"
                        } else {
                            "ctrl-carousel__dot"
                        };
                        rsx! {
                            button {
                                key: "{i}",
                                class: "{dot_class}",
                                onclick: move |_| go_to(i),
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{carousel_class}",
            style: "height: {props.height}; {props.style}",
            div { class: "ctrl-carousel__viewport",
                div {
                    class: "ctrl-carousel__track",
                    style: "transform: translateX(-{active_index() * 100}%);",
                    {props.children}
                }
            }
            if props.arrows {
                button {
                    class: "ctrl-carousel__arrow ctrl-carousel__arrow--prev",
                    onclick: prev,
                    "‹"
                }
                button {
                    class: "ctrl-carousel__arrow ctrl-carousel__arrow--next",
                    onclick: next,
                    "›"
                }
            }
            {dots_ui}
        }
    }
}