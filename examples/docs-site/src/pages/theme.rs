use dioxus::prelude::*;
use ctrl::prelude::*;
use crate::components::codeblock::CodeBlock;

#[component]
#[allow(non_snake_case)]
pub fn ThemePage() -> Element {
    let css_vars_code = [
        ":root {",
        "  --ctrl-primary: #059669;",
        "  --ctrl-radius-md: 8px;",
        "  --ctrl-font-family: 'Inter', system-ui, sans-serif;",
        "  --ctrl-transition: 0.2s ease;",
        "}",
    ].join("\n");

    rsx! {
        h1 { "主题定制" }
        p { "Ctrl UI 通过 CSS 变量实现主题系统。你可以通过三种方式定制主题。" }

        h2 { "方式一：ThemeProvider 传参（推荐）" }
        p { "在应用根节点通过 ThemeProvider 传入自定义 Theme 对象，所有组件自动使用新主题。这是最推荐的方式，尤其适用于需要完整主题切换的应用。" }

        h3 { "部分覆盖" }
        CodeBlock { code: [
            "use ctrl::theme::{Theme, ColorPalette};",
            "",
            "let custom_theme = Theme {",
            "    colors: ColorPalette {",
            "        primary: \"#059669\",",
            "        primary_hover: \"#047857\",",
            "        primary_active: \"#065F46\",",
            "        primary_light: \"#ECFDF5\",",
            "        ..Default::default()",
            "    },",
            "    radius_md: \"0.5rem\",",
            "    ..Default::default()",
            "};",
            "",
            "rsx! {",
            "    ThemeProvider {",
            "        theme: custom_theme,",
            "        App {}",
            "    }",
            "}",
        ].join("\n"), lang: Some("rust".to_string()) }

        h3 { "暗色主题示例" }
        CodeBlock { code: [
            "let dark_theme = Theme {",
            "    colors: ColorPalette {",
            "        primary: \"#818CF8\",",
            "        primary_hover: \"#6366F1\",",
            "        primary_active: \"#4F46E5\",",
            "        primary_light: \"#1E1B4B\",",
            "        bg: \"#1F2937\",",
            "        bg_secondary: \"#111827\",",
            "        text: \"#F9FAFB\",",
            "        text_secondary: \"#D1D5DB\",",
            "        text_disabled: \"#6B7280\",",
            "        border: \"#374151\",",
            "        border_hover: \"#4B5563\",",
            "        ..Default::default()",
            "    },",
            "    ..Default::default()",
            "};",
        ].join("\n"), lang: Some("rust".to_string()) }

        h2 { "方式二：CSS 变量覆盖" }
        p { "直接在你的 CSS 文件中（或通过 style 标签）覆盖 :root 中的 CSS 变量。这种方式不需要修改 Rust 代码，适合只需要微调几个颜色值的场景。" }
        CodeBlock { code: css_vars_code, lang: Some("css".to_string()) }

        h2 { "方式三：单个组件覆盖" }
        p { "通过组件的 style 或 class 属性单独覆盖某个组件的样式。" }
        CodeBlock { code: [
            "Button {",
            "    style: \"border-radius: 24px; font-weight: 700;\",",
            "    \"自定义按钮\"",
            "}",
        ].join("\n"), lang: Some("rust".to_string()) }

        h2 { "Theme 所有可配置字段" }
        ThemeTable {}

        h2 { "实时预览" }
        p { "点击页面右上角的主题切换按钮，可以切换暗色/亮色主题，实时查看组件在不同主题下的效果。" }

        Card {
            header: rsx! { "组件预览" },
            Space { gap: "md".to_string(), direction: Direction::Vertical,
                Space { gap: "md".to_string(),
                    Button { variant: Variant::Primary, "Primary" }
                    Button { variant: Variant::Secondary, "Secondary" }
                    Button { variant: Variant::Outline, "Outline" }
                    Button { variant: Variant::Ghost, "Ghost" }
                }
                Space { gap: "md".to_string(), direction: Direction::Vertical, style: "max-width:360px;",
                    Input { placeholder: "默认" }
                    Input { placeholder: "错误状态", error: true }
                }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn ThemeTable() -> Element {
    let rows = get_theme_rows();
    let columns = vec![
        TableColumn { title: "CSS 变量".to_string(), width: Some("30%".to_string()), align: None },
        TableColumn { title: "Theme 路径".to_string(), width: Some("30%".to_string()), align: None },
        TableColumn { title: "默认值".to_string(), width: Some("20%".to_string()), align: None },
        TableColumn { title: "说明".to_string(), width: None, align: None },
    ];
    let data: Vec<Vec<String>> = rows.iter().map(|(a, b, c, d)| {
        vec![a.to_string(), b.to_string(), c.to_string(), d.to_string()]
    }).collect();

    rsx! {
        Table { columns, data, bordered: true }
    }
}

fn get_theme_rows() -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
    vec![
        ("--ctrl-primary", "colors.primary", "#4F46E5", "品牌主色"),
        ("--ctrl-primary-hover", "colors.primary_hover", "#4338CA", "主色悬停"),
        ("--ctrl-primary-active", "colors.primary_active", "#3730A3", "主色激活"),
        ("--ctrl-primary-light", "colors.primary_light", "#EEF2FF", "主色浅背景"),
        ("--ctrl-secondary", "colors.secondary", "#6B7280", "次级色"),
        ("--ctrl-secondary-hover", "colors.secondary_hover", "#4B5563", "次级色悬停"),
        ("--ctrl-success", "colors.success", "#10B981", "成功色"),
        ("--ctrl-warning", "colors.warning", "#F59E0B", "警告色"),
        ("--ctrl-danger", "colors.danger", "#EF4444", "危险/错误色"),
        ("--ctrl-info", "colors.info", "#3B82F6", "信息色"),
        ("--ctrl-bg", "colors.bg", "#FFFFFF", "页面背景"),
        ("--ctrl-bg-secondary", "colors.bg_secondary", "#F9FAFB", "次要背景"),
        ("--ctrl-text", "colors.text", "#111827", "主文字色"),
        ("--ctrl-text-secondary", "colors.text_secondary", "#6B7280", "次要文字"),
        ("--ctrl-text-disabled", "colors.text_disabled", "#D1D5DB", "禁用文字"),
        ("--ctrl-border", "colors.border", "#E5E7EB", "边框色"),
        ("--ctrl-border-hover", "colors.border_hover", "#D1D5DB", "边框悬停"),
        ("--ctrl-font-family", "font_family", "系统字体栈", "字体族"),
        ("--ctrl-font-size-sm", "font_size_sm", "0.75rem", "小号字体"),
        ("--ctrl-font-size-md", "font_size_md", "0.875rem", "中号字体"),
        ("--ctrl-font-size-lg", "font_size_lg", "1rem", "大号字体"),
        ("--ctrl-radius-sm", "radius_sm", "0.25rem", "小圆角"),
        ("--ctrl-radius-md", "radius_md", "0.375rem", "中圆角"),
        ("--ctrl-radius-lg", "radius_lg", "0.5rem", "大圆角"),
        ("--ctrl-shadow-sm", "shadow_sm", "阴影值", "小阴影"),
        ("--ctrl-shadow-md", "shadow_md", "阴影值", "中阴影"),
        ("--ctrl-transition", "transition", "0.15s ease", "过渡动效"),
    ]
}
