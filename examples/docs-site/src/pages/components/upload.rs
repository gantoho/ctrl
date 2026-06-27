use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn UploadPage() -> Element {
    rsx! {
div { id: "upload", style: "margin-top: 64px;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;", "Upload 上传" }
            p { style: "font-size: 1rem; color: var(--ctrl-text-secondary); margin-bottom: 32px;", "上传组件用于文件上传，支持点击上传和拖拽上传两种模式，可展示文件列表。" }

            DemoBox {
                title: "点击上传".to_string(),
                description: Some("基本点击上传按钮。".to_string()),
                demo: rsx! { ClickUploadDemo {} },
                code: "let mut files = use_signal(|| Vec::new());\n\nUpload {\n    files: files(),\n    onchange: move |f| files.set(f),\n    Button { variant: Variant::Primary, \"选择文件\" }\n}".to_string(),
            }

            DemoBox {
                title: "拖拽上传".to_string(),
                description: Some("设置 drag 为 true 启用拖拽区域。".to_string()),
                demo: rsx! {
                    div { style: "max-width: 400px;",
                        Upload {
                            drag: true,
                            tip: "支持 JPG、PNG 格式，单文件不超过 5MB".to_string(),
                            div { style: "padding: 8px;", Button { variant: Variant::Primary, "选择文件" } }
                        }
                    }
                },
                code: "Upload {\n    drag: true,\n    tip: \"支持 JPG、PNG 格式，单文件不超过 5MB\".to_string(),\n    Button { variant: Variant::Primary, \"选择文件\" }\n}".to_string(),
            }

            DemoBox {
                title: "文件列表".to_string(),
                description: Some("展示已选文件，支持移除操作。".to_string()),
                demo: rsx! { FileListUploadDemo {} },
                code: "let mut files = use_signal(|| vec![\n    UploadFile { name: \"photo.jpg\".to_string(), size: 204800 },\n    UploadFile { name: \"doc.pdf\".to_string(), size: 1024000 },\n]);\n\nUpload {\n    files: files(),\n    onremove: move |i| { let mut f = files(); f.remove(i); files.set(f); },\n    Button { variant: Variant::Primary, \"选择文件\" }\n}".to_string(),
            }

            h2 { style: "font-size: 1.25rem; font-weight: 600; color: var(--ctrl-text); margin: 40px 0 20px;", "Upload Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("files", "Vec<UploadFile>", "[]", "文件列表"),
                ("accept", "String", "\"\"", "接受的文件类型"),
                ("multiple", "bool", "false", "是否多选"),
                ("disabled", "bool", "false", "是否禁用"),
                ("drag", "bool", "false", "是否拖拽模式"),
                ("tip", "String", "\"\"", "提示文字"),
                ("class", "String", "\"\"", "自定义 CSS 类"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<Vec<UploadFile>>>", "None", "文件选择回调"),
                ("onremove", "Option<EventHandler<usize>>", "None", "文件移除回调"),
                ("children", "Element", "—", "触发元素"),
            ]}
        }
    }
}
