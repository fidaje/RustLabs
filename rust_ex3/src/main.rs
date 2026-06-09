use std::{sync::Arc, thread, time::Duration};

use rust_ex3::lib::RankingBarrier;

fn main() {
    let barrier = Arc::new(RankingBarrier::new(5));
    let mut handles = vec![];

    for i in 0..5 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(50 + i * 100));
            println!("Thread {i} arriva");
            let (rank, state) = b.wait();
            print!("State {state}");
            println!(" Thread {i} riparte con rank {rank}");
        }));
    }

    for i in 5..10 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(50 + i * 100));
            println!("Thread {i} arriva");
            let (rank, state) = b.wait();
            print!("State {state}");
            println!(" Thread {i} riparte con rank {rank}");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
