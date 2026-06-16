
use std::{sync::{Arc, Condvar, Mutex}, thread, time::Duration};

use crate::WaitResult::{Canceled, Success, Timeout};

 #[derive(PartialEq, Eq, Debug)]
pub enum WaitResult {
    Success,
    Timeout,
    Canceled
}

pub trait CancelableLatch {
    fn new(count: usize) -> Self;
    fn count_down(&self);
    fn cancel(&self);
    fn wait(&self) -> WaitResult;
    fn wait_timeout(&self, d: Duration) -> WaitResult;
}

pub struct CancelableLatchImpl{
    lock: Mutex<(usize, bool)>,
    cv: Condvar,
}

impl CancelableLatch for CancelableLatchImpl{
    fn new(count: usize) -> Self {
        CancelableLatchImpl { 
            lock: Mutex::new((count, false)), 
            cv: Condvar::new(),
        }
    }

    fn count_down(&self) {
        let mut l = self.lock.lock().unwrap();

        if l.0 > 0 && !l.1 {
            l.0 -= 1;
            if l.0 == 0 {
                self.cv.notify_all();
            }
        }
    }

    fn cancel(&self) {
        let mut l = self.lock.lock().unwrap();

        l.1 = true;
        
        self.cv.notify_all();


    }

    fn wait(&self) -> WaitResult {
        let mut l = self.lock.lock().unwrap();

        l = self.cv.wait_while(l, | l| l.0 > 0 && !l.1 ).unwrap();

        if l.1 {
            Canceled
        } else {
            Success
        }
    }

    fn wait_timeout(&self, d: Duration) -> WaitResult {
        let l = self.lock.lock().unwrap();

        match self.cv.wait_timeout_while(l, d,|l| l.0 > 0 && !l.1).unwrap(){
            (_, timed_out) if timed_out.timed_out() => Timeout,
            (l, _) if l.1 => Canceled,
            _ => Success

        }
    }
}



fn main() {
    println!("--- Test 1: Successo (Count down completo) ---");
    let latch_success = Arc::new(CancelableLatchImpl::new(3));
    let mut handles = vec![];
    // Thread che decrementano il contatore
    for i in 0..3 {
        let latch_clone = Arc::clone(&latch_success);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(100 * i));
            latch_clone.count_down();
        }));
    }
    // Thread che attende il successo
    let latch_clone = Arc::clone(&latch_success);
    handles.push(thread::spawn(move || {
        let result = latch_clone.wait();
        println!("Risultato attesa (Successo): {:?}", result);
    }));
    for handle in handles.drain(..) {
        handle.join().unwrap();
    }
    println!("\n--- Test 2: Timeout ---");
    let latch_timeout = Arc::new(CancelableLatchImpl::new(2));

    let latch_clone = Arc::clone(&latch_timeout);
    let h1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        let result = latch_clone.wait_timeout(Duration::from_millis(200));
        println!("Risultato attesa (Timeout): {:?}", result);
    });

    let latch_clone = Arc::clone(&latch_timeout);
    let h2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000)); // Questo count_down avverrà dopo il timeout
        latch_clone.count_down();
        latch_clone.count_down();
    });
    h1.join().unwrap();
    h2.join().unwrap();

    println!("\n--- Test 3: Canceled ---");
    let latch_cancel = Arc::new(CancelableLatchImpl::new(5));
    let mut handles = vec![];
    // Thread che prova ad attendere (verrà cancellato)
    let latch_clone = Arc::clone(&latch_cancel);
    handles.push(thread::spawn(move || {
        let result = latch_clone.wait();
        println!("Risultato attesa (Canceled): {:?}", result);
    }));
    // Thread che esegue il cancel
    let latch_clone = Arc::clone(&latch_cancel);
    handles.push(thread::spawn(move || {
        thread::sleep(Duration::from_millis(50)); // Attende un po' per assicurarsi che il thread di attesa sia partito
        latch_clone.cancel();
    }));
    // Thread che decrementano (ma il cancel avrà la precedenza)
    for i in 0..3 {
        let latch_clone = Arc::clone(&latch_cancel);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(150 + 10 * i));
            latch_clone.count_down();
        }));
    }
    for handle in handles.drain(..) {
        handle.join().unwrap();
    }
    println!("\n--- Test 4: Canceled con timeout ---");
    let latch_cancel_timeout = Arc::new(CancelableLatchImpl::new(5));
    let mut handles = vec![];
    // Thread che prova ad attendere con timeout (verrà cancellato)
    let latch_clone = Arc::clone(&latch_cancel_timeout);
    handles.push(thread::spawn(move || {
        let result = latch_clone.wait_timeout(Duration::from_secs(1)); // Attende per 1 secondo
        println!("Risultato attesa (Canceled con timeout): {:?}", result);
    }));
    // Thread che esegue il cancel
    let latch_clone = Arc::clone(&latch_cancel_timeout);
    handles.push(thread::spawn(move || {
        thread::sleep(Duration::from_millis(100)); 
        latch_clone.cancel();
    }));
    for handle in handles.drain(..) {
        handle.join().unwrap();
    }
}