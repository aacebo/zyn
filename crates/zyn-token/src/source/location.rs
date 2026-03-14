/// 0 indexed char based location
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    index: usize,
    line: usize,
    column: usize,
}

impl SourceLocation {
    pub(crate) const fn new(index: usize, line: usize, column: usize) -> Self {
        Self {
            index,
            line,
            column,
        }
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub const fn line(&self) -> usize {
        self.line
    }

    pub const fn column(&self) -> usize {
        self.column
    }
}
