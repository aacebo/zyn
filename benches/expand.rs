#![feature(test)]

extern crate test;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use test::{Bencher, black_box};
use zyn_core::{Fields, FromInput, Input, Variants};

fn make_input() -> TokenStream {
    quote! {
        pub struct UserRecord {
            pub user_id: u64,
            pub first_name: String,
            pub last_name: String,
            pub is_active: bool,
            pub created_at: u64,
        }
    }
}

fn make_enum_input() -> TokenStream {
    quote! {
        pub enum Status {
            Active,
            Inactive,
            Pending,
            Suspended,
            Deleted,
        }
    }
}

#[bench]
fn parse_vanilla(b: &mut Bencher) {
    let ts = make_input();
    b.iter(|| syn::parse2::<syn::DeriveInput>(black_box(ts.clone())).unwrap())
}

#[bench]
fn parse_zyn(b: &mut Bencher) {
    let ts = make_input();
    b.iter(|| syn::parse2::<Input>(black_box(ts.clone())).unwrap())
}

#[bench]
fn extract_vanilla(b: &mut Bencher) {
    let ts = make_input();
    let derive: syn::DeriveInput = syn::parse2(ts).unwrap();
    b.iter(|| {
        let syn::Data::Struct(ref s) = black_box(&derive).data else {
            panic!()
        };
        let syn::Fields::Named(ref n) = s.fields else {
            panic!()
        };
        black_box(n.clone())
    })
}

#[bench]
fn extract_zyn(b: &mut Bencher) {
    let ts = make_input();
    let input: Input = syn::parse2(ts).unwrap();
    b.iter(|| black_box(Fields::<syn::FieldsNamed>::from_input(black_box(&input)).unwrap()))
}

#[bench]
fn codegen_vanilla(b: &mut Bencher) {
    let ts = make_input();
    let derive: syn::DeriveInput = syn::parse2(ts).unwrap();
    let syn::Data::Struct(ref data) = derive.data else {
        panic!()
    };
    let syn::Fields::Named(ref named) = data.fields else {
        panic!()
    };
    let named = named.clone();
    b.iter(|| {
        let methods = named.named.iter().map(|f| {
            let fname = f.ident.as_ref().unwrap();
            let getter = format_ident!("get_{}", fname);
            let ty = &f.ty;
            quote! { pub fn #getter(&self) -> &#ty { &self.#fname }}
        });
        black_box(quote! { impl UserRecord { #(#methods)* }})
    })
}

#[bench]
fn codegen_zyn(b: &mut Bencher) {
    let ts = make_input();
    let input: Input = syn::parse2(ts).unwrap();
    let fields = Fields::<syn::FieldsNamed>::from_input(&input).unwrap();
    b.iter(|| {
        black_box(zyn::zyn! {
            impl UserRecord {
                @for (field in fields.named.iter()) {
                    pub fn {{ field.ident.as_ref().unwrap() | ident:"get_{}" }}(&self) -> &{{ field.ty }} {
                        &self.{{ field.ident }}
                    }
                }
            }
        })
    })
}

#[bench]
fn full_vanilla(b: &mut Bencher) {
    let ts = make_input();
    b.iter(|| {
        let derive: syn::DeriveInput = syn::parse2(ts.clone()).unwrap();
        let name = &derive.ident;
        let syn::Data::Struct(ref data) = derive.data else {
            panic!()
        };
        let syn::Fields::Named(ref named) = data.fields else {
            panic!()
        };
        let methods = named.named.iter().map(|f| {
            let fname = f.ident.as_ref().unwrap();
            let getter = format_ident!("get_{}", fname);
            let ty = &f.ty;
            quote! { pub fn #getter(&self) -> &#ty { &self.#fname }}
        });
        black_box(quote! { impl #name { #(#methods)* }})
    })
}

#[bench]
fn full_zyn(b: &mut Bencher) {
    let ts = make_input();
    b.iter(|| {
        let input: Input = syn::parse2(ts.clone()).unwrap();
        let name = input.ident();
        let fields = Fields::<syn::FieldsNamed>::from_input(&input).unwrap();
        black_box(zyn::zyn! {
            impl {{ name }} {
                @for (field in fields.named.iter()) {
                    pub fn {{ field.ident.as_ref().unwrap() | ident:"get_{}" }}(&self) -> &{{ field.ty }} {
                        &self.{{ field.ident }}
                    }
                }
            }
        })
    })
}

#[bench]
fn enum_parse_vanilla(b: &mut Bencher) {
    let ts = make_enum_input();
    b.iter(|| syn::parse2::<syn::DeriveInput>(black_box(ts.clone())).unwrap())
}

#[bench]
fn enum_parse_zyn(b: &mut Bencher) {
    let ts = make_enum_input();
    b.iter(|| syn::parse2::<Input>(black_box(ts.clone())).unwrap())
}

#[bench]
fn enum_extract_vanilla(b: &mut Bencher) {
    let ts = make_enum_input();
    let derive: syn::DeriveInput = syn::parse2(ts).unwrap();
    b.iter(|| {
        let syn::Data::Enum(ref e) = black_box(&derive).data else {
            panic!()
        };
        black_box(e.variants.clone())
    })
}

#[bench]
fn enum_extract_zyn(b: &mut Bencher) {
    let ts = make_enum_input();
    let input: Input = syn::parse2(ts).unwrap();
    b.iter(|| black_box(Variants::from_input(black_box(&input)).unwrap()))
}

#[bench]
fn enum_codegen_vanilla(b: &mut Bencher) {
    let ts = make_enum_input();
    let derive: syn::DeriveInput = syn::parse2(ts).unwrap();
    let syn::Data::Enum(ref data) = derive.data else {
        panic!()
    };
    let variants = data.variants.clone();
    b.iter(|| {
        let methods = variants.iter().map(|v| {
            let name = &v.ident;
            let pred = format_ident!("is_{}", name);
            quote! { pub fn #pred(&self) -> bool { matches!(self, Self::#name) }}
        });
        black_box(quote! { impl Status { #(#methods)* }})
    })
}

#[bench]
fn enum_codegen_zyn(b: &mut Bencher) {
    let ts = make_enum_input();
    let input: Input = syn::parse2(ts).unwrap();
    let variants = Variants::from_input(&input).unwrap();
    b.iter(|| {
        black_box(zyn::zyn! {
            impl Status {
                @for (variant in variants.iter()) {
                    pub fn {{ variant.ident | ident:"is_{}" }}(&self) -> bool {
                        matches!(self, Self::{{ variant.ident }})
                    }
                }
            }
        })
    })
}

#[bench]
fn enum_full_vanilla(b: &mut Bencher) {
    let ts = make_enum_input();
    b.iter(|| {
        let derive: syn::DeriveInput = syn::parse2(ts.clone()).unwrap();
        let name = &derive.ident;
        let syn::Data::Enum(ref data) = derive.data else {
            panic!()
        };
        let methods = data.variants.iter().map(|v| {
            let vname = &v.ident;
            let pred = format_ident!("is_{}", vname);
            quote! { pub fn #pred(&self) -> bool { matches!(self, Self::#vname) }}
        });
        black_box(quote! { impl #name { #(#methods)* }})
    })
}

#[bench]
fn enum_full_zyn(b: &mut Bencher) {
    let ts = make_enum_input();
    b.iter(|| {
        let input: Input = syn::parse2(ts.clone()).unwrap();
        let name = input.ident();
        let variants = Variants::from_input(&input).unwrap();
        black_box(zyn::zyn! {
            impl {{ name }} {
                @for (variant in variants.iter()) {
                    pub fn {{ variant.ident | ident:"is_{}" }}(&self) -> bool {
                        matches!(self, Self::{{ variant.ident }})
                    }
                }
            }
        })
    })
}
