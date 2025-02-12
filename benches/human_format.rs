#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_3rd_crate(c: &mut Criterion) {
    let mut group = c.benchmark_group("Main");

    let human_format = human_format::Formatter::new();
    let human_format_next = human_format_next::Formatter::SI;
    let human_format_next_old = human_format_next_old::Formatter::SI;

    group.bench_function("human_format/0", |b| {
        b.iter(|| std::hint::black_box(human_format.format(0.0)));
    });
    group.bench_function("human_format_next/v0.1.0/0", |b| {
        b.iter(|| std::hint::black_box(human_format_next_old.format(0, None).to_string()));
    });
    group.bench_function("human_format_next/0", |b| {
        b.iter(|| std::hint::black_box(human_format_next.format_general(0, None).to_string()));
    });

    let u32_max_f64 = 0xFFFFFFFFu32 as f64;
    group.bench_function("human_format/u32::MAX", |b| {
        b.iter(|| std::hint::black_box(human_format.format(u32_max_f64)));
    });
    group.bench_function("human_format_next/v0.1.0/u32::MAX", |b| {
        b.iter(|| std::hint::black_box(human_format_next_old.format(0xFFFFFFFF, None).to_string()));
    });
    group.bench_function("human_format_next/u32::MAX", |b| {
        b.iter(|| std::hint::black_box(human_format_next.format_general(0xFFFFFFFF, None).to_string()));
    });
}

fn bench_perf(c: &mut Criterion) {
    let mut group = c.benchmark_group("Perf");

    let human_format_next = human_format_next::Formatter::BINARY;

    macro_rules! bench_number {
        ($($expr:expr),*) => {
            $(
                group.bench_function(concat!("human_format_next/", stringify!($expr)), |b| {
                    b.iter(|| std::hint::black_box(human_format_next.format($expr).to_string()));
                });
                group.bench_function(concat!("human_format_next/", stringify!($expr), "/large"), |b| {
                    b.iter(|| std::hint::black_box(human_format_next.format($expr).to_string()));
                });
            )*
        };
    }

    bench_number!(
        0_u128,
        0xF_u128,
        0xFF_u128,
        0xFFF_u128,
        0xFFFF_u128,
        0xFFFFF_u128,
        0xFFFFFF_u128,
        0xFFFFFFF_u128,
        0xFFFFFFFF_u128,
        0xFFFFFFFFF_u128,
        0xFFFFFFFFFF_u128,
        0xFFFFFFFFFFF_u128,
        0xFFFFFFFFFFFF_u128,
        0xFFFFFFFFFFFFF_u128,
        0xFFFFFFFFFFFFFF_u128,
        0xFFFFFFFFFFFFFFF_u128,
        0xFFFFFFFFFFFFFFFF_u128
    );
}

criterion_group!(benches, bench_3rd_crate, bench_perf);
criterion_main!(benches);
