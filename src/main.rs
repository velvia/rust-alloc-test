use std::io::BufRead;
use std::thread;
use std::time::{Duration, Instant};
use std::default::Default;
use std::collections::HashMap;


use jemalloc_ctl::{stats, epoch};
use nested::Nested;
use deepsize::DeepSizeOf;
use serde_json::{Result as SJResult, Value};
use json::{self, JsonValue};

// #[global_allocator]
// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

// NOTE: Dhat modifies global allocator to track heap usage
// use dhat::{Dhat, DhatAlloc};

// #[global_allocator]
// static ALLOCATOR: DhatAlloc = DhatAlloc;

pub mod error;
pub use error::MyError;


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


#[derive(Debug, Default, Clone)]
struct ColStats {
    pub num_short_str: usize,
    pub num_long_str: usize,
    pub num_number: usize,
    pub num_object: usize,
    pub num_array: usize,
}

fn inner_serde_json_parse(v: &Value, stats: &mut HashMap<String, ColStats>) -> SJResult<()> {
    for (key, val) in v.as_object().unwrap() {
        if !stats.contains_key(key) {
            let new_stat = ColStats { ..Default::default() };
            stats.insert(key.clone(), new_stat);
        }
        let mut stat = stats.get_mut(key).unwrap();
        match val {
            Value::String(s) => {
                if s.len() > 10 {
                    stat.num_long_str += 1;
                } else {
                    stat.num_short_str += 1;
                }
            }
            Value::Number(_) => { stat.num_number += 1; }
            Value::Array(_) => { stat.num_array += 1; }
            Value::Object(_m) => {
                stat.num_object += 1;
                inner_serde_json_parse(val, stats)?;
            }
            _ => {
                println!("Unknown value {:?}", val);
            }
        }
    }
    Ok(())
}

fn inner_json_rust_parse(v: &JsonValue, stats: &mut HashMap<String, ColStats>) {
    for (key, val) in v.entries() {
        if !stats.contains_key(key) {
            let new_stat = ColStats { ..Default::default() };
            stats.insert(key.to_string(), new_stat);
        }
        let mut stat = stats.get_mut(key).unwrap();
        match val {
            JsonValue::Short(_) => { stat.num_short_str += 1; }
            JsonValue::String(_) => { stat.num_long_str += 1; }
            JsonValue::Number(_) => { stat.num_number += 1; }
            JsonValue::Array(_) => { stat.num_array += 1; }
            JsonValue::Object(_m) => {
                stat.num_object += 1;
                inner_json_rust_parse(val, stats);
            }
            _ => {
                println!("Unknown value {:?}", val);
            }
        }
    }
}

/// serde-json JSON line parser
fn serde_json_line_process(line: &str, stats: &mut HashMap<String, ColStats>) -> SJResult<()> {
    let v: Value = serde_json::from_str(line)?;
    inner_serde_json_parse(&v, stats)
}

const FILENAME: &str = "airlines-json-per-line";

/// Read lines from a file, reusing String buffer to avoid allocations
/// then hand each line to inner processor
fn parse_json_per_line_file() -> Result<(), MyError> {
    let file = std::fs::File::open(FILENAME)?;
    let mut reader = std::io::BufReader::new(file);
    let mut buf = String::new();

    dump_mem_stats();

    let start = Instant::now();
    let mut stats = HashMap::new();

    loop {
        buf.clear();
        let num_read = reader.read_line(&mut buf)?;
        if num_read == 0 { break; }

        // serde_json_line_process(&buf, &mut stats)?;
        //
        inner_json_rust_parse(&json::parse(&buf)?, &mut stats);
    }

    println!("Stats from reading file: {:?}", stats);
    println!("Elapsed time: {:?}", start.elapsed());
    dump_mem_stats();

    Ok(())
}


fn main() {
    // The output file generated by DHAT can be viewed using online viewer at
    // https://nnethercote.github.io/dh_view/dh_view.html
    // let _dhat = Dhat::start_heap_profiling();

    // compare_string_storage();
    parse_json_per_line_file().unwrap();
}
