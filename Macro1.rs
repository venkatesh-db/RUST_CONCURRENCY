
macro_rules! birla{
    ()=>{
        7
    }
}

macro_rules! give_six {
    () => {
        "love rust"
    };
}


fn happy(){

      println!("happy functions");
}

macro_rules! functions {
    () => {
        happy()  // encapsulation 
    };
}


macro_rules! might_print {

    ($input:expr) => {
    
    if $input == "windows"{
    
        println!("You gave me: {}", $input);
    }else{
        happy()
    }
    }
}

fn main(){
    
  println!();  
  
  let age:i32 =birla!();

               // substute code in block { } 
                // 7
  
  let name:&str = give_six!();
  
  println!("{} {}",age ,name);
 
  functions!();
  
  might_print!("linux");

  //  substute code in happy()
    
}