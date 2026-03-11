//! Span utilities for diagnostics.

use proc_macro2::Span;

/// A type that can be converted into a list of [`Span`]s.
///
/// Implemented for `Span`, `Vec<Span>`, and `&[Span]`.
pub trait MultiSpan {
    fn into_spans(self) -> Vec<Span>;
}

impl MultiSpan for Span {
    fn into_spans(self) -> Vec<Span> {
        vec![self]
    }
}

impl MultiSpan for Vec<Span> {
    fn into_spans(self) -> Vec<Span> {
        self
    }
}

impl MultiSpan for &[Span] {
    fn into_spans(self) -> Vec<Span> {
        self.to_vec()
    }
}
