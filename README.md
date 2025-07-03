# bench-hyperloglogs
Collection of benchmarks for https://github.com/tomtomwombat/hyperloglockless/

# Overview

The code/scripts is used for benchmarking error rate and performance for the hyperloglockless, making sure it matches theory and is performance competitive with other crates.

Multi-threaded performance and error rate benchmarks are run in main.rs (`cargo run --release`) and the results are written to `Acc/` or printed. You can also run `cargo bench` for performance non-threaded perf benchmarks. Modify the code directly to change the benchmarks.

err.py and perf.py are graphs for displaying results. Modify these directly to change data source (e.g. new outputs from main.rs).

The code is a bit messy!