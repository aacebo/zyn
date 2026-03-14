use crate::{SourceFile, Span};

#[derive(Debug, Default)]
pub struct SourceMap(Vec<SourceFile>);

impl SourceMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn files(&self) -> &[SourceFile] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn find(&self, span: Span) -> Option<&SourceFile> {
        match self.find_index(span) {
            None => None,
            Some(i) => Some(&self.0[i]),
        }
    }

    pub fn find_mut(&mut self, span: Span) -> Option<&mut SourceFile> {
        match self.find_index(span) {
            None => None,
            Some(i) => Some(&mut self.0[i]),
        }
    }

    pub fn find_path(&self, span: Span) -> Option<String> {
        if let Some(i) = self.find_index(span) {
            return if i == 0 {
                Some("<unspecified>".to_owned())
            } else {
                Some(format!("<parsed string {}>", i))
            };
        }

        None
    }

    pub fn find_index(&self, span: Span) -> Option<usize> {
        self.0.binary_search_by(|file| file.span().cmp(&span)).ok()
    }

    pub fn add(&mut self, src: impl Into<String>) -> Span {
        let start = self.0.last().map(|file| file.span().end()).unwrap_or(0);
        let file = SourceFile::new(start, src);
        let span = file.span();
        self.0.push(file);
        span
    }
}
