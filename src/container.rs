use hyperloglogplus::HyperLogLog as _;
use std::hash::BuildHasher;
use std::hash::Hash;

pub trait Container<X: Hash> {
    fn put(&mut self, s: &X);
    fn get_count(&mut self) -> f64;
    fn init(precision: u8) -> Self;
    fn name() -> &'static str;
}

impl<S: BuildHasher + Default> Container<u64> for hyperloglockless::HyperLogLog<S> {
    #[inline]
    fn put(&mut self, s: &u64) {
        self.insert(s);
    }
    #[inline]
    fn get_count(&mut self) -> f64 {
        self.raw_count()
    }
    fn init(precision: u8) -> Self {
        hyperloglockless::HyperLogLog::with_hasher(precision, S::default())
    }
    fn name() -> &'static str {
        "hyperloglockless::HyperLogLog"
    }
}

impl Container<u64> for hyperloglockless::HyperLogLogPlus<ahash::RandomState> {
    #[inline]
    fn put(&mut self, s: &u64) {
        self.insert(s);
    }
    #[inline]
    fn get_count(&mut self) -> f64 {
        self.raw_count()
    }
    fn init(precision: u8) -> Self {
        hyperloglockless::HyperLogLogPlus::with_hasher(precision, ahash::RandomState::default())
    }
    fn name() -> &'static str {
        "hyperloglockless::HyperLogLogPlus"
    }
}

impl Container<u64> for hyperloglockless::AtomicHyperLogLog<ahash::RandomState> {
    #[inline]
    fn put(&mut self, s: &u64) {
        self.insert(s);
    }
    #[inline]
    fn get_count(&mut self) -> f64 {
        self.raw_count()
    }
    fn init(precision: u8) -> Self {
        hyperloglockless::AtomicHyperLogLog::with_hasher(precision, ahash::RandomState::default())
    }
    fn name() -> &'static str {
        "hyperloglockless::AtomicHyperLogLog"
    }
}

impl Container<u64> for hyperloglogplus::HyperLogLogPlus<u64, ahash::RandomState> {
    #[inline]
    fn put(&mut self, s: &u64) {
        self.insert(s);
    }
    #[inline]
    fn get_count(&mut self) -> f64 {
        self.count()
    }
    fn init(precision: u8) -> Self {
        hyperloglogplus::HyperLogLogPlus::new(precision, ahash::RandomState::default()).unwrap()
    }
    fn name() -> &'static str {
        "hyperloglogplus::HyperLogLogPlus"
    }
}

impl Container<u64> for hyperloglogplus::HyperLogLogPF<u64, ahash::RandomState> {
    #[inline]
    fn put(&mut self, s: &u64) {
        self.insert(s);
    }
    #[inline]
    fn get_count(&mut self) -> f64 {
        self.count()
    }
    fn init(precision: u8) -> Self {
        hyperloglogplus::HyperLogLogPF::new(precision, ahash::RandomState::default()).unwrap()
    }
    fn name() -> &'static str {
        "hyperloglogplus::HyperLogLogPF"
    }
}

impl Container<u64>
    for probabilistic_collections::hyperloglog::HyperLogLog<u64, ahash::RandomState>
{
    #[inline]
    fn put(&mut self, s: &u64) {
        self.insert(s);
    }
    #[inline]
    fn get_count(&mut self) -> f64 {
        self.len()
    }
    fn init(precision: u8) -> Self {
        // let p = (1.04 / error_probability).powi(2).ln().ceil() as usize;
        let err = 1.04 / (2.71f64.powf(precision as f64)).sqrt();
        let p = (1.04 / err).powi(2).ln().ceil() as usize;
        assert_eq!(precision, p as u8);
        probabilistic_collections::hyperloglog::HyperLogLog::<u64, ahash::RandomState>::with_hasher(
            err,
            ahash::RandomState::default(),
        )
    }
    fn name() -> &'static str {
        "probabilistic_collections::HyperLogLog"
    }
}

impl Container<u64> for hyperloglog::HyperLogLog {
    #[inline]
    fn put(&mut self, s: &u64) {
        self.insert(s);
    }
    #[inline]
    fn get_count(&mut self) -> f64 {
        self.len()
    }
    fn init(precision: u8) -> Self {
        let error_rate: f64 = 0.0005;
        let sr = 1.04 / error_rate;
        let p = f64::ln(sr * sr).ceil() as u8;
        assert_eq!(precision, p as u8);
        hyperloglog::HyperLogLog::new(error_rate)
    }
    fn name() -> &'static str {
        "hyperloglog::HyperLogLog"
    }
}

macro_rules! impl_card {
    ($size:literal) => {
        impl Container<u64>
            for cardinality_estimator::CardinalityEstimator<u64, ahash::AHasher, $size, 6>
        {
            #[inline]
            fn put(&mut self, s: &u64) {
                self.insert(s);
            }
            #[inline]
            fn get_count(&mut self) -> f64 {
                self.estimate() as f64
            }
            fn init(precision: u8) -> Self {
                assert_eq!(precision, $size);
                cardinality_estimator::CardinalityEstimator::<u64, ahash::AHasher, $size, 6>::new()
            }
            fn name() -> &'static str {
                "cardinality_estimator::CardinalityEstimator"
            }
        }
    };
}

impl_card!(16);
impl_card!(14);
impl_card!(12);
impl_card!(10);

impl Container<u64> for amadeus_streaming::HyperLogLog<u64> {
    #[inline]
    fn put(&mut self, s: &u64) {
        self.push(s);
    }
    #[inline]
    fn get_count(&mut self) -> f64 {
        self.len() as f64
    }
    fn init(precision: u8) -> Self {
        let err = 1.04 / 2.0f64.powf(precision as f64 / 2.0);
        assert_eq!((f64::log2(1.04 / err) * 2.0).ceil() as u8, precision);
        amadeus_streaming::HyperLogLog::<u64>::new(err)
    }
    fn name() -> &'static str {
        "amadeus_streaming::HyperLogLog"
    }
}

impl Container<u64> for crate::apache_hll::HyperLogLog<u64> {
    #[inline]
    fn put(&mut self, s: &u64) {
        self.add(s);
    }
    #[inline]
    fn get_count(&mut self) -> f64 {
        self.count() as f64
    }
    fn init(precision: u8) -> Self {
        assert_eq!(precision, 14);
        crate::apache_hll::HyperLogLog::<u64>::new()
    }
    fn name() -> &'static str {
        "apache_datafusion::HyperLogLog"
    }
}
