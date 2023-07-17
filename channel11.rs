

use std::sync::mpsc;

//use std::sync::mpsc;

use std::thread;

fn main() {

    let (tx, rx) = mpsc::channel(); // memory in heap

    thread::spawn(move || {

          // data from thread memory to channel 
        let val = String::from("hi");  // heap memory

         let smileboy = 12;

        tx.send(smileboy).unwrap(); // thread data copied to heap memory

       println!("{}",smileboy);

    });


    // channel memory data copied to local variables
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
