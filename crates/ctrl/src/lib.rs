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
//! dioxus = { version = "0.7", features = ["web"] }
//! ```
//!
//! 然后在 `main.rs` 中：
//!
//! ```rust,no_run
//! use dioxus::prelude::*;
//! use ctrl::*;
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

// 核心层
pub use ctrl_core::*;

// 组件层
pub use ctrl_components::*;

/// 便捷导入
pub mod prelude {
    pub use ctrl_core::theme::{ThemeProvider, ThemeProviderProps};
    pub use ctrl_core::types::*;
    pub use ctrl_core::utils::cn;
    pub use ctrl_components::*;
}
