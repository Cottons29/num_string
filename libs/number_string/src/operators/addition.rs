use std::ops::Add;
use calculate_struct_trait::CalculateStringTrait;
use crate::{NumberString};

impl Add for NumberString {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut is_positive = true;
        if self.is_negative() && other.is_negative(){
            if self == other {
                is_positive = false;
            }else if self.to_unsigned() > other.to_unsigned() {
                let mut result = self.to_unsigned() - other.to_unsigned();
                result.to_negative();
                return result
            } else {
                let mut result = other.to_unsigned() + self.to_unsigned();
                result.to_negative();
                return result
            }
        }
        if self.is_positive() && other.is_negative(){
            return self.to_unsigned() - other.to_unsigned();
        }
        if self.is_negative() && other.is_positive(){
            return other.to_unsigned() - self.to_unsigned();
        }
        // if self.is_positive() && other.is_positive(){
        //     return self.to_unsigned() - other.to_unsigned();
        // }
        // Pad both to the same length
        let (a, b) = if self.to_unsigned().len() > other.to_unsigned().len() {
            (self.clone().to_unsigned(), other.to_unsigned().pad_to_match(&self))
        } else {
            (self.to_unsigned().pad_to_match(&other), other.to_unsigned())
        };

        let chars_a = a.to_char();
        let chars_b = b.to_char();
        let mut result = String::new();
        let mut carry = 0;

        for i in (0..chars_a.len()).rev() {
            let digit1 = chars_a[i].to_digit(10).unwrap();
            let digit2 = chars_b[i].to_digit(10).unwrap();

            let mut sum = digit1 + digit2 + carry;
            carry = sum / 10;
            sum %= 10;

            result.insert(0, std::char::from_digit(sum, 10).unwrap());
        }

        // Handle carry at the highest digit
        if carry > 0 {
            result.insert(0, std::char::from_digit(carry, 10).unwrap());
        }
        if !is_positive {
            result.insert(0, '-');
        }

        NumberString::new_with_string(&result).unwrap()
    }
}