


fn stress(){
  println!("working more hours")
}

fn callback ( tea:i8 , headache:fn() ){
    headache();
}

use std::thread;

fn main(){

    callback(2, stress);

    callback(2, ||{println!("working weekends rust hours") });
   
   let handle = thread::spawn(move || {
        println!("working thread hours")
     });// os execute the code

     println!("before thread executes ");
     handle.join().unwrap();
     println!("after thread executes ");


}