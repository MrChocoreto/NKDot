use std::{char, vec};
use std::collections::HashMap;

// <<---------Cons---------->>

// <----SBK Dictionary---->
// sbjp => short base of japanese pairs
fn get_sbjp_dic() -> HashMap<char, &'static str> {
    let mut dictionary = HashMap::new();
    dictionary.insert('a', "zr");
    dictionary.insert('s', "ih");
    dictionary.insert('d', "ni");
    dictionary.insert('s', "sa");
    dictionary.insert('d', "on");
    dictionary.insert('d', "og");
    dictionary.insert('d', "rk");
    dictionary.insert('d', "na");
    dictionary.insert('d', "hc");
    dictionary.insert('d', "ky");
    dictionary.insert('d', "ju");
    dictionary.insert('d', "ji");
    dictionary
}

// sbj => short base of japanese
fn get_sbj_dic()->HashMap<i32,char>{
    let mut dic = HashMap::new();
    dic.insert(0, '0');
    dic.insert(1, '1');
    dic.insert(2, '2');
    dic.insert(3, '3');
    dic.insert(4, '4');
    dic.insert(5, '5');
    dic.insert(6, 'k');
    dic.insert(7, 'n');
    dic.insert(8, 'h');
    dic.insert(9, 'y');
    dic.insert(10, 'u');
    dic.insert(11, 'j');
    dic
}




// <<---------Funcs---------->>

// <----Natex---->

//natex => natural expression
pub fn natex(data:String){
    let mut vec_data:Vec<i32> = vec![];
    for item in data.chars() {
        vec_data.push(item as i32);
    }
    let _new_data = sbjp(vec_data);
}


fn sbjp(data:Vec<i32>)->Vec<String>{
    let mut result:Vec<String> = vec![];
    
    for item in data.clone(){
        let num = sbj_value(item);
        result.push(num); 
    }
    result
}


fn sbj_value(number:i32)-> String{
    // Crear un HashMap con valores predefinidos
    let dic_sbj: HashMap<char, &str> = get_sbjp_dic();
    println!("el dato es: {}", dic_sbj[&'a']); 
    // result.push(String::from(dic_sbj[&_num]));
 
    String::from(number.to_string())
}








// <----Revex------>

//revex => reverse expression
pub fn _revex(){

}

