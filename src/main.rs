use bench_hll::*;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

// https://crates.io/crates/hyperloglog-rs

fn write_acc_data<T: Container<u64>>(precision: u8) -> std::io::Result<()> {
    let now = Instant::now();
    let res = accuarcy::<T>(1_000_000_000, Step::Pow2(6), precision);
    let mut file = File::create(format!("Acc/{}.csv", T::name()))?;
    for (num_items, avg, min, max) in res {
        let row = format!("{},{},{},{}\n", num_items, avg, min, max);
        file.write_all(row.as_bytes())?;
    }
    println!(
        "{} complete in {} seconds",
        T::name(),
        now.elapsed().as_secs()
    );
    Ok(())
}

fn main() {
    //for p in 12..=16 {
    //    write_acc_data::<hyperloglockless::HyperLogLog<ahash::RandomState>>(p).unwrap();
    //}
    perf();

    let p = 16;
    //write_acc_data::<hyperloglockless::HyperLogLog<ahash::RandomState>>(p).unwrap();
    //write_acc_data::<hyperloglogplus::HyperLogLogPF<u64, ahash::RandomState>>(p).unwrap();
    //write_acc_data::<hyperloglogplus::HyperLogLogPlus<u64, ahash::RandomState>>(p).unwrap();
    //write_acc_data::<cardinality_estimator::CardinalityEstimator<u64, crate::RandomSeedAHasher, 16, 6>>(p).unwrap();
    //write_acc_data::<amadeus_streaming::HyperLogLog<u64>>(p).unwrap();

    // This produces `inf` error rate quickly
    //write_acc_data::<probabilistic_collections::hyperloglog::HyperLogLog::<u64, ahash::RandomState>>(p).unwrap();

    // This crashes index out of bound
    // write_acc_data::<hyperloglog::HyperLogLog>(16).unwrap();
}
