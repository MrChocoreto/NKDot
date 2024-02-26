use crate::anti_atoms::anti_mg;
// use crate::anti_atoms::anti_fe;

pub fn _backward(){
    anti_mg::_revex();
    // anti_fe::_revex();
}


pub fn forward(data:String){
    anti_mg::natex(data);
    // anti_fe::_natex(String::from(""));
}
