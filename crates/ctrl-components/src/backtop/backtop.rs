use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::window;
use ctrl_core::types::{Placement, ScrollBehavior, Easing};

/// Backtop 回到顶部 / 回到底部组件属性
#[derive(Props, PartialEq, Clone)]
pub struct BacktopProps {
    /// 滚动容器选择器（默认监听 window）
    #[props(default = "".to_string())]
    pub target: String,

    /// 显示阈值（滚动距离超过该值才显示，px），bottom 模式时为距离底部的距离
    #[props(default = 200)]
    pub visibility_height: u32,

    /// 滚动行为
    #[props(default = ScrollBehavior::Smooth)]
    pub behavior: ScrollBehavior,

    /// 滚动动画时长（ms），behavior=auto 时无效
    #[props(default = 400)]
    pub duration: u32,

    /// 缓动函数
    #[props(default = Easing::EaseOutCubic)]
    pub easing: Easing,

    /// 是否启用弹簧阻尼效果（启用后忽略 duration 和 easing，使用物理弹簧模型）
    #[props(default = false)]
    pub damping: bool,

    /// 目标位置
    #[props(default = Placement::Top)]
    pub target_position: Placement,

    /// 点击事件
    pub onclick: Option<EventHandler<()>>,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,
}

fn get_max_scroll() -> &'static str {
    "Math.max(document.body.scrollHeight,document.documentElement.scrollHeight)-window.innerHeight"
}

/// 构建自定义滚动 JS 代码
fn build_scroll_js(props: &BacktopProps) -> String {
    let duration = props.duration;
    let behavior = props.behavior;
    let is_bottom = props.target_position == Placement::Bottom;

    // 弹簧阻尼模式
    if props.damping {
        let target = if is_bottom {
            format!("var target={};", get_max_scroll())
        } else {
            "var target=0;".to_string()
        };
        return format!(
            r#"(function(){{
                var sy=window.scrollY||document.documentElement.scrollTop||document.body.scrollTop||0;
                {target}
                if(Math.abs(sy-target)<5)return;
                var pos=sy,vel=0,stiff=0.008,damp=0.82;
                function step(){{
                    var diff=target-pos;vel+=diff*stiff;vel*=damp;pos+=vel;
                    window.scrollTo(0,Math.max(0,Math.min(pos,{max})));
                    if(Math.abs(pos-target)>0.5||Math.abs(vel)>0.1){{requestAnimationFrame(step)}}
                    else{{window.scrollTo(0,target)}}
                }}
                requestAnimationFrame(step)
            }})()"#,
            max = get_max_scroll()
        );
    }

    // 标准 behavior="auto"，不做动画
    if behavior == ScrollBehavior::Auto {
        if is_bottom {
            return format!(
                r#"window.scrollTo({{top:{},behavior:'auto'}})"#,
                get_max_scroll()
            );
        }
        return r#"window.scrollTo({top:0,behavior:'auto'})"#.to_string();
    }

    // 自定义缓动动画
    let easing_fn = match props.easing {
        Easing::EaseOutQuad => "function(t){return t*(2-t)}",
        Easing::EaseOutCubic => "function(t){return 1-Math.pow(1-t,3)}",
        Easing::EaseOutQuart => "function(t){return 1-Math.pow(1-t,4)}",
        Easing::EaseOutQuint => "function(t){return 1-Math.pow(1-t,5)}",
        Easing::EaseOutExpo => "function(t){return t===1?1:1-Math.pow(2,-10*t)}",
        Easing::EaseOutBack => "function(t){var c=1.70158;return 1+c*Math.pow(t-1,3)+c*Math.pow(t-1,2)}",
        Easing::EaseOutElastic => "function(t){if(t===0||t===1)return t;return Math.pow(2,-10*t)*Math.sin((t*10-0.75)*2*Math.PI/3)+1}",
        _ => "function(t){return 1-Math.pow(1-t,3)}",
    };

    if is_bottom {
        format!(
            r#"(function(){{
                var sy=window.scrollY||document.documentElement.scrollTop||document.body.scrollTop||0;
                var max={max};
                if(Math.abs(sy-max)<5)return;
                var st=null,d={duration},ease={easing_fn};
                function anim(ct){{
                    if(st===null)st=ct;
                    var p=Math.min((ct-st)/d,1);
                    window.scrollTo(0,sy+(max-sy)*ease(p));
                    if(p<1)requestAnimationFrame(anim)
                }}
                requestAnimationFrame(anim)
            }})()"#,
            max = get_max_scroll()
        )
    } else {
        format!(
            r#"(function(){{
                var sy=window.scrollY||document.documentElement.scrollTop||document.body.scrollTop||0;
                if(sy<=0)return;
                var st=null,d={duration},ease={easing_fn};
                function anim(ct){{
                    if(st===null)st=ct;
                    var p=Math.min((ct-st)/d,1);
                    window.scrollTo(0,sy*(1-ease(p)));
                    if(p<1)requestAnimationFrame(anim)
                }}
                requestAnimationFrame(anim)
            }})()"#
        )
    }
}

/// Backtop 回到顶部 / 回到底部组件
#[allow(non_snake_case)]
pub fn Backtop(props: BacktopProps) -> Element {
    const CSS: &str = include_str!("../../assets/backtop.css");
    let visible = use_signal(|| false);
    let is_bottom = props.target_position == Placement::Bottom;

    // 存储 scroll 监听器闭包，组件卸载时通过 use_drop 清理
    let scroll_listener: Signal<Rc<RefCell<Option<Closure<dyn FnMut()>>>>> =
        use_signal(|| Rc::new(RefCell::new(None)));

    let threshold = props.visibility_height;
    let mut v = visible.clone();
    let sl = scroll_listener.clone();

    use_effect(move || {
        let win = match window() {
            Some(w) => w,
            None => return,
        };

        if is_bottom {
            let Some(doc) = win.document() else { return; };
            let scroll_y = win.scroll_y().unwrap_or(0.0);
            let inner_h = win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
            let doc_h = doc.document_element().map(|d| d.scroll_height() as f64).unwrap_or(0.0);
            let dist_from_bottom = doc_h - scroll_y - inner_h;
            if let Ok(mut w) = v.try_write() { *w = dist_from_bottom > threshold as f64; }

            let win2 = win.clone();
            let closure = Closure::<dyn FnMut()>::new(move || {
                let sy = win2.scroll_y().unwrap_or(0.0);
                let ih = win2.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
                let dh = win2.document().and_then(|d| d.document_element()).map(|d| d.scroll_height() as f64).unwrap_or(0.0);
                if let Ok(mut w) = v.try_write() { *w = dh - sy - ih > threshold as f64; }
            });
            if let Err(e) = win.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref()) {
                if let Ok(mut w) = v.try_write() { *w = true; }
                _ = e;
            }
            *sl().borrow_mut() = Some(closure);
        } else {
            let scroll_y = win.scroll_y().unwrap_or(0.0);
            if let Ok(mut w) = v.try_write() { *w = scroll_y > threshold as f64; }

            let win2 = win.clone();
            let closure = Closure::<dyn FnMut()>::new(move || {
                let sy = win2.scroll_y().unwrap_or(0.0);
                if let Ok(mut w) = v.try_write() { *w = sy > threshold as f64; }
            });
            if let Err(e) = win.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref()) {
                if let Ok(mut w) = v.try_write() { *w = true; }
                _ = e;
            }
            *sl().borrow_mut() = Some(closure);
        }
    });

    // 组件卸载时移除 scroll 监听器
    use_drop(move || {
        if let Some(closure) = scroll_listener().borrow_mut().take() {
            if let Some(win) = window() {
                let _ = win.remove_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            }
        }
    });

    let onclick = props.onclick.clone();
    let js_code = build_scroll_js(&props);
    let default_click = move |_| {
        use dioxus::document;
        document::eval(&js_code);
        if let Some(ref handler) = onclick {
            handler.call(());
        }
    };

    let icon = if is_bottom { "↓" } else { "↑" };
    let hidden_class = if !visible() { " ctrl-backtop--hidden" } else { "" };
    let bottom_class = if is_bottom { " ctrl-backtop--bottom" } else { "" };
    let wrapper_class = if props.class.is_empty() {
        format!("ctrl-backtop{}{}", bottom_class, hidden_class)
    } else {
        format!("ctrl-backtop {} {}{}", props.class, bottom_class, hidden_class)
    };

    rsx! {
        style { {CSS} }
        div {
            class: "{wrapper_class}",
            onclick: default_click,
            "{icon}"
        }
    }
}