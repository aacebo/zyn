pub mod ast;
pub mod case;
pub mod debug;
pub mod diagnostic;
pub mod extract;
pub mod ident;
pub mod input;
pub mod meta;
pub mod pipes;

#[cfg(feature = "ext")]
pub mod ext;

pub use diagnostic::*;
pub use extract::*;
pub use input::*;
pub use meta::*;
pub use pipes::*;

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
    fn render(&self) -> proc_macro2::TokenStream;
}

pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn pipe(&self, input: Self::Input) -> Self::Output;
}
