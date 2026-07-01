use dioxus::prelude::*;
use ctrl::prelude::*;

use crate::components::demobox::DemoBox;
use crate::pages::components::shared::PropsTable;

#[component]
#[allow(non_snake_case)]
pub fn TypographyPage() -> Element {
    rsx! {
        h1 { "Typography 排版" }
        p { "用于标题、正文、链接等排版元素，提供一致的字体、颜色和行高。" }

        DemoBox { title: "标题 Heading".to_string(), description: Some("通过 level 属性设置标题级别 1-6。".to_string()),
            demo: rsx! {
                Space { direction: Direction::Vertical, gap: "sm".to_string(),
                    Heading { level: 1, "一级标题" }
                    Heading { level: 2, "二级标题" }
                    Heading { level: 3, "三级标题" }
                    Heading { level: 4, "四级标题" }
                    Heading { level: 5, "五级标题" }
                    Heading { level: 6, "六级标题" }
                }
            },
            code: "Heading { level: 1, \"一级标题\" }\nHeading { level: 2, \"二级标题\" }\nHeading { level: 3, \"三级标题\" }\nHeading { level: 4, \"四级标题\" }\nHeading { level: 5, \"五级标题\" }\nHeading { level: 6, \"六级标题\" }".to_string(),
        }

        DemoBox { title: "文本 Text".to_string(), description: Some("通过 r#type 设置文本颜色，支持 strong、italic、underline、delete、code、mark 等样式。".to_string()),
            demo: rsx! {
                Space { direction: Direction::Vertical, gap: "sm".to_string(),
                    Text { "默认文本" }
                    Text { r#type: "secondary".to_string(), "次要文本" }
                    Text { r#type: "success".to_string(), "成功文本" }
                    Text { r#type: "warning".to_string(), "警告文本" }
                    Text { r#type: "danger".to_string(), "危险文本" }
                }
            },
            code: "Text { \"默认文本\" }\nText { r#type: \"secondary\", \"次要文本\" }\nText { r#type: \"success\", \"成功文本\" }\nText { r#type: \"warning\", \"警告文本\" }\nText { r#type: \"danger\", \"危险文本\" }".to_string(),
        }

        DemoBox { title: "文本样式".to_string(), description: Some("支持加粗、斜体、下划线、删除线、代码、高亮标记等多种内联样式。".to_string()),
            demo: rsx! {
                Space { direction: Direction::Vertical, gap: "sm".to_string(),
                    Text { strong: true, "加粗文本" }
                    Text { italic: true, "斜体文本" }
                    Text { underline: true, "下划线文本" }
                    Text { delete: true, "删除线文本" }
                    Text { code: true, "code_block" }
                    Text { mark: true, "高亮标记" }
                    Text { strong: true, italic: true, "加粗 + 斜体" }
                }
            },
            code: "Text { strong: true, \"加粗文本\" }\nText { italic: true, \"斜体文本\" }\nText { underline: true, \"下划线文本\" }\nText { delete: true, \"删除线文本\" }\nText { code: true, \"code_block\" }\nText { mark: true, \"高亮标记\" }".to_string(),
        }

        DemoBox { title: "链接 HyperLink".to_string(), description: Some("用于页面跳转或外部链接，默认带下划线。可与 Dioxus 内置 Link 共存。".to_string()),
            demo: rsx! {
                Space { gap: "lg".to_string(),
                    HyperLink { href: "https://dioxuslabs.com".to_string(), "Dioxus 官网" }
                    HyperLink { href: "https://github.com".to_string(), underline: false, "GitHub（无下划线）" }
                    HyperLink { href: "".to_string(), disabled: true, "禁用链接" }
                }
            },
            code: "HyperLink { href: \"https://dioxuslabs.com\", \"Dioxus 官网\" }\nHyperLink { href: \"https://github.com\", underline: false, \"GitHub（无下划线）\" }\nHyperLink { href: \"\", disabled: true, \"禁用链接\" }".to_string(),
        }

        h2 { "Heading Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("level", "u8", "1", "标题级别 1-6"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}

        h2 { "Text Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("r#type", "String", "\"default\"", "文本类型：default / secondary / success / warning / danger"),
            ("strong", "bool", "false", "加粗"),
            ("italic", "bool", "false", "斜体"),
            ("underline", "bool", "false", "下划线"),
            ("delete", "bool", "false", "删除线"),
            ("code", "bool", "false", "代码样式"),
            ("mark", "bool", "false", "高亮标记"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}

        h2 { "HyperLink Props" }
        PropsTable { headers: vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()], rows: vec![
            ("href", "String", "\"\"", "链接地址"),
            ("target", "String", "\"\"", "打开方式，如 _blank"),
            ("underline", "bool", "true", "是否显示下划线"),
            ("disabled", "bool", "false", "是否禁用"),
            ("class", "String", "\"\"", "自定义 CSS 类"),
            ("style", "String", "\"\"", "自定义内联样式"),
        ]}
    }
}
