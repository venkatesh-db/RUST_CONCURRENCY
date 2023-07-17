
use std::sync::{Arc, Mutex};
use std::thread;

mod Teamouting{

   pub  struct jolly {
 
         fun :String

     }

     impl jolly{

      pub fn new() ->jolly {
          jolly{fun:"lonaala".to_string()}
        }

       pub fn set(&self){
           println!("{}",self.fun)
        }

        pub fn get(&self){

        }
     }

}

fn main() {

    let object  = Teamouting::jolly::new();
    object.set();
    let counter = Arc::new(Mutex::new(0));
    // ARC                // Mutex object
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        println!(" thread before execution");   
        let handle = thread::spawn(move || {
            println!(" thread code execution");   
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("{}",num);
        });
        handles.push(handle);
    }

    println!("thread may excute");
    for handle in handles {
        handle.join().unwrap(); // thread execution
    }

    println!("Result: {}", *counter.lock().unwrap());
}
