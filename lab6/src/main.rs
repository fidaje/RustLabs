mod game;
mod primes;
use crate::game::{prepare, verify};
use std::thread;
// use primes::primes::find_primes;
// use primes::primes::find_primes_2;
// use std::time::Instant;

fn main() {
    // print!("1° esercizio");

    // println!("1° method");
    // for i in 1..=16 {
    //     let start = Instant::now();
    //     find_primes(1000000, i);
    //     let elapsed = start.elapsed().as_secs_f64();
    //     println!("# {} - Elapsed time {elapsed}", i);
    // }

    // println!();
    // println!("2° method");
    // for i in 1..=16 {
    //     let start = Instant::now();
    //     find_primes_2(1000000, i);
    //     let elapsed = start.elapsed().as_secs_f64();
    //     println!("# {} - Elapsed time {elapsed}", i);
    // }

    println!("2° esercizio");
    let nthreads = 5;
    let permutations = prepare("12345");
    let chunk_size = permutations.len() / nthreads;
    let mut results = vec![];

    thread::scope(|s| {
        let mut handles = vec![];

        for chunk in permutations.chunks(chunk_size) {
            handles.push(s.spawn(|| verify(chunk)));
        }

        results.extend(handles.into_iter().flat_map(|h| h.join().unwrap()));
    });

    println!("{:?}", results);
    println!("{:?}", results.len());
}
