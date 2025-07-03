use hyperloglogplus::HyperLogLog as _;
use std::hash::Hash;

pub trait Container<X: Hash> {
    fn put(&mut self, s: &X);
    fn get_count(&mut self) -> f64;
    fn new(precision: u8) -> Self;
    fn name() -> &'static str;
}

impl Container<u64> for hyperloglockless::HyperLogLog<ahash::RandomState> {
    fn put(&mut self, s: &u64) {
        self.insert(s);
    }
    fn get_count(&mut self) -> f64 {
        self.raw_count()
    }
    fn new(precision: u8) -> Self {
        hyperloglockless::HyperLogLog::with_hasher(precision, ahash::RandomState::default())
    }
    fn name() -> &'static str {
        "hyperloglockless"
    }
}

impl Container<u64> for hyperloglogplus::HyperLogLogPF<u64, ahash::RandomState> {
    fn put(&mut self, s: &u64) {
        self.insert(s);
    }
    fn get_count(&mut self) -> f64 {
        self.count()
    }
    fn new(precision: u8) -> Self {
        hyperloglogplus::HyperLogLogPF::new(precision, ahash::RandomState::default()).unwrap()
    }
    fn name() -> &'static str {
        "hyperloglogplus"
    }
}

impl Container<u64>
    for probabilistic_collections::hyperloglog::HyperLogLog<u64, ahash::RandomState>
{
    fn put(&mut self, s: &u64) {
        self.insert(s);
    }
    fn get_count(&mut self) -> f64 {
        self.len()
    }
    fn new(precision: u8) -> Self {
        let error_probability: f64 = 0.0005;
        let p = (1.04 / error_probability).powi(2).ln().ceil() as usize;
        assert_eq!(precision, p as u8);
        probabilistic_collections::hyperloglog::HyperLogLog::<u64, ahash::RandomState>::with_hasher(
            error_probability,
            ahash::RandomState::default(),
        )
    }
    fn name() -> &'static str {
        "probabilistic-collections"
    }
}

impl Container<u64> for hyperloglog::HyperLogLog {
    fn put(&mut self, s: &u64) {
        self.insert(s);
    }
    fn get_count(&mut self) -> f64 {
        self.len()
    }
    fn new(precision: u8) -> Self {
        let error_rate: f64 = 0.0005;
        let sr = 1.04 / error_rate;
        let p = f64::ln(sr * sr).ceil() as u8;
        assert_eq!(precision, p as u8);
        hyperloglog::HyperLogLog::new(error_rate)
    }
    fn name() -> &'static str {
        "hyperloglog"
    }
}
