use proc_macro2::Span;

use crate::input::Input;

use super::FromInput;

/// Converts `syn::Data` into a specific data representation.
///
/// Implementations exist for `syn::Data` (any kind), `syn::DataStruct`,
/// `syn::DataEnum`, and `syn::DataUnion`.
pub trait FromData: Sized {
    fn from_data(data: syn::Data) -> syn::Result<Self>;
}

impl FromData for syn::Data {
    fn from_data(data: syn::Data) -> syn::Result<Self> {
        Ok(data)
    }
}

impl FromData for syn::DataStruct {
    fn from_data(data: syn::Data) -> syn::Result<Self> {
        match data {
            syn::Data::Struct(s) => Ok(s),
            _ => Err(syn::Error::new(Span::call_site(), "expected struct data")),
        }
    }
}

impl FromData for syn::DataEnum {
    fn from_data(data: syn::Data) -> syn::Result<Self> {
        match data {
            syn::Data::Enum(e) => Ok(e),
            _ => Err(syn::Error::new(Span::call_site(), "expected enum data")),
        }
    }
}

impl FromData for syn::DataUnion {
    fn from_data(data: syn::Data) -> syn::Result<Self> {
        match data {
            syn::Data::Union(u) => Ok(u),
            _ => Err(syn::Error::new(Span::call_site(), "expected union data")),
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
    type Error = syn::Error;

    fn from_input(input: &Input) -> Result<Self, Self::Error> {
        match input {
            Input::Derive(d) => T::from_data(d.data.clone()).map(Data),
            _ => Err(syn::Error::new(
                Span::call_site(),
                "Data extractor requires derive input",
            )),
        }
    }
}
