use std::vec;
use std::collections::HashMap;

// <<---------Cons---------->>

// <----SBK Dictionary---->
fn get_dictionary() -> HashMap<i32, &'static str> {
    let mut dictionary = HashMap::new();
    dictionary.insert(0, "zr");
    dictionary.insert(1, "ih");
    dictionary.insert(2, "ni");
    dictionary.insert(3, "sa");
    dictionary.insert(4, "on");
    dictionary.insert(5, "og");
    dictionary.insert(6, "rk");
    dictionary.insert(7, "na");
    dictionary.insert(8, "hc");
    dictionary.insert(9, "ky");
    dictionary.insert(10, "ju");
    dictionary.insert(11, "ji");
    // dictionary.insert(72, "pp");
    // dictionary.insert(111, "oo");
    // dictionary.insert(108, "tt");
    dictionary
}



// <<---------Funcs---------->>

// <----Natex---->

//natex => natural expression
pub fn natex(data:String){
    let mut vec_data:Vec<i32> = vec![];
    for item in data.chars() {
        vec_data.push(item as i32);
    }
    let _new_data = sbj(vec_data);
}


fn sbj(data:Vec<i32>)->Vec<String>{
    // Crear un HashMap con valores predefinidos
    let dic_sbj: HashMap<i32, &str> = get_dictionary();
    let mut result:Vec<String>= vec![];
    
    for item in data.clone(){
        let num = sbj_value(item);
        println!("el dato es: {}", dic_sbj[&item]);
        
        result.push(String::from(dic_sbj[&item]))
    }
    result
}


fn sbj_value(number:i32)-> i32{

    
    1
}








// <----Revex------>

//revex => reverse expression
pub fn _revex(){

}

