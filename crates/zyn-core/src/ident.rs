//! Internal identifier generation for template expansion.
//!
//! [`Iter`] produces sequential hygienic identifiers (`__zyn_ts_0`, `__zyn_ts_1`,
//! ...) used by the template expander to name temporaries in expanded code. Not
//! intended for direct use by proc macro authors.
//!
//! # Example
//!
//! ```ignore
//! let mut iter = zyn_core::ident::Iter::new();
//! assert_eq!(iter.next().unwrap().to_string(), "__zyn_ts_0");
//! assert_eq!(iter.next().unwrap().to_string(), "__zyn_ts_1");
//! ```

use proc_macro2::Ident;
use proc_macro2::Span;

/// An iterator that yields unique internal identifiers for template expansion.
pub struct Iter {
    counter: usize,
}

impl Iter {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Default for Iter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Iter {
    type Item = Ident;

    fn next(&mut self) -> Option<Self::Item> {
        let id = Ident::new(&format!("__zyn_ts_{}", self.counter), Span::call_site());

        self.counter += 1;
        Some(id)
    }
}
