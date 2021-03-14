use std::sync::mpsc;
use std::thread;

fn main() {
    let (transmitter, receiver) = mpsc::channel();
    let (transmitter2, receiver2) = mpsc::channel();

    // This code reproduces a deadlock, showing that this isn't impossible here;
    // Since each channel depends on each other, no one is able to finish itself;
    // The same can occur using mutexes/shared-state concurrency
    thread::spawn(move || {
        println!("Wait result 1");
        receiver.recv().unwrap();

        println!("Send result 2");
        transmitter2.send(());
    });

    println!("Wait result 2");
    receiver2.recv().unwrap();

    println!("Send result 1");
    transmitter.send(());
}
