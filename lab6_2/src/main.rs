use std::{thread, time::Duration};

use lab6_2::{Item, MyChannel};
fn main() {
    let size = 5;

    let mychannel: MyChannel<u8> = MyChannel::new(size);

    let cloned_channel = mychannel.clone();
    let h1 = thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_secs(2));
            match cloned_channel.write(Item::Value(i as u8)) {
                Ok(_) => println!("Inserito il valore {i}"),
                Err(_) => break,
            };
        }

        cloned_channel.close();
    });

    let another_clone = mychannel.clone();
    let h2 = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            match another_clone.read() {
                Ok(Item::Value(num)) => println!("Letto: {}", num),
                Ok(Item::Stop) => println!("Letto Stop"),
                Err(_) => {
                    print!("Non ci sono più dati");
                    break;
                }
            };
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
