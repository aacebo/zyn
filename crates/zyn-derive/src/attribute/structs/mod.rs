mod field_meta;
mod struct_meta;

pub use field_meta::FieldDefault;
pub use field_meta::FieldKey;
pub use field_meta::FieldMeta;
pub use struct_meta::StructMeta;

use zyn_core::mark::Diagnostic;
use zyn_core::proc_macro2::TokenStream;
use zyn_core::quote::quote;
use zyn_core::syn::DeriveInput;

use super::emit;

pub fn expand(input: DeriveInput) -> TokenStream {
    let struct_meta = match StructMeta::parse(&input.attrs) {
        Ok(m) => m,
        Err(e) => return Diagnostic::from(e).emit(),
    };

    let fields = match FieldMeta::parse(&input) {
        Ok(f) => f,
        Err(e) => return Diagnostic::from(e).emit(),
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let from_args = emit::from_args(name, &fields, &impl_generics, &ty_generics, where_clause);
    let from_arg = emit::from_arg(name, &impl_generics, &ty_generics, where_clause);
    let from_input = if struct_meta.attr_name.is_some() {
        Some(emit::from_input(
            name,
            &struct_meta,
            &impl_generics,
            &ty_generics,
            where_clause,
        ))
    } else {
        None
    };

    let about = if struct_meta.attr_name.is_some() {
        Some(emit::about(
            name,
            &struct_meta,
            &fields,
            &impl_generics,
            &ty_generics,
            where_clause,
        ))
    } else {
        None
    };

    quote! {
        #from_args
        #from_arg
        #from_input
        #about
    }
}
