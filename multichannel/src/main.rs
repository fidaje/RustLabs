use std::{sync::{Mutex, mpsc::{Receiver, SendError, Sender, channel}}, thread};




pub struct MultiChannel{
    channels: Mutex<Vec<Sender<u8>>>
}



impl MultiChannel{

    fn new() -> Self{
        Self { 
            channels: Mutex::new(Vec::new())
         }
    }

    fn subscribe(&self) -> Receiver<u8> {

        let mut lock = self.channels.lock().unwrap();

        let (tx, rx) = channel();


        lock.push(tx);

        rx

    }

    fn send(&self, data: u8) -> Result<(), SendError<u8>> {
        
        let mut lock = self.channels.lock().unwrap();

        if lock.is_empty(){
            return Err(SendError(data));
        }

        lock.retain(|l| l.send(data).is_ok());
        
        Ok(())

    }


}


fn main() {
    let mut handles = Vec::new();
    {
        let multi_channel = MultiChannel::new();

        // Subscriber 1
        let rx1 = multi_channel.subscribe();
        handles.push( thread::spawn(move || {
        while let Ok(data) = rx1.recv() {
            println!("Subscriber 1 received: {}", data);
        }

        }) );

        // Subscriber 2
        let rx2 = multi_channel.subscribe();
        handles.push( thread::spawn(move || {
        while let Ok(data) = rx2.recv() {
            println!("Subscriber 2 received: {}", data);
        }

        }) );

        // Send data
        multi_channel.send(10).unwrap();
        multi_channel.send(20).unwrap();
        multi_channel.send(30).unwrap();
        }
    for handle in handles { handle.join().unwrap(); }

}
