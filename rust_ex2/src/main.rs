use std::{sync::Arc, thread, time::Duration};

use rust_ex2::lib::{CancelableLatch, CancelableLatchImpl};

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
