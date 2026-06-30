# Ctrl UI 组件库 —— 全组件技术文档

> 基于 Dioxus 0.7，共 **47 个组件**，覆盖基础、数据录入、数据展示、反馈、导航、布局六大类。

---

## 目录

- [核心基础设施](#核心基础设施)
- [基础组件](#基础组件)
- [数据录入组件](#数据录入组件)
- [数据展示组件](#数据展示组件)
- [反馈组件](#反馈组件)
- [导航组件](#导航组件)
- [布局组件](#布局组件)

---

## 核心基础设施

### ThemeProvider

**文件：** `ctrl-core/src/theme/provider.rs`

主题系统核心组件，负责注入 CSS 变量和提供 Context。

**技术实现：**
- 通过 `build_css_vars()` 将 `Theme` 转为 CSS 变量字符串
- 同时注入浅色 (`:root`) 和深色 (`[data-theme="dark"]`) 两套变量
- 深色色板可通过 `ColorPalette::dark()` 自动从浅色推算
- 使用 `use_context_provider(|| theme)` 向子组件传递主题
- 注入 `reset.css` 全局重置样式

**注入的 CSS 变量（35 个）：**

| 分类 | 变量 |
|------|------|
| 品牌色 | `--ctrl-primary`, `--ctrl-primary-hover`, `--ctrl-primary-active`, `--ctrl-primary-light` |
| 功能色 | `--ctrl-secondary`, `--ctrl-secondary-hover`, `--ctrl-success`, `--ctrl-warning`, `--ctrl-danger`, `--ctrl-info` |
| 中性色 | `--ctrl-bg`, `--ctrl-bg-secondary`, `--ctrl-bg-disabled`, `--ctrl-text`, `--ctrl-text-secondary`, `--ctrl-text-disabled`, `--ctrl-border`, `--ctrl-border-hover`, `--ctrl-text-on-primary`, `--ctrl-mask-bg` |
| 排版 | `--ctrl-font-family`, `--ctrl-font-size-xs/sm/md/lg` |
| 间距 | `--ctrl-spacing-xs/sm/md/lg/xl` |
| 形状 | `--ctrl-radius-sm/md/lg`, `--ctrl-shadow-sm/md` |
| 动效 | `--ctrl-transition` |

### ColorPalette

**文件：** `ctrl-core/src/theme/colors.rs`

色板结构体，所有颜色值为 `&'static str`。提供 `dark()` 方法自动从浅色色板推算深色色板（基于相对亮度算法）。

### Theme

**文件：** `ctrl-core/src/theme/theme.rs`

主题配置结构体，包含 `colors`、`dark_colors: Option<ColorPalette>`、字体、间距、圆角、阴影、过渡等配置。

### Size 枚举

**文件：** `ctrl-core/src/types/size.rs`

`Sm | Md | Lg`，提供 `height()`, `padding()`, `font_size_var()`, `input_padding()` 方法。

### Variant 枚举

**文件：** `ctrl-core/src/types/variant.rs`

`Primary | Secondary | Outline | Ghost`，实现 `Display` 用于 CSS 类名拼接。

### cn() 工具函数

**文件：** `ctrl-core/src/utils/cn.rs`

合并 class 名称，过滤空字符串。`cn(&["btn", "btn-primary", ""])` → `"btn btn-primary"`。

### 颜色工具

**文件：** `ctrl-core/src/utils/color.rs`

`hex_to_rgb`, `rgb_to_hex`, `relative_luminance`, `lighten`, `darken` —— 用于深色模式自动推算。

---

## 基础组件

### Button

**文件：** `ctrl-components/src/button/button.rs`（108 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `variant` | `Variant` | `Primary` | 按钮变体 |
| `size` | `Size` | `Md` | 尺寸 |
| `disabled` | `bool` | `false` | 禁用 |
| `loading` | `bool` | `false` | 加载中 |
| `block` | `bool` | `false` | 块级（width 100%） |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `r#type` | `String` | `"button"` | 原生 type |
| `onclick` | `Option<EventHandler<MouseEvent>>` | `None` | 点击事件 |
| `children` | `Element` | — | 子元素 |

**技术实现：** `build_button_class()` 根据 variant/size/disabled/block 拼接 CSS 类名。CSS 通过 `include_str!("../../assets/button.css")` 嵌入。

### Input

**文件：** `ctrl-components/src/input/input.rs`（124 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `size` | `Size` | `Md` | 尺寸 |
| `value` | `String` | `""` | 当前值（受控） |
| `placeholder` | `String` | `""` | 占位文本 |
| `disabled` | `bool` | `false` | 禁用 |
| `readonly` | `bool` | `false` | 只读 |
| `error` | `bool` | `false` | 错误状态 |
| `r#type` | `String` | `"text"` | 原生 type |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `oninput` | `Option<EventHandler<FormEvent>>` | `None` | 输入事件 |
| `onfocus` | `Option<EventHandler<FocusEvent>>` | `None` | 获得焦点 |
| `onblur` | `Option<EventHandler<FocusEvent>>` | `None` | 失去焦点 |

**技术实现：** 受控组件模式，`build_input_class()` 根据 size/disabled/error/readonly 拼接类名。使用 `onfocusin`/`onfocusout` 事件。

### Switch

**文件：** `ctrl-components/src/switch/switch.rs`（87 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `checked` | `bool` | `false` | 是否选中 |
| `disabled` | `bool` | `false` | 禁用 |
| `size` | `Size` | `Md` | 尺寸 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onchange` | `Option<EventHandler<bool>>` | `None` | 状态变化事件 |

**技术实现：** 使用 `role="switch"` + `aria_checked` 实现无障碍。点击时取反 checked 值并通过 onchange 返回新值。

### Checkbox

**文件：** `ctrl-components/src/checkbox/checkbox.rs`（110 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `checked` | `bool` | `false` | 是否选中 |
| `disabled` | `bool` | `false` | 禁用 |
| `indeterminate` | `bool` | `false` | 半选状态 |
| `label` | `String` | `""` | 标签文本 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onchange` | `Option<EventHandler<bool>>` | `None` | 状态变化事件 |

**技术实现：** 内联 SVG 图标（对勾/横条），`stop_propagation()` 防止 label 冒泡。半选优先级高于 checked。

### Radio

**文件：** `ctrl-components/src/radio/radio.rs`（80 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `checked` | `bool` | `false` | 是否选中 |
| `disabled` | `bool` | `false` | 禁用 |
| `value` | `String` | `""` | 单选值 |
| `label` | `String` | `""` | 标签文本 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onchange` | `Option<EventHandler<String>>` | `None` | 选中变化事件 |

**技术实现：** 选中时圆圈填充 `var(--ctrl-primary)`。onchange 返回选中的 value。

### Select

**文件：** `ctrl-components/src/select/select.rs`（160 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `size` | `Size` | `Md` | 尺寸 |
| `placeholder` | `String` | `"请选择"` | 占位文本 |
| `value` | `String` | `""` | 当前选中值 |
| `disabled` | `bool` | `false` | 禁用 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onchange` | `Option<EventHandler<String>>` | `None` | 选中变化事件 |
| `options` | `Vec<(String, String, bool)>` | `[]` | 选项 (value, label, disabled) |

**技术实现：** 内部维护 `open: Signal<bool>` 控制下拉面板。点击选项后自动关闭面板。SVG 箭头图标。

### Tag

**文件：** `ctrl-components/src/tag/tag.rs`（81 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `color` | `String` | `"var(--ctrl-primary)"` | 标签颜色 |
| `closable` | `bool` | `false` | 是否可关闭 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onclose` | `Option<EventHandler<()>>` | `None` | 关闭事件 |
| `children` | `Element` | — | 子元素 |

**技术实现：** 使用 `use_signal(|| true)` 控制可见性。颜色通过 CSS 变量传递，背景色为 `color + 14`（8% 透明度），边框为 `color + 30`（19% 透明度）。

### Card

**文件：** `ctrl-components/src/card/card.rs`（76 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `title` | `String` | `""` | 卡片标题 |
| `bordered` | `bool` | `true` | 显示边框 |
| `shadow` | `bool` | `false` | 带阴影 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `header` | `Option<Element>` | `None` | 自定义头部插槽 |
| `children` | `Element` | — | 卡片内容 |

**技术实现：** header 插槽优先级高于 title。

---

## 数据录入组件

### Slider

**文件：** `ctrl-components/src/slider/slider.rs`（459 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `i32` | `0` | 当前值 |
| `min` | `i32` | `0` | 最小值 |
| `max` | `i32` | `100` | 最大值 |
| `step` | `i32` | `1` | 步长 |
| `disabled` | `bool` | `false` | 禁用 |
| `vertical` | `bool` | `false` | 垂直方向 |
| `marks` | `bool` | `false` | 显示刻度标记 |
| `show_label` | `bool` | `false` | 显示数值标签 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onchange` | `Option<EventHandler<i32>>` | `None` | 值变化回调 |

**技术实现：** 最复杂的交互组件之一。使用 `wasm_bindgen` 直接操作 DOM：
- `begin_drag()` 绑定全局 mousemove/mouseup/touchmove/touchend 事件
- `update_dom_track()` 直接设置 track 和 handle 的 CSS 样式实现即时响应
- `calc_slider_value()` 将鼠标位置映射为滑块值（支持垂直反转）
- 使用 `Closure::forget()` 保持事件监听器存活
- 支持触摸事件

### Rate

**文件：** `ctrl-components/src/rate/rate.rs`（266 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `f64` | `0.0` | 当前分值 |
| `count` | `i32` | `5` | 星星总数 |
| `allow_half` | `bool` | `false` | 允许半星 |
| `readonly` | `bool` | `false` | 只读 |
| `disabled` | `bool` | `false` | 禁用 |
| `show_text` | `bool` | `false` | 显示分值文字 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onchange` | `Option<EventHandler<f64>>` | `None` | 值变化回调 |
| `icon_full` | `Option<String>` | `None` | 自定义全选图标 URL |
| `icon_half` | `Option<String>` | `None` | 自定义半选图标 URL |
| `icon_empty` | `Option<String>` | `None` | 自定义未选图标 URL |

**技术实现：** 半星模式将每颗星分为左右两个热区（`ctrl-rate__star-half-zone` / `ctrl-rate__star-full-zone`），通过 mouseenter/mouseleave 事件精确选中 0.5 或 1.0。支持自定义图标 URL。

### InputNumber

**文件：** `ctrl-components/src/input_number/input_number.rs`（174 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `i64` | `0` | 当前值 |
| `min` | `Option<i64>` | `None` | 最小值 |
| `max` | `Option<i64>` | `None` | 最大值 |
| `step` | `i64` | `1` | 步长 |
| `size` | `Size` | `Md` | 尺寸 |
| `disabled` | `bool` | `false` | 禁用 |
| `placeholder` | `String` | `""` | 占位文字 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onchange` | `Option<EventHandler<i64>>` | `None` | 值变化回调 |

**技术实现：** 双信号管理：`value: Signal<i64>` 存数值，`input_value: Signal<String>` 存输入框显示文本。允许空字符串和负号前缀，失焦时修正显示值。`clamp()` 函数限制范围。

### Upload

**文件：** `ctrl-components/src/upload/upload.rs`（338 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `files` | `Vec<UploadFile>` | `[]` | 文件列表 |
| `accept` | `String` | `""` | 接受的文件类型 |
| `multiple` | `bool` | `false` | 允许多选 |
| `disabled` | `bool` | `false` | 禁用 |
| `drag` | `bool` | `false` | 拖拽上传 |
| `max_size` | `u64` | `0` | 单文件大小限制（字节） |
| `tip` | `String` | `""` | 提示文字 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onchange` | `Option<EventHandler<Vec<UploadFile>>>` | `None` | 文件选择回调 |
| `onremove` | `Option<EventHandler<usize>>` | `None` | 文件移除回调 |
| `onerror` | `Option<EventHandler<String>>` | `None` | 校验失败回调 |
| `children` | `Element` | — | 触发元素 |

**技术实现：** 使用 `#[cfg(target_arch = "wasm32")]` 条件编译。拖拽上传通过 `ondrop`/`ondragover`/`ondragleave` 事件。文件选择通过隐藏 `<input type="file">`。支持文件大小校验。

### DatePicker

**文件：** `ctrl-components/src/date_picker/date_picker.rs`（330 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `String` | `""` | 选中日期 (YYYY-MM-DD) |
| `placeholder` | `String` | `"请选择日期"` | 占位文本 |
| `disabled` | `bool` | `false` | 禁用 |
| `clearable` | `bool` | `true` | 可清除 |
| `format` | `String` | `"YYYY-MM-DD"` | 日期格式 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onchange` | `Option<EventHandler<String>>` | `None` | 值变化回调 |

**技术实现：** 纯 Rust 实现的日历逻辑。`days_in_month()` 计算每月天数（含闰年），`first_day_of_week()` 使用 Zeller 公式计算每月第一天是星期几。日历网格固定 7×6=42 格。使用 `web_sys::js_sys::Date` 获取当前日期。

### Form / FormItem

**文件：** `ctrl-components/src/form/form.rs`（125 行）

| Form 属性 | 类型 | 默认值 | 说明 |
|-----------|------|--------|------|
| `layout` | `String` | `"vertical"` | 布局方式 (vertical/horizontal/inline) |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onsubmit` | `Option<EventHandler<Rc<FormData>>>` | `None` | 提交回调 |
| `children` | `Element` | — | 子元素 |

| FormItem 属性 | 类型 | 默认值 | 说明 |
|---------------|------|--------|------|
| `label` | `String` | `""` | 标签文本 |
| `required` | `bool` | `false` | 必填 |
| `help` | `String` | `""` | 帮助文本 |
| `error` | `String` | `""` | 错误信息 |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | 表单控件 |

**技术实现：** Form 使用原生 `<form>` 元素，`prevent_default()` 阻止默认提交。FormItem 优先显示 error，其次显示 help。

### Segmented

**文件：** `ctrl-components/src/segmented/segmented.rs`（109 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `String` | `""` | 当前选中值 |
| `options` | `Vec<(String, String)>` | — | 选项列表 (label, value) |
| `size` | `Size` | `Md` | 尺寸 |
| `block` | `bool` | `false` | 占满宽度 |
| `disabled` | `bool` | `false` | 禁用 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onchange` | `Option<EventHandler<String>>` | `None` | 值变化回调 |

---

## 数据展示组件

### Table

**文件：** `ctrl-components/src/table/table.rs`（128 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `columns` | `Vec<TableColumn>` | `[]` | 列定义 |
| `data` | `Vec<Vec<String>>` | `[]` | 行数据 |
| `striped` | `bool` | `false` | 斑马纹 |
| `bordered` | `bool` | `true` | 边框 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |

**TableColumn：** `title: String`, `width: Option<String>`, `align: Option<String>`

**技术实现：** 使用 `.iter().map()` 生成表头和行。斑马纹通过 `row_idx % 2 == 1` 判断。数据列数超过 columns 数时截断。

### Badge

**文件：** `ctrl-components/src/badge/badge.rs`（87 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `count` | `String` | `""` | 徽标内容 |
| `dot` | `bool` | `false` | 小圆点模式 |
| `max` | `u32` | `99` | 最大显示数字 |
| `color` | `String` | `"var(--ctrl-danger)"` | 徽标颜色 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `children` | `Element` | — | 包裹的容器 |

**技术实现：** 数字溢出时显示 `{max}+`。使用 `<sup>` 标签定位徽标。

### Avatar

**文件：** `ctrl-components/src/avatar/avatar.rs`（92 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `src` | `String` | `""` | 图片地址 |
| `alt` | `String` | `""` | 替代文字 |
| `size` | `Size` | `Md` | 尺寸 |
| `shape` | `String` | `"circle"` | 形状 (circle/square) |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `children` | `Element` | — | 无 src 时的 fallback 内容 |

**技术实现：** 有 src 时渲染 `<img>`，无 src 时渲染 children 作为文字头像。

### Progress

**文件：** `ctrl-components/src/progress/progress.rs`（73 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `percent` | `f64` | `0.0` | 进度值 0-100 |
| `color` | `String` | `"var(--ctrl-primary)"` | 进度条颜色 |
| `show_text` | `bool` | `false` | 显示百分比文字 |
| `height` | `u32` | `8` | 进度条高度 (px) |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |

**技术实现：** 使用 CSS 变量 `--ctrl-progress-color` 传递颜色。percent 通过 `clamp(0.0, 100.0)` 限制范围。

### Tabs

**文件：** `ctrl-components/src/tabs/tabs.rs`（148 行）

**组件族：** `Tabs`, `Tab`, `TabNav`, `TabContent`

| Tabs 属性 | 类型 | 默认值 | 说明 |
|-----------|------|--------|------|
| `active` | `String` | `"0"` | 当前激活 tab key |
| `onchange` | `Option<EventHandler<String>>` | `None` | 切换回调 |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | Tab 组件 |

| Tab 属性 | 类型 | 默认值 | 说明 |
|----------|------|--------|------|
| `tab_key` | `String` | `""` | 唯一标识 |
| `title` | `String` | `""` | 标题文字 |
| `disabled` | `bool` | `false` | 禁用 |
| `children` | `Element` | — | 面板内容 |

| TabNav 属性 | 类型 | 默认值 | 说明 |
|-------------|------|--------|------|
| `items` | `Vec<(String, String, bool)>` | `[]` | 标签项 (key, title, disabled) |
| `active` | `String` | `"0"` | 当前激活 key |
| `onchange` | `Option<EventHandler<String>>` | `None` | 切换回调 |
| `class` | `String` | `""` | 自定义类名 |

### Breadcrumb

**文件：** `ctrl-components/src/breadcrumb/breadcrumb.rs`（80 行）

| Breadcrumb 属性 | 类型 | 默认值 | 说明 |
|-----------------|------|--------|------|
| `separator` | `String` | `"/"` | 分隔符 |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | BreadcrumbItem |

| BreadcrumbItem 属性 | 类型 | 默认值 | 说明 |
|---------------------|------|--------|------|
| `href` | `String` | `""` | 链接地址 |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | 子元素 |

**技术实现：** 使用 `<nav>` 语义化标签。有 href 时渲染 `<a>` 链接，无 href 时添加 `--active` 类名。

### Pagination

**文件：** `ctrl-components/src/pagination/pagination.rs`（113 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `current` | `u32` | `1` | 当前页码 |
| `total` | `u32` | `0` | 总条数 |
| `page_size` | `u32` | `10` | 每页条数 |
| `onchange` | `Option<EventHandler<u32>>` | `None` | 页码切换回调 |
| `class` | `String` | `""` | 自定义类名 |

**技术实现：** 页数通过 `(total / page_size).ceil()` 计算。当前页通过 `clamp(1, page_count)` 限制范围。

### Skeleton

**文件：** `ctrl-components/src/skeleton/skeleton.rs`（345 行）

**组件族：** `Skeleton`, `SkeletonCard`, `SkeletonList`, `SkeletonRow`

| Skeleton 属性 | 类型 | 默认值 | 说明 |
|---------------|------|--------|------|
| `variant` | `String` | `"text"` | 变体 (text/title/avatar/image/button/rect) |
| `rows` | `usize` | `3` | 行数 |
| `shape` | `String` | `"default"` | 形状 (default/circle/round) |
| `count` | `usize` | `0` | 重复次数 |
| `gap` | `String` | `"8px"` | 重复项间距 |
| `width` | `String` | `""` | 宽度 |
| `height` | `String` | `""` | 高度 |
| `animated` | `bool` | `true` | 显示动画 |
| `loading` | `Option<bool>` | `None` | 加载控制 |
| `children` | `Element` | — | 加载完成后内容 |
| `class` | `String` | `""` | 自定义类名 |

**技术实现：** `loading: None` 始终显示骨架；`Some(true)` 显示骨架；`Some(false)` 显示 children。SkeletonCard 预设 图片+标题+文字 行布局。SkeletonList 预设 头像+文字 条目布局。

### Empty

**文件：** `ctrl-components/src/empty/empty.rs`（60 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `description` | `String` | `"暂无数据"` | 描述文字 |
| `image` | `String` | `""` | 自定义图片 URL |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | 底部操作区 |

### Steps

**文件：** `ctrl-components/src/steps/steps.rs`（85 行）

| Steps 属性 | 类型 | 默认值 | 说明 |
|------------|------|--------|------|
| `current` | `i32` | `-1` | 当前进度索引 |
| `direction` | `String` | `"horizontal"` | 排列方向 |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | Step 组件 |

| Step 属性 | 类型 | 默认值 | 说明 |
|-----------|------|--------|------|
| `title` | `String` | `""` | 步骤标题 |
| `description` | `String` | `""` | 步骤描述 |
| `class` | `String` | `""` | 自定义类名 |

### Timeline

**文件：** `ctrl-components/src/timeline/timeline.rs`（88 行）

| Timeline 属性 | 类型 | 默认值 | 说明 |
|---------------|------|--------|------|
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | TimelineItem |

| TimelineItem 属性 | 类型 | 默认值 | 说明 |
|--------------------|------|--------|------|
| `timestamp` | `String` | `""` | 时间标签 |
| `color` | `String` | `"default"` | 圆点颜色 (default/primary/success/warning/danger) |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | 内容 |

### Image

**文件：** `ctrl-components/src/image/image.rs`（364 行）

**组件族：** `Image`, `ImagePreviewProvider`

| Image 属性 | 类型 | 默认值 | 说明 |
|------------|------|--------|------|
| `src` | `String` | `""` | 图片地址 |
| `alt` | `String` | `""` | 替代文字 |
| `width` | `String` | `"auto"` | 宽度 |
| `height` | `String` | `"auto"` | 高度 |
| `fit` | `String` | `"cover"` | 填充模式 (cover/contain/fill/none) |
| `shape` | `String` | `"default"` | 形状 (default/rounded/circle) |
| `preview` | `bool` | `false` | 可预览 |
| `preview_urls` | `Vec<String>` | `[]` | 多图预览列表 |
| `fallback` | `String` | `"加载失败"` | 加载失败占位 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |

**技术实现：** 支持加载中/加载失败/加载完成三种状态。预览功能支持多图左右切换，键盘 ESC 关闭、左右箭头切换。`ImagePreviewProvider` 提供全局预览 API (`use_image_preview()`)。

---

## 反馈组件

### Dialog

**文件：** `ctrl-components/src/dialog/dialog.rs`（322 行）

**双模式：** 声明式 `Dialog` + 命令式 `DialogProvider` + `use_dialog()`

| Dialog 属性 | 类型 | 默认值 | 说明 |
|-------------|------|--------|------|
| `visible` | `bool` | `false` | 是否显示 |
| `title` | `String` | `""` | 标题 |
| `width` | `String` | `"480px"` | 宽度 |
| `show_close` | `bool` | `true` | 显示关闭按钮 |
| `mask_closable` | `bool` | `true` | 点击遮罩关闭 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onclose` | `Option<EventHandler<()>>` | `None` | 关闭事件 |
| `footer` | `Option<Element>` | `None` | 底部操作区插槽 |
| `children` | `Element` | — | 弹窗内容 |

| DialogConfig | 类型 | 默认值 | 说明 |
|--------------|------|--------|------|
| `title` | `String` | `""` | 标题 |
| `content` | `String` | `""` | 内容（纯文本） |
| `confirm_text` | `String` | `"确定"` | 确认按钮文字 |
| `cancel_text` | `String` | `"取消"` | 取消按钮文字 |
| `width` | `String` | `"480px"` | 宽度 |
| `mask_closable` | `bool` | `true` | 点击遮罩关闭 |
| `on_confirm` | `Option<EventHandler<()>>` | `None` | 确认回调 |
| `on_cancel` | `Option<EventHandler<()>>` | `None` | 取消回调 |
| `on_close` | `Option<EventHandler<()>>` | `None` | 关闭回调 |

**技术实现：** DialogProvider 通过 `use_context_provider(|| api)` 提供 DialogAPI。DialogAPI 内部维护 `visible: Signal<bool>` 和 `config: Signal<DialogConfig>`。遮罩层使用 `ctrl-dialog-overlay`（fixed 定位）。

### Drawer

**文件：** `ctrl-components/src/drawer/drawer.rs`（257 行）

**双模式：** 声明式 `Drawer` + 命令式 `DrawerProvider` + `use_drawer()`

| Drawer 属性 | 类型 | 默认值 | 说明 |
|-------------|------|--------|------|
| `visible` | `bool` | `false` | 是否打开 |
| `title` | `String` | `""` | 标题 |
| `placement` | `String` | `"right"` | 位置 (left/right/top/bottom) |
| `size` | `String` | `"380px"` | 宽度或高度 |
| `show_close` | `bool` | `true` | 显示关闭按钮 |
| `class` | `String` | `""` | 自定义类名 |
| `onclose` | `Option<EventHandler<()>>` | `None` | 关闭事件 |
| `footer` | `Option<Element>` | `None` | 底部操作区 |
| `children` | `Element` | — | 抽屉内容 |

### Alert

**文件：** `ctrl-components/src/alert/alert.rs`（473 行）

**双模式：** 声明式 `Alert` + 命令式 `AlertBannerProvider` + `use_alert_banner()`

| Alert 属性 | 类型 | 默认值 | 说明 |
|------------|------|--------|------|
| `r#type` | `AlertType` | `Info` | 类型 (Info/Success/Warning/Error) |
| `title` | `String` | `""` | 标题 |
| `description` | `String` | `""` | 描述 |
| `closable` | `bool` | `false` | 可关闭 |
| `show_icon` | `bool` | `true` | 显示图标 |
| `mode` | `AlertMode` | `Inline` | 模式 (Inline/Banner) |
| `duration` | `u64` | `0` | 自动关闭时间 (ms) |
| `closing` | `bool` | `false` | 外部关闭信号 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `onclose` | `Option<EventHandler<()>>` | `None` | 关闭回调 |

**技术实现：** Banner 模式使用 `gloo_timers::future::TimeoutFuture` 实现自动关闭。退出动画通过 `leaving: Signal<bool>` 控制，动画结束后触发 onclose 回调。使用 `use_future` 定时清理已离开的条目。

### Message

**文件：** `ctrl-components/src/message/message.rs`（455 行）

**双模式：** 声明式 `Message` + 命令式 `MessageProvider` + `use_message()`

| MessageProvider 属性 | 类型 | 默认值 | 说明 |
|----------------------|------|--------|------|
| `placement` | `MessagePlacement` | `Top` | 弹出位置 (Top/TopRight/TopLeft/Bottom) |
| `class` | `String` | `""` | 自定义类名 |
| `max_count` | `usize` | `5` | 最大显示数量 |
| `children` | `Element` | — | 子元素 |

| MessageAPI 方法 | 说明 |
|-----------------|------|
| `info(content)` | Info 类型消息 |
| `success(content)` | Success 类型消息 |
| `warning(content)` | Warning 类型消息 |
| `error(content)` | Error 类型消息 |
| `open(type, content)` | 自定义类型消息 |
| `open_with_duration(type, content, duration)` | 自定义时长 |

**技术实现：** 内部使用 `VecDeque<MessageEntry>` 管理消息队列。自动关闭默认 3000ms。退出动画 400ms。

### Notification

**文件：** `ctrl-components/src/notification/notification.rs`（445 行）

**双模式：** 声明式 `Notification` + 命令式 `NotificationProvider` + `use_notification()`

| NotificationProvider 属性 | 类型 | 默认值 | 说明 |
|---------------------------|------|--------|------|
| `placement` | `NotificationPlacement` | `TopRight` | 位置 (TopRight/TopLeft/BottomRight/BottomLeft) |
| `class` | `String` | `""` | 自定义类名 |
| `max_count` | `usize` | `5` | 最大显示数量 |
| `children` | `Element` | — | 子元素 |

**技术实现：** 默认关闭时长 4500ms。支持 jitter（随机抖动）避免多条通知同时消失。

### Loading

**文件：** `ctrl-components/src/loading/loading.rs`（63 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `loading` | `bool` | `true` | 是否显示 |
| `text` | `String` | `""` | 加载文案 |
| `size` | `Size` | `Md` | 尺寸 |
| `fullscreen` | `bool` | `false` | 全屏遮罩 |
| `class` | `String` | `""` | 自定义类名 |

### Tooltip

**文件：** `ctrl-components/src/tooltip/tooltip.rs`（50 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `content` | `String` | `""` | 提示文字 |
| `placement` | `String` | `"top"` | 弹出位置 (top/bottom/left/right) |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | 触发元素 |

**技术实现：** 纯 CSS 实现，通过 `onmouseenter`/`onmouseleave` 事件切换 `use_signal(|| false)` 控制显隐。

### Popover

**文件：** `ctrl-components/src/popover/popover.rs`（335 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `placement` | `String` | `"top"` | 弹出位置 (top/bottom/left/right) |
| `title` | `String` | `""` | 标题 |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | 触发元素 |
| `content` | `Element` | — | 气泡内容 |

**技术实现：** 使用 `wasm_bindgen` 直接操作 DOM 定位。`position_card()` 测量 trigger 和 card 的 `get_bounding_client_rect()`，计算最佳 `position: fixed` 坐标。`resolve_placement()` 自动翻转（空间不足时翻转方向）。监听 window resize、document click（外部点击关闭）、scroll（滚动关闭）事件。

---

## 导航组件

### Menu

**文件：** `ctrl-components/src/menu/menu.rs`（110 行）

| Menu 属性 | 类型 | 默认值 | 说明 |
|-----------|------|--------|------|
| `direction` | `String` | `"vertical"` | 方向 (vertical/horizontal) |
| `active` | `String` | `""` | 当前激活项 key |
| `onchange` | `Option<EventHandler<String>>` | `None` | 切换回调 |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | MenuItem |

| MenuItem 属性 | 类型 | 默认值 | 说明 |
|---------------|------|--------|------|
| `item_key` | `String` | `""` | 唯一标识 |
| `disabled` | `bool` | `false` | 禁用 |
| `class` | `String` | `""` | 自定义类名 |
| `onclick` | `Option<EventHandler<()>>` | `None` | 点击事件 |
| `icon` | `Option<Element>` | `None` | 图标占位 |
| `children` | `Element` | — | 菜单文字 |

### Dropdown

**文件：** `ctrl-components/src/dropdown/dropdown.rs`（366 行）

| Dropdown 属性 | 类型 | 默认值 | 说明 |
|---------------|------|--------|------|
| `placement` | `String` | `"bottom"` | 弹出位置 (bottom/bottom-start/bottom-end/top/top-start/top-end) |
| `class` | `String` | `""` | 自定义类名 |
| `trigger` | `Element` | — | 触发元素 |
| `children` | `Element` | — | 菜单项 |

| DropdownItem 属性 | 类型 | 默认值 | 说明 |
|-------------------|------|--------|------|
| `disabled` | `bool` | `false` | 禁用 |
| `onclick` | `Option<EventHandler<()>>` | `None` | 点击事件 |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | 菜单项文字 |

**技术实现：** 与 Popover 类似的 DOM 定位方案。支持 6 种 placement 位置。自动翻转、resize 响应、外部点击关闭、滚动关闭。

### Backtop

**文件：** `ctrl-components/src/backtop/backtop.rs`（226 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `target` | `String` | `""` | 滚动容器选择器 |
| `visibility_height` | `u32` | `200` | 显示阈值 (px) |
| `behavior` | `String` | `"smooth"` | 滚动行为 (smooth/auto) |
| `duration` | `u32` | `400` | 动画时长 (ms) |
| `easing` | `String` | `"easeOutCubic"` | 缓动函数 |
| `damping` | `bool` | `false` | 弹簧阻尼效果 |
| `target_position` | `String` | `"top"` | 目标位置 (top/bottom) |
| `onclick` | `Option<EventHandler<()>>` | `None` | 点击事件 |
| `class` | `String` | `""` | 自定义类名 |

**技术实现：** 最复杂的 DOM 交互组件之一。使用 `wasm_bindgen` + `web_sys::window` 监听 scroll 事件。支持 7 种缓动函数（easeOutQuad/Cubic/Quart/Quint/Expo/Back/Elastic）和弹簧阻尼物理模型。通过 `dioxus::document::eval()` 执行自定义滚动 JS。支持 top/bottom 两种目标位置。

---

## 布局组件

### Divider

**文件：** `ctrl-components/src/divider/divider.rs`（59 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `direction` | `String` | `"horizontal"` | 方向 (horizontal/vertical) |
| `content` | `String` | `""` | 中间文字 |
| `dashed` | `bool` | `false` | 虚线 |
| `class` | `String` | `""` | 自定义类名 |

### Space

**文件：** `ctrl-components/src/space/space.rs`（97 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `gap` | `String` | `"md"` | 间距 (sm/md/lg/xl/自定义 CSS) |
| `direction` | `String` | `"horizontal"` | 排列方向 |
| `align` | `String` | `"center"` | 对齐方式 (start/center/end/baseline) |
| `wrap` | `bool` | `false` | 换行 |
| `block` | `bool` | `false` | 占满整行 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `children` | `Element` | — | 子元素 |

**技术实现：** gap 值映射到 CSS 变量：`sm` → `var(--ctrl-spacing-sm)` 等。

### Collapse

**文件：** `ctrl-components/src/collapse/collapse.rs`（228 行）

| Collapse 属性 | 类型 | 默认值 | 说明 |
|---------------|------|--------|------|
| `class` | `String` | `""` | 自定义类名 |
| `borderless` | `bool` | `false` | 无边框 |
| `accordion` | `bool` | `false` | 手风琴模式 |
| `children` | `Element` | — | CollapseItem |

| CollapseItem 属性 | 类型 | 默认值 | 说明 |
|-------------------|------|--------|------|
| `title` | `String` | `""` | 标题 |
| `expanded` | `bool` | `false` | 是否展开 |
| `disabled` | `bool` | `false` | 禁用 |
| `show_arrow` | `bool` | `true` | 显示箭头 |
| `animated` | `bool` | `true` | 展开/收起动画 |
| `class` | `String` | `""` | 自定义类名 |
| `children` | `Element` | — | 展开内容 |

**技术实现：** 手风琴模式通过 `CollapseCtx` Context 共享 `active_id: Signal<Option<u16>>`。使用 `AtomicU16` 生成唯一 ID。展开/收起动画通过 `use_effect` + `use_reactive` 监听状态变化，用 `web_sys` 精确设置 `max-height`。`use_memo` 计算最终展开状态。

### Carousel

**文件：** `ctrl-components/src/carousel/carousel.rs`（164 行）

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `autoplay` | `bool` | `true` | 自动播放 |
| `interval` | `u64` | `3000` | 自动播放间隔 (ms) |
| `arrows` | `bool` | `true` | 显示箭头 |
| `dots` | `bool` | `true` | 显示指示器 |
| `effect` | `String` | `"slide"` | 过渡效果 (slide/fade) |
| `height` | `String` | `"300px"` | 容器高度 |
| `class` | `String` | `""` | 自定义类名 |
| `style` | `String` | `""` | 自定义样式 |
| `children` | `Element` | — | 轮播项 |

**技术实现：** 使用 CSS `transform: translateX()` 实现 slide 效果。自动播放通过 `use_resource` + `gloo_timers` 实现。

---

## 组件统计

| 分类 | 数量 | 组件 |
|------|------|------|
| 基础 | 7 | Button, Input, Switch, Checkbox, Radio, Select, Tag, Card |
| 数据录入 | 7 | Slider, Rate, InputNumber, Upload, DatePicker, Form/FormItem, Segmented |
| 数据展示 | 11 | Table, Badge, Avatar, Progress, Tabs, Breadcrumb, Pagination, Skeleton, Empty, Steps, Timeline, Image |
| 反馈 | 9 | Dialog, Drawer, Alert, Message, Notification, Loading, Tooltip, Popover |
| 导航 | 3 | Menu, Dropdown, Backtop |
| 布局 | 4 | Divider, Space, Collapse, Carousel |
| **总计** | **47** | |

## 技术模式总结

### 样式方案

所有组件使用 **CSS 文件 + CSS 类名** 方案：
- 每个组件通过 `include_str!("../../assets/xxx.css")` 嵌入 CSS
- 组件内部通过 `build_xxx_class()` 函数动态拼接类名
- 用户通过 `class` 和 `style` 属性覆盖样式

### 事件处理

所有事件处理器使用 `Option<EventHandler<T>>` 类型，组件内部通过 `.clone()` 后在闭包中调用：
```rust
let onclick = props.onclick.clone();
onclick: move |evt| {
    if let Some(ref handler) = onclick {
        handler.call(evt);
    }
},
```

### 命令式 API

Dialog、Drawer、Alert、Message、Notification、Image 均提供双模式：
- **声明式：** 直接使用组件 + visible prop
- **命令式：** `XxxProvider` + `use_xxx()` 获取 API，调用 `api.open(config)`

### WASM 交互

Backtop、Popover、Dropdown、Slider 使用 `wasm_bindgen` + `web_sys` 直接操作 DOM：
- 事件监听：`add_event_listener_with_callback` + `Closure::forget()`
- DOM 操作：`get_bounding_client_rect()`, `set_attribute()`
- 条件编译：`#[cfg(target_arch = "wasm32")]`

### 状态管理

- 所有组件使用 `use_signal` 管理内部状态（Dioxus 0.7 原生）
- 跨组件状态通过 `use_context_provider` / `use_context` 共享
- 复杂派生状态使用 `use_memo`
- 异步任务使用 `spawn` + `gloo_timers`
