struct vec <T> {
    age:T,
}

fn main(){

    let venkat = vec{age:23};  
    
    /*
    
    struct vec  {
    age:i32,
}
   
    */
   
    let naveen = vec{age:"23 years 3 months"};
    
    
     /*
     
    struct vec  {
    age:&str,
}
    */
    
    
      let ds = vec{age:"23 years 3 months".to_string()};
      
          /*
     
      struct vec  {
       age:String,
         }
         
    */
     
   
    let mut ds=vec![10,2,3];

    /*

       struct vector<T>{
           buffer:T
           len : i32
           cap:  i32
       }

    */
    
    ds.push(1);      
    
    let mut ds=vec!["c","cpp","java"];
    
    ds.push("golang");
}