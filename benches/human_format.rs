#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_3rd_crate(c: &mut Criterion) {
    let mut group = c.benchmark_group("Main");

    let human_format = human_format::Formatter::new();
    let human_format_next = human_format_next::Formatter::SI;

    group.bench_function("human_format/0", |b| {
        b.iter(|| std::hint::black_box(human_format.format(0.0)));
    });
    group.bench_function("human_format/u32::MAX", |b| {
        b.iter(|| std::hint::black_box(human_format.format(u32::MAX as f64)));
    });
    group.bench_function("human_format_next/0", |b| {
        b.iter(|| std::hint::black_box(human_format_next.format_float(0.0)));
    });
    group.bench_function("human_format_next/u32::MAX", |b| {
        b.iter(|| std::hint::black_box(human_format_next.format_float(u32::MAX as f64)));
    });
}

criterion_group!(benches, bench_3rd_crate);
criterion_main!(benches);
