
use std::ops::Add;
use calculate_struct_trait::CalculateStringTrait;
use crate::FloatNumberString;
use crate::operators::utils::{set_dot, set_frac_pair};

impl Add for FloatNumberString {
    type Output = FloatNumberString;
    fn add(mut self, mut other: FloatNumberString) -> Self::Output {
        (self, other) = set_frac_pair(self, other);
        let frac_len = self.get_frac_part().len();
        let temp_result = self.get_no_dot() + other.get_no_dot();
        let final_res = set_dot(temp_result, frac_len);
        // println!("self = {}, other = {}", self, other);
        
        final_res
    }
}

impl Add for &FloatNumberString {
    type Output = FloatNumberString;
    fn add(self, other: &FloatNumberString) -> Self::Output {
        self.clone() + other.clone()
    }
}



