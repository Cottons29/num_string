use calculate_struct_trait::CalculateStringTrait;
use number_string::NumberString;
use crate::FloatNumberString;

pub fn set_dot(number: NumberString, frac_len: usize) -> FloatNumberString {
    if number == NumberString::new(){
        return FloatNumberString::new();
    }
    let mut temp = number.value();
    let dot_pos = temp.len() - frac_len;
    temp.insert(dot_pos, '.');
    FloatNumberString::from(temp)
}

pub fn set_frac_pair(mut num1: FloatNumberString, mut num2: FloatNumberString) -> (FloatNumberString, FloatNumberString) {
    if num1.get_frac_part().len() > num2.get_frac_part().len() {
        num2.match_frac(&num1);
    }else{
        num1.match_frac(&num2);
    }
    (num1, num2)
}
