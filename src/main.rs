mod anti_atoms;
mod keys;

use crate::anti_atoms::anti_fusion;

fn main() {
    nkrypt();
}



fn _dkrypt(){
    anti_fusion::_backward()
}

fn nkrypt(){
    anti_fusion::forward(String::from("Hello, world!"))
}
