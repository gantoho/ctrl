use dioxus::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ctrl_core::types::Layout;

/// 验证规则
#[derive(Clone)]
pub enum ValidationRule {
    /// 必填
    Required(String),
    /// 最小长度
    MinLength(usize, String),
    /// 最大长度
    MaxLength(usize, String),
    /// 邮箱格式
    Email(String),
    /// 正则匹配
    Pattern(String, String),
    /// 自定义验证
    Custom(Rc<dyn Fn(&str) -> Result<(), String>>),
}

impl ValidationRule {
    pub fn required(msg: impl Into<String>) -> Self {
        ValidationRule::Required(msg.into())
    }
    pub fn min_length(len: usize, msg: impl Into<String>) -> Self {
        ValidationRule::MinLength(len, msg.into())
    }
    pub fn max_length(len: usize, msg: impl Into<String>) -> Self {
        ValidationRule::MaxLength(len, msg.into())
    }
    pub fn email(msg: impl Into<String>) -> Self {
        ValidationRule::Email(msg.into())
    }
    pub fn pattern(pattern: impl Into<String>, msg: impl Into<String>) -> Self {
        ValidationRule::Pattern(pattern.into(), msg.into())
    }
    pub fn custom<F: Fn(&str) -> Result<(), String> + 'static>(f: F) -> Self {
        ValidationRule::Custom(Rc::new(f))
    }

    /// 执行验证，返回 None 表示通过，Some(String) 表示错误信息
    pub fn validate(&self, value: &str) -> Option<String> {
        match self {
            ValidationRule::Required(msg) => {
                if value.trim().is_empty() { Some(msg.clone()) } else { None }
            }
            ValidationRule::MinLength(len, msg) => {
                if value.chars().count() < *len { Some(msg.clone()) } else { None }
            }
            ValidationRule::MaxLength(len, msg) => {
                if value.chars().count() > *len { Some(msg.clone()) } else { None }
            }
            ValidationRule::Email(msg) => {
                if !value.is_empty() && (!value.contains('@') || !value.contains('.')) {
                    Some(msg.clone())
                } else {
                    None
                }
            }
            ValidationRule::Pattern(pattern, msg) => {
                if !value.is_empty() && !value.contains(pattern.as_str()) {
                    Some(msg.clone())
                } else {
                    None
                }
            }
            ValidationRule::Custom(f) => f(value).err(),
        }
    }
}

impl PartialEq for ValidationRule {
    fn eq(&self, _other: &Self) -> bool {
        false  // Validation closures can't be compared meaningfully
    }
}

/// 验证触发时机
#[derive(Clone, Copy, PartialEq)]
pub enum ValidationTrigger {
    /// 提交时
    Submit,
    /// 失焦时
    Blur,
    /// 值变化时
    Change,
}

impl Default for ValidationTrigger {
    fn default() -> Self { ValidationTrigger::Submit }
}

/// 表单上下文，用于跨组件通信
pub struct FormContext {
    /// 字段及其验证规则: (value_signal, rules)
    pub fields: Signal<Rc<RefCell<HashMap<String, FormFieldState>>>>,
    /// 全局验证触发时机
    pub validate_trigger: ValidationTrigger,
}

/// 单个字段的验证状态
pub struct FormFieldState {
    pub value: String,
    pub rules: Vec<ValidationRule>,
    pub error: String,
}

impl FormContext {
    /// 验证所有字段，返回 (是否通过, 字段错误列表)
    pub fn validate_all(&self) -> (bool, Vec<(String, String)>) {
        let guard = self.fields.read();
        let fields = guard.borrow();
        let mut all_valid = true;
        let mut errors = Vec::new();
        for (name, state) in fields.iter() {
            let mut field_error = String::new();
            for rule in &state.rules {
                if let Some(err) = rule.validate(&state.value) {
                    field_error = err;
                    all_valid = false;
                    break;
                }
            }
            if !field_error.is_empty() {
                errors.push((name.clone(), field_error));
            }
        }
        (all_valid, errors)
    }
}

impl Clone for FormContext {
    fn clone(&self) -> Self {
        FormContext {
            fields: self.fields.clone(),
            validate_trigger: self.validate_trigger,
        }
    }
}

impl PartialEq for FormContext {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            &*self.fields.read() as *const _,
            &*other.fields.read() as *const _,
        )
    }
}

/// Form 表单组件属性
#[derive(Props, PartialEq, Clone)]
pub struct FormProps {
    /// 布局方式
    #[props(default = Layout::Vertical)]
    pub layout: Layout,

    /// 全局验证触发时机
    #[props(default = ValidationTrigger::Submit)]
    pub validate_trigger: ValidationTrigger,

    /// 提交时是否自动校验
    #[props(default = true)]
    pub validate_on_submit: bool,

    /// 校验失败时是否滚动到第一个错误字段
    #[props(default = false)]
    pub scroll_to_error: bool,

    /// 是否禁用整个表单
    #[props(default = false)]
    pub disabled: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 提交回调
    pub onsubmit: Option<EventHandler<Rc<FormData>>>,

    /// 校验完成回调
    pub onvalidate: Option<EventHandler<FormValidationResult>>,

    /// 子元素
    pub children: Element,
}

/// 表单验证结果
#[derive(Clone, PartialEq)]
pub struct FormValidationResult {
    pub valid: bool,
    pub errors: Vec<(String, String)>,
}

/// FormItem 表单项属性
#[derive(Props, PartialEq, Clone)]
pub struct FormItemProps {
    /// 字段唯一标识，用于注册到 FormContext
    #[props(default = "".to_string())]
    pub name: String,

    /// 标签文本
    #[props(default = "".to_string())]
    pub label: String,

    /// 当前字段值（用于校验）
    #[props(default = "".to_string())]
    pub value: String,

    /// 是否必填
    #[props(default = false)]
    pub required: bool,

    /// 验证规则列表
    #[props(default = vec![])]
    pub rules: Vec<ValidationRule>,

    /// 字段级验证触发时机，覆盖全局设置
    pub validate_trigger: Option<ValidationTrigger>,

    /// 帮助文本
    #[props(default = "".to_string())]
    pub help: String,

    /// 手动错误信息（优先级高于自动校验）
    #[props(default = "".to_string())]
    pub error: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 子元素（表单控件）
    pub children: Element,
}

/// Form 表单组件
#[allow(non_snake_case)]
pub fn Form(props: FormProps) -> Element {
    const CSS: &str = include_str!("../../assets/form.css");

    let form_class = {
        let mut c = String::from("ctrl-form");
        if props.layout == Layout::Horizontal {
            c.push_str(" ctrl-form--horizontal");
        } else if props.layout == Layout::Inline {
            c.push_str(" ctrl-form--inline");
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    // 表单上下文，收集所有 FormItem 的字段验证信息
    let form_ctx = FormContext {
        fields: use_signal(|| Rc::new(RefCell::new(HashMap::new()))),
        validate_trigger: props.validate_trigger,
    };
    use_context_provider(|| form_ctx.clone());

    let handle_submit = {
        let form_ctx = form_ctx.clone();
        let onsubmit = props.onsubmit.clone();
        let onvalidate = props.onvalidate.clone();
        let validate_on_submit = props.validate_on_submit;
        move |evt: FormEvent| {
            evt.prevent_default();
            if validate_on_submit {
                let (valid, errors) = form_ctx.validate_all();
                if let Some(ref cb) = onvalidate {
                    cb.call(FormValidationResult {
                        valid,
                        errors: errors.clone(),
                    });
                }
                if !valid { return; }
            }
            if let Some(ref cb) = onsubmit {
                cb.call(evt.data());
            }
        }
    };

    rsx! {
        style { {CSS} }
        form {
            class: "{form_class}",
            style: "{props.style}",
            onsubmit: handle_submit,
            {props.children}
        }
    }
}

/// FormItem 表单项组件
#[allow(non_snake_case)]
pub fn FormItem(props: FormItemProps) -> Element {
    let label_class = {
        if props.required {
            "ctrl-form__label ctrl-form__label--required"
        } else {
            "ctrl-form__label"
        }
    };

    let item_class = {
        let mut c = String::from("ctrl-form__item");
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    // 注册到 FormContext
    let validation_error = use_signal(|| String::new());
    if !props.name.is_empty() {
        if let Some(ctx) = try_use_context::<FormContext>() {
            let name = props.name.clone();
            let rules = props.rules.clone();
            let required = props.required;
            let value = props.value.clone();

            // 注册字段到上下文
            {
                let guard = ctx.fields.read();
                let mut fields = guard.borrow_mut();
                let mut all_rules = rules.clone();
                if required {
                    all_rules.push(ValidationRule::required(format!("{} 为必填项", props.label)));
                }
                fields.insert(name.clone(), FormFieldState {
                    value: value.clone(),
                    rules: all_rules,
                    error: String::new(),
                });
            }

            // 当 value 变化时更新上下文字段值
            use_effect(use_reactive(&props.value, move |v| {
                if let Some(ctx) = try_use_context::<FormContext>() {
                    let guard = ctx.fields.read();
                    let mut fields = guard.borrow_mut();
                    if let Some(state) = fields.get_mut(&name) {
                        state.value = v.clone();
                    }
                }
            }));
        }
    }

    // 显示的错误信息：手动 error 优先级最高
    let display_error = if !props.error.is_empty() {
        props.error.clone()
    } else {
        validation_error()
    };

    rsx! {
        div { class: "{item_class}",
            if !props.label.is_empty() {
                label { class: "{label_class}", "{props.label}" }
            }
            div { class: "ctrl-form__control",
                {props.children}
            }
            if !display_error.is_empty() {
                div { class: "ctrl-form__error", "{display_error}" }
            } else if !props.help.is_empty() {
                div { class: "ctrl-form__help", "{props.help}" }
            }
        }
    }
}