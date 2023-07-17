

trait fmt {

    fn display(&mut self); // exepctation from programmer will assign value to struct variable 
                            // struct and method needs be implememted by coder
 }
 
 #[derive(Copy,Clone)]
 
 struct variables{
     
      datatypes:i8,    // Box::new(variables{datatypes:8})
 }
 impl fmt for variables{
  
  fn display(& mut self){  // stack memory
      
      self.datatypes = 7;  // value is assigned to strut variables object memory
      
        println!("variables display");
  }
     
 }
 
 #[derive(Copy,Clone)]
 
 struct object{
     
     structvariables:i8,
 }
 
 impl fmt for object{
     
  fn display(& mut self){       // stack memory
      
      self.structvariables = 9; // value is assigned to strut variables object memory
      
      println!("objects display");
  }
     
 }
 
 fn main(){
     
     
    let mut obj:Vec<Box<dyn fmt>> = Vec::new();
                                   // new function call return object of vec < box >
    // obj stack points to heap
    
    let programmer:variables  =  variables{datatypes:8} ; // stack
    
    println!("{:p}",&programmer);
    
    let freshers:object       = object{structvariables:100}  ;
    
    
     let b = Box::new(programmer); // stack data copied to heap mmeory data copy boroowed 
    
     obj.push(Box::new(freshers)); // stack data copied to heap mmeory data copy boroowed
    
     obj.push(Box::new(programmer));
   
     println!("{}",programmer.datatypes);
     
     //  obj.push(Box::new(variables));
    
   
   for mut i in obj{
       i.display();
   }
     
 }
 