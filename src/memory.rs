#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unsafe_op_in_unsafe_fn)]

use std::hash::{Hash, Hasher};
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::collections::HashSet;
use std::time::Instant;
use std::io::Write;

use crate::container::Container;

const PRECISION: u8 = 14;


fn allocator_memory<T: Container<u64>>() {
    
    ALLOCATED.store(0, Ordering::Relaxed);
    let now = Instant::now();

    const N: u64 = 10000;
    let mut memory: [usize; N as usize] = [0; N as usize];
    let mut set = T::init(PRECISION);
    memory[0] = ALLOCATED.load(Ordering::SeqCst);
    let items: [u64; N as usize] = core::array::from_fn(|i| i as u64);
    set.put_all(items.iter().map(|x| {
        memory[*x as usize] = ALLOCATED.load(Ordering::SeqCst);
        x
    }));

    let name = format!("{}.csv", T::name().replace(":", "_"));
    println!("Writing to {:?}", name);
    let mut file = std::fs::File::create(name).unwrap();
    for items in 0..N {
        let row = format!("{},{}\n", items, memory[items as usize]);
        file.write_all(row.as_bytes()).unwrap();
    }
    println!("Count: {:?}", set.get_count());
    println!("Done in {} s.", now.elapsed().as_millis() as f64 / 1000.0);
}

#[test]
fn memory() {

    //let mut set: hyperloglogplus::HyperLogLogPlus<u64, ahash::RandomState> = hyperloglogplus::HyperLogLogPlus::new(PRECISION,  ahash::RandomState::default()).unwrap();
    //let mut set = hyperloglockless::HyperLogLogPlus::with_hasher(PRECISION, ahash::RandomState::default());
    //let mut set = hyperloglockless::HyperLogLog::with_hasher(PRECISION, ahash::RandomState::default());
    //let mut set = cardinality_estimator::CardinalityEstimator::<u64, ahash::AHasher, { PRECISION as usize }, 6>::init(PRECISION);
    //let mut set = amadeus_streaming::HyperLogLog::<u64>::init(PRECISION);
    //let mut set = probabilistic_collections::hyperloglog::HyperLogLog::<u64, ahash::RandomState>::init(PRECISION);
    //let mut set: hyperloglogplus::HyperLogLogPF<u64, _> = hyperloglogplus::HyperLogLogPF::init(PRECISION, hasher).unwrap();
    //let mut set: HashSet<String> = HashSet::new();

    // allocator_memory::<cardinality_estimator::CardinalityEstimator::<u64, ahash::AHasher, { PRECISION as usize }, 6>>();
    allocator_memory::<hyperloglockless::HyperLogLogPlus<ahash::RandomState>>();
}

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingAlloc;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        ALLOCATED.fetch_sub(layout.size(), Ordering::Relaxed);
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static A: CountingAlloc = CountingAlloc;