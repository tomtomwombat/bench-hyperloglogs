use ahash::RandomState;
use bench_hll::Container;
use criterion::measurement::WallTime;
use criterion::{BenchmarkGroup, Criterion, criterion_group, criterion_main};
use hyperloglogplus::{HyperLogLog as HyperLogLogTrait, HyperLogLogPF, HyperLogLogPlus};
const P: u8 = 10;

fn insert_bench<T: Container<u64>>(group: &mut BenchmarkGroup<'_, WallTime>, hll: &mut T) {
    group.bench_function(T::name(), |b| {
        b.iter(|| {
            for x in 0..1000 {
                hll.put(&x);
            }
        })
    });
}

fn count_bench<T: Container<u64>>(group: &mut BenchmarkGroup<'_, WallTime>, hll: &mut T) {
    group.bench_function(T::name(), |b| {
        b.iter(|| std::hint::black_box(hll.get_count()))
    });
}

fn bench(c: &mut Criterion) {
    let mut lockless = hyperloglockless::HyperLogLog::init(P);
    let mut lockless_atomic = hyperloglockless::AtomicHyperLogLog::init(P);
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

    // 310_000 is where all HLL's have similar accuracy and are using HLL algorithm (e.g. no sparse repr).
    for x in 1000..=310_000 {
        lockless.insert(&x);
        lockless_atomic.insert(&x);
        hll.insert(&x);
        hll_plus.insert(&x);
        prob.insert(&x);
        card.insert(&x);
        amad.push(&x);
    }
    println!("{:?}", lockless.raw_count());

    let mut group = c.benchmark_group("Insert");
    insert_bench(&mut group, &mut lockless);
    insert_bench(&mut group, &mut lockless_atomic);
    insert_bench(&mut group, &mut hll);
    insert_bench(&mut group, &mut hll_plus);
    insert_bench(&mut group, &mut prob);
    insert_bench(&mut group, &mut card);
    insert_bench(&mut group, &mut amad);
    group.finish();

    let mut group = c.benchmark_group("Count");
    count_bench(&mut group, &mut lockless);
    count_bench(&mut group, &mut lockless_atomic);
    count_bench(&mut group, &mut hll);
    count_bench(&mut group, &mut hll_plus);
    count_bench(&mut group, &mut prob);
    count_bench(&mut group, &mut card);
    count_bench(&mut group, &mut amad);
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench
);
criterion_main!(benches);
