use std::ops::Mul;
use calculate_struct_trait::CalculateStringTrait;
use crate::FloatNumberString;
use crate::operators::utils::set_dot;

impl Mul for FloatNumberString{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let total_frac = self.get_frac_len() + other.get_frac_len();
        println!("total frac: {}", total_frac);
        let temp_res = self.get_no_dot() * other.get_no_dot();
        let final_res = set_dot(temp_res, total_frac);
        
        final_res
    }
}