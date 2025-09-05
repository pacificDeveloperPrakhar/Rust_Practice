use std::thread;
use std::time;
use std::sync::mpsc;

// we will create multiple threads and then produce the clones of transmitter and the single receiver 
// will be used by the receiving thread to listen for the incomming message data from other transmitter
fn main()
{

    let (tx,rx)=mpsc::channel();

    // lets create three four threads
    let tx1=tx.clone();
    thread::spawn(move ||{
        // remember for the production level app this is not how u tackle the error 
     tx1.send(String::from("hey this is message from the first thread")).unwrap()
    });


    let tx1=tx.clone();
    thread::spawn(move ||{

     tx1.send(String::from("hey this is message from the first thread")).unwrap()
    });

    let tx1=tx.clone();
    thread::spawn(move ||{

     tx1.send(String::from("hey this is message from the  thread")).unwrap()
    });

    let tx1=tx.clone();
    thread::spawn(move ||{
        
     tx1.send(String::from("hey this is message from the  thread")).unwrap()
    });


    // now we create the thread that will receive all the message from all the prior threads
    let receiver:thread::JoinHandle<()>=thread::spawn(move||
    {
      for mssg in rx
      {
        println!("{}",mssg);
      }
    });

    receiver.join().unwrap();

}