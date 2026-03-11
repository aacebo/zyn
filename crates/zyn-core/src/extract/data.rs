use syn::spanned::Spanned;

use crate::mark;
use crate::types::Input;

use super::FromInput;

/// Converts `syn::Data` into a specific data representation.
///
/// Implementations exist for `syn::Data` (any kind), `syn::DataStruct`,
/// `syn::DataEnum`, and `syn::DataUnion`.
pub trait FromData: Sized {
    fn from_data(data: syn::Data) -> crate::Result<Self>;
}

impl FromData for syn::Data {
    fn from_data(data: syn::Data) -> crate::Result<Self> {
        Ok(data)
    }
}

impl FromData for syn::DataStruct {
    fn from_data(data: syn::Data) -> crate::Result<Self> {
        match data {
            syn::Data::Struct(s) => Ok(s),
            syn::Data::Enum(e) => Err(mark::error("expected struct data")
                .span(e.enum_token.span())
                .build()),
            syn::Data::Union(u) => Err(mark::error("expected struct data")
                .span(u.union_token.span())
                .build()),
        }
    }
}

impl FromData for syn::DataEnum {
    fn from_data(data: syn::Data) -> crate::Result<Self> {
        match data {
            syn::Data::Enum(e) => Ok(e),
            syn::Data::Struct(s) => Err(mark::error("expected enum data")
                .span(s.struct_token.span())
                .build()),
            syn::Data::Union(u) => Err(mark::error("expected enum data")
                .span(u.union_token.span())
                .build()),
        }
    }
}

impl FromData for syn::DataUnion {
    fn from_data(data: syn::Data) -> crate::Result<Self> {
        match data {
            syn::Data::Union(u) => Ok(u),
            syn::Data::Struct(s) => Err(mark::error("expected union data")
                .span(s.struct_token.span())
                .build()),
            syn::Data::Enum(e) => Err(mark::error("expected union data")
                .span(e.enum_token.span())
                .build()),
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
            Input::Derive(d) => T::from_data(d.data.clone()).map(Data),
            _ => Err(mark::error("Data extractor requires derive input")
                .span(input.span())
                .build()),
        }
    }
}
