use bumpalo::Bump;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dod_benchmarks::{dod, standard};

const INPUT: &str = include_str!("/home/folkertdev/rust/dod-benchmarks/data.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    let arena = Bump::new();
    c.bench_function("parse standard", |b| {
        b.iter(|| standard::parser(&arena, black_box(INPUT)))
    });

    let arena = Bump::new();
    c.bench_function("parse dod", |b| {
        b.iter(|| dod::parser(&arena, black_box(INPUT)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
