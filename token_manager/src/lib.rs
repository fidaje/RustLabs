mod token {
    use std::{
        sync::{Condvar, Mutex},
        time::Instant,
    };

    #[derive(PartialEq, Eq)]
    pub enum TokenManagerState {
        Empty,
        Pending,
        Valid,
    }

    pub struct TokenManager {
        pub state: Mutex<(Box<TokenAcquirer>, TokenManagerState, (String, Instant))>,
        cv: Condvar,
    }

    pub type TokenAcquirer = dyn Fn() -> Result<(String, Instant), String> + Sync + Send;

    impl TokenManager {
        pub fn new(acquire_token: Box<TokenAcquirer>) -> Self {
            TokenManager {
                state: Mutex::new((
                    acquire_token,
                    TokenManagerState::Empty,
                    (String::default(), Instant::now()),
                )),
                cv: Condvar::new(),
            }
        }

        pub fn get_token(&self) -> Result<String, String> {
            let mut lock = self.state.lock().unwrap();

            loop {
                match lock.1 {
                    TokenManagerState::Valid if Instant::now() <= lock.2.1 => {
                        return Ok(lock.2.0.clone());
                    }

                    TokenManagerState::Pending => {
                        lock = self
                            .cv
                            .wait_while(lock, |c| c.1 == TokenManagerState::Pending)
                            .unwrap();

                        // Ok(lock.2.0.clone())
                    }

                    _ => {
                        lock.1 = TokenManagerState::Pending;

                        match lock.0() {
                            Ok((token, validity)) => {
                                lock.2.0 = token;
                                lock.2.1 = validity;
                                lock.1 = TokenManagerState::Valid;
                                self.cv.notify_all();
                                return Ok(lock.2.0.clone());
                            }

                            _ => {
                                lock.1 = TokenManagerState::Empty;
                                self.cv.notify_all();

                                return Err("failure".to_string());
                            }
                        }
                    }
                }
            }
        }

        pub fn try_get_token(&self) -> Option<String> {
            let lock = self.state.lock().unwrap();

            match lock.1 {
                TokenManagerState::Valid if Instant::now() <= lock.2.1 => {
                    return Some((lock.0)().unwrap().0);
                }
                _ => None,
            }
        }
    }
}

use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use token::{TokenAcquirer, TokenManager};

#[test]
fn a_new_manager_contains_no_token() {
    let a: Box<TokenAcquirer> = Box::new(|| Err("failure".to_string()));
    let manager = TokenManager::new(a);
    assert!(manager.try_get_token().is_none());
}

#[test]
fn a_failing_acquirer_always_returns_an_error() {
    let a: Box<TokenAcquirer> = Box::new(|| Err("failure".to_string()));
    let manager = TokenManager::new(a);
    assert_eq!(manager.get_token(), Err("failure".to_string()));
    assert_eq!(manager.get_token(), Err("failure".to_string()));
}
#[test]
fn a_successful_acquirer_always_returns_success() {
    let a: Box<TokenAcquirer> =
        Box::new(|| Ok(("bobo".to_string(), Instant::now() + Duration::from_secs(10))));

    let manager = TokenManager::new(a);

    assert_eq!(manager.get_token(), Ok("bobo".to_string()));

    let prova = Box::new(|| {
        Ok((
            "failure".to_string(),
            Instant::now() + Duration::from_secs(10),
        ))
    });

    manager.state.lock().unwrap().0 = prova;

    thread::sleep(Duration::from_secs(15));
    assert_eq!(manager.get_token(), Ok("failure".to_string()));
}

#[test]
fn a_slow_acquirer_causes_other_threads_to_wait() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Usiamo un contatore atomico per verificare quante volte viene invocato l'acquisitore
    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = Arc::clone(&call_count);

    let a: Box<TokenAcquirer> = Box::new(move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        // Simuliamo un'acquisizione lenta
        thread::sleep(Duration::from_millis(500));
        Ok((
            "token_lento".to_string(),
            Instant::now() + Duration::from_secs(10),
        ))
    });

    let manager = Arc::new(TokenManager::new(a));
    let mut handles = vec![];

    // Lanciamo 5 thread che richiedono il token in contemporanea
    for _ in 0..5 {
        let manager_clone = Arc::clone(&manager);
        handles.push(thread::spawn(move || manager_clone.get_token()));
    }

    // Attendiamo che tutti i thread abbiano finito
    for handle in handles {
        let result = handle.join().unwrap();
        assert_eq!(result, Ok("token_lento".to_string()));
    }

    // Se la sincronizzazione funziona, l'acquisitore è stato chiamato 1 sola volta.
    // Gli altri 4 thread hanno atteso la Condvar.
    assert_eq!(call_count.load(Ordering::SeqCst), 1);
}
