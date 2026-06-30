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
│       │   ├── mod.rs
│       │   └── input.rs       # Input 组件实现
│       ├── switch/            # Switch 开关
│       ├── checkbox/          # Checkbox 复选框
│       ├── radio/             # Radio 单选框
│       ├── select/            # Select 下拉选择
│       ├── tag/               # Tag 标签
│       ├── card/              # Card 卡片
│       ├── dialog/            # Dialog 对话框
│       ├── table/             # Table 表格
│       ├── textarea/          # Textarea 多行输入
│       ├── result/            # Result 结果页
│       ├── statistic/         # Statistic 统计数值
│       ├── descriptions/      # Descriptions 描述列表
│       ├── grid/              # Grid 栅格布局 (Row + Col)
│       └── ... (共 48 个组件目录)
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

### 4.1 核心思路：CSS 文件 + BEM 类名

Ctrl UI 采用 **CSS 文件 + BEM 类名** 方案，样式以独立的 `.css` 文件存在，通过 Dioxus 的 `include_str!()` 宏内联到组件中。

| 方案 | 优点 | 缺点 | 我们的选择 |
|------|------|------|-----------|
| 外部 CSS 文件 | 完整的伪类支持 | 需要构建工具配置 | 通过 include_str! 内联解决 |
| CSS-in-JS 库 | 运行时动态样式 | 增加依赖 | ❌ |
| 纯内联样式 | 零配置 | 无伪类/动画支持 | ❌ |
| CSS 文件 + BEM + include_str! | 伪类/动画 + 零外部依赖 | 运行时注入 style 标签 | ✅ 采用 |

### 4.2 CSS 文件组织结构

每个组件对应一个独立的 CSS 文件，位于 `crates/ctrl-components/assets/`：

```
assets/
├── button.css
├── input.css
├── switch.css
├── checkbox.css
├── radio.css
├── select.css
├── tag.css
├── card.css
├── dialog.css
├── drawer.css
├── table.css
├── carousel.css
├── collapse.css
├── tree.css
├── form.css
├── date-picker.css
├── segmented.css
├── skeleton.css
├── ...  (共 48 个组件 CSS 文件)
└── theme.css            # ThemeProvider 注入的全局 CSS 变量
```

### 4.3 BEM 命名规范

所有 CSS 类名遵循 **BEM (Block-Element-Modifier)** 约定：

```
.ctrl-{component}                      # Block（块）
.ctrl-{component}__{element}           # Element（元素）
.ctrl-{component}--{modifier}          # Modifier（修饰符）
.ctrl-{component}__{element}--{modifier}  # 元素修饰符
```

**示例（Button）：**
```css
.ctrl-button { }                          /* 块 */
.ctrl-button__icon { }                    /* 元素 */
.ctrl-button--primary { }                 /* Primary 变体修饰符 */
.ctrl-button--sm { }                     /* 尺寸修饰符 */
.ctrl-button__icon--loading { }           /* 加载中的图标 */
```

### 4.4 CSS 注入方式

组件通过 `include_str!()` 在编译期将 CSS 内容嵌入，然后在运行时通过 `<style>` 标签注入：

```rust
#[allow(non_snake_case)]
pub fn Button(props: ButtonProps) -> Element {
    const CSS: &str = include_str!("../../assets/button.css");
    rsx! {
        style { {CSS} }
        button { class: "ctrl-button ctrl-button--primary", "按钮" }
    }
}
```

**优点：**
- 编译期嵌入，零运行时 I/O
- 每个页面只注入访问过的组件的 CSS（按需加载）
- 享受完整的 CSS 伪类（`:hover`、`:focus`、`:disabled`）、CSS 动画（`@keyframes`）、CSS 变量等特性
- 天然支持 `@media` 响应式查询

### 4.5 主题定制 & CSS 变量

`ThemeProvider` 在 `<style>` 标签中注入全局 CSS 变量到 `:root`：

```css
:root {
    --ctrl-primary: #4F46E5;
    --ctrl-primary-hover: #4338CA;
    --ctrl-bg: #FFFFFF;
    --ctrl-text: #111827;
    --ctrl-border: #E5E7EB;
    --ctrl-font-family: system-ui, -apple-system, sans-serif;
    --ctrl-font-size-sm: 0.75rem;
    --ctrl-font-size-md: 0.875rem;
    --ctrl-font-size-lg: 1rem;
    --ctrl-radius-sm: 0.25rem;
    --ctrl-radius-md: 0.375rem;
    --ctrl-radius-lg: 0.5rem;
    --ctrl-transition: 0.15s ease;
    /* ... 共 27 个 CSS 变量 */
}
```

各组件 CSS 文件引用这些变量，实现全局一致的视觉体系。用户可通过覆盖 CSS 变量自定义主题。

### 4.6 受控/非受控组件规范

所有表单类组件统一采用**受控模式**：prop 驱动，父组件通过 `use_signal` 管理状态。

```rust
let mut value = use_signal(|| String::new());

Input {
    value: value(),
    oninput: move |evt: FormEvent| value.set(evt.value()),
}
```

内部通过 `use_effect(use_reactive(...))` 确保外部 prop 变更时同步内部状态：

```rust
let mut internal = use_signal(|| props.value);
use_effect(use_reactive(&props.value, move |v| {
    internal.set(v);
}));
```

---

## 5. 组件设计模式

### 5.1 组件结构规范

每个组件遵循统一的结构：

```
┌─────────────────────────────────────────────────┐
│  Props 结构体（#[derive(Props, PartialEq, Clone)]）  │
│  ├── 功能属性（variant, size, disabled 等）             │
│  ├── 样式属性（class, style）                          │
│  ├── 事件属性（onclick, onchange 等）                  │
│  └── 子元素（children: Element）                      │
├─────────────────────────────────────────────────┤
│  组件函数（#[allow(non_snake_case)]）                  │
│  ├── include_str! 嵌入 CSS 文件                       │
│  ├── use_signal 管理内部状态                          │
│  ├── use_effect 同步外部 prop → 内部信号               │
│  ├── BEM 类名拼接（基于 variant / size / state）       │
│  └── rsx! 渲染（style → HTML → BEM 类名）             │
└─────────────────────────────────────────────────┘
```

### 5.2 Props 设计约定

**通用属性（每个组件都应包含）：**
- `class: String` — 自定义 CSS 类名，默认空字符串
- `style: String` — 自定义内联样式，追加到组件默认样式之后
- `size: Size` — 尺寸控制，默认 `Size::Md`（部分组件）

**可选通用属性：**
- `disabled: bool` — 禁用状态
- `variant: Variant` — 语义变体

**事件属性：**
- 使用 `Option<EventHandler<T>>` 类型，默认 `None`
- 在组件函数中 `.clone()` 后移入闭包
- 命名统一：`onchange` 传递新值，`onclose` 传递关闭事件

### 5.3 受控组件模式

所有表单类组件（Input、Switch、Select、Slider 等）统一采用受控模式：

```rust
// 父组件管理状态
let mut checked = use_signal(|| false);

Switch {
    checked: checked(),
    onchange: move |val: bool| checked.set(val),
}
```

对于需要内部状态缓冲的组件，使用 `use_effect(use_reactive(...))` 同步外部 prop：

```rust
let mut internal = use_signal(|| props.value);
use_effect(use_reactive(&props.value, move |v| {
    internal.set(v);
}));
```

### 5.4 命令式 API 模式

部分组件（Dialog、Drawer、Notification、Message、Alert）提供命令式调用接口，采用统一的 **Provider + Context + API** 模式：

```
┌──────────────────────────────────┐
│  XxxProvider (Provider 组件)     │
│  ├── 渲染命令式 Xxx 实例         │
│  ├── use_context_provider API    │
│  └── 管理 visible + config 状态  │
├──────────────────────────────────┤
│  XxxConfig (配置结构体)          │
│  ├── title, content, ...         │
│  └── onconfirm, onclose 等回调  │
├──────────────────────────────────┤
│  XxxAPI (Context 类型)           │
│  ├── open(config) → 打开并配置   │
│  └── close() → 关闭             │
├──────────────────────────────────┤
│  use_xxx() → Hook                │
│  子组件调用获取 XxxAPI 引用      │
└──────────────────────────────────┘
```

**使用示例（Dialog）：**
```rust
DialogProvider {
    // 子组件中：
    let mut dialog = use_dialog();
    dialog.open(DialogConfig {
        title: "确认删除".into(),
        content: rsx! { p { "确定要删除吗？" } },
        onconfirm: Some(EventHandler::new(|_| { /* 删除 */ })),
        ..Default::default()
    });
}
```

### 5.5 父子上下文通信模式

对于需要父组件向子组件传递运行时状态的场景（Steps/Step、Carousel/CarouselSlide），使用 Dioxus Context：

```rust
// 父组件注入
struct MyCtx { active: Signal<usize>, ... }
use_context_provider(|| MyCtx { ... });

// 子组件读取
let ctx = use_context::<MyCtx>();
```

### 5.6 Overlay 生命周期管理

弹出层组件（Select、Popover、Dropdown、DatePicker）使用 `overlay.rs` 辅助模块统一管理：
- `setup_click_outside()` — 绑定 click-outside 监听器
- `OverlayClosures` — 存储事件闭包，随组件 drop 时自动 cleanup
- `use_drop` — Dioxus 生命周期钩子，组件卸载时移除 DOM 监听器

### 5.7 命名规范

| 元素 | 规范 | 示例 |
|------|------|------|
| 组件函数 | PascalCase + `#[allow(non_snake_case)]` | `pub fn Button(...)` |
| Props 结构体 | PascalCase + `Props` 后缀 | `ButtonProps` |
| Config 结构体 | PascalCase + `Config` 后缀 | `DialogConfig` |
| API 类型 | PascalCase + `API` 后缀 | `DialogAPI` |
| Provider 组件 | PascalCase + `Provider` 后缀 | `DialogProvider` |
| 模块文件 | 小写 + 下划线 | `button.rs` |
| CSS 文件 | 小写 + 连字符 | `button.css` |
| CSS 类名 | `ctrl-` 前缀 + BEM | `ctrl-button--primary` |

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

假设要添加一个 `Avatar` 组件：

**步骤 1：创建目录和文件**

```
crates/ctrl-components/src/avatar/
├── mod.rs
└── avatar.rs

crates/ctrl-components/assets/
└── avatar.css
```

**步骤 2：编写 CSS 文件（avatar.css）**

```css
.ctrl-avatar {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--ctrl-bg-secondary, #f0f0f0);
    color: var(--ctrl-text, #333);
    font-size: var(--ctrl-font-size-md, 14px);
    overflow: hidden;
}

.ctrl-avatar--sm { width: 32px; height: 32px; }
.ctrl-avatar--md { width: 40px; height: 40px; }
.ctrl-avatar--lg { width: 48px; height: 48px; }

.ctrl-avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
}
```

**步骤 3：实现组件（avatar.rs）**

```rust
use dioxus::prelude::*;
use ctrl_core::types::Size;

#[derive(Props, PartialEq, Clone)]
pub struct AvatarProps {
    #[props(default = "".to_string())]
    pub src: String,
    #[props(default = "".to_string())]
    pub alt: String,
    #[props(default = Size::default())]
    pub size: Size,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[allow(non_snake_case)]
pub fn Avatar(props: AvatarProps) -> Element {
    const CSS: &str = include_str!("../../assets/avatar.css");
    let size_class = match props.size {
        Size::Sm => "ctrl-avatar--sm",
        Size::Md => "ctrl-avatar--md",
        Size::Lg => "ctrl-avatar--lg",
    };
    let class = if props.class.is_empty() {
        format!("ctrl-avatar {}", size_class)
    } else {
        format!("ctrl-avatar {} {}", size_class, props.class)
    };

    rsx! {
        style { {CSS} }
        div { class: "{class}",
            if props.src.is_empty() {
                {props.children}
            } else {
                img { src: "{props.src}", alt: "{props.alt}" }
            }
        }
    }
}
```

**步骤 4：注册导出**

```rust
// crates/ctrl-components/src/avatar/mod.rs
pub mod avatar;
pub use avatar::{Avatar, AvatarProps};

// crates/ctrl-components/src/lib.rs
pub mod avatar;
pub use avatar::{Avatar, AvatarProps};

// crates/ctrl/src/lib.rs
pub use ctrl_components::{Avatar, AvatarProps};
```

### 8.2 样式指南

**CSS 变量（必须使用）：**
- 颜色类：`var(--ctrl-primary)`、`var(--ctrl-bg)`、`var(--ctrl-text)`、`var(--ctrl-border)` 等
- 尺寸类：`var(--ctrl-font-size-sm)`、`var(--ctrl-radius-md)` 等
- 动效类：`var(--ctrl-transition)`
- 始终提供 fallback：`var(--ctrl-border, #d9d9d9)`

**BEM 命名（必须遵循）：**
```css
.ctrl-{component}           /* 块，最外层元素 */
.ctrl-{component}__{el}     /* 内部元素 */
.ctrl-{component}--{mod}    /* 变体/状态修饰符 */
```

**直接硬编码（可以接受）：**
- 布局属性：`display: flex`、`position: relative`
- 盒模型固定值：不需要定制化的 `width` / `height`
- 光标：`cursor: pointer`
- z-index：如 `z-index: 1000` 用于遮罩层

---

## 9. 设计决策与权衡

### 9.1 为什么使用 CSS 文件 + BEM 类名

**决策：** 使用独立的 CSS 文件 + BEM 命名 + `include_str!()` 内联，不使用内联样式。

**原因：**
1. **完整的 CSS 特性支持：** `:hover`、`:focus`、`:disabled`、`@keyframes`、`@media` 查询等，无需事件模拟
2. **开箱即用：** `include_str!()` 编译期嵌入，用户无需配置 CSS 构建工具
3. **BEM 隔离性：** `.ctrl-{component}` 前缀保证样式不会污染全局，也不被外部样式意外覆盖
4. **CSS 变量主题：** 所有组件的颜色/字号/圆角统一通过 `var(--ctrl-xxx)` 引用，ThemeProvider 注入全局变量即可实现动态主题
5. **响应式支持：** 可直接在 CSS 中编写 `@media` 查询，无需 Rust 端判断视口尺寸

**代价：**
1. 每个组件渲染时注入一个 `<style>` 标签 → 标签数量 = 渲染的组件数量，生产中可合并优化
2. CSS 内容在编译后内联到 wasm binary → 增大 binary 体积，但 48 个 CSS 文件合计约 55KB（gzip 后更小）

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
2. 在 Dioxus 0.7 中，`EventHandler` 不能方便地设置默认值（需要 `noop` 工厂函数）
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

**决策：** 使用 Dioxus 0.7 系列。

**原因：**
1. 0.7 是当前最新稳定版本，API 成熟
2. 原生支持 `use_signal`、`Signal::new` 等现代响应式 API
3. 路由、文档系统等生态完善

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