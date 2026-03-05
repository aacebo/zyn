use proc_macro2::Span;

use crate::input::Input;

use super::FromInput;

/// Element extractor that pulls enum variants from the input.
///
/// Errors at compile time if the input is not an enum. Access the inner
/// `Vec<syn::Variant>` via `Deref` or the `inner()` method.
///
/// ```ignore
/// #[zyn::element]
/// fn my_element(#[zyn(input)] variants: zyn::Variants) -> proc_macro2::TokenStream {
///     // variants.iter() — accessed via Deref to Vec<syn::Variant>
/// }
/// ```
pub struct Variants(Vec<syn::Variant>);

impl Variants {
    pub fn inner(self) -> Vec<syn::Variant> {
        self.0
    }
}

impl std::ops::Deref for Variants {
    type Target = Vec<syn::Variant>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Variants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromInput for Variants {
    type Error = syn::Error;

    fn from_input(input: &Input) -> Result<Self, Self::Error> {
        match input {
            Input::Derive(d) => match &d.data {
                syn::Data::Enum(e) => Ok(Variants(e.variants.iter().cloned().collect())),
                _ => Err(syn::Error::new(
                    d.ident.span(),
                    "expected enum input for Variants extractor",
                )),
            },
            Input::Item(syn::Item::Enum(e)) => Ok(Variants(e.variants.iter().cloned().collect())),
            _ => Err(syn::Error::new(
                Span::call_site(),
                "expected enum input for Variants extractor",
            )),
        }
    }
}
