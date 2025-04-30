use std::ops::{Sub};
use calculate_struct_trait::CalculateStringTrait;
use crate::FloatNumberString;
use crate::operators::utils::{set_dot, set_frac_pair};

impl Sub for FloatNumberString {
    type Output = Self;
    fn sub(mut self, mut other: Self) -> Self::Output {
        (self, other ) = set_frac_pair(self, other);
        let frac_len = self.get_frac_len();
        println!("subtracting : {} - {}", self.get_no_dot() , other.get_no_dot());
        let temp_result = self.get_no_dot() - other.get_no_dot();
        
        let final_res = set_dot(temp_result, frac_len);
        
        final_res
    }
}