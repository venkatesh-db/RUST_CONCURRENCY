
#[derive( Copy , Clone)] // compiler trait copy , clone
struct coder {
     loc:i32,
     lang:String,
}


fn main(){

    //,lang:"rust".to_string()
let naveen :coder =coder {loc:3000 , lang:"rust".to_string() };

    // stack  one variable -> heap
    
   // stack data copied to another stack  

let pooja:coder =naveen; // loc: borrowed  ,lang: move

println!("{} {} " ,pooja.loc ,naveen.loc );

    
}