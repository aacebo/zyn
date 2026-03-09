//! Attribute argument parsing types.
//!
//! [`Args`] is the parsed representation of a `(key = value, flag, nested(...))`
//! argument list. [`Arg`] is a single argument in that list.
//!
//! # Argument forms
//!
//! ```ignore
//! // Flag:           #[attr(skip)]
//! // Key-value:      #[attr(rename = "foo")]
//! // Key-value expr: #[attr(count = 1 + 1)]
//! // Nested list:    #[attr(tags("a", "b"))]
//! ```
//!
//! # Examples
//!
//! Parsing `Args` from a string (useful in tests):
//!
//! ```ignore
//! use zyn_core::meta::Args;
//!
//! let args: Args = zyn_core::parse!("skip, rename = \"foo\"").unwrap();
//! assert!(args.has("skip"));
//! assert_eq!(args.get("rename").unwrap().as_str(), "foo");
//! ```

mod arg;
mod args;
mod distance;

pub use arg::*;
pub use args::*;
pub use distance::*;
