pub mod lib {
    use std::sync::{
        Arc, Mutex,
        mpsc::{Receiver, SendError, Sender, channel},
    };

    pub struct MultiChannel {
        senders: Arc<Mutex<Vec<Sender<u8>>>>,
    }

    impl MultiChannel {
        pub fn new() -> Self {
            MultiChannel {
                senders: Arc::new(Mutex::new(vec![])),
            }
        }

        pub fn subscribe(&self) -> Receiver<u8> {
            let mut lock = self.senders.lock().unwrap();

            let (tx, rx) = channel();

            // equivalente a lock.push
            (*lock).push(tx);

            rx
        }

        pub fn send(&self, data: u8) -> Result<(), SendError<u8>> {
            let mut lock = self.senders.lock().unwrap();

            if lock.is_empty() {
                return Err(SendError(data));
            }
            lock.retain(|sender| sender.send(data).is_ok());
            Ok(())
        }
    }
}
