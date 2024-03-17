use std::vec;
use std::collections::HashMap;

// <<---------Cons---------->>

// <----SBK Dictionary---->
//
// sbj => short base of japanese
fn get_sbj_dic()->HashMap<i32,&'static str>{
    let mut dic = HashMap::new();
    dic.insert(0, "zr");
    dic.insert(1, "ih");
    dic.insert(2, "ni");
    dic.insert(3, "sa");
    dic.insert(4, "on");
    dic.insert(5, "og");
    dic.insert(6, "rk");
    dic.insert(7, "na");
    dic.insert(8, "hc");
    dic.insert(9, "ky");
    dic.insert(10, "ju");
    dic
}




// <<---------Funcs---------->>

// <----Natex---->

//natex => natural expression
pub fn natex(data:String){
    let mut vec_data:Vec<i32> = vec![];
    //recorre la data para convertirlo en un arreglo de datos
    //de esta manera trabajar individualemente cada dato por separado
    for item in data.chars() {
        vec_data.push(item as i32);
    }

    //creo un arreglo nuevo de objetos de tipo String en base a 
    //los valores ascii de cada dato dado, para preservar el valores
    //y poder reestructurar cada dato
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
    let _dic_sbj:HashMap<i32,&'static str> = get_sbj_dic();
    // if number % 2 == 0 {
    //     println!("el dato es: {}", number.to_string()); 
    // }
    String::from(number.to_string())
}








// <----Revex------>

//revex => reverse expression
pub fn _revex(){

}

