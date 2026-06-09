pub mod lib {
    use std::{
        sync::{Condvar, Mutex},
        time::Duration,
    };

    #[derive(PartialEq, Eq, Debug)]
    pub enum WaitResult {
        Success,
        Timeout,
        Canceled,
    }

    pub trait CancelableLatch {
        fn new(count: usize) -> Self;
        fn count_down(&self);
        fn cancel(&self);
        fn wait(&self) -> WaitResult;
        fn wait_timeout(&self, d: Duration) -> WaitResult;
    }

    pub struct CancelableLatchImpl {
        // false => not canceled
        // true => canceled
        pub counter: Mutex<(usize, bool)>,
        pub cv: Condvar,
    }

    impl CancelableLatch for CancelableLatchImpl {
        fn new(count: usize) -> Self {
            CancelableLatchImpl {
                counter: Mutex::new((count, false)),
                cv: (Condvar::new()),
            }
        }

        fn count_down(&self) {
            let mut lock = self.counter.lock().unwrap();

            if lock.0 > 0 && !lock.1 {
                lock.0 -= 1;

                if lock.0 == 0 {
                    self.cv.notify_all();
                }
            }
        }

        fn cancel(&self) {
            self.counter.lock().unwrap().1 = true;
            self.cv.notify_all();
        }

        fn wait(&self) -> WaitResult {
            let mut lock = self.counter.lock().unwrap();

            lock = self
                .cv
                .wait_while(lock, |&mut c| c.0 > 0 && c.1 == false)
                .unwrap();

            match *lock {
                (_, true) => WaitResult::Canceled,
                _ => WaitResult::Success,
            }
        }

        fn wait_timeout(&self, d: Duration) -> WaitResult {
            let lock = self.counter.lock().unwrap();

            // let result = self
            //     .cv
            //     .wait_timeout_while(lock, d, |&mut c| c.0 > 0 && c.1 == false);

            // let (mutex, timeout) = result.unwrap();

            // if timeout.timed_out() == true {
            //     WaitResult::Timeout
            // } else {
            //     match *mutex {
            //         (_, true) => Canceled,
            //         _ => WaitResult::Success,
            //     }
            // }

            // soluzione più compatta
            match self
                .cv
                .wait_timeout_while(lock, d, |&mut c| c.0 > 0 && c.1 == false)
                .unwrap()
            {
                (_, r) if r.timed_out() => WaitResult::Timeout,
                (d, _) if d.1 => WaitResult::Canceled,
                _ => WaitResult::Success,
            }
        }
    }
}
