use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::diagnostic::Diagnostics;
use crate::types::Input;

use super::FromInput;

/// Converts `syn::Fields` into a specific fields representation.
///
/// Implementations exist for `syn::Fields` (any kind), `syn::FieldsNamed`
/// (named only), and `syn::FieldsUnnamed` (tuple only).
pub trait FromFields: Sized {
    fn from_fields(fields: syn::Fields) -> crate::Result<Self>;
}

impl FromFields for syn::Fields {
    fn from_fields(fields: syn::Fields) -> crate::Result<Self> {
        Ok(fields)
    }
}

impl FromFields for syn::FieldsNamed {
    fn from_fields(fields: syn::Fields) -> crate::Result<Self> {
        match fields {
            syn::Fields::Named(f) => Ok(f),
            _ => Err(Diagnostics::error(
                Span::call_site(),
                "expected named fields",
            )),
        }
    }
}

impl FromFields for syn::FieldsUnnamed {
    fn from_fields(fields: syn::Fields) -> crate::Result<Self> {
        match fields {
            syn::Fields::Unnamed(f) => Ok(f),
            _ => Err(Diagnostics::error(
                Span::call_site(),
                "expected unnamed fields",
            )),
        }
    }
}

/// Element extractor that pulls struct fields from the input.
///
/// Defaults to `syn::Fields` (accepts any field kind). Parameterize with
/// `syn::FieldsNamed` or `syn::FieldsUnnamed` to restrict and validate.
/// Access the inner value via `Deref` or the `inner()` method.
///
/// ```ignore
/// #[zyn::element]
/// fn my_element(#[zyn(input)] fields: zyn::Fields<syn::FieldsNamed>) -> proc_macro2::TokenStream {
///     // fields.named — accessed via Deref to syn::FieldsNamed
/// }
/// ```
pub struct Fields<T: FromFields = syn::Fields>(T);

impl<T: FromFields> Fields<T> {
    /// Consumes the wrapper and returns the inner value.
    pub fn inner(self) -> T {
        self.0
    }
}

impl<T: FromFields> std::ops::Deref for Fields<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: FromFields> std::ops::DerefMut for Fields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: FromFields> FromInput for Fields<T> {
    fn from_input(input: &Input) -> crate::Result<Self> {
        let raw = match input {
            Input::Derive(d) => match &d.data {
                syn::Data::Struct(s) => s.fields.clone(),
                _ => {
                    return Err(Diagnostics::error(
                        d.ident.span(),
                        "expected struct input for Fields extractor",
                    ));
                }
            },
            Input::Item(syn::Item::Struct(s)) => s.fields.clone(),
            _ => {
                return Err(Diagnostics::error(
                    input.span(),
                    "expected struct input for Fields extractor",
                ));
            }
        };

        T::from_fields(raw).map(Fields)
    }
}
