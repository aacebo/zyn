use criterion::{Criterion, black_box, criterion_group, criterion_main};
use darling::FromDeriveInput;
use quote::quote;
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

fn attr_benchmarks(c: &mut Criterion) {
    let ts = quote! {
        #[my_attr(name = "hello", count = 5)]
        pub struct UserRecord {
            pub user_id: u64,
            pub first_name: String,
        }
    };

    let ast: syn::DeriveInput = syn::parse2(ts.clone()).unwrap();
    let input: zyn_core::Input = syn::parse2(ts.clone()).unwrap();
    let mut group = c.benchmark_group("attr");

    group.bench_function("parse", |b| {
        b.iter(|| black_box(syn::parse2::<syn::DeriveInput>(black_box(ts.clone())).unwrap()))
    });

    group.bench_function("darling", |b| {
        b.iter(|| black_box(DarlingArgs::from_derive_input(black_box(&ast)).unwrap()))
    });

    group.bench_function("zyn", |b| {
        b.iter(|| black_box(ZynArgs::from_input(black_box(&input)).unwrap()))
    });

    group.finish();
}

criterion_group!(benches, attr_benchmarks);
criterion_main!(benches);
