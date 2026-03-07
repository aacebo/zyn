use criterion::{Criterion, black_box, criterion_group, criterion_main};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use zyn_core::{Fields, FromInput, Input, Pipe, pipes};

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

fn parse_group(c: &mut Criterion) {
    let ts = make_input();
    let mut group = c.benchmark_group("parse");

    group.bench_function("vanilla", |b| {
        b.iter(|| syn::parse2::<syn::DeriveInput>(black_box(ts.clone())).unwrap())
    });

    group.bench_function("zyn", |b| {
        b.iter(|| syn::parse2::<Input>(black_box(ts.clone())).unwrap())
    });

    group.finish();
}

fn extract_group(c: &mut Criterion) {
    let ts = make_input();
    let derive: syn::DeriveInput = syn::parse2(ts.clone()).unwrap();
    let input: Input = syn::parse2(ts).unwrap();
    let mut group = c.benchmark_group("extract");

    group.bench_function("vanilla", |b| {
        b.iter(|| {
            let syn::Data::Struct(ref s) = black_box(&derive).data else {
                panic!()
            };

            let syn::Fields::Named(ref n) = s.fields else {
                panic!()
            };

            black_box(n.clone())
        })
    });

    group.bench_function("zyn", |b| {
        b.iter(|| black_box(Fields::<syn::FieldsNamed>::from_input(black_box(&input)).unwrap()))
    });

    group.finish();
}

fn codegen_group(c: &mut Criterion) {
    let ts = make_input();
    let derive: syn::DeriveInput = syn::parse2(ts.clone()).unwrap();
    let input: Input = syn::parse2(ts).unwrap();
    let syn::Data::Struct(ref data) = derive.data else {
        panic!()
    };
    let syn::Fields::Named(ref named) = data.fields else {
        panic!()
    };
    let fields = Fields::<syn::FieldsNamed>::from_input(&input).unwrap();
    let mut group = c.benchmark_group("codegen");

    group.bench_function("vanilla", |b| {
        b.iter(|| {
            let methods = named.named.iter().map(|f| {
                let fname = f.ident.as_ref().unwrap();
                let getter = format_ident!("get_{}", fname);
                let ty = &f.ty;
                quote! { pub fn #getter(&self) -> &#ty { &self.#fname } }
            });

            black_box(quote! { impl UserRecord { #(#methods)* } })
        })
    });

    group.bench_function("zyn", |b| {
        b.iter(|| {
            let methods = fields.named.iter().map(|f| {
                let fname = f.ident.as_ref().unwrap();
                let getter = pipes::Ident("get_{}").pipe(fname.to_string());
                let ty = &f.ty;
                quote! { pub fn #getter(&self) -> &#ty { &self.#fname } }
            });

            black_box(quote! { impl UserRecord { #(#methods)* } })
        })
    });

    group.finish();
}

fn full_group(c: &mut Criterion) {
    let ts = make_input();
    let mut group = c.benchmark_group("full");

    group.bench_function("vanilla", |b| {
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
                quote! { pub fn #getter(&self) -> &#ty { &self.#fname } }
            });

            black_box(quote! { impl #name { #(#methods)* } })
        })
    });

    group.bench_function("zyn", |b| {
        b.iter(|| {
            let input: Input = syn::parse2(ts.clone()).unwrap();
            let name = input.ident();
            let fields = Fields::<syn::FieldsNamed>::from_input(&input).unwrap();
            let methods = fields.named.iter().map(|f| {
                let fname = f.ident.as_ref().unwrap();
                let getter = pipes::Ident("get_{}").pipe(fname.to_string());
                let ty = &f.ty;
                quote! { pub fn #getter(&self) -> &#ty { &self.#fname } }
            });

            black_box(quote! { impl #name { #(#methods)* } })
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    parse_group,
    extract_group,
    codegen_group,
    full_group
);
criterion_main!(benches);
