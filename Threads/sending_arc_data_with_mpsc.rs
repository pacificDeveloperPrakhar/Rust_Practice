use std::sync::{mpsc, Arc};
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<Arc<String>>();

    // Clone the sender for another thread
    let tx1 = tx.clone();

    // Spawn first thread
    thread::spawn(move || {
        let msg = Arc::new("Hello from thread 1".to_string());
        tx1.send(msg).unwrap();
    });

    // Spawn second thread
    thread::spawn(move || {
        let msg = Arc::new("Hello from thread 2".to_string());
        tx.send(msg).unwrap();
    });

    // Receive messages in main thread
    for received in rx{
        println!("Got: {}", received);
    }
}
