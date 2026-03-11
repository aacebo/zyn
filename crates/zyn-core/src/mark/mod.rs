//! Diagnostic construction utilities.
//!
//! The [`mark`](self) module provides free functions returning a [`Builder`],
//! which accumulates diagnostics and is finalized into an immutable [`Diagnostic`]
//! via [`Builder::build`].
//!
//! # Quick start
//!
//! ```ignore
//! // Single error
//! return Err(mark::error("field `name` is required").build());
//!
//! // Accumulator
//! let d = mark::new()
//!     .add(mark::error("missing `x`"))
//!     .add(mark::help("add `x: u32` to your struct"))
//!     .build();
//! if d.is_error() { return d.emit(); }
//! ```

mod diagnostic;
mod level;
mod span;

pub use diagnostic::Builder;
pub use diagnostic::Diagnostic;
pub use diagnostic::Result;
pub use diagnostic::Walk;
pub use level::*;
pub use span::*;

/// Creates an empty diagnostic builder.
pub fn new() -> Builder {
    Builder::default()
}

/// Creates an error diagnostic builder with the given message.
pub fn error(message: impl Into<String>) -> Builder {
    Builder::default().level(Level::Error).message(message)
}

/// Creates a warning diagnostic builder with the given message.
pub fn warning(message: impl Into<String>) -> Builder {
    Builder::default().level(Level::Warning).message(message)
}

/// Creates a note diagnostic builder with the given message.
pub fn note(message: impl Into<String>) -> Builder {
    Builder::default().level(Level::Note).message(message)
}

/// Creates a help diagnostic builder with the given message.
pub fn help(message: impl Into<String>) -> Builder {
    Builder::default().level(Level::Help).message(message)
}
