use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (transmitter, receiver) = mpsc::channel();
    let transmitter2 = transmitter.clone();
    thread::spawn(move || {
        let messages = vec![
            String::from("hey!"),
            String::from("we'll"),
            String::from("be"),
        ];

        for msg in messages {
            transmitter2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("sent"),
            String::from("concurrently"),
            String::from("in 1sec intervals")
        ];

        for msg in messages {
            transmitter.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in receiver {
        println!("Received: {}", received);
    }
}