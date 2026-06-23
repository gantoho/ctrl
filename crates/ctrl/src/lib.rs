//! # Ctrl UI
//!
//! 开箱即用的 Dioxus UI 组件库。
//!
//! ## 快速开始
//!
//! 在你的 `Cargo.toml` 中添加：
//!
//! ```toml
//! [dependencies]
//! ctrl = "0.1"
//! dioxus = { version = "0.5", features = ["web"] }
//! ```
//!
//! 然后在 `main.rs` 中：
//!
//! ```rust,no_run
//! use dioxus::prelude::*;
//! use ctrl::prelude::*;
//!
//! fn App() -> Element {
//!     rsx! {
//!         ThemeProvider {
//!             div { style: "padding: 20px;",
//!                 Button { variant: Variant::Primary, "Hello Ctrl" }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## 主题定制
//!
//! ```rust,no_run
//! use ctrl::theme::{Theme, ColorPalette};
//!
//! let my_theme = Theme {
//!     colors: ColorPalette {
//!         primary: "#FF6B35",
//!         ..Default::default()
//!     },
//!     ..Default::default()
//! };
//! ```

// 核心层
pub use ctrl_core::*;

// 组件层
pub use ctrl_components::*;

/// 便捷导入：一次性导入所有常用类型和组件
pub mod prelude {
    pub use ctrl_core::theme::{ThemeProvider, ThemeProviderProps};
    pub use ctrl_core::types::*;
    pub use ctrl_core::utils::cn;
    pub use ctrl_components::{Button, ButtonProps, Input, InputProps};
}