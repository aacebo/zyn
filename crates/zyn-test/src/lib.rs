//! Test utilities for the zyn proc macro framework.
//!
//! Provides assertion macros for comparing token streams and inspecting
//! diagnostics produced by [`Output`](zyn_core::Output).
//!
//! # Quick reference
//!
//! ```ignore
//! use zyn_test::*;
//!
//! // Token comparison (clean-formatted diff on failure)
//! assert_tokens!(output, expected);
//! assert_tokens_empty!(output);
//! assert_tokens_contain!(output, "struct Foo");
//!
//! // Diagnostic assertions (works on zyn::Output)
//! assert_diagnostic_error!(output, "field not found");
//! assert_diagnostic_warning!(output, "unused");
//! assert_compile_error!(output, "missing field");
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
