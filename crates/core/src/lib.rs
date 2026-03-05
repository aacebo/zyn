pub mod ast;
pub mod case;
pub mod debug;
pub mod diagnostic;
pub mod extract;
pub mod ident;
pub mod meta;
pub mod pipes;
pub mod types;

#[cfg(feature = "ext")]
pub mod ext;

pub use diagnostic::*;
pub use extract::*;
pub use meta::*;
pub use types::Input;

#[macro_export]
macro_rules! parse {
    ($s:literal => $ty:ty) => {
        $crate::__private::syn::parse_str::<$ty>($s)
    };
    ($s:literal) => {
        $crate::__private::syn::parse_str($s)
    };
    ($ts:expr => $ty:ty) => {
        $crate::__private::syn::parse2::<$ty>($ts)
    };
    ($ts:expr) => {
        $crate::__private::syn::parse2($ts)
    };
}

#[macro_export]
macro_rules! parse_input {
    ($($tt:tt)*) => { $crate::__private::syn::parse_macro_input!($($tt)*) }
}

pub use proc_macro2::{Span, TokenStream};
pub use quote::{ToTokens, format_ident};

#[doc(hidden)]
pub mod __private {
    pub use proc_macro2;
    pub use quote;
    pub use syn;
}

pub trait Expand {
    fn expand(
        &self,
        output: &proc_macro2::Ident,
        idents: &mut ident::Iter,
    ) -> proc_macro2::TokenStream;
}

pub trait Render {
    fn render(&self, input: &types::Input) -> proc_macro2::TokenStream;
}

pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn pipe(&self, input: Self::Input) -> Self::Output;
}
