#![feature(test)]

extern crate test;

use darling::FromDeriveInput;
use test::{Bencher, black_box};
use zyn::quote::quote;
use zyn_core::FromInput;

#[derive(darling::FromDeriveInput)]
#[darling(attributes(my_attr))]
#[allow(dead_code)]
struct DarlingArgs {
    name: String,
    count: i64,
}

#[derive(zyn::Attribute)]
#[zyn("my_attr")]
#[allow(dead_code)]
struct ZynArgs {
    name: String,
    count: i64,
}

fn make_ts() -> zyn::proc_macro2::TokenStream {
    quote! {
        #[my_attr(name = "hello", count = 5)]
        pub struct UserRecord {
            pub user_id: u64,
            pub first_name: String,
        }
    }
}

#[bench]
fn attr_parse(b: &mut Bencher) {
    let ts = make_ts();
    b.iter(|| black_box(zyn::syn::parse2::<zyn::syn::DeriveInput>(black_box(ts.clone())).unwrap()))
}

#[bench]
fn attr_darling(b: &mut Bencher) {
    let ts = make_ts();
    let ast: zyn::syn::DeriveInput = zyn::syn::parse2(ts).unwrap();
    b.iter(|| black_box(DarlingArgs::from_derive_input(black_box(&ast)).unwrap()))
}

#[bench]
fn attr_zyn(b: &mut Bencher) {
    let ts = make_ts();
    let input: zyn_core::Input = zyn::syn::parse2(ts).unwrap();
    b.iter(|| black_box(ZynArgs::from_input(black_box(&input)).unwrap()))
}
