mod anti_atoms;
mod keys;

use crate::anti_atoms::anti_fusion;

fn main() {
    println!("Hello, world!");
    nkrypt();
}



fn _dkrypt(){
    anti_fusion::backward()
}

fn nkrypt(){
    anti_fusion::forward()
}
