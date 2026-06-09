pub mod lib {
    use std::sync::{Condvar, Mutex};

    pub struct RankingBarrier {
        threads: usize,
        cv: Condvar,
        controllo: Mutex<(usize, bool)>,
    }

    impl RankingBarrier {
        pub fn new(n: usize) -> Self {
            RankingBarrier {
                threads: (n),
                cv: (Condvar::new()),
                controllo: Mutex::new((0, false)),
            }
        }

        pub fn wait(&self) -> (usize, bool) {
            let mut control = self.controllo.lock().unwrap();

            control.0 += 1;

            let rank = control.0;

            if rank == self.threads {
                control.1 = true;
                control.0 = 0;

                self.cv.notify_all();
            } else {
                control.1 = false;
                control = self.cv.wait_while(control, |c| c.1 == false).unwrap();
            }

            (rank, control.1)
        }
    }
}
