#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Span {
    /// the start char index (inclusive).
    start: u32,

    /// the end char index (exclusive).
    end: u32,
}

impl Span {
    pub(crate) const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub const fn call_site() -> Self {
        Self::new(0, 0)
    }

    pub const fn mixed_site() -> Self {
        Self::call_site()
    }

    pub const fn def_site() -> Self {
        Self::call_site()
    }

    pub const fn start(&self) -> usize {
        self.start as usize
    }

    pub const fn end(&self) -> usize {
        self.end as usize
    }

    pub const fn len(&self) -> usize {
        (self.end - self.start) as usize
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn contains(&self, i: usize) -> bool {
        i >= self.start as usize && i < self.end as usize
    }

    pub const fn is_subset(&self, other: &Self) -> bool {
        other.start >= self.start && other.end < self.end
    }

    pub const fn join(self, other: Self) -> Self {
        let start = if self.start < other.start {
            self.start
        } else {
            other.start
        };

        let end = if self.end > other.end {
            self.end
        } else {
            other.end
        };

        Self { start, end }
    }
}
