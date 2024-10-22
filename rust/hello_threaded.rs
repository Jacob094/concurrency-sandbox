#![allow(unused_must_use)]
use std::thread;

fn main() {
  let t1 = thread::spawn(f);
  let t2 = thread::spawn(f);
  
  println!("In main thread");

  t1.join();
  t2.join();
}

fn f() {
  let id = thread::current().id();
  println!("In thread: {id:?}"); 
}
