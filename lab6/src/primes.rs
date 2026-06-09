pub mod primes {

    use std::{
        sync::{Arc, Mutex},
        thread::{self, JoinHandle},
    };

    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as u64) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    pub fn find_primes(limit: u64, n_threads: u64) -> Vec<u64> {
        // non serve mut perché il mutex gestisce la mutabilità interna
        let counter = Arc::new(Mutex::new(2 as u64));
        let mut handles: Vec<JoinHandle<Vec<u64>>> = vec![];

        for i in 0..n_threads {
            // creo un riferimento al counter
            let counter_clone = counter.clone();

            let handle = thread::spawn(move || {
                let mut local_primes: Vec<u64> = vec![];

                loop {
                    let mut lock = counter_clone.lock().unwrap();

                    let number = *lock;

                    *lock += 1;

                    drop(lock);

                    if number > limit {
                        break;
                    }

                    //println!("thread {i} - numero {number}");

                    if is_prime(number) {
                        local_primes.push(number);
                    }
                }

                local_primes
            });
            handles.push(handle);
        }

        let mut primes = vec![];

        // for handle in handles {
        //     let vec = handle.join().unwrap();
        //     primes.extend(vec);
        // }

        primes.extend(handles.into_iter().flat_map(|h| h.join().unwrap()));

        primes
    }

    pub fn find_primes_2(limit: u64, n_threads: u64) -> Vec<u64> {
        let mut handles: Vec<JoinHandle<Vec<u64>>> = vec![];

        for i in 0..n_threads {
            let handle = thread::spawn(move || {
                let mut local_primes: Vec<u64> = vec![];

                // loop {
                //     if number > limit {
                //         break;
                //     }

                //     println!("thread {i} - numero {number}");

                //     if is_prime(number) {
                //         local_primes.push(number);
                //     }

                //     number += n_threads;
                // }

                for number in (2 + i..=limit).step_by(n_threads as usize) {
                    //println!("thread {i} - numero {number}");

                    if is_prime(number) {
                        local_primes.push(number);
                    }
                }

                local_primes
            });
            handles.push(handle);
        }

        let mut primes = vec![];

        // for handle in handles {
        //     let vec = handle.join().unwrap();
        //     primes.extend(vec);
        // }

        primes.extend(handles.into_iter().flat_map(|h| h.join().unwrap()));

        primes
    }
}
