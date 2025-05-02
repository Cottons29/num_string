use std::fmt::format;
use std::ops::Div;
use calculate_struct_trait::CalculateStringTrait;
use number_string::NumberString;
use crate::FloatNumberString;

impl Div for FloatNumberString{
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        
        let both_len = find_both_frac_len(&self, &other);
        let mut self_cp = self.clone();
        let mut other_cp =  other.clone();
        self_cp.match_frac(&mut other_cp);
        
        let self_nd = self_cp.get_no_dot();
        let other_nd = other_cp.get_no_dot();
        
        let int_res = self_nd.clone() / other_nd.clone();
        let mut temp_remainder = self_nd % other_nd.clone();
        temp_remainder.set_zero_count(20);
        let mut frac_res = temp_remainder / other_nd;
        frac_res.to_positive();
        
        let final_res = combine_int_frac(int_res, frac_res);
        final_res
    }
}

fn find_both_frac_len(first: &FloatNumberString, sec: &FloatNumberString) -> usize{
    let self_len = first.get_frac_len();
    let other_len = sec.get_frac_len();
    if self_len == other_len{
        return self_len
    }
    if self_len > other_len{
        return self_len
    }else{
        other_len
    }
}

fn combine_int_frac(int_p: NumberString, frac_p: NumberString) -> FloatNumberString{
    
    let res = format!("{}.{}", int_p, frac_p);
    println!(" result after combined : {}", res);
    
    FloatNumberString::from(res)
}