use dioxus::prelude::*;
use ctrl_core::types::Effect;
use std::cell::Cell;
use std::rc::Rc;

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

    /// 幻灯片总数
    #[props(default = 1)]
    pub total: usize,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素（轮播项）
    pub children: Element,
}

/// Carousel 走马灯组件
#[allow(non_snake_case)]
pub fn Carousel(props: CarouselProps) -> Element {
    const CSS: &str = include_str!("../../assets/carousel.css");
    let active_index = use_signal(|| 0usize);
    let total = props.total.max(1);
    let loop_play = props.loop_play;

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
        let mut index = active_index.clone();
        let interval = props.interval;
        let total = total;
        let loop_play = loop_play;
        let cancelled = Rc::new(Cell::new(false));
        let cancelled_clone = cancelled.clone();

        use_effect(move || {
            let cancelled = cancelled_clone.clone();
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
                let _ = (index, interval, total, loop_play, cancelled);
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