pub mod fc {
    use std::sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender, channel},
    };

    use crate::fc::MessageState::{Forgotten, Pending, Received};

    /// Rappresenta un riferimento a un messaggio in volo che può essere annullato
    /// prima che il ricevitore lo elabori.
    ///
    /// Un valore che implementa `Forgettable` viene restituito da
    /// [`ForgettableSender::send`] e consente al mittente di "dimenticare"
    /// il messaggio già accodato.
    pub trait Forgettable {
        /// Tenta di annullare il messaggio associato.
        ///
        /// Restituisce `true` se il messaggio era ancora in attesa di essere
        /// ricevuto oppure se il ricevitore è stato terminato prima che il messaggio fosse ricevuto,
        /// `false` se il ricevitore lo aveva già elaborato.
        fn forget(self) -> bool;
    }

    /// Rappresenta il lato mittente di un canale di messaggi dimenticabili.
    ///
    /// Implementa [`Clone`] per consentire a più thread di condividere lo stesso
    /// endpoint di invio (canale multi-produttore, ricevitore singolo).
    pub trait ForgettableSender<T: Send + 'static>: Clone {
        /// Invia il valore `t` nel canale.
        ///
        /// Restituisce `Some(handle)` in caso di successo: `handle` implementa
        /// [`Forgettable`] e può essere usato in seguito per annullare il
        /// messaggio prima che venga ricevuto.
        /// Restituisce `None` se il ricevitore è stato già eliminato (canale
        /// disconnesso).
        fn send(&self, t: T) -> Option<impl Forgettable + 'static>;
    }

    /// Rappresenta il lato ricevitore di un canale di messaggi dimenticabili.
    ///
    /// Riceve i messaggi in ordine FIFO, saltando silenziosamente quelli che
    /// sono stati annullati tramite [`Forgettable::forget`] prima di essere
    /// estratti dalla coda.
    pub trait ForgettableReceiver<T: Send + 'static> {
        /// Blocca il thread corrente finché non è disponibile un messaggio
        /// non annullato, quindi lo restituisce.
        ///
        /// Restituisce `None` quando tutti i mittenti sono stati eliminati e
        /// la coda è vuota (canale chiuso).
        /// I messaggi annullati vengono consumati dalla coda e scartati
        /// internamente; il chiamante non li vede mai.
        fn recv(&self) -> Option<T>;
    }

    pub enum MessageState {
        Pending,
        Forgotten,
        Received,
    }

    struct Message<T> {
        value: T,
        state: Arc<Mutex<MessageState>>,
    }

    struct Handler {
        state: Arc<Mutex<MessageState>>,
    }

    impl Forgettable for Handler {
        fn forget(self) -> bool {
            let mut s = self.state.lock().unwrap();

            match *s {
                Forgotten => true,
                Received => false,
                Pending => {
                    *s = Forgotten;
                    true
                }
            }
        }
    }

    pub struct ForgettableSenderImpl<T> {
        sender: Sender<Message<T>>,
    }

    impl<T: Send + 'static> ForgettableSender<T> for ForgettableSenderImpl<T> {
        fn send(&self, t: T) -> Option<impl Forgettable + 'static> {
            let state = Arc::new(Mutex::new(Pending));
            let cloned_state = state.clone();
            if let Ok(()) = self.sender.send(Message { value: t, state }) {
                Some(Handler {
                    state: cloned_state,
                })
            } else {
                None
            }
        }
    }

    impl<T> Clone for ForgettableSenderImpl<T> {
        fn clone(&self) -> Self {
            ForgettableSenderImpl {
                sender: self.sender.clone(),
            }
        }
    }

    pub struct ForgettableReceiverImpl<T> {
        receiver: Receiver<Message<T>>,
    }

    impl<T: Send + 'static> ForgettableReceiver<T> for ForgettableReceiverImpl<T> {
        fn recv(&self) -> Option<T> {
            loop {
                if let Ok(message) = self.receiver.recv() {
                    let mut lock = message.state.lock().unwrap();
                    match *lock {
                        Pending => {
                            *lock = Received;
                            return Some(message.value);
                        }
                        Forgotten => continue,
                        _ => {
                            return None;
                        }
                    }
                }
                return None;
            }
        }
    }

    pub fn forgettable_channel<T: Send + 'static>()
    -> (impl ForgettableSender<T>, impl ForgettableReceiver<T>) {
        let (tx, rx) = channel::<Message<T>>();

        (
            ForgettableSenderImpl { sender: tx },
            ForgettableReceiverImpl { receiver: rx },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::fc::{Forgettable, ForgettableReceiver, ForgettableSender, forgettable_channel};
    use std::collections::HashMap;
    use std::sync::mpsc::channel;

    #[test]
    fn a_channel_can_send_data() {
        let (sender, receiver) = forgettable_channel::<i32>();
        assert!(sender.send(42).is_some());
        assert_eq!(receiver.recv().unwrap(), 42);
    }

    #[test]
    fn a_channel_can_send_multiple_data() {
        let (sender, receiver) = forgettable_channel::<i32>();
        assert!(sender.send(42).is_some());
        assert!(sender.send(43).is_some());
        assert_eq!(receiver.recv().unwrap(), 42);
        assert_eq!(receiver.recv().unwrap(), 43);
    }
    #[test]
    fn when_the_sender_is_dropped_the_receiver_unblocks() {
        let (sender, receiver) = forgettable_channel::<i32>();
        assert!(sender.send(42).is_some());
        drop(sender);
        assert_eq!(receiver.recv().unwrap(), 42);
        assert!(receiver.recv().is_none());
    }

    #[test]
    fn when_the_receiver_is_dropped_the_sender_returns_none() {
        let (sender, receiver) = forgettable_channel::<i32>();
        drop(receiver);
        assert!(sender.send(42).is_none());
    }

    #[test]
    fn multiple_senders_can_send_concurrently() {
        let (sender, receiver) = forgettable_channel::<i32>();
        let sender2 = sender.clone();
        let t1 = std::thread::spawn(move || {
            for i in 0..1000 {
                assert!(sender.send(i).is_some());
            }
        });
        let t2 = std::thread::spawn(move || {
            for i in 1000..2000 {
                assert!(sender2.send(i).is_some());
            }
        });
        t1.join().unwrap();
        t2.join().unwrap();
        let mut received: Vec<i32> = std::iter::from_fn(|| receiver.recv()).collect();
        received.sort();
        assert_eq!(received, (0..2000).collect::<Vec<_>>());
    }

    #[test]
    fn receiver_drains_queue_before_returning_none() {
        let (sender, receiver) = forgettable_channel::<i32>();
        for i in 0..10 {
            assert!(sender.send(i).is_some());
        }
        drop(sender);
        let received: Vec<i32> = std::iter::from_fn(|| receiver.recv()).collect();
        assert_eq!(received, (0..10).collect::<Vec<_>>());
    }

    #[test]
    fn a_channel_can_forget_its_messages() {
        let (sender, receiver) = forgettable_channel::<i32>();
        let handle = sender.send(42).unwrap();
        assert!(handle.forget());
        drop(sender);
        assert!(receiver.recv().is_none());
    }

    #[test]
    fn invoking_forget_after_receiving_a_message_returns_false() {
        let (sender, receiver) = forgettable_channel::<i32>();
        let handle = sender.send(42).unwrap();
        assert_eq!(receiver.recv().unwrap(), 42);
        assert!(!handle.forget());
        drop(sender);
        assert!(receiver.recv().is_none());
    }

    #[test]
    fn forget_returns_true_when_channel_is_dropped() {
        let (sender, receiver) = forgettable_channel::<i32>();
        let handle = sender.send(42).unwrap();
        drop(sender);
        drop(receiver);
        assert!(handle.forget());
    }

    enum TestMessageState<T: Send + 'static> {
        Pending(T),
        Forgotten(T),
        Processed(T),
    }

    #[test]
    fn messages_are_either_processed_or_forgotten() {
        let (ctx, crx) = channel::<TestMessageState<i32>>();
        let ctx1 = ctx.clone();
        let n = 100_000;
        let mut map = HashMap::new();
        for i in 0..n {
            map.insert(i, TestMessageState::Pending(i));
        }
        let (sender, receiver) = forgettable_channel::<i32>();
        let t1 = std::thread::spawn(move || {
            for i in 0..n {
                let h = sender.send(i).unwrap();
                if h.forget() {
                    ctx.send(TestMessageState::Forgotten(i)).unwrap();
                }
            }
        });
        let t2 = std::thread::spawn(move || {
            while let Some(i) = receiver.recv() {
                ctx1.send(TestMessageState::Processed(i)).unwrap();
            }
        });
        let t3 = std::thread::spawn(move || {
            let mut processed = 0;
            let mut forgotten = 0;
            while let Ok(msg) = crx.recv() {
                match msg {
                    TestMessageState::Forgotten(i) => {
                        let v = map.get_mut(&i).unwrap();
                        match v {
                            TestMessageState::Pending(_) => {
                                *v = TestMessageState::Forgotten(i);
                                forgotten += 1;
                            }
                            TestMessageState::Forgotten(_) => {
                                assert!(false, "Message {i} was forgotten twice");
                            }
                            TestMessageState::Processed(_) => {
                                assert!(false, "Message {i} was both forgotten and processed");
                            }
                        }
                    }
                    TestMessageState::Processed(i) => {
                        let v = map.get_mut(&i).unwrap();
                        match v {
                            TestMessageState::Pending(_) => {
                                *v = TestMessageState::Forgotten(i);
                                processed += 1;
                            }
                            TestMessageState::Forgotten(_) => {
                                assert!(false, "Message {i} was both forgotten and processed");
                            }
                            TestMessageState::Processed(_) => {
                                assert!(false, "Message {i} was processed twice");
                            }
                        }
                    }
                    _ => {
                        unreachable!("Invalid state for message");
                    }
                }
            }
            assert!(forgotten + processed == n, "Some messages were lost");
        });
        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();
    }
}
