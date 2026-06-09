use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

#[derive(Debug)]
pub struct SendError;
#[derive(Debug)]
pub struct RecvError;

struct SharedState<T> {
    buffer: Mutex<(VecDeque<Item<T>>, bool)>,
    cv_producer: Condvar,
    cv_consumer: Condvar,
    n: usize,
}

impl<T> SharedState<T> {
    fn new(size: usize) -> SharedState<T> {
        SharedState {
            buffer: (Mutex::new((VecDeque::with_capacity(size), false))),
            cv_producer: (Condvar::new()),
            cv_consumer: (Condvar::new()),
            n: (size),
        }
    }
}

pub struct MyChannel<T> {
    shared: Arc<SharedState<T>>,
}

pub enum Item<T> {
    Value(T),
    Stop,
}

impl<T> Clone for MyChannel<T> {
    fn clone(&self) -> Self {
        MyChannel {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> MyChannel<T> {
    pub fn new(size: usize) -> MyChannel<T> {
        MyChannel {
            shared: Arc::new(SharedState::new(size)),
        }
    }

    pub fn write(&self, item: Item<T>) -> Result<(), SendError> {
        // acquisizione lock
        let mut lock = self.shared.buffer.lock().unwrap();

        // se il canale è chiuso => SendError
        if lock.1 {
            return Err(SendError);
        }
        // se la coda è piena => wait (non posso scrivere)
        else if lock.0.len() == self.shared.n {
            println!("[Produttore] Coda piena! Mi metto in attesa...");
            lock = self
                .shared
                .cv_producer
                .wait_while(lock, |l| l.0.len() == self.shared.n)
                .unwrap();
        }
        // scrivo
        lock.0.push_back(item);
        // ho appena scritto, la coda NON è vuota => risveglio i consumer
        self.shared.cv_consumer.notify_all();

        Ok(())
    }
    pub fn read(&self) -> Result<Item<T>, RecvError> {
        // acquisizione lock
        let mut lock = self.shared.buffer.lock().unwrap();

        // se il canale è chiuso devo leggere fino a trovare Stop e poi esco

        // non c'è bisogno di utilizzare if per andare in sleep
        // la funzione wait_while controllerà prima la closure prima di addormentare il thread

        // if lock.0.is_empty() {
        //     lock = self
        //         .shared
        //         .cv_consumer
        //         .wait_while(lock, |l| l.0.is_empty() && !l.1)
        //         .unwrap();
        // }

        // se la coda è vuota => wait (non posso leggere)
        if lock.0.is_empty() {
            println!("[Consumatore] Coda vuota! Mi metto in attesa...");
        }

        lock = self
            .shared
            .cv_consumer
            .wait_while(lock, |l| l.0.is_empty() && !l.1)
            .unwrap();

        let read_value = match lock.0.pop_front() {
            Some(val) => val,
            None => return Err(RecvError), // coda è vuota e canale chiuso
        };
        // notifico i producer
        self.shared.cv_producer.notify_all();

        if lock.1 && matches!(read_value, Item::Stop) {
            return Err(RecvError);
        }

        Ok(read_value)
    }
    pub fn close(&self) {
        // acquisisco il lock
        let mut lock = self.shared.buffer.lock().unwrap();
        println!("[Chiudendo] Il canale è stato chiuso!");
        // segnalo che il canale è chiuso
        lock.1 = true;
        // scrivo Stop nella coda
        lock.0.push_back(Item::Stop);
        // risveglio i consumer
        self.shared.cv_consumer.notify_all();
    }
}
