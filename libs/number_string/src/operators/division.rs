use std::ops::Div;
use std::panic;
use std::thread::sleep;
use std::time::Duration;
use colored::Colorize;
use calculate_struct_trait::CalculateStringTrait;
use crate::{NumberString};

impl Div for NumberString{
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        let is_positive_res =( self.is_positive() && other.is_positive()) || (self.is_negative() && other.is_negative());
        let mut unsign_self = self.to_unsigned();
        let mut unsign_other = other.to_unsigned();
        if unsign_self == unsign_other {
            return if is_positive_res {
                NumberString::from("1")
            } else {
                NumberString::from("-1")
            }
        }
        if unsign_other.is_zero(){
            panic!("division by zero is not allowed");
        }
        if unsign_self.is_zero() {
            return self;
        }
        
        // println!("self: {}, other: {}", self, other);
        
        let self_zero_count = unsign_self.get_zero_count();
        let other_zero_count = unsign_other.get_zero_count();
        if self_zero_count != other_zero_count {
            let (self_num, self_zero_len) = self.split_num_zero_len();
            let (other_num, other_zero_len) = other.split_num_zero_len();
            if self_num == other_num {
                let mut temp_res = NumberString::from("1");
                let diff_zero_len = self_zero_len - other_zero_len;
                temp_res.set_zero_count(diff_zero_len);
                if !is_positive_res {
                    temp_res.to_negative()
                }
                return temp_res;
            }
            if self_zero_count > other_zero_count {
                for _ in 1..other_zero_count {
                    unsign_other.pop_zero();
                    unsign_self.pop_zero();
                }
            }
        }
        
        if unsign_other > unsign_self {
            // println!("other is greater than self");
            return NumberString::new()
        }
        let (mut res, _) = div_helper(unsign_self, unsign_other);
        if !is_positive_res {
            res.to_negative()
        }
        res
    }
}

pub fn div_helper(first: NumberString, sec: NumberString) -> (NumberString, NumberString) {
    let mut res = String::new();
    let mut remainder = first.clone();
    let mut shifted_divisor = sec.clone();

    let diff_len = first.len() - sec.len();
    shifted_divisor.set_zero_count(diff_len as u16); // equivalent to multiplying by 10^diff_len
    let mut to_break = false;
    loop {
        if shifted_divisor.clone() < sec {
            break;
        }
        if shifted_divisor.clone() == sec {
            to_break = true;
        }

        // Find max digit d (1..=9) such that shifted_divisor * d <= remainder
        let mut digit = 0;
        for d in 0..10 {
            let product = shifted_divisor.clone() * NumberString::from(d);
            if product <= remainder {
                digit = d;
            } else {
                break;
            }
        }

        // Subtract and append result digit
        let product = shifted_divisor.clone() * NumberString::from(digit);
        remainder = remainder - product;
        res.push(char::from_digit(digit as u32, 10).unwrap());
        shifted_divisor.pop_zero();
        if to_break {
            break;
        }
        
    }
    println!("{} {}", "remainder = ".bright_green().bold(), remainder);
    println!("{} {}", "final division result = ".bright_green().bold(), res);
    (NumberString::from(res), remainder)
}
