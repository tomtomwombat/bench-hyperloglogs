use criterion::{Criterion, criterion_group, criterion_main};

use ahash::RandomState;
use hyperloglogplus::{HyperLogLog as HyperLogLogTrait, HyperLogLogPF};

fn bench(c: &mut Criterion) {
    let h1 = hyperloglockless::HyperLogLog::with_hasher(16, RandomState::with_seeds(0, 0, 0, 0));
    let mut h2: HyperLogLogPF<u64, _> =
        HyperLogLogPF::new(16, RandomState::with_seeds(0, 0, 0, 0)).unwrap();

    for x in 1..=1_000_000 {
        h1.insert(&x);
        h2.insert(&x);
    }
    println!("{:?}", h1.raw_count());
    c.bench_function("hyperloglockless insert", |b| {
        b.iter(|| {
            for x in 0..1000 {
                h1.insert(&x);
            }
        })
    });

    c.bench_function("hyperloglogplus insert", |b| {
        b.iter(|| {
            for x in 0..1000 {
                h2.insert(&x);
            }
        })
    });

    let mut h3 =
        probabilistic_collections::hyperloglog::HyperLogLog::<u64, RandomState>::with_hasher(
            0.0005,
            RandomState::with_seeds(0, 0, 0, 0),
        );
    c.bench_function("probabilistic-collections insert", |b| {
        b.iter(|| {
            for x in 0..1000 {
                h3.insert(&x);
            }
        })
    });

    c.bench_function("hyperloglockless count", |b| b.iter(|| h1.raw_count()));
    c.bench_function("hyperloglogplus count", |b| b.iter(|| h2.count()));
    c.bench_function("probabilistic-collections count", |b| b.iter(|| h3.len()));
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench
);
criterion_main!(benches);
