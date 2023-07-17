use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() { // function exceutes by process 
            //  exe file is called process 
            //  exe code of function , struct

            
    let (tx, rx) = mpsc::channel();  // heap memory

    thread::spawn(move || { // thread
        println!("thread execution");
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ]; // heap memory

        for val in vals {  // line 1 hi borrowed
            tx.send(val).unwrap();  // val addres copied  to heap
            thread::sleep(Duration::from_secs(1)); // sleep 1 sec
        }
    });

    println!("receiveing value from channel");

    for received in rx { // value recived from channel
    // no value in channel make the thread excution

        println!("Got: {}", received);
    } // line 1

    println!("end of main");
}

