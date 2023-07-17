

struct GenVal<T> {
    gen_val: T,
}

// impl of GenVal for a generic type `T`

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

trait clone{
    
    fn happy(){
        
        println!("happy happy")
    }
    
    fn angry( & mut self);
    
}

struct scold {
    
    notlist:String,
}

impl clone for scold {
    
    fn angry( & mut self){
        self.notlist = "not listeining to me".to_string();
    }
    
}


fn main(){
    
    let manager : scold ;

   let rocking: GenVal  =GenVal{gen_val:10}

   /* 
   
compiler generate the code for the struct & methods 

struct GenVal {
    gen_val: i8,
}

// impl of GenVal for a generic type `T`

impl GenVal {
    fn value(&self) -> &i8 {
        &self.gen_val
    }
}
    
  //  let  venkat : dyn clone = &manager ;
  */
    
}