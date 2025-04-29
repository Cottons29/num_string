
use std::ops::Mul;
use calculate_struct_trait::CalculateStringTrait;
use crate::{NumberString};

impl Mul<NumberString> for NumberString {
    type Output = NumberString;
    fn mul(self, other: NumberString) -> Self::Output {
        let is_positive_res = (self.is_positive() && other.is_positive()) || (self.is_negative() && other.is_negative());
        let mut final_result = NumberString::from("0");
        let (main_number, multiplier) = if self.to_unsigned().is_greater_than(&other.to_unsigned()) {
            (self.to_unsigned(), other.to_unsigned())
        }else{
            (other.to_unsigned(), self.to_unsigned())
        };
        let multi_chars = multiplier.to_char();
        
        for i in (0..multi_chars.len()).rev() {
            let mut temp_result = NumberString::from("0");
            // println!("chat at index{} = {}", i, multi_chars[i]);
            let loop_count:u8 = multi_chars[i].to_digit(10).unwrap() as u8;
            // println!("loop count {}", loop_count);
            for a in 0..loop_count {
                temp_result = temp_result +  main_number.clone();
                // println!("attempt-{}temp_result : {}",a,  temp_result);
            }
            temp_result.insert_zero((multi_chars.len() - i - 1) as u16);
            final_result = final_result + temp_result;
            // println!("final_result : {}", final_result);
        }
        if !is_positive_res {
            final_result = NumberString::from("0") - final_result;
        }
        final_result
    }
}