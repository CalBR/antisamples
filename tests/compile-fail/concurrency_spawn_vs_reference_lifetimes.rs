// Non-`'static` references can't be shared across threads using `std::thread::spawn`.
//
// error-pattern: the type `[closure@tests/compile-fail/concurrency_spawn_vs_reference_lifetimes.rs:35:19: 35:60 worklist:std::vec::Vec<std::string::String>, glossary:&std::collections::BTreeMap<std::string::String, std::string::String>]` does not fulfill the required lifetime

#![allow(dead_code)]

use std::io;
use std::collections::BTreeMap;
use std::thread::spawn;

type GigabyteMap = BTreeMap<String, String>;

fn split_vec_into_chunks<T>(mut vec: Vec<T>, nchunks: usize) -> Vec<Vec<T>> {
    unimplemented!()
}

fn process_files(filenames: Vec<String>, glossary: &GigabyteMap)
    -> io::Result<()>
{
    unimplemented!()
}

fn process_files_in_parallel(filenames: Vec<String>,
                             glossary: &GigabyteMap)
    -> io::Result<()>
{
    // Divide the work into several chunks.
    const NTHREADS: usize = 8;
    let worklists = split_vec_into_chunks(filenames, NTHREADS);

    // Fork: Spawn a thread to handle each chunk.
    let mut thread_handles = vec![];
    for worklist in worklists {
        thread_handles.push(
            spawn(move || process_files(worklist, glossary))  // error
        );
    }

    // Join: Wait for all threads to finish.
    for handle in thread_handles {
        handle.join().unwrap()?;
    }

    Ok(())
}

fn main() {
}
