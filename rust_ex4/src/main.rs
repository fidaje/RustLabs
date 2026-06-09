use rust_ex4::lib::MultiChannel;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    {
        let multi_channel = MultiChannel::new();

        // Subscriber 1
        let rx1 = multi_channel.subscribe();
        handles.push(thread::spawn(move || {
            while let Ok(data) = rx1.recv() {
                println!("Subscriber 1 received: {}", data);
            }
        }));

        // Subscriber 2
        let rx2 = multi_channel.subscribe();
        handles.push(thread::spawn(move || {
            while let Ok(data) = rx2.recv() {
                println!("Subscriber 2 received: {}", data);
            }
        }));

        // Send data
        multi_channel.send(10).unwrap();
        multi_channel.send(20).unwrap();
        multi_channel.send(30).unwrap();
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
