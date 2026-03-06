//! A template engine and framework for Rust procedural macros.
//!
//! Zyn provides a `zyn!` template macro with interpolation, control flow, pipes,
//! reusable elements, and proc macro entry points — replacing the typical
//! `syn` + `quote` + `heck` + `proc-macro-error` stack with a single dependency.
//!
//! # Example
//!
//! ```ignore
//! use zyn::prelude::*;
//!
//! zyn! {
//!     impl {{ ident }} {
//!         @for (field in fields.iter()) {
//!             pub fn {{ field.ident | snake | ident:"get_{}" }}(&self) -> &{{ field.ty }} {
//!                 &self.{{ field.ident }}
//!             }
//!         }
//!     }
//! }
//! ```

pub use zyn_core::*;

#[cfg(feature = "derive")]
pub use zyn_derive::*;

/// The zyn prelude. Re-exports all built-in pipes, core traits, and proc macros.
pub mod prelude {
    pub use crate::pipes::*;
    pub use crate::{Pipe, Render};

    #[cfg(feature = "derive")]
    pub use zyn_derive::*;

    #[cfg(feature = "ext")]
    pub use zyn_core::ext::*;
}
