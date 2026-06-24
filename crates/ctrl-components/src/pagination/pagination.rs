use dioxus::prelude::*;

/// Pagination 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct PaginationProps {
    /// 当前页码
    #[props(default = 1)]
    pub current: u32,

    /// 总条数
    #[props(default = 0)]
    pub total: u32,

    /// 每页条数
    #[props(default = 10)]
    pub page_size: u32,

    /// 页码切换回调
    pub onchange: Option<EventHandler<u32>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

/// Pagination 分页组件
#[allow(non_snake_case)]
pub fn Pagination(props: PaginationProps) -> Element {
    const CSS: &str = include_str!("../../assets/pagination.css");
    let page_count = if props.page_size == 0 {
        1
    } else {
        ((props.total as f64) / (props.page_size as f64)).ceil() as u32
    }.max(1);

    let current = props.current.clamp(1, page_count);
    let wrapper_class = if props.class.is_empty() {
        "ctrl-pagination".to_string()
    } else {
        format!("ctrl-pagination {}", props.class)
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            // 上一页
            {
                let disabled = current <= 1;
                let mut btn_class = "ctrl-pagination__btn".to_string();
                if disabled { btn_class.push_str(" ctrl-pagination__btn--disabled"); }
                let onchange = props.onchange.clone();
                rsx! {
                    button {
                        class: "{btn_class}",
                        disabled: disabled,
                        onclick: move |_| {
                            if current > 1 {
                                if let Some(ref handler) = onchange {
                                    handler.call(current - 1);
                                }
                            }
                        },
                        "‹"
                    }
                }
            }

            // 页码
            for page in 1..=page_count {
                {
                    let is_active = page == current;
                    let mut page_class = "ctrl-pagination__btn".to_string();
                    if is_active { page_class.push_str(" ctrl-pagination__btn--active"); }
                    let onchange = props.onchange.clone();
                    rsx! {
                        button {
                            class: "{page_class}",
                            onclick: move |_| {
                                if let Some(ref handler) = onchange {
                                    handler.call(page);
                                }
                            },
                            "{page}"
                        }
                    }
                }
            }

            // 下一页
            {
                let disabled = current >= page_count;
                let mut btn_class = "ctrl-pagination__btn".to_string();
                if disabled { btn_class.push_str(" ctrl-pagination__btn--disabled"); }
                let onchange = props.onchange.clone();
                rsx! {
                    button {
                        class: "{btn_class}",
                        disabled: disabled,
                        onclick: move |_| {
                            if current < page_count {
                                if let Some(ref handler) = onchange {
                                    handler.call(current + 1);
                                }
                            }
                        },
                        "›"
                    }
                }
            }
        }
    }
}
