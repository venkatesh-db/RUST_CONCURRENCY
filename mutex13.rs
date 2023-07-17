use std::sync::Mutex;
use std::thread;


static age:i8=25;

fn main() {

    let mut counter = Mutex::new(0); // object Mutex

    let mut handles = vec![];

    for _ in 0..10 { // below block excutes 0 to 9 10 times

        let handle = thread::spawn(move || {

            println!(" thread code execution");             
            let mut num = counter.lock().unwrap();

            *num += 1;

            println!("{} {}",*num, age);
        });

        handles.push(handle); //  first object stored in heap
    }

    println!(" before thread code execution");    

    for handle in handles {
        println!(" loop execution");   
        handle.join().unwrap();// one thread executes 
    }

    println!("Result: {}", *counter.lock().unwrap());
}
