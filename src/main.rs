use std::thread;
use std::time::Duration;

use jemalloc_ctl::{stats, epoch};
use nested::Nested;
use deepsize::DeepSizeOf;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;


fn dump_mem_stats() {
    thread::sleep(Duration::from_secs(1));

    let e = epoch::mib().unwrap();
    let allocated = stats::allocated::mib().unwrap();
    let resident = stats::resident::mib().unwrap();

    // Many statistics are cached and only updated
    // when the epoch is advanced:
    e.advance().unwrap();

    // Read statistics using MIB key:
    let allocated = allocated.read().unwrap();
    let resident = resident.read().unwrap();
    println!("{} bytes allocated/{} bytes resident", allocated, resident);

}

const NUM_STRINGS: usize = 50000;
const BASE_STRING: &str = "foobar";

#[derive(DeepSizeOf)]
struct SomeStrings {
    pub strings: Vec<String>,
}

/// Generate a bunch of small strings and stuff them in a Vec.  Compare that to using other
/// space-saving crates, and maybe use memory utilites to measure total usage
fn compare_string_storage() {
    println!("Initial memory usage:");
    dump_mem_stats();

    // Now create a vec and stuff it full of strings
    let mut strings = Vec::new();
    for i in 0..NUM_STRINGS {
        // This will allocate a new String and push its struct into the Vec
        strings.push(format!("{}{:?}", BASE_STRING, i).to_string());
    }
    assert_eq!(strings.len(), NUM_STRINGS);

    // Now, measure the memory usage again
    println!("After Vec<String> allocation:");
    dump_mem_stats();

    // Now use nested, which saves space for nested strings
    let mut v = Nested::<String>::new();
    v.extend(&strings);
    println!("After Nested<String> allocation:");
    dump_mem_stats();

    // NOTE: dropping doesn't really do anything, we can't measure the freed space apparently
    let some_strs = SomeStrings { strings };
    println!("Using DeepSizeOf to find nested space usage: {:?}", some_strs.deep_size_of());
}


fn main() {
    println!("Hello, world!");
    compare_string_storage();
}
