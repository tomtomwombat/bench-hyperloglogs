use ahash::RandomState;
use bench_hll::Container;
use criterion::BatchSize;
use criterion::measurement::WallTime;
use criterion::{BenchmarkGroup, Criterion, criterion_group, criterion_main};
use hyperloglogplus::{HyperLogLog as HyperLogLogTrait, HyperLogLogPF, HyperLogLogPlus};
use std::hint::black_box;

const P: u8 = 14;

fn insert_bench<T: Container<u64>>(group: &mut BenchmarkGroup<'_, WallTime>, hll: &mut T) {
    group.bench_function(T::name(), |b| {
        b.iter(|| {
            for x in 0..1000 {
                black_box(hll.put(&x));
            }
        })
    });
}

fn fill_bench<T: Container<u64>>(group: &mut BenchmarkGroup<'_, WallTime>, num: u64) {
    let name = format!("items-{}-{}", num, T::name());
    group.bench_function(name, |b| {
        b.iter_batched(
            || black_box(T::init(P)),
            |mut hll| {
                for x in 0..num {
                    black_box(hll.put(&x));
                }
                let _ = black_box(hll.get_count());
            },
            BatchSize::SmallInput,
        )
    });
}

fn count_bench<T: Container<u64>>(group: &mut BenchmarkGroup<'_, WallTime>, hll: &mut T) {
    group.bench_function(T::name(), |b| b.iter(|| black_box(hll.get_count())));
}

fn bench(c: &mut Criterion) {
    let mut lockless = hyperloglockless::HyperLogLog::<ahash::RandomState>::init(P);
    let mut lockless_atomic = hyperloglockless::AtomicHyperLogLog::init(P);
    let mut lockless_plus = hyperloglockless::HyperLogLogPlus::init(P);

    let mut hll: HyperLogLogPF<u64, _> = HyperLogLogPF::init(P);
    let mut hll_plus: HyperLogLogPlus<u64, _> = HyperLogLogPlus::init(P);

    let mut prob = probabilistic_collections::hyperloglog::HyperLogLog::<u64, RandomState>::init(P);
    let mut card = cardinality_estimator::CardinalityEstimator::<
        u64,
        ahash::AHasher,
        { P as usize },
        6,
    >::init(P);
    let mut amad = amadeus_streaming::HyperLogLog::<u64>::init(P);
    let mut apache = bench_hll::apache_hll::HyperLogLog::<u64>::init(P);

    // 310_000 is where all HLL's have similar accuracy and are using HLL algorithm (e.g. no sparse repr).
    for x in 1000..=310_000 {
        lockless.insert(&x);
        lockless_atomic.insert(&x);
        lockless_plus.insert(&x);
        hll.insert(&x);
        hll_plus.insert(&x);
        prob.insert(&x);
        card.insert(&x);
        amad.push(&x);
        apache.put(&x);
    }
    assert!(!lockless_plus.is_sparse());

    let mut group = c.benchmark_group("Insert");
    insert_bench(&mut group, &mut lockless);
    insert_bench(&mut group, &mut lockless_atomic);
    insert_bench(&mut group, &mut lockless_plus);
    insert_bench(&mut group, &mut hll);
    insert_bench(&mut group, &mut hll_plus);
    insert_bench(&mut group, &mut prob);
    insert_bench(&mut group, &mut card);
    insert_bench(&mut group, &mut amad);
    insert_bench(&mut group, &mut apache);
    group.finish();

    let mut group = c.benchmark_group("Count");
    count_bench(&mut group, &mut lockless);
    count_bench(&mut group, &mut lockless_atomic);
    count_bench(&mut group, &mut lockless_plus);
    count_bench(&mut group, &mut hll);
    count_bench(&mut group, &mut hll_plus);
    count_bench(&mut group, &mut prob);
    count_bench(&mut group, &mut card);
    count_bench(&mut group, &mut amad);
    count_bench(&mut group, &mut apache);
    group.finish();
}

fn bench_fill(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fill");
    let it = PowerIterator::new(2.0f64.sqrt());
    for num in it.into_iter().skip(12).take(31) {
        fill_bench::<hyperloglockless::HyperLogLog<ahash::RandomState>>(&mut group, num);
        fill_bench::<hyperloglockless::HyperLogLogPlus<ahash::RandomState>>(&mut group, num);

        fill_bench::<HyperLogLogPF<u64, _>>(&mut group, num);
        fill_bench::<HyperLogLogPlus<u64, _>>(&mut group, num);

        fill_bench::<
            cardinality_estimator::CardinalityEstimator<u64, ahash::AHasher, { P as usize }, 6>,
        >(&mut group, num);
        fill_bench::<amadeus_streaming::HyperLogLog<u64>>(&mut group, num);

        fill_bench::<probabilistic_collections::hyperloglog::HyperLogLog<u64, ahash::RandomState>>(
            &mut group, num,
        );
        fill_bench::<bench_hll::apache_hll::HyperLogLog<u64>>(&mut group, num);
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_fill, bench,
);
criterion_main!(benches);

struct PowerIterator {
    base: f64,
    current_exponent: u32,
}

impl PowerIterator {
    fn new(base: f64) -> Self {
        Self {
            base,
            current_exponent: 0,
        }
    }
}

impl Iterator for PowerIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Compute base^exponent
        let result = self.base.powi(self.current_exponent as i32);

        // Handle potential overflow or floating point infinity
        if result > u64::MAX as f64 || result.is_infinite() {
            return None;
        }

        self.current_exponent += 1;
        Some(result.round() as u64)
    }
}
