use dioxus::prelude::*;
use ctrl::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[allow(non_snake_case)]
fn App() -> Element {
    let mut input_value = use_signal(|| String::new());

    rsx! {
        ThemeProvider {
            div {
                style: "max-width: 640px; margin: 0 auto; padding: 40px 20px; font-family: var(--ctrl-font-family);",

                // ── 标题 ──
                h1 { style: "font-size: 1.5rem; font-weight: 700; color: var(--ctrl-text); margin-bottom: 8px;",
                    "Ctrl UI 组件演示"
                }
                p { style: "color: var(--ctrl-text-secondary); margin-bottom: 32px; font-size: var(--ctrl-font-size-md);",
                    "开箱即用的 Dioxus UI 组件库"
                }

                // ── 按钮 ──
                Section { title: "Button 按钮",
                    SubSection { title: "变体 Variant" }
                    div { style: "display: flex; gap: 12px; flex-wrap: wrap; align-items: center;",
                        Button { variant: Variant::Primary, "Primary" }
                        Button { variant: Variant::Secondary, "Secondary" }
                        Button { variant: Variant::Outline, "Outline" }
                        Button { variant: Variant::Ghost, "Ghost" }
                    }

                    SubSection { title: "尺寸 Size" }
                    div { style: "display: flex; gap: 12px; flex-wrap: wrap; align-items: center;",
                        Button { variant: Variant::Primary, size: Size::Sm, "Small" }
                        Button { variant: Variant::Primary, size: Size::Md, "Medium" }
                        Button { variant: Variant::Primary, size: Size::Lg, "Large" }
                    }

                    SubSection { title: "状态" }
                    div { style: "display: flex; gap: 12px; flex-wrap: wrap; align-items: center;",
                        Button { variant: Variant::Primary, disabled: true, "禁用" }
                        Button { variant: Variant::Primary, block: true, "块级按钮" }
                    }
                }

                // ── 输入框 ──
                Section { title: "Input 输入框",
                    SubSection { title: "基础用法" }
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 360px;",
                        Input {
                            placeholder: "请输入内容",
                            value: input_value(),
                            oninput: move |evt: FormEvent| input_value.set(evt.value()),
                        }
                        p { style: "color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm);",
                            "当前输入: {input_value()}"
                        }
                    }

                    SubSection { title: "尺寸" }
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 360px;",
                        Input { placeholder: "Small", size: Size::Sm }
                        Input { placeholder: "Medium", size: Size::Md }
                        Input { placeholder: "Large", size: Size::Lg }
                    }

                    SubSection { title: "状态" }
                    div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 360px;",
                        Input { placeholder: "错误状态", error: true }
                        Input { placeholder: "禁用状态", disabled: true }
                    }
                }

                // ── 主题定制说明 ──
                Section { title: "主题定制",
                    div { style: "background: var(--ctrl-bg-secondary); border-radius: var(--ctrl-radius-md); padding: 16px;",
                        p { style: "color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm); margin-bottom: 12px;",
                            "Ctrl UI 通过 CSS 变量实现主题定制。你可以在 ThemeProvider 中传入自定义主题，"
                        }
                        p { style: "color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-sm); margin-bottom: 12px;",
                            "也可以直接在你的 CSS 中覆盖 CSS 变量来修改样式："
                        }
                        code { style: "display: block; background: var(--ctrl-bg); padding: 12px; border-radius: var(--ctrl-radius-sm); font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text);",
                            ":root {{\n    --ctrl-primary: #FF6B35;\n    --ctrl-radius-md: 8px;\n}}"
                        }
                    }
                }
            }
        }
    }
}

/// 区块标题
#[component]
fn Section(title: String, children: Element) -> Element {
    rsx! {
        div { style: "margin-bottom: 40px;",
            h2 { style: "font-size: 1.125rem; font-weight: 600; color: var(--ctrl-text); margin-bottom: 16px; padding-bottom: 8px; border-bottom: 1px solid var(--ctrl-border);",
                "{title}"
            }
            {children}
        }
    }
}

/// 子区块标题
#[component]
fn SubSection(title: String, children: Element) -> Element {
    rsx! {
        div { style: "margin-bottom: 16px;",
            h3 { style: "font-size: var(--ctrl-font-size-md); font-weight: 500; color: var(--ctrl-text-secondary); margin-bottom: 8px;",
                "{title}"
            }
            {children}
        }
    }
}