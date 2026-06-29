# Ctrl UI

开箱即用的 **Dioxus Web** 组件库，提供 40+ 个高质量 UI 组件，覆盖表单、数据展示、反馈、导航等场景。

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Dioxus](https://img.shields.io/badge/Dioxus-0.7-blue.svg)](https://dioxuslabs.com)

---

## 特性

- **40+ 组件** — 覆盖通用、布局、表单、数据展示、反馈、导航六大类别
- **主题定制** — 通过 `ThemeProvider` 一键切换浅色/深色，或自定义完整色彩体系
- **CSS 零依赖** — 所有样式通过 `include_str!` 内嵌，无需额外加载 CSS 文件
- **双模式驱动** — 全部组件支持声明式（Declarative）用法；反馈类组件额外支持命令式（Imperative）用法，通过 `use_xxx()` Hook 在任意位置触发，无需在 rsx! 中声明组件
- **类型安全** — 基于 Rust 类型系统的 Props，编译期即可发现错误

---

## 快速开始

### 安装

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
ctrl = "0.1"
```

### 基础示例

```rust
use dioxus::prelude::*;
use ctrl::prelude::*;

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        ThemeProvider {
            div { style: "padding: 20px;",
                Button { variant: Variant::Primary, "Hello Ctrl" }
            }
        }
    }
}
```

### 接入命令式 API Provider（可选）

以下 Provider 只需包裹在根组件中，即可在任意子组件中通过 Hook 触发对应 UI：

```rust
fn App() -> Element {
    rsx! {
        ThemeProvider {
            NotificationProvider {
                placement: NotificationPlacement::TopRight,
                MessageProvider {
                    placement: MessagePlacement::Top,
                    ImagePreviewProvider {
                        DialogProvider {
                            DrawerProvider {
                                AlertBannerProvider {
                                    // 你的应用路由或页面
                                    Router::<Route> {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

---

## 声明式 & 命令式

Ctrl UI 所有组件均支持**声明式**（Declarative）用法——在 `rsx!` 中直接书写组件标签并传入 Props。

部分组件额外提供了**命令式**（Imperative）用法——通过 `Provider` + `use_xxx()` Hook，无需在 `rsx!` 中声明组件标签，直接在事件回调中调用 API。这类似于 Ant Design 的 `message.info()`、Element UI 的 `this.$message()` 或 React 的 Imperative Handle。

> **注意**：命令式组件的声明式用法仍然可用，两种模式可混用。命令式适用于"触发即忘"场景（如提示、弹窗），声明式适用于需要受控状态的场景。

### 支持命令式的组件

| 组件 | Hook 函数 | Provider | 示例 |
|------|----------|----------|------|
| **Notification** | `use_notification()` | `NotificationProvider` | `api.info("标题", "内容")` |
| **Message** | `use_message()` | `MessageProvider` | `api.success("操作成功！")` |
| **Dialog** | `use_dialog()` | `DialogProvider` | `api.open(DialogConfig { ... })` |
| **Drawer** | `use_drawer()` | `DrawerProvider` | `api.open(DrawerConfig { ... })` |
| **Alert Banner** | `use_alert_banner()` | `AlertBannerProvider` | `api.warning("警告", "描述")` |
| **Image Preview** | `use_image_preview()` | `ImagePreviewProvider` | `api.open(urls, 0)` |

### 使用方式

```rust
// 消息提示
let mut msg = use_message();
msg.success("保存成功！".to_string());
msg.error("网络请求失败".to_string());

// 对话框
let mut dialog = use_dialog();
dialog.open(DialogConfig {
    title: "确认删除".into(),
    content: "删除后不可恢复，确定继续？".into(),
    confirm_text: "确定".into(),
    cancel_text: "取消".into(),
    on_confirm: Some(EventHandler::new(|_| { /* 删除逻辑 */ })),
    ..Default::default()
});

// 抽屉
let mut drawer = use_drawer();
drawer.open(DrawerConfig {
    title: "详情".into(),
    content: "这里是侧边栏内容".into(),
    placement: "right".into(),
    ..Default::default()
});

// 全局横幅
let mut alert = use_alert_banner();
alert.success("操作成功".into(), "数据已提交".into());

// 图片预览
let mut preview = use_image_preview();
preview.open(vec!["img1.jpg".into(), "img2.jpg".into()], 0);
```

---

## 组件列表

### 通用组件

| 组件 | 说明 | 关键 Props |
|------|------|-----------|
| **Button** | 按钮，支持 4 种变体 + 3 种尺寸 | `variant`, `size`, `disabled`, `block`, `loading` |
| **Tag** | 标签，用于标记和分类 | `color`, `closable`, `size` |
| **Space** | 间距容器，统一子元素间距 | `gap`, `direction`, `wrap` |
| **Divider** | 分割线 | `direction`, `content` |

### 布局组件

| 组件 | 说明 | 关键 Props |
|------|------|-----------|
| **Card** | 卡片容器 | `title`, `shadow` |

### 表单组件

| 组件 | 说明 | 关键 Props |
|------|------|-----------|
| **Input** | 输入框 | `placeholder`, `value`, `r#type`, `error`, `size` |
| **InputNumber** | 数字输入框 | `value`, `min`, `max`, `step` |
| **Select** | 下拉选择器 | `value`, `options`, `placeholder` |
| **Switch** | 开关 | `checked`, `disabled` |
| **Checkbox** | 多选框 | `checked`, `indeterminate` |
| **Radio** | 单选框 | `checked`, `disabled` |
| **Slider** | 滑块 | `value`, `min`, `max`, `step`, `marks`, `show_label` |
| **Rate** | 评分 | `value`, `count`, `allow_half`, `icons`, `half_icons` |
| **DatePicker** | 日期选择器 | `value`, `placeholder` |
| **Upload** | 上传 | `files`, `accept`, `multiple` |
| **Form** | 表单容器 | `layout`, `onsubmit` |
| **Segmented** | 分段控制器 | `options`, `value` |

### 数据展示

| 组件 | 说明 | 关键 Props |
|------|------|-----------|
| **Table** | 表格 | `columns`, `data`, `striped`, `bordered` |
| **Badge** | 徽标 | `count`, `dot`, `color` |
| **Avatar** | 头像 | `src`, `size`, `shape` |
| **Image** | 图片 | `src`, `preview`, `preview_urls`, `fallback` |
| **Tag** | 标签 | `color`, `closable` |
| **Progress** | 进度条 | `percent`, `status`, `show_info` |
| **Tree** | 树形控件 | `data`, `selected_key`, `checkable` |
| **Skeleton** | 骨架屏 | `loading`, `rows` |
| **Empty** | 空状态 | `description`, `image` |
| **Timeline** | 时间线 | items |
| **Carousel** | 走马灯 | `autoplay`, `interval`, `effect` |

### 反馈组件

| 组件 | 说明 | 模式 |
|------|------|------|
| **Alert** | 警告提示 | 声明式 / 命令式 |
| **Notification** | 通知提醒 | 声明式 / 命令式 |
| **Message** | 全局提示 | 声明式 / 命令式 |
| **Dialog** | 对话框 | 声明式 / 命令式 |
| **Drawer** | 抽屉 | 声明式 / 命令式 |
| **Image** | 图片（预览） | 声明式 / 命令式 |
| **Tooltip** | 文字提示 | 声明式 |
| **Popover** | 气泡卡片 | 声明式 |
| **Loading** | 加载中 | 声明式 |

### 导航组件

| 组件 | 说明 | 关键 Props |
|------|------|-----------|
| **Tabs** | 标签页 | `active`, `onchange` |
| **Breadcrumb** | 面包屑 | items |
| **Pagination** | 分页 | `current`, `total`, `page_size` |
| **Steps** | 步骤条 | `current`, items |
| **Menu** | 菜单 | items |
| **Dropdown** | 下拉菜单 | `trigger` |
| **Backtop** | 回到顶部 | `visibility_height` |
| **Collapse** | 折叠面板 | `active_keys` |

---

## 共享类型

### Variant（语义变体）

```rust
#[derive(Default)]
pub enum Variant {
    Primary,   // 主要（默认）
    Secondary, // 次要
    Outline,   // 描边
    Ghost,     // 幽灵
}
```

### Size（组件尺寸）

```rust
#[derive(Default)]
pub enum Size {
    Sm,  // 小号
    Md,  // 中号（默认）
    Lg,  // 大号
}
```

### ColorPalette（色彩配置）

可通过 `ThemeProvider` 的 `theme` 属性覆盖全部 20 个颜色变量：

```rust
ThemeProvider {
    theme: Theme {
        colors: ColorPalette {
            primary: "#818CF8",
            bg: "#0F172A",
            text: "#F1F5F9",
            // ... 覆盖任意颜色
            ..Default::default()
        },
    },
}
```

完整颜色列表：`primary`, `primary_hover`, `primary_active`, `primary_light`, `secondary`, `secondary_hover`, `success`, `warning`, `danger`, `info`, `bg`, `bg_secondary`, `bg_disabled`, `text`, `text_secondary`, `text_disabled`, `border`, `border_hover`, `text_on_primary`, `mask_bg`。

---

## 项目结构

```
ctrl/
├── crates/
│   ├── ctrl-core/          # 核心层：主题、类型工具
│   │   └── src/
│   │       ├── theme/      # ThemeProvider、ColorPalette
│   │       ├── types/      # Variant、Size 等共享枚举
│   │       └── utils/      # cn() 等工具函数
│   ├── ctrl-components/    # 组件层：所有 UI 组件
│   │   ├── src/            # 组件 Rust 源码（每个组件一个目录）
│   │   └── assets/         # 组件 CSS 样式（内嵌编译）
│   └── ctrl/               # 聚合导出 crate（用户直接依赖）
└── examples/
    ├── demo/               # 演示项目
    └── docs-site/          # 官方文档站点
```

## 架构设计

### 组件模式

每个组件遵循统一的文件结构：

```
crates/ctrl-components/src/button/
├── mod.rs       # 模块声明 + 导出
├── button.rs    # 组件实现 + Props
```

CSS 样式通过 `include_str!("../../assets/button.css")` 在组件函数内内嵌输出，用户零配置。

### 命令式 API 模式

命令式 API 通过 Dioxus Context 实现，核心模式如下：

```
Provider 组件 ── use_context_provider ──→ 注入 API 到 Context
                                          ↓
子组件 ── use_context::<XxxAPI>() ──→ 获取 API, 调用 open/close 方法
                                          ↓
Provider 内监听 Signal 变化 ──→ 渲染对应的 UI 层
```

### Props 规范

- 所有 Props 提供合理默认值，最小化必须参数
- 事件回调统一使用 `Option<EventHandler<T>>` 类型
- 样式扩展通过 `class` 和 `style` 字段支持

---

## 常见问题与排查

### "Unable to retrieve the hook that was initialized at this index" panic

**现象**：在 Dioxus 0.7 中，路由页面（如 `/components/select` ↔ `/components/image`）切换时触发 panic：
```
panicked at dioxus-core-0.7.9/src/scope_context.rs:387:9:
Unable to retrieve the hook that was initialized at this index.
```

**关键原因**：Dioxus 0.7 的虚拟 DOM diff 算法在比较两次渲染的 VNode 时，依据 **Template（模板）** 是否相同来决定走哪条路径：
- `old.template == new.template` → **scope diff**（复用组件 scope，假设 hooks 数量/顺序不变）
- `old.template != new.template` → **full template replacement**（销毁旧 scope + 创建新 scope）

当 `match` 语句的不同分支**直接调用**不同组件函数（如 `select::SelectPage()` / `image::ImagePage()`），Dioxus 的 `#[component]` 宏可能为这些调用生成**相同的 VNode Template**。这导致切换页面时 diff 走 scope diff 路径，框架尝试在旧 scope（Select 的 hooks：`use_signal` ×4 + `use_effect` ×2 + `use_drop` 等）中运行新页面（Image 的 hooks：`use_signal` ×2 + `use_context` 等），hook 索引和类型完全不匹配，触发 `push_hook_value` 中的 panic。

**修复方案**：在每个 `match` 分支中使用独立的 `rsx! {}` 调用，确保每个分支产生**唯一的 VNode Template**，强制 Dioxus 走 full template replacement 路径：

```rust
// ❌ 错误写法 — 不同分支可能共享同一 Template
match name.as_str() {
    "select" => select::SelectPage(),
    "image" => image::ImagePage(),
}

// ✅ 正确写法 — 每个分支独立 rsx! {} 产生唯一 Template
match name.as_str() {
    "select" => rsx! { select::SelectPage {} },
    "image" => rsx! { image::ImagePage {} },
}
```

**原理总结**：`rsx! {}` 每次调用编译时生成独立的匿名模板类型。不同 `rsx!` 块之间 `template` 必然不同（`old.template != new.template`），diff 一开始就会走 replace 路径，彻底避免 scope 复用。

**排查步骤**（遇到类似 panic 时）：
1. 确认是否是页面/组件切换场景，新旧页面 hooks 数量或类型不同
2. 检查路由分发组件中是否使用了 `match` / `if-else` 直接返回组件调用（而不是 `rsx! {}` 包裹）
3. 如果使用了 `key` 属性仍无效，改用每个分支独立 `rsx! {}` 的方案
4. 可通过 `cargo expand` 查看 `#[component]` 展开后的 VNode Template 类型确认

---

## 浏览器兼容性

基于 Dioxus 0.7 + WebAssembly 构建，支持所有现代浏览器。

## License

MIT
