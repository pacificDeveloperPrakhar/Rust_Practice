use std::thread;
use std::time;
// mpsc is the module lies in the sync crate
use std::sync;
// mpsc stands for multiple producer and single consumer 
// this means a thread can have multiple transmitter but one receiver
fn main()
{
    let (tx,rx)=sync::mpsc::channel();
    // move keyword tells that any variable used inside of the closure will be moved to the closure
    // every variable in the thread spawn closure is always has to be moved
    thread::spawn(move||{
     let mut v=23;
    //  this return the result 
    // now v is send to another thread
     tx.send(v).unwrap();
    });
 
    let handler:thread::JoinHandle<()>=thread::spawn(move ||{
     let v=rx.recv().unwrap();
     println!("{}",v);
    });
  handler.join().unwrap();
}