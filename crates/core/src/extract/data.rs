use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::mark;
use crate::types::Input;

use super::FromInput;

/// Converts `syn::Data` into a specific data representation.
///
/// Implementations exist for `syn::Data` (any kind), `syn::DataStruct`,
/// `syn::DataEnum`, and `syn::DataUnion`.
pub trait FromData: Sized {
    fn from_data(data: syn::Data, span: Span) -> crate::Result<Self>;
}

impl FromData for syn::Data {
    fn from_data(data: syn::Data, _span: Span) -> crate::Result<Self> {
        Ok(data)
    }
}

impl FromData for syn::DataStruct {
    fn from_data(data: syn::Data, span: Span) -> crate::Result<Self> {
        match data {
            syn::Data::Struct(s) => Ok(s),
            _ => Err(mark::error("expected struct data").span(span).build()),
        }
    }
}

impl FromData for syn::DataEnum {
    fn from_data(data: syn::Data, span: Span) -> crate::Result<Self> {
        match data {
            syn::Data::Enum(e) => Ok(e),
            _ => Err(mark::error("expected enum data").span(span).build()),
        }
    }
}

impl FromData for syn::DataUnion {
    fn from_data(data: syn::Data, span: Span) -> crate::Result<Self> {
        match data {
            syn::Data::Union(u) => Ok(u),
            _ => Err(mark::error("expected union data").span(span).build()),
        }
    }
}

/// Element extractor that pulls the `syn::Data` from a derive input.
///
/// Defaults to `syn::Data` (accepts any kind). Parameterize with
/// `syn::DataStruct`, `syn::DataEnum`, or `syn::DataUnion` to restrict
/// and validate. Access the inner value via `Deref` or the `inner()` method.
///
/// ```ignore
/// #[zyn::element]
/// fn my_element(#[zyn(input)] data: zyn::Data<syn::DataStruct>) -> proc_macro2::TokenStream {
///     // data.fields — accessed via Deref to syn::DataStruct
/// }
/// ```
pub struct Data<T: FromData = syn::Data>(T);

impl<T: FromData> Data<T> {
    /// Consumes the wrapper and returns the inner value.
    pub fn inner(self) -> T {
        self.0
    }
}

impl<T: FromData> std::ops::Deref for Data<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: FromData> std::ops::DerefMut for Data<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: FromData> FromInput for Data<T> {
    fn from_input(input: &Input) -> crate::Result<Self> {
        match input {
            Input::Derive(d) => T::from_data(d.data.clone(), d.ident.span()).map(Data),
            _ => Err(mark::error("Data extractor requires derive input")
                .span(input.span())
                .build()),
        }
    }
}
