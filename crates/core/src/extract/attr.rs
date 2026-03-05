use crate::types::Input;

use super::FromInput;

/// Element extractor that resolves a `#[derive(Attribute)]` struct from the
/// input's attributes.
///
/// Use as an element parameter type to auto-extract a parsed attribute config
/// without passing it as a prop at the call site. Access the inner value via
/// `Deref` or consume with `inner()`.
///
/// ```ignore
/// #[zyn::element]
/// fn my_element(#[zyn(input)] cfg: zyn::Attr<MyConfig>) -> proc_macro2::TokenStream {
///     // cfg.my_field — accessed via Deref
/// }
/// ```
pub struct Attr<T: FromInput>(T);

impl<T: FromInput> Attr<T> {
    pub fn inner(self) -> T {
        self.0
    }
}

impl<T: FromInput> std::ops::Deref for Attr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: FromInput> std::ops::DerefMut for Attr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: FromInput> FromInput for Attr<T> {
    fn from_input(input: &Input) -> crate::Result<Self> {
        T::from_input(input).map(Attr)
    }
}
