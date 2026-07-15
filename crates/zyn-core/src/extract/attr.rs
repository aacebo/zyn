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
pub struct Attr<T>(T);

impl<T> Attr<T> {
    /// Consumes the wrapper and returns the inner value.
    pub fn inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for Attr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Attr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'i, T> FromInput<'i> for Attr<T>
where
    T: FromInput<'i>,
{
    fn from_input(input: &'i Input) -> crate::Result<Self> {
        T::from_input(input).map(Attr)
    }
}
