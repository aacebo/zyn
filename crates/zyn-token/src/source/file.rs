use std::{cell::RefCell, collections::BTreeMap};

use crate::{SourceLocation, Span};

/// Primarily used to map spans (0 based character index ranges)
/// to bytes.
#[derive(Debug)]
pub struct SourceFile {
    /// raw source text
    text: String,

    /// file-wide lo..hi in proc-macro2 space
    span: Span,

    /// line start offsets, in char units
    lines: Vec<usize>,

    /// Cache mapping character indices to UTF-8 byte offsets for efficient span slicing
    char_to_byte: RefCell<BTreeMap<usize, usize>>,
}

impl SourceFile {
    pub(crate) fn new(start: usize, src: impl Into<String>) -> Self {
        let text = src.into();
        let mut lines = vec![0];
        let mut total = 0usize;

        for ch in text.chars() {
            total += 1;

            if ch == '\n' {
                lines.push(total);
            }
        }

        Self {
            text,
            span: Span::new(start as u32, (start + total) as u32),
            lines,
            ..Default::default()
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn span(&self) -> Span {
        self.span
    }

    /// Resolves the given span into a byte index range.
    pub fn range(&self, span: Span) -> std::ops::Range<usize> {
        self.byte(span.start())..self.byte(span.end())
    }

    /// Gets a sub span of source text from the file.
    pub fn slice(&self, span: Span) -> String {
        self.text[self.range(span)].to_owned()
    }

    /// Resolves a global character index within this file into a 0-based `SourceLocation`.
    pub fn location(&self, i: usize) -> SourceLocation {
        let index = i - self.span.start();

        match self.lines.binary_search(&index) {
            Err(next) => SourceLocation::new(index, next - 1, index - self.lines[next - 1]),
            Ok(line) => SourceLocation::new(index, line, 0),
        }
    }

    /// Returns the UTF-8 byte index corresponding to a global character index.
    pub fn byte(&self, i: usize) -> usize {
        let index = i - self.span.start();
        let mut cache = self.char_to_byte.borrow_mut();

        if let Some(byte_index) = cache.get(&index) {
            return *byte_index;
        }

        let (&ci, &bi) = cache.range(..=index).next_back().unwrap();

        let mut char_index = ci;
        let mut byte_index = bi;

        #[allow(clippy::explicit_counter_loop)]
        for ch in self.text[bi..].chars() {
            if char_index == index {
                cache.insert(index, byte_index);
                return byte_index;
            }

            char_index += 1;
            byte_index += ch.len_utf8();
        }

        cache.insert(index, byte_index);
        byte_index
    }
}

impl Default for SourceFile {
    fn default() -> Self {
        Self {
            text: String::default(),
            span: Span::default(),
            lines: vec![0],
            char_to_byte: RefCell::new(BTreeMap::default()),
        }
    }
}
