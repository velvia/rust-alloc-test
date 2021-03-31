# rust-alloc-test

A small demonstration of Rust allocation strategies, memory benchmarking and saving allocations.

## compare_string_storage()

Shows off the effects of using [nested](https://crates.io/crates/nested) vs standard `Vec<String>`, plus using `jemalloc-ctl` to measure static memory usage, and [deepsize](https://crates.io/crates/deepsize) to find nested allocations of data structures.

## parse_json_per_line_file()

Runs benchmark which reads in the airlines JSON dataset and measures how much dynamic memory was used (using [DHAT](https://www.valgrind.org/docs/manual/dh-manual.html)) and time spent.

Notes:
* DHAT and JeMalloc are both disabled by default.
* To measure memory, uncomment the DHAT global allocator lines at the top of main, and also the `start_heap_profiling()` in main().   Note that JeMalloc cannot be used together with this.
* The DHAT online viewer can be used on the resulting `dhat-heap.json` file:  https://nnethercote.github.io/dh_view/dh_view.html
* To measure effect of using JeMalloc, comment the DHAT lines above and uncomment the JeMalloc global allocator lines at the top.
* Be sure to run `cargo run --release` to get accurate benchmarking times.  To be honest this should be put into a proper benchmark like Bencher or Criterion.