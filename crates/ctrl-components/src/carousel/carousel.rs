use dioxus::prelude::*;
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

    /// 是否显示箭头
    #[props(default = true)]
    pub arrows: bool,

    /// 是否显示指示器
    #[props(default = true)]
    pub dots: bool,

    /// 过渡效果："slide" | "fade"
    #[props(default = "slide".to_string())]
    pub effect: String,

    /// 容器高度（如 "300px"）
    #[props(default = "300px".to_string())]
    pub height: String,

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

    let carousel_class = {
        let mut c = String::from("ctrl-carousel");
        if props.effect == "fade" {
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
        let index = active_index.clone();
        let interval = props.interval;
        let cancelled = Rc::new(Cell::new(false));
        let cancelled_clone = cancelled.clone();

        let _ = use_resource(move || {
            let mut index = index.clone();
            let cancelled = cancelled_clone.clone();
            let interval = interval;
            async move {
                #[cfg(target_arch = "wasm32")]
                {
                    loop {
                        if cancelled.get() {
                            break;
                        }
                        gloo_timers::future::TimeoutFuture::new(interval as u32).await;
                        if cancelled.get() {
                            break;
                        }
                        index.set(index() + 1);
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = (index, interval, cancelled);
                }
            }
        });

        // 组件卸载时取消循环
        use_drop(move || {
            cancelled.set(true);
        });
    }

    let prev = {
        let mut index = active_index.clone();
        move |_| {
            let new = if index() == 0 { 0 } else { index() - 1 };
            index.set(new);
        }
    };

    let next = {
        let mut index = active_index.clone();
        move |_| {
            index.set(index() + 1);
        }
    };

    let mut go_to = {
        let mut index = active_index.clone();
        move |i: usize| {
            index.set(i);
        }
    };

    // 渲染指示器
    let dots_ui = if props.dots {
        rsx! {
            div { class: "ctrl-carousel__dots",
                for (i, _) in (0..3).enumerate() {
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