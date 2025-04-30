use calculate_struct_trait::CalculateStringTrait;
use number_string::NumberString;
use crate::FloatNumberString;

pub fn set_dot(number: NumberString, frac_len: usize) -> FloatNumberString {
    if number == NumberString::new(){
        return FloatNumberString::new();
    }
    
    // println!("set_dot: number : {}", number);
    // println!("set_dot: num len : {}, frac len : {}", number.len(), frac_len);
    let mut temp = number.value();
    if temp.len() <= frac_len {
        let diff = frac_len - temp.len() + 1;
        for _i in 0..diff {
            temp.insert(0, '0');
        }
    }
    let dot_pos = temp.len() - frac_len;
    temp.insert(dot_pos, '.');
    println!("res : {}", temp);
    FloatNumberString::from(temp)
}

pub fn set_frac_pair(mut num1: FloatNumberString, mut num2: FloatNumberString) -> (FloatNumberString, FloatNumberString) {
    println!("before set frac_pair num1 : {}", num1);
    println!("before set frac_pair num2 : {}", num2);
    num1.match_frac(&mut num2);
    (num1 , num2)
}
