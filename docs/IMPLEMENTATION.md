# Ctrl UI 实现细节文档

## 目录

1. [项目架构](#1-项目架构)
2. [分层设计](#2-分层设计)
3. [主题系统](#3-主题系统)
4. [样式方案](#4-样式方案)
5. [组件设计模式](#5-组件设计模式)
6. [CSS 变量对照表](#6-css-变量对照表)
7. [现有组件详解](#7-现有组件详解)
8. [扩展指南](#8-扩展指南)
9. [设计决策与权衡](#9-设计决策与权衡)

---

## 1. 项目架构

```
ctrl/                              # 仓库根目录
├── Cargo.toml                     # Workspace 配置，定义共享依赖
├── docs/                          # 文档
├── crates/
│   ├── ctrl-core/                 # 核心层 crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs             # 统一导出：theme + types + utils
│   │       ├── theme/
│   │       │   ├── mod.rs         # 主题模块导出
│   │       │   ├── colors.rs      # ColorPalette 色板结构体
│   │       │   ├── theme.rs       # Theme 主题配置结构体
│   │       │   └── provider.rs    # ThemeProvider 组件 + CSS 变量注入
│   │       ├── types/
│   │       │   ├── mod.rs         # 类型模块导出
│   │       │   ├── size.rs        # Size 枚举（Sm/Md/Lg）
│   │       │   └── variant.rs     # Variant 枚举（语义变体）
│   │       └── utils/
│   │           ├── mod.rs         # 工具模块导出
│   │           └── cn.rs          # cn() classname 合并工具
│   │
│   ├── ctrl-components/           # 组件层 crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs             # 统一导出所有组件
│   │       ├── button/
│   │       │   ├── mod.rs
│   │       │   └── button.rs      # Button 组件实现
│   │       ├── input/
│   │       │   ├── mod.rs
│   │       │   └── input.rs       # Input 组件实现
│   │       ├── switch/            # Switch 开关
│   │       ├── checkbox/          # Checkbox 复选框
│   │       ├── radio/             # Radio 单选框
│   │       ├── select/            # Select 下拉选择
│   │       ├── tag/               # Tag 标签
│   │       ├── card/              # Card 卡片
│   │       ├── dialog/            # Dialog 对话框
│   │       └── table/             # Table 表格
│   │
│   └── ctrl/                      # 聚合导出 crate（用户唯一依赖）
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs             # re-export ctrl-core + ctrl-components
│
└── examples/
    └── demo/                      # 演示应用
        ├── Cargo.toml
        └── src/
            └── main.rs
```

### 依赖关系图

```
ctrl (聚合层)
├── ctrl-core (核心层)
│   └── dioxus
└── ctrl-components (组件层)
    ├── ctrl-core
    └── dioxus
```

用户只需依赖 `ctrl` 一个 crate，即可获得全部功能。

---

## 2. 分层设计

### 2.1 ctrl-core —— 核心层

**职责：** 提供与 UI 无关的基础设施，不包含任何具体组件。

| 模块 | 路径 | 内容 |
|------|------|------|
| `theme` | `src/theme/` | `Theme` 主题配置、`ColorPalette` 色板、`ThemeProvider` 组件 |
| `types` | `src/types/` | `Size` 尺寸枚举、`Variant` 语义变体枚举 |
| `utils` | `src/utils/` | `cn()` classname 合并工具函数 |

**设计原则：**
- 零组件依赖：不依赖 `ctrl-components`，可以被其他 crate 独立使用
- 纯数据 + 基础组件：`ThemeProvider` 是唯一包含在 core 中的"组件"，因为它属于主题基础设施
- 所有类型实现 `Debug + Clone + PartialEq`，确保在 Dioxus 的响应式系统中正常工作

### 2.2 ctrl-components —— 组件层

**职责：** 实现所有具体 UI 组件。

**依赖：**
- `dioxus`：渲染框架
- `ctrl-core`：使用 `Size`、`Variant` 等公共类型

**组件目录约定：**
```
src/
├── button/
│   ├── mod.rs          # 导出 Button、ButtonProps
│   └── button.rs       # 组件函数 + 样式构建函数
└── input/
    ├── mod.rs
    └── input.rs
```

每个组件一个独立目录，包含：
- `mod.rs`：模块声明与 re-export
- `组件名.rs`：组件实现（Props 结构体 + 组件函数 + 样式构建函数）

### 2.3 ctrl —— 聚合层

**职责：** 将 `ctrl-core` 和 `ctrl-components` 合并为一个统一的对外接口。

**核心代码：**
```rust
// 核心层
pub use ctrl_core::*;

// 组件层
pub use ctrl_components::*;

/// 便捷导入
pub mod prelude {
    pub use ctrl_core::theme::{ThemeProvider, ThemeProviderProps};
    pub use ctrl_core::types::*;
    pub use ctrl_core::utils::cn;
    pub use ctrl_components::{
        Button, ButtonProps, Input, InputProps,
        Switch, SwitchProps, Checkbox, CheckboxProps,
        Radio, RadioProps, Select, SelectProps,
        Tag, TagProps, Card, CardProps,
        Dialog, DialogProps, Table, TableColumn, TableProps,
    };
}
```

**设计意图：** 用户无需关心内部 crate 划分，只需 `use ctrl::prelude::*` 即可获得所有常用类型和组件。

---

## 3. 主题系统

### 3.1 数据模型

主题系统由两层结构组成：

#### ColorPalette（色板）

```rust
pub struct ColorPalette {
    // 品牌色
    pub primary: &'static str,        // 主色 #4F46E5
    pub primary_hover: &'static str,  // 主色悬停 #4338CA
    pub primary_active: &'static str, // 主色激活 #3730A3
    pub primary_light: &'static str,  // 主色浅底 #EEF2FF

    // 功能色
    pub secondary: &'static str,      // 次级色 #6B7280
    pub secondary_hover: &'static str,
    pub success: &'static str,        // 成功色 #10B981
    pub warning: &'static str,        // 警告色 #F59E0B
    pub danger: &'static str,          // 危险色 #EF4444
    pub info: &'static str,           // 信息色 #3B82F6

    // 中性色
    pub bg: &'static str,             // 背景色 #FFFFFF
    pub bg_secondary: &'static str,   // 次级背景 #F9FAFB
    pub text: &'static str,           // 文字色 #111827
    pub text_secondary: &'static str, // 次级文字 #6B7280
    pub text_disabled: &'static str,  // 禁用文字 #D1D5DB
    pub border: &'static str,         // 边框色 #E5E7EB
    pub border_hover: &'static str,   // 边框悬停 #D1D5DB
}
```

#### Theme（主题）

```rust
pub struct Theme {
    pub colors: ColorPalette,         // 色板
    pub font_family: &'static str,    // 字体族
    pub font_size_sm: &'static str,   // 小号字体 0.75rem
    pub font_size_md: &'static str,   // 中号字体 0.875rem
    pub font_size_lg: &'static str,   // 大号字体 1rem
    pub radius_sm: &'static str,      // 小圆角 0.25rem
    pub radius_md: &'static str,      // 中圆角 0.375rem
    pub radius_lg: &'static str,      // 大圆角 0.5rem
    pub shadow_sm: &'static str,      // 小阴影
    pub shadow_md: &'static str,      // 中阴影
    pub transition: &'static str,     // 过渡动画 0.15s ease
}
```

### 3.2 ThemeProvider 工作机制

`ThemeProvider` 是主题系统的核心，它做了两件事：

**第一步：注入 CSS 变量**

`ThemeProvider` 渲染一个 `<style>` 标签，将 `Theme` 中的所有配置转换为 CSS 自定义属性，注入到 `:root`：

```css
:root {
    --ctrl-primary: #4F46E5;
    --ctrl-primary-hover: #4338CA;
    --ctrl-primary-active: #3730A3;
    /* ... 共 27 个 CSS 变量 */
}
```

**第二步：通过 Dioxus Context 传递主题**

```rust
use_context_provider(|| theme);
```

子组件可以通过 `use_context::<Theme>()` 获取主题配置（虽然当前组件主要使用 CSS 变量，但这为未来需要编程式访问主题的场景预留了接口）。

### 3.3 主题定制流程

```
用户自定义 Theme
    │
    ▼
ThemeProvider (build_css_vars)
    │
    ▼
<style> 标签注入 :root CSS 变量
    │
    ▼
组件通过 var(--ctrl-xxx) 引用 CSS 变量
    │
    ▼
最终渲染
```

用户也可以在外部 CSS 中直接覆盖 CSS 变量，实现更灵活的主题定制。

---

## 4. 样式方案

### 4.1 核心思路：CSS 变量 + 内联样式

Ctrl UI 采用 **CSS 变量 + 内联样式** 的混合方案，无需任何外部 CSS 文件。

| 方案 | 优点 | 缺点 | 我们的选择 |
|------|------|------|-----------|
| 外部 CSS 文件 | 完整的伪类支持 | 需要构建工具配置，不够"开箱即用" | ❌ |
| CSS-in-JS 库 | 运行时动态样式 | 增加依赖，Dioxus 生态不成熟 | ❌ |
| 纯内联样式 | 零配置 | 无法使用 `:hover`、`:focus` 等伪类 | 部分使用 |
| CSS 变量 + 内联 + 事件模拟 | 零配置 + 伪类效果 | 轻微运行时开销 | ✅ 采用 |

### 4.2 样式构建流程

每个组件包含一个私有的 `build_xxx_style()` 函数，根据 props 动态构建样式字符串：

```
Props 输入
    │
    ▼
build_xxx_style(variant, size, hovered, disabled, ...)
    │
    ▼
Vec<String> 样式声明列表
    │
    ▼
join("; ") → 最终 style 字符串
    │
    ▼
注入到 HTML 元素的 style 属性
```

### 4.3 样式优先级（从低到高）

```
1. 组件默认样式（build_xxx_style 中的基础样式）
2. 主题 CSS 变量（var(--ctrl-primary) 等）
3. Props 驱动的动态样式（variant、size、hovered 等）
4. 用户自定义 style 属性（props.style，追加在最后）
5. 用户自定义 class 属性（props.class，CSS 选择器优先级最高）
```

这确保了用户可以通过 `style` 和 `class` 属性轻松覆盖任何默认样式。

### 4.4 伪类模拟

由于内联样式不支持 `:hover`、`:focus` 等伪类，Ctrl UI 使用 Dioxus 事件处理器模拟：

| CSS 伪类 | 模拟方式 |
|----------|---------|
| `:hover` | `onmouseenter` / `onmouseleave` 切换 `hovered` 信号 |
| `:focus` | `onfocusin` / `onfocusout` 切换 `focused` 信号 |
| `:active` | 暂未实现（后续可扩展） |
| `:disabled` | 通过 `disabled` prop 直接判断 |

**实现示例（Button 悬停）：**
```rust
let mut hovered = use_signal(|| false);

// 在 rsx! 中：
onmouseenter: move |_| hovered.set(true),
onmouseleave: move |_| hovered.set(false),

// 样式函数根据 hovered 状态切换颜色：
let bg = if hovered {
    "var(--ctrl-primary-hover)"
} else {
    "var(--ctrl-primary)"
};
```

### 4.5 自定义样式合并

用户传入的 `style` 属性会**追加**到组件默认样式之后，因此可以覆盖任何默认值：

```rust
// 组件内部：
styles.push("font-weight: 500".into());
// ... 其他默认样式 ...
styles.push(custom_style.to_string());  // 用户样式追加在最后

// 最终效果：用户的 font-weight 会覆盖默认值
```

---

## 5. 组件设计模式

### 5.1 组件结构规范

每个组件遵循统一的结构：

```
┌─────────────────────────────────────────┐
│  Props 结构体（#[derive(Props, PartialEq, Clone)]）  │
│  ├── 功能属性（variant, size, disabled 等）         │
│  ├── 样式属性（class, style）                      │
│  ├── 事件属性（onclick, oninput 等）              │
│  └── 子元素（children: Element）                  │
├─────────────────────────────────────────┤
│  build_xxx_style() 私有函数                       │
│  根据 props 构建样式字符串                        │
├─────────────────────────────────────────┤
│  组件函数（#[allow(non_snake_case)]）              │
│  ├── use_signal 管理内部状态                       │
│  ├── 调用 build_xxx_style 构建样式                 │
│  ├── 复制事件处理器（避免所有权问题）               │
│  └── rsx! 渲染 HTML 元素                         │
└─────────────────────────────────────────┘
```

### 5.2 Props 设计约定

所有组件的 Props 遵循以下约定：

**通用属性（每个组件都应包含）：**
- `class: String` — 自定义 CSS 类名，默认空字符串
- `style: String` — 自定义内联样式，追加到组件默认样式之后
- `size: Size` — 尺寸控制，默认 `Size::Md`

**可选通用属性：**
- `disabled: bool` — 禁用状态
- `variant: Variant` — 语义变体（仅部分组件需要）

**事件属性：**
- 使用 `Option<EventHandler<T>>` 类型，默认 `None`
- 在组件函数中 `.clone()` 后移入闭包

### 5.3 事件处理模式

由于 Dioxus 的 `EventHandler::call()` 需要所有权，组件内部使用 clone 模式：

```rust
// 从 props 中 clone 事件处理器
let onclick = props.onclick.clone();

// 在 rsx! 闭包中调用
onclick: move |evt| {
    if let Some(ref handler) = onclick {
        handler.call(evt);
    }
},
```

### 5.4 命名规范

| 元素 | 规范 | 示例 |
|------|------|------|
| 组件函数 | PascalCase + `#[allow(non_snake_case)]` | `pub fn Button(...)` |
| Props 结构体 | PascalCase + `Props` 后缀 | `ButtonProps` |
| 样式构建函数 | snake_case + `build_` 前缀 | `build_button_style()` |
| 模块文件 | 小写 + 下划线 | `button.rs` |

---

## 6. CSS 变量对照表

以下是 `ThemeProvider` 注入的所有 CSS 变量及其用途：

### 品牌色

| CSS 变量 | 默认值 | 用途 |
|----------|--------|------|
| `--ctrl-primary` | `#4F46E5` | 主色调，用于 Primary 按钮、聚焦边框等 |
| `--ctrl-primary-hover` | `#4338CA` | 主色悬停态 |
| `--ctrl-primary-active` | `#3730A3` | 主色激活态 |
| `--ctrl-primary-light` | `#EEF2FF` | 主色浅底，用于 Ghost 按钮悬停 |

### 功能色

| CSS 变量 | 默认值 | 用途 |
|----------|--------|------|
| `--ctrl-secondary` | `#6B7280` | 次级色 |
| `--ctrl-secondary-hover` | `#4B5563` | 次级色悬停 |
| `--ctrl-success` | `#10B981` | 成功色 |
| `--ctrl-warning` | `#F59E0B` | 警告色 |
| `--ctrl-danger` | `#EF4444` | 危险色 / 错误状态边框 |
| `--ctrl-info` | `#3B82F6` | 信息色 |

### 中性色

| CSS 变量 | 默认值 | 用途 |
|----------|--------|------|
| `--ctrl-bg` | `#FFFFFF` | 页面背景色 |
| `--ctrl-bg-secondary` | `#F9FAFB` | 次级背景（禁用态、卡片等） |
| `--ctrl-text` | `#111827` | 主文字色 |
| `--ctrl-text-secondary` | `#6B7280` | 次级文字色 |
| `--ctrl-text-disabled` | `#D1D5DB` | 禁用文字 / placeholder |
| `--ctrl-border` | `#E5E7EB` | 默认边框色 |
| `--ctrl-border-hover` | `#D1D5DB` | 边框悬停色 |

### 排版

| CSS 变量 | 默认值 | 用途 |
|----------|--------|------|
| `--ctrl-font-family` | 系统字体栈 | 全局字体族 |
| `--ctrl-font-size-sm` | `0.75rem` | 小号字体（Size::Sm） |
| `--ctrl-font-size-md` | `0.875rem` | 中号字体（Size::Md） |
| `--ctrl-font-size-lg` | `1rem` | 大号字体（Size::Lg） |

### 形状与动效

| CSS 变量 | 默认值 | 用途 |
|----------|--------|------|
| `--ctrl-radius-sm` | `0.25rem` | 小圆角 |
| `--ctrl-radius-md` | `0.375rem` | 中圆角（默认） |
| `--ctrl-radius-lg` | `0.5rem` | 大圆角 |
| `--ctrl-shadow-sm` | `0 1px 2px 0 rgba(0,0,0,0.05)` | 小阴影 |
| `--ctrl-shadow-md` | `0 4px 6px -1px rgba(0,0,0,0.1)` | 中阴影 |
| `--ctrl-transition` | `0.15s ease` | 过渡动效 |

---

## 7. 现有组件详解

### 7.1 Button

**文件：** `crates/ctrl-components/src/button/button.rs`

#### Props 清单

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `variant` | `Variant` | `Primary` | 按钮变体 |
| `size` | `Size` | `Md` | 按钮尺寸 |
| `disabled` | `bool` | `false` | 是否禁用 |
| `loading` | `bool` | `false` | 是否加载中（预留） |
| `block` | `bool` | `false` | 是否块级（宽度 100%） |
| `class` | `String` | `""` | 自定义 CSS 类名 |
| `style` | `String` | `""` | 自定义内联样式 |
| `r#type` | `String` | `"button"` | 原生 button type |
| `onclick` | `Option<EventHandler<MouseEvent>>` | `None` | 点击事件 |
| `children` | `Element` | — | 子元素 |

#### 变体样式映射

| 变体 | 背景 | 文字色 | 边框 | 悬停效果 |
|------|------|--------|------|---------|
| `Primary` | `--ctrl-primary` | `white` | 同背景色 | 背景切换为 `--ctrl-primary-hover` |
| `Secondary` | `--ctrl-secondary` | `white` | 同背景色 | 背景切换为 `--ctrl-secondary-hover` |
| `Outline` | `transparent` | `--ctrl-primary` | 同文字色 | 文字/边框切换为 `--ctrl-primary-hover` |
| `Ghost` | `transparent` | `--ctrl-primary` | `transparent` | 背景切换为 `--ctrl-primary-light` |

#### 尺寸映射

| 尺寸 | 高度 | 内边距 | 字体 |
|------|------|--------|------|
| `Sm` | `28px` | `4px 12px` | `--ctrl-font-size-sm` |
| `Md` | `36px` | `8px 16px` | `--ctrl-font-size-md` |
| `Lg` | `44px` | `12px 24px` | `--ctrl-font-size-lg` |

#### 内部状态

- `hovered: Signal<bool>` — 通过 `onmouseenter`/`onmouseleave` 事件切换

### 7.2 Input

**文件：** `crates/ctrl-components/src/input/input.rs`

#### Props 清单

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `size` | `Size` | `Md` | 输入框尺寸 |
| `value` | `String` | `""` | 当前值（受控组件） |
| `placeholder` | `String` | `""` | 占位文本 |
| `disabled` | `bool` | `false` | 是否禁用 |
| `readonly` | `bool` | `false` | 是否只读 |
| `error` | `bool` | `false` | 是否错误状态 |
| `r#type` | `String` | `"text"` | 原生 input type |
| `class` | `String` | `""` | 自定义 CSS 类名 |
| `style` | `String` | `""` | 自定义内联样式 |
| `oninput` | `Option<EventHandler<FormEvent>>` | `None` | 输入事件 |
| `onfocus` | `Option<EventHandler<FocusEvent>>` | `None` | 获得焦点 |
| `onblur` | `Option<EventHandler<FocusEvent>>` | `None` | 失去焦点 |

#### 状态样式映射

| 状态 | 边框色 | 额外样式 |
|------|--------|---------|
| 默认 | `--ctrl-border` | — |
| 聚焦 | `--ctrl-primary` | `box-shadow: 0 0 0 1px --ctrl-primary` |
| 错误 | `--ctrl-danger` | `box-shadow: 0 0 0 1px --ctrl-danger` |
| 禁用 | 保持边框 | `opacity: 0.5`, `background: --ctrl-bg-secondary` |

**状态优先级：** error > focus > default

#### 尺寸映射

| 尺寸 | 高度 | 内边距 | 字体 |
|------|------|--------|------|
| `Sm` | `28px` | `4px 10px` | `--ctrl-font-size-sm` |
| `Md` | `36px` | `6px 12px` | `--ctrl-font-size-md` |
| `Lg` | `44px` | `8px 16px` | `--ctrl-font-size-lg` |

#### 内部状态

- `focused: Signal<bool>` — 通过 `onfocusin`/`onfocusout` 事件切换

#### 受控组件模式

Input 采用**受控组件**模式，父组件负责管理 `value` 状态：

```rust
let mut value = use_signal(|| String::new());

Input {
    value: value(),
    oninput: move |evt: FormEvent| value.set(evt.value()),
}
```

### 7.3 Switch 开关

**文件：** `crates/ctrl-components/src/switch/switch.rs`

#### Props 清单

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `checked` | `bool` | `false` | 是否选中 |
| `disabled` | `bool` | `false` | 是否禁用 |
| `size` | `Size` | `Md` | 开关尺寸 |
| `onchange` | `Option<EventHandler<bool>>` | `None` | 状态变化事件，返回新值 |

#### 内部状态

- `checked: bool` — 由父组件通过 `checked` prop 控制，`onchange` 返回切换后的新值

#### 尺寸映射

| 尺寸 | 轨道尺寸 | 圆点尺寸 |
|------|---------|---------|
| `Sm` | `32×18px` | `14px` |
| `Md` | `40×22px` | `18px` |
| `Lg` | `48×26px` | `22px` |

### 7.4 Checkbox 复选框

**文件：** `crates/ctrl-components/src/checkbox/checkbox.rs`

#### Props 清单

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `checked` | `bool` | `false` | 是否选中 |
| `disabled` | `bool` | `false` | 是否禁用 |
| `indeterminate` | `bool` | `false` | 半选状态（优先级高于 checked） |
| `label` | `String` | `""` | 标签文本 |
| `onchange` | `Option<EventHandler<bool>>` | `None` | 状态变化事件 |

#### 图标渲染

- 选中时：白色对勾 SVG 图标
- 半选时：白色横条 SVG 图标
- 未选中时：无图标

### 7.5 Radio 单选框

**文件：** `crates/ctrl-components/src/radio/radio.rs`

#### Props 清单

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `checked` | `bool` | `false` | 是否选中 |
| `disabled` | `bool` | `false` | 是否禁用 |
| `value` | `String` | `""` | 单选项的值 |
| `label` | `String` | `""` | 标签文本 |
| `onchange` | `Option<EventHandler<String>>` | `None` | 选中变化事件，返回选中值 |

#### 选中样式

选中时边框变为 `4px solid var(--ctrl-primary)`，形成外圈实心效果。

### 7.6 Select 下拉选择

**文件：** `crates/ctrl-components/src/select/select.rs`

#### Props 清单

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `options` | `Vec<(String, String, bool)>` | `[]` | 选项列表 (值, 标签, 是否禁用) |
| `value` | `String` | `""` | 当前选中值 |
| `placeholder` | `String` | `"请选择"` | 占位文本 |
| `size` | `Size` | `Md` | 选择器尺寸 |
| `disabled` | `bool` | `false` | 是否禁用 |
| `onchange` | `Option<EventHandler<String>>` | `None` | 选中变化事件 |

#### 内部状态

- `open: Signal<bool>` — 下拉面板开/关状态
- 点击选项后自动关闭面板

### 7.7 Tag 标签

**文件：** `crates/ctrl-components/src/tag/tag.rs`

#### Props 清单

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `color` | `String` | `var(--ctrl-primary)` | 标签颜色（CSS 颜色值） |
| `closable` | `bool` | `false` | 是否可关闭 |
| `onclose` | `Option<EventHandler<()>>` | `None` | 关闭事件回调 |

#### 样式设计

标签使用传入的 `color` 同时设置文字色、背景色（透明度 8%）和边框色（透明度 19%），实现统一的色调。

### 7.8 Card 卡片

**文件：** `crates/ctrl-components/src/card/card.rs`

#### Props 清单

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `title` | `String` | `""` | 卡片标题 |
| `bordered` | `bool` | `true` | 是否显示边框 |
| `shadow` | `bool` | `false` | 是否带阴影 |
| `header` | `Option<Element>` | `None` | 自定义头部插槽（优先级高于 title） |

#### 布局

- 头部区域：可选的标题栏，由 `title` 或 `header` 控制内容
- 内容区域：`children` 传入的任意元素，内边距 20px

### 7.9 Dialog 对话框

**文件：** `crates/ctrl-components/src/dialog/dialog.rs`

#### Props 清单

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `visible` | `bool` | `false` | 是否显示 |
| `title` | `String` | `""` | 对话框标题 |
| `width` | `String` | `"480px"` | 对话框宽度 |
| `show_close` | `bool` | `true` | 是否显示右上角关闭按钮 |
| `mask_closable` | `bool` | `true` | 点击遮罩是否关闭 |
| `onclose` | `Option<EventHandler<()>>` | `None` | 关闭事件 |
| `footer` | `Option<Element>` | `None` | 底部操作区插槽 |

#### 层级结构

- 遮罩层：固定定位、半透明黑色背景、z-index 1000
- 弹窗主体：居中显示、白色背景、圆角、阴影
- 头部（可选）：标题 + 关闭按钮
- 内容区：可滚动
- 底部（可选）：操作按钮区域

### 7.10 Table 表格

**文件：** `crates/ctrl-components/src/table/table.rs`

#### Props 清单

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `columns` | `Vec<TableColumn>` | `[]` | 列定义 |
| `data` | `Vec<Vec<String>>` | `[]` | 行数据 |
| `striped` | `bool` | `false` | 是否显示斑马纹 |
| `bordered` | `bool` | `true` | 是否显示边框 |

#### TableColumn 属性

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `title` | `String` | `""` | 列标题 |
| `width` | `Option<String>` | `None` | 列宽 |
| `align` | `Option<String>` | `None` | 对齐方式 |

---

## 8. 扩展指南

### 8.1 添加新组件的步骤

假设要添加一个 `Switch` 组件：

**步骤 1：创建目录和文件**

```
crates/ctrl-components/src/switch/
├── mod.rs
└── switch.rs
```

**步骤 2：实现组件**

```rust
// switch.rs
use dioxus::prelude::*;
use ctrl_core::types::Size;

#[derive(Props, PartialEq, Clone)]
pub struct SwitchProps {
    #[props(default = false)]
    pub checked: bool,
    #[props(default = Size::default())]
    pub size: Size,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = "".to_string())]
    pub style: String,
    pub onchange: Option<EventHandler<FormEvent>>,
}

fn build_switch_style(size: Size, checked: bool, disabled: bool, custom_style: &str) -> String {
    // 构建样式字符串...
    String::new()
}

#[allow(non_snake_case)]
pub fn Switch(props: SwitchProps) -> Element {
    // 组件实现...
    todo!()
}
```

**步骤 3：注册导出**

```rust
// crates/ctrl-components/src/switch/mod.rs
pub mod switch;
pub use switch::{Switch, SwitchProps};

// crates/ctrl-components/src/lib.rs
pub mod switch;
pub use switch::{Switch, SwitchProps};

// crates/ctrl/src/lib.rs (prelude 模块)
pub use ctrl_components::{Switch, SwitchProps};
```

### 8.2 样式构建函数模板

```rust
fn build_xxx_style(
    // 从 Props 传入的参数
    variant: Variant,
    size: Size,
    // 内部状态
    hovered: bool,
    disabled: bool,
    // 用户自定义
    custom_style: &str,
) -> String {
    let mut styles: Vec<String> = vec![
        // 1. 布局样式
        "display: inline-flex".into(),
        "align-items: center".into(),
        // 2. 主题相关样式（使用 CSS 变量）
        "font-family: var(--ctrl-font-family)".into(),
        format!("font-size: {}", size.font_size_var()),
        "border-radius: var(--ctrl-radius-md)".into(),
        "transition: all var(--ctrl-transition)".into(),
        // 3. 尺寸相关
        format!("padding: {}", size.padding()),
        format!("height: {}", size.height()),
        // 4. 变体相关
        // ...
    ];

    // 5. 状态样式
    if disabled {
        styles.push("opacity: 0.5".into());
        styles.push("cursor: not-allowed".into());
    }

    // 6. 用户自定义样式（追加在最后）
    if !custom_style.is_empty() {
        styles.push(custom_style.to_string());
    }

    styles.join("; ")
}
```

### 8.3 何时使用 CSS 变量 vs 硬编码

| 场景 | 使用 CSS 变量 | 硬编码 |
|------|-------------|--------|
| 颜色值 | ✅ `var(--ctrl-primary)` | ❌ |
| 字体 | ✅ `var(--ctrl-font-family)` | ❌ |
| 圆角 | ✅ `var(--ctrl-radius-md)` | ❌ |
| 过渡动效 | ✅ `var(--ctrl-transition)` | ❌ |
| 布局属性（display, position） | ❌ | ✅ `display: inline-flex` |
| 盒模型（padding 由 size 决定） | ❌ | ✅ 通过 Size 方法获取 |
| 宽度/高度 | ❌ | ✅ 通过 Size 方法获取 |
| 光标样式 | ❌ | ✅ `cursor: pointer` |

**原则：** 可能与主题定制相关的值使用 CSS 变量，与主题无关的固定布局属性直接硬编码。

---

## 9. 设计决策与权衡

### 9.1 为什么使用内联样式而不是 CSS 文件

**决策：** 使用内联样式 + CSS 变量，不使用外部 CSS 文件。

**原因：**
1. **开箱即用：** 用户无需配置 CSS 构建工具（如 tailwind、postcss 等）
2. **零依赖：** 不需要额外的 CSS 处理工具链
3. **动态性：** 样式可以直接根据 Rust 状态动态计算，无需类名切换
4. **隔离性：** 每个组件的样式天然隔离，不会污染全局

**代价：**
1. 无法使用 `:hover`、`:focus` 等伪类 → 通过事件模拟解决
2. 生成的 HTML 内联样式较长 → 实际影响可忽略
3. 无法使用 CSS 动画关键帧 → 未来可扩展为注入全局 `<style>`

### 9.2 为什么 ColorPalette 使用 `&'static str`

**决策：** `ColorPalette` 的字段类型为 `&'static str` 而非 `String`。

**原因：**
1. 颜色值通常是编译期常量，不需要运行时分配
2. 避免不必要的内存分配和 clone 开销
3. `PartialEq` 比较更高效（指针比较 vs 字符串比较）

**代价：**
1. 用户无法使用运行时动态生成的颜色字符串 → 对于大多数场景这不是问题，因为颜色值在编译时就已经确定

### 9.3 为什么 ThemeProvider 放在 ctrl-core 而不是 ctrl-components

**决策：** `ThemeProvider` 放在 `ctrl-core` crate 中。

**原因：**
1. `ThemeProvider` 是主题系统的基础设施，而非具体 UI 组件
2. 它只依赖 `Theme` 和 `ColorPalette`（都在 `ctrl-core` 中），不依赖任何组件
3. 放在 `ctrl-core` 中允许其他 crate 独立使用主题系统而不引入组件依赖

### 9.4 为什么 EventHandler 使用 Option 包装

**决策：** 所有事件处理器使用 `Option<EventHandler<T>>` 类型。

**原因：**
1. 让事件处理器成为可选属性，用户只需传入需要的
2. 在 Dioxus 0.5 中，`EventHandler` 不能方便地设置默认值（需要 `noop` 工厂函数）
3. 使用 `Option` 更符合 Rust 惯例

**组件内部处理：**
```rust
let onclick = props.onclick.clone();
// ...
onclick: move |evt| {
    if let Some(ref handler) = onclick {
        handler.call(evt);
    }
},
```

### 9.5 为什么聚合层使用独立的 `ctrl` crate

**决策：** 创建独立的 `ctrl` crate 作为聚合导出层。

**原因：**
1. **用户体验：** 用户只需一个依赖 `ctrl = "0.1"`
2. **内部解耦：** 内部 crate 可以独立发布和版本管理
3. **prelude 模块：** 提供精心设计的 `ctrl::prelude`，用户用 `use ctrl::prelude::*` 即可导入所有常用项
4. **未来扩展：** 如果后续需要拆分更多 crate（如 `ctrl-icons`），只需在 `ctrl` 中添加 re-export 即可，用户无需修改依赖

### 9.6 Dioxus 版本选择

**决策：** 使用 Dioxus 0.5.x 系列。

**原因：**
1. 0.5 是成熟稳定版本，API 稳定
2. 生态兼容性最好
3. 后续可升级到 0.6+，但需要评估 API 变更

---

## 附录：文件清单

```
crates/ctrl-core/src/
├── lib.rs                          (7 行)    统一导出
├── theme/
│   ├── mod.rs                      (5 行)    主题模块导出
│   ├── colors.rs                   (62 行)   ColorPalette
│   ├── theme.rs                    (35 行)   Theme
│   └── provider.rs                 (109 行)  ThemeProvider + CSS 变量生成
├── types/
│   ├── mod.rs                      (4 行)    类型模块导出
│   ├── size.rs                     (58 行)   Size 枚举
│   └── variant.rs                  (22 行)   Variant 枚举
└── utils/
    ├── mod.rs                      (3 行)    工具模块导出
    └── cn.rs                       (16 行)   cn() 函数

crates/ctrl-components/src/
├── lib.rs                          (5 行)    组件统一导出
├── button/
│   ├── mod.rs                      (3 行)    按钮模块导出
│   └── button.rs                   (181 行)  Button 组件
└── input/
    ├── mod.rs                      (3 行)    输入框模块导出
    └── input.rs                    (167 行)  Input 组件

crates/ctrl/src/
└── lib.rs                          (58 行)   聚合导出 + prelude
```