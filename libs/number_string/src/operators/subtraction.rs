use std::ops::Sub;
use calculate_struct_trait::CalculateStringTrait;
use crate::NumberString;

impl Sub for NumberString {
    type Output = NumberString;
    fn sub(self, other: Self) -> Self::Output {
        if self.is_zero(){
            return if other.is_negative() {
                let mut temp = other.clone();
                temp.to_positive();
                temp
            } else {
                let mut temp = other.clone();
                temp.to_negative();
                temp
            }
        }
        if other.is_zero(){
            return self;
        }

        if self.is_negative() && other.is_negative() {
            if self > other {
                // println!("{} > {} result.negate", self, other);
                let mut result =  self.to_unsigned() - other.to_unsigned();
                result.to_positive();
                return result
            }
            if self < other {
                // println!("{} < {} result.positive", self, other);
                let mut result =  other.to_unsigned() - self.to_unsigned();
                result.to_negative();
                return result
            }
            if self == other {
                return NumberString::new()
            }
        }
        
        if self.is_positive() && other.is_negative() {
            return self + other.to_unsigned();
        }
        if self.is_negative() && other.is_positive(){
            let mut res = self.to_unsigned() + other.to_unsigned();
            res.to_negative();
            return res
        }
        // println!("{} - {}", self, other);
        
        
        
        let is_positive_res = self.is_greater_than(&other);
        // Pad both to the same length
        let (number_a, number_b) = if self.len() > other.len() {
            (self.clone(), other.pad_to_match(&self))
        } else {
            (self.pad_to_match(&other), other)
        };
        
        let (chars_a, chars_b) = if is_positive_res{
            (number_a.to_char(), number_b.to_char())
        }else{
            (number_b.to_char(), number_a.to_char())
        };
        let mut result = String::new();
        let mut loan = 0;

        for i in (0..chars_a.len()).rev() {
            // println!("{} {} {}", "digit 1 : {} | digit 2 : {}".bright_green().bold(), chars_a[i], chars_b[i]);
            let digit1: i8 = chars_a[i].to_digit(10).unwrap() as i8;
            let digit2: i8 = chars_b[i].to_digit(10).unwrap() as i8;
            
            let mut sub: i8 = digit1 - (digit2 + loan);
            
            if sub < 0 {
                loan =  1;
                sub = -sub;
            }else{
                loan = 0;
            }

            sub %= 10;
            // println!("sub result is = {:?}", sub);
             result.insert(0, std::char::from_digit(sub as u8 as u32, 10).unwrap());
        }

        // Handle carry at the highest digit
        if loan > 0 {
            result.insert(0, std::char::from_digit(loan as u32, 10).unwrap());
        }
        while result.starts_with('0'){
            result.remove(0);
        }
        if !is_positive_res {
            result.insert(0, '-');
        }
        // println!("final sub result is = {:?}", result);
        NumberString::new_with_string(&result).unwrap()
    }
}