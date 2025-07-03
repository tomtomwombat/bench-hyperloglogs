use ahash::RandomState;
use rand::Rng;
use rayon::prelude::*;
use std::sync::{LazyLock, RwLock};
use std::thread;
use std::time::Instant;

mod container;
pub use container::Container;

use hyperloglockless::HyperLogLog;
#[allow(unused_imports)]
use hyperloglogplus::{HyperLogLog as HyperLogLogTrait, HyperLogLogPF};

const NUM_THREADS: usize = 16;
const TOTAL_ITERATIONS: usize = 100_000_000;
const NUM_ITERATIONS: usize = TOTAL_ITERATIONS / NUM_THREADS;
const PRECISION: u8 = 16;

#[allow(dead_code)]
static LOCKED: LazyLock<RwLock<HyperLogLogPF<u64, RandomState>>> = LazyLock::new(|| {
    RwLock::new(HyperLogLogPF::new(PRECISION, RandomState::with_seeds(0, 0, 0, 0)).unwrap())
});

#[allow(dead_code)]
static LOCKLESS: LazyLock<HyperLogLog<RandomState>> =
    LazyLock::new(|| HyperLogLog::with_hasher(PRECISION, RandomState::with_seeds(0, 0, 0, 0)));

pub fn insert(val: u64) {
    //LOCKED.write().unwrap().insert(&val);
    LOCKLESS.insert(&val);
}

pub fn count() -> usize {
    //LOCKED.write().unwrap().count() as usize
    LOCKLESS.count() as usize
}

pub fn perf() {
    let now = Instant::now();
    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|_| {
            thread::spawn(move || {
                let mut rng = rand::thread_rng();
                for _ in 0..NUM_ITERATIONS {
                    insert(rng.gen_range(0..u64::MAX));
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    /*
    (0..NUM_THREADS)
        .into_par_iter()
        .for_each(|_| {
            let mut rng = rand::thread_rng();
            for _ in 0..NUM_ITERATIONS {
                insert(rng.gen_range(0..u64::MAX));
            }
        });
    */
    println!("\nFinal count: {}", count());
    let time = now.elapsed();
    println!("Time: {} ms", time.as_millis());
    println!(
        "Time per insert: {} ns",
        time.as_nanos() as f64 / TOTAL_ITERATIONS as f64
    );
}

#[derive(Clone, Copy, Debug)]
pub enum Step {
    Linear(u64),
    Pow2,
}

pub fn accuarcy<T: Container<u64>>(
    max_size: u64,
    step: Step,
    precision: u8,
) -> impl Iterator<Item = (u64, f64, f64, f64)> {
    let num_trials = 32;
    let data = (1..=num_trials)
        .into_par_iter()
        .map(|_| single_trial_accuarcy::<T>(max_size, step, precision))
        .collect::<Vec<_>>();

    let rows = data[0].len();
    (0..rows).map(move |i| {
        let mut total = 0.0f64;
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for j in 0..num_trials {
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

pub fn single_trial_accuarcy<T: Container<u64>>(
    max_size: u64,
    step: Step,
    precision: u8,
) -> Vec<(u64, f64)> {
    let res_size = match step {
        Step::Linear(s) => max_size.div_ceil(s) as usize,
        Step::Pow2 => max_size.ilog2() as usize,
    };
    let mut res = Vec::with_capacity(res_size);
    let mut hll = T::new(precision);
    for x in 1..=max_size {
        hll.put(&x);
        // if x % 1_000_000_000 == 0 { println!("{} iteration", x); }

        let record = match step {
            Step::Linear(s) => x % s == 0,
            Step::Pow2 => x.is_power_of_two() && x > 1024,
        };
        if record {
            let real = x as f64;
            let diff = (hll.get_count() - real).abs();
            let err = diff / real;
            res.push((x, err));
        }
    }
    res
}
