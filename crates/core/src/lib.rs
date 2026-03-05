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

pub type Result<T> = diagnostic::Result<T>;

#[macro_export]
macro_rules! parse {
    ($s:literal => $ty:ty) => {
        $crate::syn::parse_str::<$ty>($s)
    };
    ($s:literal) => {
        $crate::syn::parse_str($s)
    };
    ($ts:expr => $ty:ty) => {
        $crate::syn::parse2::<$ty>($ts)
    };
    ($ts:expr) => {
        $crate::syn::parse2($ts)
    };
}

#[macro_export]
macro_rules! parse_input {
    ($($tt:tt)*) => { $crate::syn::parse_macro_input!($($tt)*) }
}

pub use proc_macro2::{Span, TokenStream};
pub use quote::{ToTokens, format_ident};

pub use proc_macro2;
pub use quote;
pub use syn;

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
