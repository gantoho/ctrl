use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use dioxus::web::WebEventExt;

/// 上传文件状态
#[derive(PartialEq, Clone, Debug)]
pub enum UploadFileStatus {
    Ready,
    Uploading,
    Success,
    Error,
}

/// 上传文件信息
#[derive(PartialEq, Clone, Props)]
pub struct UploadFile {
    /// 文件名
    pub name: String,
    /// 文件大小（字节）
    #[props(default = 0)]
    pub size: u64,
    /// 文件状态
    #[props(default = UploadFileStatus::Ready)]
    pub status: UploadFileStatus,
}

/// Upload 上传组件属性
#[derive(Props, PartialEq, Clone)]
pub struct UploadProps {
    /// 文件列表
    #[props(default = Vec::new())]
    pub files: Vec<UploadFile>,

    /// 接受的文件类型（如 "image/*"）
    #[props(default = "".to_string())]
    pub accept: String,

    /// 是否允许多选
    #[props(default = false)]
    pub multiple: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否拖拽上传
    #[props(default = false)]
    pub drag: bool,

    /// 提示文字
    #[props(default = "".to_string())]
    pub tip: String,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 文件选择回调
    pub onchange: Option<EventHandler<Vec<UploadFile>>>,

    /// 文件移除回调
    pub onremove: Option<EventHandler<usize>>,

    /// 触发元素（如按钮）
    pub children: Element,
}

/// 格式化文件大小
fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// Upload 上传组件
#[allow(non_snake_case)]
pub fn Upload(props: UploadProps) -> Element {
    const CSS: &str = include_str!("../../assets/upload.css");
    let drag_active = use_signal(|| false);

    let upload_class = {
        let mut c = String::from("ctrl-upload");
        if props.disabled {
            c.push_str(" ctrl-upload--disabled");
        }
        if props.drag {
            c.push_str(" ctrl-upload--drag");
        }
        if drag_active() {
            c.push_str(" ctrl-upload--drag-active");
        }
        if !props.class.is_empty() {
            c.push_str(" ");
            c.push_str(&props.class);
        }
        c
    };

    let handle_drag_over = {
        let mut drag_active = drag_active.clone();
        move |evt: DragEvent| {
            evt.prevent_default();
            if !props.disabled {
                drag_active.set(true);
            }
        }
    };

    let handle_drag_leave = {
        let mut drag_active = drag_active.clone();
        move |_evt: DragEvent| {
            drag_active.set(false);
        }
    };

    let handle_drop = {
        let mut drag_active = drag_active.clone();
        let onchange = props.onchange.clone();
        move |evt: DragEvent| {
            evt.prevent_default();
            drag_active.set(false);
            if props.disabled {
                return;
            }
            #[cfg(target_arch = "wasm32")]
            {
                let dt = evt.data_transfer();
                let files = dt.files();
                let mut new_files = Vec::new();
                for i in 0..files.len() {
                    let file = files.get(i);
                    if let Some(file) = file {
                        new_files.push(UploadFile {
                            name: file.name(),
                            size: file.size() as u64,
                            status: UploadFileStatus::Ready,
                        });
                    }
                }
                if !new_files.is_empty() {
                    if let Some(ref cb) = onchange {
                        cb.call(new_files);
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (evt, onchange);
            }
        }
    };

    let handle_input_change = {
        let onchange = props.onchange.clone();
        move |evt: FormEvent| {
            if props.disabled {
                return;
            }
            #[cfg(target_arch = "wasm32")]
            {
                use web_sys::HtmlInputElement;
                use wasm_bindgen::JsCast;
                if let Some(input) = evt.try_as_web_event().and_then(|e| e.target()).and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                    if let Some(file_list) = input.files() {
                        let mut new_files = Vec::new();
                        for i in 0..file_list.length() {
                            if let Some(file) = file_list.get(i) {
                                new_files.push(UploadFile {
                                    name: file.name(),
                                    size: file.size() as u64,
                                    status: UploadFileStatus::Ready,
                                });
                            }
                        }
                        if !new_files.is_empty() {
                            if let Some(ref cb) = onchange {
                                cb.call(new_files);
                            }
                        }
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (evt, onchange);
            }
        }
    };

    let file_list = if !props.files.is_empty() {
        rsx! {
            div { class: "ctrl-upload__list",
                for (i, file) in props.files.iter().enumerate() {
                    {
                        let onremove = props.onremove.clone();
                        let status_class = match file.status {
                            UploadFileStatus::Success => "ctrl-upload__file-status--success",
                            UploadFileStatus::Error => "ctrl-upload__file-status--error",
                            UploadFileStatus::Uploading => "ctrl-upload__file-status--uploading",
                            _ => "",
                        };
                        let status_text = match file.status {
                            UploadFileStatus::Success => "✓",
                            UploadFileStatus::Error => "✗",
                            UploadFileStatus::Uploading => "⏳",
                            _ => "",
                        };
                        rsx! {
                            div { key: "{i}", class: "ctrl-upload__file",
                                span { class: "ctrl-upload__file-name", "{file.name}" }
                                span { class: "ctrl-upload__file-size", "{format_size(file.size)}" }
                                if !status_text.is_empty() {
                                    span { class: "ctrl-upload__file-status {status_class}", "{status_text}" }
                                }
                                button {
                                    class: "ctrl-upload__file-remove",
                                    title: "移除",
                                    onclick: move |_| {
                                        if let Some(ref cb) = onremove {
                                            cb.call(i);
                                        }
                                    },
                                    "✕"
                                }
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
            class: "{upload_class}",
            style: "{props.style}",
            ondrop: handle_drop,
            ondragover: handle_drag_over,
            ondragleave: handle_drag_leave,
            if props.drag && props.files.is_empty() {
                div { class: "ctrl-upload__drag-icon", "📁" }
                div { class: "ctrl-upload__drag-text", "点击或拖拽文件到此区域上传" }
                if !props.tip.is_empty() {
                    div { class: "ctrl-upload__drag-hint", "{props.tip}" }
                }
            }
            label { class: "ctrl-upload__trigger",
                input {
                    class: "ctrl-upload__input",
                    r#type: "file",
                    accept: if !props.accept.is_empty() { props.accept.clone() } else { String::new() },
                    multiple: props.multiple,
                    disabled: props.disabled,
                    onchange: handle_input_change,
                }
                {props.children}
            }
            {file_list}
            if !props.drag && !props.tip.is_empty() {
                div { class: "ctrl-upload__tip", "{props.tip}" }
            }
        }
    }
}