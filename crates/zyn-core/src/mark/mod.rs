//! Diagnostic construction utilities.
//!
//! The [`mark`](self) module provides free functions returning a [`DiagnosticBuilder`],
//! which accumulates diagnostics and is finalized into an immutable [`Diagnostic`]
//! via [`DiagnosticBuilder::build`].
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

pub use diagnostic::Diagnostic;
pub use diagnostic::DiagnosticBuilder;
pub use diagnostic::Result;
pub use level::*;
pub use span::*;

/// Creates an empty diagnostic builder.
pub fn new() -> DiagnosticBuilder {
    DiagnosticBuilder::default()
}

/// Creates an error diagnostic builder with the given message.
pub fn error(message: impl Into<String>) -> DiagnosticBuilder {
    DiagnosticBuilder::default()
        .level(Level::Error)
        .message(message)
}

/// Creates a warning diagnostic builder with the given message.
pub fn warning(message: impl Into<String>) -> DiagnosticBuilder {
    DiagnosticBuilder::default()
        .level(Level::Warning)
        .message(message)
}

/// Creates a note diagnostic builder with the given message.
pub fn note(message: impl Into<String>) -> DiagnosticBuilder {
    DiagnosticBuilder::default()
        .level(Level::Note)
        .message(message)
}

/// Creates a help diagnostic builder with the given message.
pub fn help(message: impl Into<String>) -> DiagnosticBuilder {
    DiagnosticBuilder::default()
        .level(Level::Help)
        .message(message)
}
