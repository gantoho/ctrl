# Ctrl UI 使用指南

## 目录

1. [快速开始](#1-快速开始)
2. [ThemeProvider 主题配置](#2-themeprovider-主题配置)
3. [Button 按钮](#3-button-按钮)
4. [Input 输入框](#4-input-输入框)
5. [Switch 开关](#5-switch-开关)
6. [Checkbox 复选框](#6-checkbox-复选框)
7. [Radio 单选框](#7-radio-单选框)
8. [Select 下拉选择](#8-select-下拉选择)
9. [Tag 标签](#9-tag-标签)
10. [Card 卡片](#10-card-卡片)
11. [Dialog 对话框](#11-dialog-对话框)
12. [Table 表格](#12-table-表格)
13. [样式覆盖](#13-样式覆盖)
14. [主题定制](#14-主题定制)
15. [完整示例](#15-完整示例)
16. [注意事项](#16-注意事项)
17. [常见问题](#17-常见问题)

---

## 1. 快速开始

### 1.1 添加依赖

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
ctrl = { path = "path/to/ctrl/crates/ctrl" }
```

> 注意：当前 Ctrl UI 尚未发布到 crates.io，使用本地路径依赖。发布后改为 `ctrl = "0.1"`。

### 1.2 最小可运行示例

```rust
use dioxus::prelude::*;
use ctrl::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[allow(non_snake_case)]
fn App() -> Element {
    rsx! {
        ThemeProvider {
            div {
                style: "padding: 40px; max-width: 400px; margin: 0 auto;",

                h1 { "Ctrl UI 示例" }

                Button {
                    variant: Variant::Primary,
                    onclick: move |_| {
                        // 处理点击事件
                    },
                    "点击我"
                }
            }
        }
    }
}
```

### 1.3 导入方式

**方式一：prelude 便捷导入（推荐）**

```rust
use ctrl::prelude::*;
```

这会导入所有常用类型和组件：`ThemeProvider`、`Button`、`Input`、`Size`、`Variant`、`cn` 等。

**方式二：按需导入**

```rust
use ctrl::Button;
use ctrl::theme::ThemeProvider;
use ctrl::types::{Size, Variant};
```

### 1.4 关键规则

> **必须用 `ThemeProvider` 包裹应用根组件，否则 CSS 变量不会被注入，组件将无样式。**

```rust
// ✅ 正确
rsx! {
    ThemeProvider {
        // 你的应用内容
    }
}

// ❌ 错误：组件不会有任何样式
rsx! {
    Button { "无样式按钮" }
}
```

---

## 2. ThemeProvider 主题配置

### 2.1 使用默认主题

不传 `theme` 属性时，自动使用默认主题：

```rust
rsx! {
    ThemeProvider {
        // 应用内容
    }
}
```

### 2.2 自定义主题

```rust
use ctrl::theme::{Theme, ColorPalette};

let my_theme = Theme {
    colors: ColorPalette {
        primary: "#FF6B35",
        primary_hover: "#E55A2B",
        primary_active: "#CC4A1F",
        primary_light: "#FFF0EB",
        ..Default::default()
    },
    radius_md: "0.5rem",
    ..Default::default()
};

rsx! {
    ThemeProvider {
        theme: my_theme,
        // 应用内容
    }
}
```

### 2.3 常用主题配置项

| 配置项 | 路径 | 说明 |
|--------|------|------|
| 主色 | `theme.colors.primary` | 品牌主色调 |
| 主色悬停 | `theme.colors.primary_hover` | 鼠标悬停时主色 |
| 主色浅底 | `theme.colors.primary_light` | Ghost 按钮悬停背景 |
| 危险色 | `theme.colors.danger` | 错误状态颜色 |
| 圆角 | `theme.radius_md` | 组件默认圆角 |
| 字体 | `theme.font_family` | 全局字体族 |
| 过渡 | `theme.transition` | 动画过渡时间 |

### 2.4 主题配置的完整字段

```rust
Theme {
    // 色板
    colors: ColorPalette { .. },

    // 字体
    font_family: "system-ui, sans-serif",
    font_size_sm: "0.75rem",
    font_size_md: "0.875rem",
    font_size_lg: "1rem",

    // 圆角
    radius_sm: "0.25rem",
    radius_md: "0.375rem",
    radius_lg: "0.5rem",

    // 阴影
    shadow_sm: "0 1px 2px 0 rgba(0,0,0,0.05)",
    shadow_md: "0 4px 6px -1px rgba(0,0,0,0.1)",

    // 动效
    transition: "0.15s ease",
}
```

---

## 3. Button 按钮

### 3.1 基础用法

```rust
use ctrl::prelude::*;

// 默认 Primary + Md
Button { "默认按钮" }

// 指定变体
Button { variant: Variant::Secondary, "次级按钮" }
Button { variant: Variant::Outline, "描边按钮" }
Button { variant: Variant::Ghost, "幽灵按钮" }
```

### 3.2 四种变体效果

| 变体 | 视觉效果 | 适用场景 |
|------|---------|---------|
| `Primary` | 实心主色背景，白色文字 | 主要操作（提交、确认） |
| `Secondary` | 实心灰色背景，白色文字 | 次要操作（取消、返回） |
| `Outline` | 透明背景，主色边框+文字 | 中等强调（编辑、查看） |
| `Ghost` | 完全透明，主色文字 | 低强调（更多、链接） |

### 3.3 尺寸控制

```rust
Button { size: Size::Sm, "小按钮" }   // 高度 28px
Button { size: Size::Md, "中按钮" }   // 高度 36px（默认）
Button { size: Size::Lg, "大按钮" }   // 高度 44px
```

### 3.4 状态

```rust
// 禁用
Button { disabled: true, "禁用按钮" }

// 块级（撑满容器宽度）
Button { block: true, "块级按钮" }
```

### 3.5 事件处理

```rust
let mut count = use_signal(|| 0);

rsx! {
    Button {
        onclick: move |_| count.set(count() + 1),
        "点击次数: {count()}"
    }
}
```

### 3.6 按钮内嵌复杂内容

```rust
Button {
    variant: Variant::Primary,
    onclick: move |_| {},
    // 可以放任意子元素
    div {
        style: "display: flex; align-items: center; gap: 6px;",
        span { "🚀" }
        span { "提交" }
    }
}
```

### 3.7 完整 Props 参考

| 属性 | 类型 | 默认值 | 必填 | 说明 |
|------|------|--------|------|------|
| `variant` | `Variant` | `Primary` | 否 | 按钮变体 |
| `size` | `Size` | `Md` | 否 | 按钮尺寸 |
| `disabled` | `bool` | `false` | 否 | 是否禁用 |
| `loading` | `bool` | `false` | 否 | 是否加载中 |
| `block` | `bool` | `false` | 否 | 是否块级 |
| `class` | `String` | `""` | 否 | 自定义 CSS 类名 |
| `style` | `String` | `""` | 否 | 自定义内联样式 |
| `r#type` | `String` | `"button"` | 否 | 原生 button type |
| `onclick` | `Option<EventHandler<MouseEvent>>` | `None` | 否 | 点击事件 |
| `children` | `Element` | — | 是 | 子元素 |

---

## 4. Input 输入框

### 4.1 基础用法（受控组件）

Input 采用**受控组件**模式，必须由父组件管理 `value` 状态：

```rust
let mut value = use_signal(|| String::new());

rsx! {
    Input {
        placeholder: "请输入内容",
        value: value(),
        oninput: move |evt: FormEvent| value.set(evt.value()),
    }
}
```

> **重要：** `oninput` 闭包参数必须显式标注类型 `FormEvent`，否则 Rust 编译器无法推断类型。

### 4.2 尺寸

```rust
Input { placeholder: "小", size: Size::Sm }
Input { placeholder: "中", size: Size::Md }  // 默认
Input { placeholder: "大", size: Size::Lg }
```

### 4.3 状态

```rust
// 禁用
Input { placeholder: "禁用状态", disabled: true }

// 只读
Input { placeholder: "只读状态", readonly: true }

// 错误
Input { placeholder: "错误状态", error: true }
```

### 4.4 不同输入类型

```rust
Input { r#type: "text", placeholder: "文本" }
Input { r#type: "password", placeholder: "密码" }
Input { r#type: "email", placeholder: "邮箱" }
Input { r#type: "number", placeholder: "数字" }
```

### 4.5 完整表单示例

```rust
let mut username = use_signal(|| String::new());
let mut password = use_signal(|| String::new());
let mut username_error = use_signal(|| false);

let handle_submit = move |_| {
    if username().is_empty() {
        username_error.set(true);
    } else {
        username_error.set(false);
        // 执行提交逻辑
    }
};

rsx! {
    div {
        style: "display: flex; flex-direction: column; gap: 16px; max-width: 360px;",

        div {
            style: "display: flex; flex-direction: column; gap: 4px;",
            label { "用户名" }
            Input {
                placeholder: "请输入用户名",
                value: username(),
                error: username_error(),
                oninput: move |evt: FormEvent| username.set(evt.value()),
            }
            if username_error() {
                span {
                    style: "color: var(--ctrl-danger); font-size: var(--ctrl-font-size-sm);",
                    "用户名不能为空"
                }
            }
        }

        div {
            style: "display: flex; flex-direction: column; gap: 4px;",
            label { "密码" }
            Input {
                r#type: "password",
                placeholder: "请输入密码",
                value: password(),
                oninput: move |evt: FormEvent| password.set(evt.value()),
            }
        }

        Button {
            variant: Variant::Primary,
            block: true,
            onclick: handle_submit,
            "登录"
        }
    }
}
```

### 4.6 完整 Props 参考

| 属性 | 类型 | 默认值 | 必填 | 说明 |
|------|------|--------|------|------|
| `size` | `Size` | `Md` | 否 | 输入框尺寸 |
| `value` | `String` | `""` | 否 | 当前值 |
| `placeholder` | `String` | `""` | 否 | 占位文本 |
| `disabled` | `bool` | `false` | 否 | 是否禁用 |
| `readonly` | `bool` | `false` | 否 | 是否只读 |
| `error` | `bool` | `false` | 否 | 是否错误状态 |
| `r#type` | `String` | `"text"` | 否 | 原生 input type |
| `class` | `String` | `""` | 否 | 自定义 CSS 类名 |
| `style` | `String` | `""` | 否 | 自定义内联样式 |
| `oninput` | `Option<EventHandler<FormEvent>>` | `None` | 否 | 输入事件 |
| `onfocus` | `Option<EventHandler<FocusEvent>>` | `None` | 否 | 获得焦点 |
| `onblur` | `Option<EventHandler<FocusEvent>>` | `None` | 否 | 失去焦点 |

---

## 5. Switch 开关

### 5.1 基础用法

通过 `checked` 和 `onchange` 管理开关状态：

```rust
let mut on = use_signal(|| false);

rsx! {
    Switch {
        checked: on(),
        onchange: move |v| on.set(v),
    }
    span { if on() { "已开启" } else { "已关闭" } }
}
```

### 5.2 尺寸

```rust
Switch { size: Size::Sm }           // 小
Switch { size: Size::Md }           // 中（默认）
Switch { size: Size::Lg }           // 大
```

### 5.3 禁用状态

```rust
Switch { disabled: true }            // 关闭-禁用
Switch { disabled: true, checked: true }  // 开启-禁用
```

### 5.4 完整 Props 参考

| 属性 | 类型 | 默认值 | 必填 | 说明 |
|------|------|--------|------|------|
| `checked` | `bool` | `false` | 否 | 是否选中 |
| `disabled` | `bool` | `false` | 否 | 是否禁用 |
| `size` | `Size` | `Md` | 否 | 开关尺寸 |
| `class` | `String` | `""` | 否 | 自定义 CSS 类名 |
| `style` | `String` | `""` | 否 | 自定义内联样式 |
| `onchange` | `Option<EventHandler<bool>>` | `None` | 否 | 状态变化事件 |

---

## 6. Checkbox 复选框

### 6.1 基础用法

```rust
let mut checked = use_signal(|| false);

rsx! {
    Checkbox {
        checked: checked(),
        label: "同意协议".to_string(),
        onchange: move |v| checked.set(v),
    }
}
```

### 6.2 各种状态

```rust
// 未选中
Checkbox { label: "未选中".to_string() }

// 已选中
Checkbox { checked: true, label: "已选中".to_string() }

// 半选状态（常用于全选场景）
Checkbox { indeterminate: true, label: "半选".to_string() }

// 禁用
Checkbox { disabled: true, label: "禁用".to_string() }
Checkbox { checked: true, disabled: true, label: "禁用已选中".to_string() }
```

### 6.3 全选示例

```rust
let items = vec!["选项 A", "选项 B", "选项 C"];
let items_len = items.len();
let mut checked = use_signal(|| vec![false; items_len]);

let all = checked().iter().all(|&c| c);
let some = checked().iter().any(|&c| c);
let indet = some && !all;

rsx! {
    Checkbox {
        checked: all,
        indeterminate: indet,
        label: "全选".to_string(),
        onchange: move |v| checked.set(vec![v; items_len]),
    }
    for (i, item) in items.iter().enumerate() {
        Checkbox {
            key: "{i}",
            checked: checked()[i],
            label: item.to_string(),
            onchange: move |v| { let mut c = checked(); c[i] = v; checked.set(c); },
        }
    }
}
```

### 6.4 完整 Props 参考

| 属性 | 类型 | 默认值 | 必填 | 说明 |
|------|------|--------|------|------|
| `checked` | `bool` | `false` | 否 | 是否选中 |
| `disabled` | `bool` | `false` | 否 | 是否禁用 |
| `indeterminate` | `bool` | `false` | 否 | 半选状态 |
| `label` | `String` | `""` | 否 | 标签文本 |
| `class` | `String` | `""` | 否 | 自定义 CSS 类名 |
| `style` | `String` | `""` | 否 | 自定义内联样式 |
| `onchange` | `Option<EventHandler<bool>>` | `None` | 否 | 状态变化事件 |

---

## 7. Radio 单选框

### 7.1 基础用法

Radio 通过 `value` + `onchange` 实现互斥选择：

```rust
let mut selected = use_signal(|| "a".to_string());

rsx! {
    Radio { value: "a".to_string(), label: "选项 A".to_string(), checked: selected() == "a", onchange: move |v| selected.set(v) }
    Radio { value: "b".to_string(), label: "选项 B".to_string(), checked: selected() == "b", onchange: move |v| selected.set(v) }
    Radio { value: "c".to_string(), label: "选项 C".to_string(), checked: selected() == "c", onchange: move |v| selected.set(v) }
}
```

### 7.2 禁用状态

```rust
Radio { value: "a".to_string(), label: "已禁用".to_string(), checked: true, disabled: true }
Radio { value: "b".to_string(), label: "禁用未选".to_string(), disabled: true }
```

### 7.3 完整 Props 参考

| 属性 | 类型 | 默认值 | 必填 | 说明 |
|------|------|--------|------|------|
| `checked` | `bool` | `false` | 否 | 是否选中 |
| `disabled` | `bool` | `false` | 否 | 是否禁用 |
| `value` | `String` | `""` | 是 | 单选项的值 |
| `label` | `String` | `""` | 否 | 标签文本 |
| `class` | `String` | `""` | 否 | 自定义 CSS 类名 |
| `style` | `String` | `""` | 否 | 自定义内联样式 |
| `onchange` | `Option<EventHandler<String>>` | `None` | 否 | 选中变化事件 |

---

## 8. Select 下拉选择

### 8.1 基础用法

```rust
let mut value = use_signal(|| String::new());
let options = vec![
    ("a".to_string(), "选项 A".to_string(), false),
    ("b".to_string(), "选项 B".to_string(), false),
    ("c".to_string(), "选项 C（禁用）".to_string(), true),
];

rsx! {
    Select {
        options: options,
        placeholder: "请选择".to_string(),
        value: value(),
        onchange: move |v| value.set(v),
    }
}
```

### 8.2 尺寸

```rust
Select { size: Size::Sm, options, placeholder: "小" }
Select { size: Size::Md, options, placeholder: "中" }   // 默认
Select { size: Size::Lg, options, placeholder: "大" }
```

### 8.3 禁用

```rust
Select { disabled: true, options, placeholder: "整个禁用" }
```

### 8.4 完整 Props 参考

| 属性 | 类型 | 默认值 | 必填 | 说明 |
|------|------|--------|------|------|
| `options` | `Vec<(String,String,bool)>` | `[]` | 否 | 选项列表 (值,标签,禁用) |
| `value` | `String` | `""` | 否 | 当前选中值 |
| `placeholder` | `String` | `"请选择"` | 否 | 占位文本 |
| `size` | `Size` | `Md` | 否 | 选择器尺寸 |
| `disabled` | `bool` | `false` | 否 | 是否禁用 |
| `class` | `String` | `""` | 否 | 自定义 CSS 类名 |
| `style` | `String` | `""` | 否 | 自定义内联样式 |
| `onchange` | `Option<EventHandler<String>>` | `None` | 否 | 选中变化事件 |

---

## 9. Tag 标签

### 9.1 基础用法

```rust
rsx! {
    Tag { color: "var(--ctrl-primary)".to_string(), "Primary" }
    Tag { color: "var(--ctrl-success)".to_string(), "Success" }
    Tag { color: "var(--ctrl-warning)".to_string(), "Warning" }
    Tag { color: "var(--ctrl-danger)".to_string(), "Danger" }
    Tag { color: "var(--ctrl-info)".to_string(), "Info" }
}
```

### 9.2 可关闭标签

```rust
Tag { color: "var(--ctrl-primary)".to_string(), closable: true, "可关闭" }
```

### 9.3 完整 Props 参考

| 属性 | 类型 | 默认值 | 必填 | 说明 |
|------|------|--------|------|------|
| `color` | `String` | `var(--ctrl-primary)` | 否 | 标签颜色（CSS 颜色值） |
| `closable` | `bool` | `false` | 否 | 是否可关闭 |
| `class` | `String` | `""` | 否 | 自定义 CSS 类名 |
| `style` | `String` | `""` | 否 | 自定义内联样式 |
| `onclose` | `Option<EventHandler<()>>` | `None` | 否 | 关闭事件回调 |
| `children` | `Element` | — | 是 | 标签内容 |

---

## 10. Card 卡片

### 10.1 基础用法

```rust
rsx! {
    Card { title: "卡片标题".to_string(),
        p { "这是卡片的内容区域，可以放置任何元素。" }
    }
}
```

### 10.2 选项

```rust
// 默认：带边框、无阴影
Card { title: "默认".to_string(), ... }

// 带阴影
Card { shadow: true, title: "阴影".to_string(), ... }

// 无边框
Card { bordered: false, title: "无边框".to_string(), ... }

// 自定义头部
Card { header: rsx! { div { "自定义头部" } }, ... }
```

### 10.3 完整 Props 参考

| 属性 | 类型 | 默认值 | 必填 | 说明 |
|------|------|--------|------|------|
| `title` | `String` | `""` | 否 | 卡片标题 |
| `bordered` | `bool` | `true` | 否 | 是否显示边框 |
| `shadow` | `bool` | `false` | 否 | 是否带阴影 |
| `class` | `String` | `""` | 否 | 自定义 CSS 类名 |
| `style` | `String` | `""` | 否 | 自定义内联样式 |
| `header` | `Option<Element>` | `None` | 否 | 自定义头部插槽 |
| `children` | `Element` | — | 是 | 卡片内容 |

---

## 11. Dialog 对话框

### 11.1 基础用法

```rust
let mut visible = use_signal(|| false);

rsx! {
    Button { onclick: move |_| visible.set(true), "打开对话框" }
    Dialog {
        visible: visible(),
        title: "提示".to_string(),
        onclose: move |_| visible.set(false),
        p { "这是一条提示信息" }
    }
}
```

### 11.2 带底部操作

```rust
Dialog {
    visible: visible(),
    title: "确认操作".to_string(),
    footer: rsx! {
        Button { variant: Variant::Ghost, onclick: move |_| visible.set(false), "取消" }
        Button { variant: Variant::Primary, onclick: move |_| visible.set(false), "确定" }
    },
    p { "确定要执行此操作吗？" }
}
```

### 11.3 完整 Props 参考

| 属性 | 类型 | 默认值 | 必填 | 说明 |
|------|------|--------|------|------|
| `visible` | `bool` | `false` | 否 | 是否显示 |
| `title` | `String` | `""` | 否 | 对话框标题 |
| `width` | `String` | `"480px"` | 否 | 对话框宽度 |
| `show_close` | `bool` | `true` | 否 | 是否显示关闭按钮 |
| `mask_closable` | `bool` | `true` | 否 | 点击遮罩是否关闭 |
| `class` | `String` | `""` | 否 | 自定义 CSS 类名 |
| `style` | `String` | `""` | 否 | 自定义内联样式 |
| `onclose` | `Option<EventHandler<()>>` | `None` | 否 | 关闭事件 |
| `footer` | `Option<Element>` | `None` | 否 | 底部操作区插槽 |
| `children` | `Element` | — | 是 | 对话框内容 |

---

## 12. Table 表格

### 12.1 基础用法

```rust
let columns = vec![
    TableColumn { title: "名称".into(), ..Default::default() },
    TableColumn { title: "类型".into(), ..Default::default() },
    TableColumn { title: "默认值".into(), ..Default::default() },
    TableColumn { title: "说明".into(), ..Default::default() },
];

let data = vec![
    vec!["variant".into(), "Variant".into(), "Primary".into(), "按钮变体".into()],
    vec!["size".into(), "Size".into(), "Md".into(), "按钮尺寸".into()],
];

rsx! {
    Table {
        columns: columns,
        data: data,
    }
}
```

### 12.2 斑马纹

```rust
Table { striped: true, columns: cols, data: data }
```

### 12.3 其他选项

```rust
// 无边框
Table { bordered: false, columns: cols, data: data }
```

### 12.4 完整 Props 参考

| 属性 | 类型 | 默认值 | 必填 | 说明 |
|------|------|--------|------|------|
| `columns` | `Vec<TableColumn>` | `[]` | 是 | 列定义 |
| `data` | `Vec<Vec<String>>` | `[]` | 是 | 行数据 |
| `striped` | `bool` | `false` | 否 | 是否显示斑马纹 |
| `bordered` | `bool` | `true` | 否 | 是否显示边框 |
| `class` | `String` | `""` | 否 | 自定义 CSS 类名 |
| `style` | `String` | `""` | 否 | 自定义内联样式 |

---

## 13. 样式覆盖

### 13.1 三种覆盖方式

Ctrl UI 提供三种层级的样式覆盖，从简单到复杂：

#### 方式一：通过 `style` 属性覆盖（推荐用于单次调整）

```rust
Button {
    style: "border-radius: 8px; font-weight: 700;",
    "自定义样式按钮"
}
```

用户传入的 `style` 会追加到组件默认样式之后，因此可以覆盖任何默认值。

#### 方式二：通过 `class` 属性添加 CSS 类

```rust
// 在你的全局 CSS 中定义：
// .my-button { border-radius: 8px; font-weight: 700; }

Button {
    class: "my-button",
    "自定义类名按钮"
}
```

#### 方式三：覆盖 CSS 变量（推荐用于全局调整）

在 HTML 中引入自定义 CSS 文件，覆盖 `:root` 中的变量：

```css
/* 你的 style.css */
:root {
    --ctrl-primary: #FF6B35;
    --ctrl-radius-md: 8px;
    --ctrl-font-family: "PingFang SC, Microsoft YaHei, sans-serif";
}
```

或者在 Dioxus 中直接注入：

```rust
rsx! {
    ThemeProvider {
        // 注入自定义 CSS 变量覆盖
        style {
            ":root {{
                --ctrl-primary: #FF6B35;
                --ctrl-radius-md: 8px;
            }}"
        }
        // 应用内容
    }
}
```

### 5.2 覆盖示例

```rust
// 覆盖 Logo 颜色
Button {
    style: "background: #FF6B35; border-color: #FF6B35;",
    "品牌色按钮"
}

// 覆盖圆角
Button {
    style: "border-radius: 24px;",
    "圆角按钮"
}

// 覆盖尺寸
Button {
    style: "height: 48px; padding: 12px 32px; font-size: 1.125rem;",
    "自定义尺寸"
}

// 组合覆盖
Input {
    style: "border-radius: 8px; border-width: 2px; background: #FAFAFA;",
    placeholder: "自定义输入框",
}
```

### 5.3 样式覆盖优先级

```
用户 class 属性（CSS 选择器）       ← 最高
用户 style 属性（内联样式）         ← 高
组件动态样式（状态驱动）            ← 中
组件基础样式（build_xxx_style）     ← 低
CSS 变量默认值（ThemeProvider）     ← 最低
```

---

## 14. 主题定制

### 6.1 方式一：ThemeProvider 传参（推荐）

适用于需要完整主题切换的场景（如亮色/暗色模式）：

```rust
use ctrl::theme::{Theme, ColorPalette};

// 暗色主题
let dark_theme = Theme {
    colors: ColorPalette {
        primary: "#818CF8",
        primary_hover: "#6366F1",
        primary_active: "#4F46E5",
        primary_light: "#1E1B4B",
        bg: "#1F2937",
        bg_secondary: "#111827",
        text: "#F9FAFB",
        text_secondary: "#D1D5DB",
        text_disabled: "#6B7280",
        border: "#374151",
        border_hover: "#4B5563",
        ..Default::default()
    },
    ..Default::default()
};

rsx! {
    ThemeProvider {
        theme: dark_theme,
        // 暗色主题下的应用内容
    }
}
```

### 6.2 方式二：部分覆盖（使用结构体更新语法）

```rust
let custom_theme = Theme {
    colors: ColorPalette {
        primary: "#059669",        // 只改主色为绿色
        primary_hover: "#047857",
        primary_active: "#065F46",
        primary_light: "#ECFDF5",
        ..Default::default()       // 其余颜色保持默认
    },
    radius_md: "0.5rem",           // 只改圆角
    ..Default::default()           // 其余配置保持默认
};
```

### 6.3 方式三：CSS 变量覆盖（最灵活）

适用于只需要微调几个颜色值的场景：

```css
/* 在你的 CSS 文件中 */
:root {
    --ctrl-primary: #059669;
    --ctrl-radius-md: 0.5rem;
    --ctrl-font-family: "Inter, system-ui, sans-serif";
    --ctrl-transition: 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}
```

### 6.4 主题切换示例

```rust
use ctrl::theme::{Theme, ColorPalette};

fn App() -> Element {
    let mut is_dark = use_signal(|| false);

    let light_theme = Theme::default();
    let dark_theme = Theme {
        colors: ColorPalette {
            primary: "#818CF8",
            primary_hover: "#6366F1",
            primary_active: "#4F46E5",
            primary_light: "#1E1B4B",
            bg: "#1F2937",
            bg_secondary: "#111827",
            text: "#F9FAFB",
            text_secondary: "#D1D5DB",
            text_disabled: "#6B7280",
            border: "#374151",
            border_hover: "#4B5563",
            ..Default::default()
        },
        ..Default::default()
    };

    let current_theme = if is_dark() { dark_theme } else { light_theme };

    rsx! {
        ThemeProvider {
            theme: current_theme,
            div {
                style: "padding: 40px; background: var(--ctrl-bg); color: var(--ctrl-text); min-height: 100vh;",

                Button {
                    variant: Variant::Outline,
                    onclick: move |_| is_dark.set(!is_dark()),
                    if is_dark() { "切换到亮色" } else { "切换到暗色" }
                }
            }
        }
    }
}
```

---

## 15. 完整示例

### 7.1 登录表单

```rust
use dioxus::prelude::*;
use ctrl::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[allow(non_snake_case)]
fn App() -> Element {
    rsx! {
        ThemeProvider {
            LoginForm {}
        }
    }
}

#[allow(non_snake_case)]
fn LoginForm() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut errors = use_signal(|| (false, false)); // (email_error, password_error)
    let mut loading = use_signal(|| false);

    let handle_submit = move |_| {
        let mut e = (false, false);
        if email().trim().is_empty() {
            e.0 = true;
        }
        if password().is_empty() {
            e.1 = true;
        }
        errors.set(e);

        if !e.0 && !e.1 {
            loading.set(true);
            // 模拟登录请求
            // spawn 异步任务...
        }
    };

    rsx! {
        div {
            style: "max-width: 400px; margin: 80px auto; padding: 32px;
                    background: var(--ctrl-bg);
                    border: 1px solid var(--ctrl-border);
                    border-radius: var(--ctrl-radius-lg);",

            h1 {
                style: "font-size: 1.5rem; font-weight: 700;
                        color: var(--ctrl-text); margin-bottom: 8px;
                        text-align: center;",
                "登录"
            }

            p {
                style: "color: var(--ctrl-text-secondary);
                        text-align: center; margin-bottom: 32px;",
                "欢迎回来，请输入你的账号信息"
            }

            // 邮箱
            div {
                style: "margin-bottom: 20px;",
                label {
                    style: "display: block; margin-bottom: 6px;
                            font-size: var(--ctrl-font-size-md);
                            font-weight: 500; color: var(--ctrl-text);",
                    "邮箱"
                }
                Input {
                    r#type: "email",
                    placeholder: "请输入邮箱",
                    value: email(),
                    error: errors().0,
                    oninput: move |evt: FormEvent| {
                        email.set(evt.value());
                        if errors().0 {
                            errors.set((false, errors().1));
                        }
                    },
                }
                if errors().0 {
                    span {
                        style: "display: block; margin-top: 4px;
                                color: var(--ctrl-danger);
                                font-size: var(--ctrl-font-size-sm);",
                        "请输入有效的邮箱地址"
                    }
                }
            }

            // 密码
            div {
                style: "margin-bottom: 24px;",
                label {
                    style: "display: block; margin-bottom: 6px;
                            font-size: var(--ctrl-font-size-md);
                            font-weight: 500; color: var(--ctrl-text);",
                    "密码"
                }
                Input {
                    r#type: "password",
                    placeholder: "请输入密码",
                    value: password(),
                    error: errors().1,
                    oninput: move |evt: FormEvent| {
                        password.set(evt.value());
                        if errors().1 {
                            errors.set((errors().0, false));
                        }
                    },
                }
                if errors().1 {
                    span {
                        style: "display: block; margin-top: 4px;
                                color: var(--ctrl-danger);
                                font-size: var(--ctrl-font-size-sm);",
                        "请输入密码"
                    }
                }
            }

            // 提交按钮
            Button {
                variant: Variant::Primary,
                size: Size::Lg,
                block: true,
                onclick: handle_submit,
                if loading() { "登录中..." } else { "登录" }
            }
        }
    }
}
```

### 15.2 使用 cn() 工具函数

```rust
use ctrl::utils::cn;

fn FormField(
    label: String,
    error: bool,
    children: Element,
) -> Element {
    let container_class = cn(&[
        "form-field",
        if error { "form-field--error" } else { "" },
    ]);

    rsx! {
        div {
            class: "{container_class}",
            label { "{label}" }
            {children}
        }
    }
}
```

---

## 8. 注意事项

### 8.1 必须使用 ThemeProvider

**所有组件必须在 `ThemeProvider` 内部使用**，否则不会注入 CSS 变量，组件会显示为无样式的原生 HTML 元素。

```rust
// ✅ 正确
rsx! {
    ThemeProvider {
        Button { "有样式" }
    }
}

// ❌ 错误：按钮无样式
rsx! {
    Button { "无样式" }
}
```

### 8.2 Input 的 oninput 闭包需要显式类型标注

由于 Rust 的类型推断限制，`oninput` 闭包的参数必须显式标注类型：

```rust
// ❌ 编译错误：type annotations needed
oninput: move |evt| value.set(evt.value()),

// ✅ 正确
oninput: move |evt: FormEvent| value.set(evt.value()),
```

### 8.3 Input 是受控组件

Input 的值由父组件通过 `value` prop 控制，不会自行维护内部状态。这意味着：

```rust
// ❌ 无效：不绑定 value 和 oninput
Input { placeholder: "输入内容" }
// 用户输入不会反映到组件中，因为没有被父组件状态管理

// ✅ 正确：完整的受控模式
let mut value = use_signal(|| String::new());
Input {
    value: value(),
    oninput: move |evt: FormEvent| value.set(evt.value()),
}
```

### 8.4 事件处理器的生命周期

在 Dioxus 0.7 中，`EventHandler` 在 `rsx!` 闭包中需要所有权。组件内部通过 `.clone()` 处理：

```rust
// 组件内部实现
let onclick = props.onclick.clone();
// onclick 被移入闭包，调用 .call(evt)
```

用户无需关心这一点，正常传递闭包即可。

### 16.5 自定义 class 的使用

```rust
// 传入的 class 字符串直接设置为 HTML class 属性
Button {
    class: "my-custom-class another-class",
    "按钮"
}
```

可以通过 CSS 选择器 `.my-custom-class` 为按钮添加额外样式。注意 CSS 选择器优先级高于内联样式，可用于覆盖默认样式。

### 8.6 自定义 style 的合并顺序

用户传入的 `style` 属性**追加**在组件默认样式之后：

```rust
// 组件默认：font-weight: 500; padding: 8px 16px; ...
// 用户传入：font-weight: 700; border-radius: 24px;
// 最终渲染：font-weight: 500; padding: 8px 16px; ...; font-weight: 700; border-radius: 24px;
// 效果：font-weight 被覆盖为 700，border-radius 被覆盖为 24px
```

### 8.7 组件函数命名

组件函数使用 PascalCase（如 `Button`、`Input`），并带有 `#[allow(non_snake_case)]` 属性。这是 Dioxus 的约定，与 React 组件命名习惯一致。

### 8.8 悬浮（hover）效果通过事件模拟

Ctrl UI 使用 `onmouseenter`/`onmouseleave` 事件模拟 CSS `:hover` 伪类。这意味着：

- 悬浮效果在 JavaScript 运行时生效
- 在某些极端情况下（如触摸设备），hover 效果可能不会触发
- 对于移动端，这是预期行为，因为触摸设备没有 hover 概念

### 8.9 避免在循环中直接使用组件

在 Dioxus 中，如果需要在列表中使用组件，确保每个组件有唯一的 key（通过 `key` 属性）。Ctrl UI 的组件支持所有 Dioxus 标准属性。

---

## 9. 常见问题

### Q: 为什么组件没有样式？

**A:** 检查是否用 `ThemeProvider` 包裹了应用根组件。`ThemeProvider` 负责注入 CSS 变量，没有它组件会显示为无样式原生元素。

### Q: 如何修改主色调？

**A:** 有三种方式：
1. 在 `ThemeProvider` 中传入自定义 `Theme`（推荐用于主题切换）
2. 在 CSS 中覆盖 `:root { --ctrl-primary: #你的颜色; }`（推荐用于全局微调）
3. 在单个组件的 `style` 属性中覆盖（推荐用于局部调整）

### Q: 如何让按钮撑满容器宽度？

**A:** 使用 `block` 属性：
```rust
Button { block: true, "块级按钮" }
```

### Q: 如何禁用按钮的同时保留点击事件？

**A:** `disabled` 属性设置了 `pointer-events: none`，禁用的按钮不会触发任何事件。如果需要在点击时弹出提示，不要使用 `disabled`，而是在 `onclick` 中自行判断。

### Q: 如何给 Input 添加前缀/后缀图标？

**A:** 当前版本 Input 不支持内置前缀/后缀。你可以通过包装元素实现：
```rust
div {
    style: "position: relative; display: flex; align-items: center;",
    span {
        style: "position: absolute; left: 12px; color: var(--ctrl-text-secondary);",
        "🔍"
    }
    Input {
        style: "padding-left: 36px;",
        placeholder: "搜索",
    }
}
```

### Q: 可以在非 Dioxus web 平台使用吗？

**A:** 当前 Ctrl UI 仅支持 Dioxus Web 平台（`features = ["web"]`）。Desktop 和 Mobile 平台的支持需要额外的适配工作。

### Q: 如何贡献新组件？

**A:** 参考 [IMPLEMENTATION.md](./IMPLEMENTATION.md) 中的"扩展指南"章节，按照组件的三步流程（创建目录 → 实现组件 → 注册导出）添加新组件。

### Q: 与 Dioxus 0.7 兼容吗？

**A:** 当前版本基于 Dioxus 0.7 开发，完全兼容。使用 `use_signal`、`Signal::new` 等 0.7 原生 API。

### Q: 性能如何？内联样式会不会影响渲染性能？

**A:** 内联样式对性能的影响微乎其微。CSS 变量的解析在浏览器层面完成，与外部 CSS 性能一致。style 字符串的构建在 Rust 端完成，开销极小。实际测试中，数百个组件同时渲染不会产生可感知的性能问题。