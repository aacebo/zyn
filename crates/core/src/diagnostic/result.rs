use super::diag::Diagnostic;

#[derive(Clone, Copy, Debug)]
pub enum ChildKind {
    Note,
    Help,
}

#[derive(Clone, Debug)]
pub struct Child {
    pub(super) kind: ChildKind,
    pub(super) message: String,
}

pub type Result<T = proc_macro2::TokenStream> = core::result::Result<T, Diagnostic>;
