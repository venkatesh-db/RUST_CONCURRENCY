
fn scalable(){

} // part of executable code 

#[cfg(iphone_thirteen)]
fn notused(){

}  // not part of executable code 

fn main(){

    notused();
    scalable();
}