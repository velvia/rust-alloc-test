# rust-alloc-test

A small demonstration of Rust allocation strategies, memory benchmarking and saving allocations.

## compare_string_storage()

Shows off the effects of using [nested](https://crates.io/crates/nested) vs standard `Vec<String>`, plus using `jemalloc-ctl` to measure static memory usage, and [deepsize](https://crates.io/crates/deepsize) to find nested allocations of data structures.