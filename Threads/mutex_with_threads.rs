use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // atomic referenc counter is the smart pointer work with heap memory 
    // it implements the Send trait thus that means it can be send over the mpsc channel
    //also it works the same way as Rc except it is for the threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles:Vec<thread::JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let counter=Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); 
}