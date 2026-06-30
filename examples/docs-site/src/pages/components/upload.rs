use dioxus::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;
use super::_demos::*;

#[component]
#[allow(non_snake_case)]
pub fn UploadPage() -> Element {
    rsx! {
div { id: "upload", style: "margin-top: 64px;",
            h1 { "Upload 上传" }
            p { "上传组件用于文件上传，支持点击上传和拖拽上传两种模式，可展示文件列表。组件默认即开箱可用，也支持通过回调自定义行为。" }

            DemoBox {
                title: "点击上传".to_string(),
                description: Some("基本点击上传按钮，开箱可用。".to_string()),
                demo: rsx! { ClickUploadDemo {} },
                code: "let mut files = use_signal(|| Vec::new());\n\nUpload {\n    files: files(),\n    onchange: move |f| files.set(f),\n    Button { variant: Variant::Primary, \"选择文件\" }\n}".to_string(),
            }

            DemoBox {
                title: "拖拽上传".to_string(),
                description: Some("设置 drag 为 true 启用拖拽区域。".to_string()),
                demo: rsx! { DragUploadDemo {} },
                code: "let mut files = use_signal(|| Vec::new());\n\nUpload {\n    drag: true,\n    files: files(),\n    onchange: move |f| files.set(f),\n    tip: \"支持 JPG、PNG 格式，单文件不超过 5MB\".to_string(),\n    Button { variant: Variant::Primary, \"选择文件\" }\n}".to_string(),
            }

            DemoBox {
                title: "文件列表与删除".to_string(),
                description: Some("文件列表中的删除按钮开箱可用，也可通过 onremove 添加自定义逻辑。".to_string()),
                demo: rsx! { FileListUploadDemo {} },
                code: "let mut files = use_signal(|| vec![\n    UploadFile { name: \"photo.jpg\".to_string(), size: 204800 },\n    UploadFile { name: \"doc.pdf\".to_string(), size: 1024000 },\n]);\n\nUpload {\n    files: files(),\n    // 删除按钮组件内部自动处理，onremove 可选\n    onchange: move |f| { let mut cur = files(); cur.extend(f); files.set(cur); },\n    Button { variant: Variant::Primary, \"选择文件\" }\n}".to_string(),
            }

            DemoBox {
                title: "文件大小校验".to_string(),
                description: Some("设置 max_size 限制单文件大小，超限的文件自动过滤并显示错误提示。".to_string()),
                demo: rsx! { FileSizeValidationDemo {} },
                code: "let mut files = use_signal(|| Vec::new());\n\nUpload {\n    files: files(),\n    max_size: 1048576, // 1MB\n    tip: \"文件大小限制 1MB\".to_string(),\n    onchange: move |f| files.set(f),\n    Button { variant: Variant::Primary, \"选择文件\" }\n}".to_string(),
            }

            DemoBox {
                title: "限制文件类型".to_string(),
                description: Some("通过 accept 属性限制可选择的文件类型，浏览器会自动过滤。".to_string()),
                demo: rsx! { FileTypeLimitDemo {} },
                code: "Upload {\n    files: files(),\n    accept: \"image/*\".to_string(),\n    tip: \"仅支持图片格式\".to_string(),\n    onchange: move |f| files.set(f),\n    Button { \"上传图片\" }\n}".to_string(),
            }

            DemoBox {
                title: "自定义错误提示".to_string(),
                description: Some("通过 error_format 自定义校验失败时的提示文本，UI 仍使用组件默认的红色提示框。".to_string()),
                demo: rsx! { CustomErrorFormatDemo {} },
                code: "Upload {\n    files: files(),\n    max_size: 1048576,\n    error_format: \"很抱歉，文件 \\\"{name}\\\" 大小为 {size}，超出 {max_size} 限制\".to_string(),\n    onchange: move |f| files.set(f),\n    Button { \"选择文件\" }\n}".to_string(),
            }

            DemoBox {
                title: "自定义错误处理".to_string(),
                description: Some("通过 onerror 捕获错误信息，可选择用 Message 等方式提醒用户。设置 show_error: false 可隐藏组件内置的红色提示框。".to_string()),
                demo: rsx! { CustomErrorHandlerDemo {} },
                code: "Upload {\n    files: files(),\n    max_size: 1048576,\n    show_error: false,\n    onchange: move |f| files.set(f),\n    onerror: move |msg| {\n        let message = use_message();\n        message.info(format!(\"校验失败: {msg}\"));\n    },\n    Button { \"选择文件\" }\n}".to_string(),
            }

            h2 { "Upload Props" }
            PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("files", "Vec<UploadFile>", "[]", "文件列表"),
                ("accept", "String", "\"\"", "接受的文件类型，如 \"image/*\"、\".pdf,.doc\""),
                ("multiple", "bool", "false", "是否允许多选"),
                ("disabled", "bool", "false", "是否禁用"),
                ("drag", "bool", "false", "是否启用拖拽上传区域"),
                ("max_size", "u64", "0", "单文件大小限制（字节），0 表示不限制"),
                ("error_format", "String", "\"\"", "自定义错误信息格式，支持占位符 {name} {ext} {size} {max_size}"),
                ("tip", "String", "\"\"", "引导提示文字，显示在组件底部"),
                ("class", "String", "\"\"", "自定义 CSS 类名"),
                ("style", "String", "\"\"", "自定义内联样式"),
                ("onchange", "Option<EventHandler<Vec<UploadFile>>>", "None", "文件选择回调，用于同步外部文件状态"),
                ("onremove", "Option<EventHandler<usize>>", "None", "文件移除回调，组件内部已自动移除，此回调用于添加额外逻辑"),
                ("onerror", "Option<EventHandler<String>>", "None", "校验失败回调，用于捕获错误信息（如弹出全局通知），组件内置错误提示不受影响"),
                ("show_error", "bool", "true", "是否显示内置错误提示，设为 false 可由 onerror 自行处理错误展示"),
                ("children", "Element", "—", "触发上传的元素，如 Button"),
            ]}

            h2 { "UploadFile 结构" }
            PropsTable { headers: vec!["字段".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
                ("name", "String", "—", "文件名"),
                ("size", "u64", "0", "文件大小（字节）"),
                ("status", "UploadFileStatus", "Ready", "文件状态：Ready / Uploading / Success / Error"),
            ]}
        }
    }
}
