use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fast_modulo::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("reference::mulmod_u64", |b| {
        b.iter(|| {
            reference::mulmod_u64(
                black_box(3141592652589793238),
                black_box(4626433832795028841),
                black_box(9716939937510582097),
            )
        })
    });
    c.bench_function("mulmod_u64", |b| {
        b.iter(|| {
            mulmod_u64(
                black_box(3141592652589793238),
                black_box(4626433832795028841),
                black_box(9716939937510582097),
            )
        })
    });
    c.bench_function("reference::mod_u128u64", |b| {
        b.iter(|| {
            reference::mod_u128u64(
                black_box(314159265358979323846264338327950288419),
                black_box(7169399375105820974),
            )
        })
    });
    c.bench_function("mod_u128u64", |b| {
        b.iter(|| {
            mod_u128u64(
                black_box(314159265358979323846264338327950288419),
                black_box(7169399375105820974),
            )
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_secs(10))
        .measurement_time(std::time::Duration::from_secs(60));
    targets = criterion_benchmark
}
criterion_main!(benches);
