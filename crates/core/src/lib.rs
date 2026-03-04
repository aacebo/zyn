pub mod ast;
pub mod case;
pub mod diagnostic;
pub mod ident;
pub mod input;
pub mod pipes;

pub use diagnostic::*;
pub use input::*;
pub use pipes::*;

pub trait Expand {
    fn expand(
        &self,
        output: &proc_macro2::Ident,
        idents: &mut ident::Iter,
    ) -> proc_macro2::TokenStream;
}

pub trait Render {
    fn render(&self) -> diagnostic::Result;
}

pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn pipe(&self, input: Self::Input) -> Self::Output;
}
