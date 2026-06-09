use chrono::Utc;
use chrono_tz::Europe::Rome;
use rust_ex1::lib::CountDownLock;
use std::{sync::Arc, thread, time::Duration};

fn now_rome() -> String {
    Utc::now()
        .with_timezone(&Rome)
        .format("%Y-%m-%d %H:%M:%S%.3f %Z")
        .to_string()
}

fn main() {
    // Create a lock that needs 3 decrements
    let count_down_value = 3;
    let lock = Arc::new(CountDownLock::new(count_down_value));

    // --- Scenario 1: All threads count_down and one `wait`s ---
    let lock_clone_wait = Arc::clone(&lock);
    let waiter_thread = thread::spawn(move || {
        println!(
            "[{}] [Waiter] Waiting for count to reach zero...",
            now_rome()
        );
        lock_clone_wait.wait();
        println!("[{}] [Waiter] Count reached zero. Proceeding!", now_rome());
    });

    let mut worker_threads = Vec::new();
    for i in 0..count_down_value {
        let lock_clone_worker = Arc::clone(&lock);
        worker_threads.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(100 * (i + 1) as u64)); // Simulate work
            println!("[{}] [Worker {}] Calling count_down...", now_rome(), i + 1);
            lock_clone_worker.count_down();
        }));
    }
    for thread in worker_threads {
        thread.join().unwrap();
    }
    waiter_thread.join().unwrap();

    // scenario 2

    let timed_lock = Arc::new(CountDownLock::new(1)); // Needs 1 decrement

    // Thread that will time out
    let timed_lock_clone_wait = Arc::clone(&timed_lock);
    let timeout_thread = thread::spawn(move || {
        println!("[{}] [Timeout Waiter] Waiting with timeout...", now_rome());
        let result = timed_lock_clone_wait.wait_timeout(Duration::from_secs(1));
        if result.timed_out() {
            println!(
                "[{}] [Timeout Waiter] Timed out! Condition not met.",
                now_rome()
            );
        } else {
            println!(
                "[{}] [Timeout Waiter] Condition met before timeout!",
                now_rome()
            );
        }
    });

    // A single worker that will decrement, but not enough to reach zero before timeout
    let timed_lock_clone_worker = Arc::clone(&timed_lock);
    let short_worker_thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(2000));
        println!("[{}] [Short Worker] Calling count_down...", now_rome());
        timed_lock_clone_worker.count_down();
    });

    short_worker_thread.join().unwrap();
    timeout_thread.join().unwrap();
}
