
use std::ops::Deref; // trait already impleneted here 

/*  

prewritten trait already existing 

trait Deref{

    fn deref(&Self);
}

*/

struct HoldsANumber(u8);

impl Deref for HoldsANumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct rust{
    version:f32,
}

impl Deref for rust {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.version
    }
}

fn main() {
    
    let my_number = HoldsANumber(20);
    println!("{:?}", *my_number + 20);

    let vinay =rust{version:3.4};
    println!("{:?}", *vinay + 20.34);
}