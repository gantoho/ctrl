use dioxus::prelude::*;
use std::collections::BTreeMap;

/// 环绕轨道上的单个元素
#[derive(Clone, PartialEq)]
pub struct OrbitItem {
    /// 显示内容（emoji、字母或短文本）
    pub content: String,
    /// 轨道半径（px）
    pub radius: f64,
    /// 是否反向绕行
    pub reverse: bool,
}

impl OrbitItem {
    /// 创建一个正向环绕元素
    pub fn new(content: impl Into<String>, radius: f64) -> Self {
        Self { content: content.into(), radius, reverse: false }
    }

    /// 设为反向环绕
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
}

/// OrbitingCircles 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct OrbitingCirclesProps {
    /// 环绕元素列表
    pub items: Vec<OrbitItem>,

    /// 容器边长（px）
    #[props(default = 320.0)]
    pub size: f64,

    /// 绕行一圈时长（秒）
    #[props(default = 20.0)]
    pub duration: f64,

    /// 是否绘制轨道路径圆环
    #[props(default = true)]
    pub show_path: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 中心内容
    pub children: Element,
}

/// OrbitingCircles 环绕轨道组件
///
/// 若干元素沿以中心为圆心的圆形轨道匀速绕行，同一半径上的元素自动均匀分布。
/// 常用于展示技术栈、生态集成、连接关系等。
#[allow(non_snake_case)]
pub fn OrbitingCircles(props: OrbitingCirclesProps) -> Element {
    const CSS: &str = include_str!("../../assets/orbiting_circles.css");

    let container_class = if props.class.is_empty() {
        "ctrl-orbiting-circles".to_string()
    } else {
        format!("ctrl-orbiting-circles {}", props.class)
    };

    let vars = format!("width:{0}px; height:{0}px; {1}", props.size, props.style);

    // 按半径分组，用于同轨道均匀分布相位；半径按整数 key 归组
    let mut groups: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (i, item) in props.items.iter().enumerate() {
        groups.entry(item.radius.round() as i64).or_default().push(i);
    }
    // 每个 item 的相位比例（0~1）
    let mut phase = vec![0.0f64; props.items.len()];
    for (_, idxs) in groups.iter() {
        let n = idxs.len().max(1);
        for (pos, &i) in idxs.iter().enumerate() {
            phase[i] = pos as f64 / n as f64;
        }
    }

    // 需要绘制的唯一半径（轨道环）
    let radii: Vec<f64> = if props.show_path {
        let mut rs: Vec<f64> = groups.keys().map(|k| *k as f64).collect();
        rs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        rs
    } else {
        Vec::new()
    };

    let duration = props.duration;

    rsx! {
        style { {CSS} }
        div { class: "{container_class}", style: "{vars}",
            // 轨道路径
            for (ri, r) in radii.iter().enumerate() {
                {
                    let d = r * 2.0;
                    let path_style = format!("width:{d}px; height:{d}px;");
                    rsx! {
                        div { key: "path-{ri}", class: "ctrl-orbiting-circles__path", style: "{path_style}" }
                    }
                }
            }

            // 中心
            div { class: "ctrl-orbiting-circles__center", {props.children} }

            // 环绕元素
            for (i, item) in props.items.iter().enumerate() {
                {
                    // 负延迟让元素分布在轨道不同相位上
                    let delay = -duration * phase[i];
                    let dir = if item.reverse { "reverse" } else { "normal" };
                    let orbit_style = format!(
                        "--ctrl-orbit-dur:{duration}s; --ctrl-orbit-delay:{delay:.3}s; --ctrl-orbit-dir:{dir}; --ctrl-orbit-radius:{}px;",
                        item.radius
                    );
                    rsx! {
                        div { key: "item-{i}", class: "ctrl-orbiting-circles__orbit", style: "{orbit_style}",
                            div { class: "ctrl-orbiting-circles__arm",
                                div { class: "ctrl-orbiting-circles__item", "{item.content}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
