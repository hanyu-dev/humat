#![allow(missing_docs)]
#![allow(deprecated)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::unreadable_literal)]

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_humat(c: &mut Criterion) {
    let mut group = c.benchmark_group("humat");

    let formatter = humat::Formatter::SI;

    macro_rules! bench_number {
        ($($expr:expr),*) => {
            $(
                group.bench_function(stringify!($expr), |b| {
                    b.iter(|| formatter.format(black_box($expr)).to_string());
                });
            )*
        };
    }

    bench_number!(
        0_u64,
        0xF_u64,
        0xFF_u64,
        0xFFF_u64,
        0xFFFF_u64,
        0xFFFFF_u64,
        0xFFFFFF_u64,
        0xFFFFFFF_u64,
        0xFFFFFFFF_u64,
        0xFFFFFFFFF_u64,
        0xFFFFFFFFFF_u64,
        0xFFFFFFFFFFF_u64,
        0xFFFFFFFFFFFF_u64,
        0xFFFFFFFFFFFFF_u64,
        0xFFFFFFFFFFFFFF_u64,
        0xFFFFFFFFFFFFFFF_u64,
        0xFFFFFFFFFFFFFFFF_u64,
        0_u64 as f64,
        0xF_u64 as f64,
        0xFF_u64 as f64,
        0xFFF_u64 as f64,
        0xFFFF_u64 as f64,
        0xFFFFF_u64 as f64,
        0xFFFFFF_u64 as f64,
        0xFFFFFFF_u64 as f64,
        0xFFFFFFFF_u64 as f64,
        0xFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFFFFFFF_u64 as f64
    );
}

fn bench_human_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("human_format");

    let formatter = human_format::Formatter::new();

    macro_rules! bench_number {
        ($($expr:expr),*) => {
            $(
                group.bench_function(stringify!($expr), |b| {
                    b.iter(|| formatter.format(black_box($expr)).to_string());
                });
            )*
        };
    }

    bench_number!(
        0_u64 as f64,
        0xF_u64 as f64,
        0xFF_u64 as f64,
        0xFFF_u64 as f64,
        0xFFFF_u64 as f64,
        0xFFFFF_u64 as f64,
        0xFFFFFF_u64 as f64,
        0xFFFFFFF_u64 as f64,
        0xFFFFFFFF_u64 as f64,
        0xFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFFFFFF_u64 as f64,
        0xFFFFFFFFFFFFFFFF_u64 as f64
    );
}

criterion_group!(benches, bench_humat, bench_human_format);
criterion_main!(benches);
