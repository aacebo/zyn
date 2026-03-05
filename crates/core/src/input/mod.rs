pub mod derive;
pub mod item;

pub use derive::*;
pub use item::*;

pub enum Input {
    Derive(DeriveInput),
    Item(ItemInput),
}

impl Input {
    pub fn attrs(&self) -> &[syn::Attribute] {
        match self {
            Self::Derive(d) => d.attrs(),
            Self::Item(i) => i.attrs(),
        }
    }
}

impl From<DeriveInput> for Input {
    fn from(v: DeriveInput) -> Self {
        Self::Derive(v)
    }
}

impl From<ItemInput> for Input {
    fn from(v: ItemInput) -> Self {
        Self::Item(v)
    }
}
