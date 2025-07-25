use ahash::RandomState;
use rand::Rng;
use rayon::prelude::*;
use std::sync::{LazyLock, RwLock};
use std::thread;
use std::time::Instant;

mod container;
pub use container::Container;

use hyperloglockless::{AtomicHyperLogLog, HyperLogLog};
#[allow(unused_imports)]
use hyperloglogplus::{HyperLogLog as HyperLogLogTrait, HyperLogLogPF, HyperLogLogPlus};

const TOTAL_ITERATIONS: usize = 1_00_000_000;
const PRECISION: u8 = 10;

#[allow(dead_code)]
static HYPERLOGLOG: LazyLock<RwLock<HyperLogLogPF<u64, RandomState>>> = LazyLock::new(|| {
    RwLock::new(HyperLogLogPF::new(PRECISION, RandomState::with_seeds(0, 0, 0, 0)).unwrap())
});

#[allow(dead_code)]
static HYPERLOGLOGPLUS: LazyLock<RwLock<HyperLogLogPlus<u64, RandomState>>> = LazyLock::new(|| {
    RwLock::new(HyperLogLogPlus::new(PRECISION, RandomState::with_seeds(0, 0, 0, 0)).unwrap())
});

#[allow(dead_code)]
static CARD: LazyLock<
    RwLock<cardinality_estimator::CardinalityEstimator<u64, ahash::AHasher, 16, 6>>,
> = LazyLock::new(|| {
    RwLock::new(cardinality_estimator::CardinalityEstimator::<
        u64,
        ahash::AHasher,
        16,
        6,
    >::new())
});

#[allow(dead_code)]
static PROB: LazyLock<
    RwLock<probabilistic_collections::hyperloglog::HyperLogLog<u64, ahash::RandomState>>,
> = LazyLock::new(|| {
    RwLock::new(probabilistic_collections::hyperloglog::HyperLogLog::<
        u64,
        ahash::RandomState,
    >::init(PRECISION))
});

#[allow(dead_code)]
static AM: LazyLock<RwLock<amadeus_streaming::HyperLogLog<u64>>> =
    LazyLock::new(|| RwLock::new(amadeus_streaming::HyperLogLog::<u64>::new(0.005)));

#[allow(dead_code)]
static LOCKLESS: LazyLock<AtomicHyperLogLog<RandomState>> =
    LazyLock::new(|| AtomicHyperLogLog::init(PRECISION));

#[allow(dead_code)]
static LOCKED: LazyLock<RwLock<HyperLogLog<RandomState>>> =
    LazyLock::new(|| RwLock::new(HyperLogLog::init(PRECISION)));

pub fn insert(val: u64) {
    //HYPERLOGLOG.write().unwrap().insert(&val);
    //HYPERLOGLOGPLUS.write().unwrap().insert(&val);
    //CARD.write().unwrap().insert(&val);
    //PROB.write().unwrap().insert(&val);
    //AM.write().unwrap().push(&val);
    LOCKED.write().unwrap().insert(&val);
    // LOCKLESS.insert(&val);
}

pub fn count() -> usize {
    //HYPERLOGLOG.write().unwrap().count() as usize
    //HYPERLOGLOGPLUS.write().unwrap().count() as usize
    //CARD.read().unwrap().estimate() as usize
    //PROB.read().unwrap().len() as usize
    //AM.read().unwrap().len() as usize
    LOCKED.read().unwrap().count() as usize
    // LOCKLESS.count() as usize
}

pub fn perf() {
    perf_inner(16, TOTAL_ITERATIONS);
}

fn perf_inner(num_threads: usize, total_iterations_count: usize) {
    let num_iterations = TOTAL_ITERATIONS / num_threads;
    let num_iterations_count = total_iterations_count / num_threads;

    let now = Instant::now();
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            thread::spawn(move || {
                let mut rng = rand::thread_rng();
                for _ in 0..num_iterations {
                    insert(rng.gen_range(0..u64::MAX));
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nFinal count: {}", count());
    let time = now.elapsed();
    println!("Time: {} ms", time.as_millis());
    println!(
        "Time per insert: {} ns",
        time.as_nanos() as f64 / TOTAL_ITERATIONS as f64
    );

    let now = Instant::now();
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            thread::spawn(move || {
                for _ in 0..num_iterations_count {
                    let _ = count();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let time = now.elapsed();
    println!("Time: {} ms", time.as_millis());
    println!(
        "Time per count: {} ns",
        time.as_nanos() as f64 / total_iterations_count as f64
    );
}

#[derive(Clone, Copy, Debug)]
pub enum Step {
    Linear(u64),
    Pow2(u32),
}

pub fn accuarcy<T: Container<u64>>(
    max_size: u64,
    step: Step,
    precision: u8,
) -> impl Iterator<Item = (u64, f64, f64, f64)> {
    let num_trials: u64 = 16;
    let data = (0..num_trials)
        .into_par_iter()
        .map(|offset| {
            single_trial_accuarcy::<T>(
                max_size,
                step,
                precision,
                offset.wrapping_mul(u64::MAX / num_trials),
            )
        })
        .collect::<Vec<_>>();

    let rows = min_len(&data);
    (0..rows).map(move |i| {
        let mut total = 0.0f64;
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for j in 0..num_trials as usize {
            let err = data[j][i].1;
            total += err;
            if err < min {
                min = err;
            }
            if err > max {
                max = err;
            }
        }

        (data[0][i].0, total / num_trials as f64, min, max)
    })
}

fn min_len<T>(vecs: &[Vec<T>]) -> usize {
    vecs.iter().min_by_key(|v| v.len()).unwrap().len()
}

pub fn single_trial_accuarcy<T: Container<u64>>(
    max_size: u64,
    step: Step,
    precision: u8,
    offset: u64,
) -> Vec<(u64, f64)> {
    let res_size = match step {
        Step::Linear(s) => max_size.div_ceil(s) as usize,
        Step::Pow2(s) => ((1 << s) * max_size.ilog2()) as usize,
    };
    let mut res = Vec::with_capacity(res_size);
    let mut hll = T::init(precision);
    for x in 1..=max_size {
        hll.put(&(x + offset));

        let record = match step {
            Step::Linear(s) => x % s == 0,
            Step::Pow2(s) => {
                let ilog = x.ilog2();

                if ilog > s {
                    let step = 1 << (ilog - s);
                    x % step == 0
                } else {
                    false
                }
            }
        };
        if record {
            let real = x as f64;
            let diff = (hll.get_count() - real).abs();
            let err = diff / real;
            res.push((x, err));

            //if x.is_power_of_two() {
            //    print!("{},", x);
            //}

            if err > 1000.0 {
                break;
            }
        }
    }
    res
}
