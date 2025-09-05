#![allow(unused_imports)]
use std::thread::{sleep};
use std::time;
use std::thread::spawn;
fn main()
{

    let handler=spawn(some_print);
    handler.join().unwrap();
}

fn some_print()
{
  for i in 0..10{
    println!("{}",i);
    sleep(time::Duration::from_millis(2000));
  }
}