use bench_hll::*;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn write_acc_data<T: Container<u64>>(precision: u8) -> std::io::Result<()> {
    let now = Instant::now();
    let res = accuarcy::<T>(10_000_000_000, Step::Linear(100_000), precision);
    // let res = accuarcy::<T>(1 << 35, Step::Pow2, precision);
    // let mut file = File::create(format!("Acc/{} ({} bytes).csv", T::name(), 1 << precision))?;
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
    perf()

    //write_acc_data::<hyperloglockless::HyperLogLog<ahash::RandomState>>(16).unwrap();
    //write_acc_data::<hyperloglogplus::HyperLogLogPF<u64, ahash::RandomState>>(16).unwrap();

    // This produces `inf` error rate
    // write_acc_data::<probabilistic_collections::hyperloglog::HyperLogLog::<u64, ahash::RandomState>>().unwrap();

    // This crashes index out of bound
    // write_acc_data::<hyperloglog::HyperLogLog>().unwrap();
}
