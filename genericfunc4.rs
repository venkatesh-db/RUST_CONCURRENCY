
fn collections( dress : String){   // stack memory 
    
}

fn generic<T>(dress:T) -> T{   // Dont type type to allcoate memory
    
    dress
}

fn main(){
    
    collections( "formal suit".to_string());
    
    generic("formal suit".to_string()); 
    
    /* compiler genrerate a code  function overloading
    
    fn generic(dress:String) -> String{
    
    dress
    } 
    
    */
    
    generic(2);
    
    
     /* compiler genrerate a code  function overloading
    
       fn generic(dress:i32) -> i32){
    
          dress
       } 
    
    */
    
}