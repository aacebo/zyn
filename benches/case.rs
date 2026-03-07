use criterion::{Criterion, black_box, criterion_group, criterion_main};
use heck::{ToLowerCamelCase, ToPascalCase, ToSnakeCase};

const INPUT: &str = "first_name_field";

fn snake(c: &mut Criterion) {
    let mut group = c.benchmark_group("snake");
    group.bench_function("heck", |b| b.iter(|| black_box(INPUT).to_snake_case()));
    group.bench_function("zyn", |b| {
        b.iter(|| zyn_core::case::to_snake(black_box(INPUT)))
    });
    group.finish();
}

fn pascal(c: &mut Criterion) {
    let mut group = c.benchmark_group("pascal");
    group.bench_function("heck", |b| b.iter(|| black_box(INPUT).to_pascal_case()));
    group.bench_function("zyn", |b| {
        b.iter(|| zyn_core::case::to_pascal(black_box(INPUT)))
    });
    group.finish();
}

fn camel(c: &mut Criterion) {
    let mut group = c.benchmark_group("camel");
    group.bench_function("heck", |b| {
        b.iter(|| black_box(INPUT).to_lower_camel_case())
    });
    group.bench_function("zyn", |b| {
        b.iter(|| zyn_core::case::to_camel(black_box(INPUT)))
    });
    group.finish();
}

criterion_group!(benches, snake, pascal, camel);
criterion_main!(benches);
