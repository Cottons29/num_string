
use std::ops::Add;
use calculate_struct_trait::CalculateStringTrait;
use crate::FloatNumberString;
use crate::operators::utils::{set_dot, set_frac_pair};

impl Add for FloatNumberString {
    type Output = FloatNumberString;
    fn add(mut self, mut other: FloatNumberString) -> Self::Output {
        (self, other) = set_frac_pair(self, other);
        println!("after set frac_pair num1 : {}", self);
        println!("after set frac_pair num2 : {}", other);
        let frac_len = other.get_frac_len();
        println!("frac_len : {}", frac_len);
        let temp_result = self.get_no_dot() + other.get_no_dot();
        println!("temp_result : {}", temp_result);
        let final_res = set_dot(temp_result, frac_len);
        println!("self = {}, other = {}", self, other);
        
        final_res
    }
}

impl Add for &FloatNumberString {
    type Output = FloatNumberString;
    fn add(self, other: &FloatNumberString) -> Self::Output {
        self.clone() + other.clone()
    }
}



