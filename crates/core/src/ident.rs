use proc_macro2::Ident;
use proc_macro2::Span;

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
