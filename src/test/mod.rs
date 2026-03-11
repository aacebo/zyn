//! Test utilities for the zyn proc macro framework.
//!
//! Provides assertion macros for comparing token streams and inspecting
//! diagnostics produced by [`Output`](crate::Output).
//!
//! # Quick reference
//!
//! ```ignore
//! // Token comparison (raw-formatted diff on failure)
//! zyn::assert_tokens!(output, expected);
//! zyn::assert_tokens_empty!(output);
//! zyn::assert_tokens_contain!(output, "struct Foo");
//!
//! // Diagnostic assertions (works on zyn::Output)
//! zyn::assert_diagnostic_error!(output, "field not found");
//! zyn::assert_diagnostic_warning!(output, "unused");
//! zyn::assert_compile_error!(output, "missing field");
//! ```
//!
//! # Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `pretty` | Enables [`assert_tokens_pretty!`] and [`assert_tokens_contain_pretty!`] for `prettyplease`-formatted comparison |

mod assert_diagnostic;
mod assert_tokens;
mod assert_tokens_contain;
mod assert_tokens_empty;
