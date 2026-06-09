pub mod lib {

    use std::{
        sync::{Condvar, Mutex},
        time::Duration,
    };

    pub struct CountDownLock {
        pub counter: Mutex<usize>,
        pub condvar: Condvar,
    }

    impl CountDownLock {
        pub fn new(n: usize) -> Self {
            CountDownLock {
                counter: Mutex::new(n),
                condvar: (Condvar::new()),
            }
        }

        pub fn count_down(&self) {
            let mut lock = self.counter.lock().unwrap();

            if *lock > 0 {
                *lock -= 1;
            }

            if *lock == 0 {
                self.condvar.notify_all();
            }
        }

        pub fn wait(&self) {
            // dichiaro lock come mut così posso sostituire il vecchio mutex (consumato
            // da wait_while) con quello nuovo che rilascia wait_while
            let mut lock = self.counter.lock().unwrap();
            lock = self.condvar.wait_while(lock, |&mut c| c > 0).unwrap();
        }

        pub fn wait_timeout(&self, d: Duration) -> std::sync::WaitTimeoutResult {
            let lock = self.counter.lock().unwrap();

            self.condvar
                .wait_timeout_while(lock, d, |&mut c| c > 0)
                .unwrap()
                .1
        }
    }
}
