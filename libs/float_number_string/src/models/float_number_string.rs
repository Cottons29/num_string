use std::fmt::format;
use std::io::{Error, ErrorKind};
use std::ops::Rem;
use regex::Regex;
use calculate_struct_trait::CalculateStringTrait;
use number_string::{NumberString};
#[derive(Clone, Debug)]
pub struct FloatNumberString{
    int_part: NumberString,
    frac_part: NumberString,
    negative: bool,
}

impl FloatNumberString{
    pub fn new() -> FloatNumberString{
        FloatNumberString{
            int_part: NumberString::new(),
            frac_part: NumberString::new(),
            negative: false,
        }
    }

    pub fn from_number<T: Rem + ToString>(number: T) -> FloatNumberString{
        FloatNumberString::new_with_string(&number.to_string())
    }

    fn remove_useless_float_zero(value: &mut NumberString) -> &NumberString {
        value.set_zero_count(0);
        value
    }

    pub fn new_with_string(value: &str) -> FloatNumberString {
        let format = Regex::new(r"^-?\d+(\.\d+)?$").unwrap();
        if !format.is_match(value){
            panic!("Invalid floating point format");
        }
        let (int_part,frac_part) = match value.split_once('.'){
            Some((int_part, frac_part)) => (int_part, frac_part),
            None => (value, "0"),
        };
        let mut int_part: NumberString = int_part.into();
        let mut frac_part: NumberString = frac_part.into();
        let negative = if int_part.is_negative(){
            int_part.to_positive();
            true
        }else {
            false
        };
        // println!("before remove useless float zero : 0.{}", frac_part);
        Self::remove_useless_float_zero(&mut frac_part);
        // println!("after remove useless float zero : 0.{}", frac_part);
        // println!("full floating number : {}.{}", int_part, frac_part);

        FloatNumberString{
            int_part,
            frac_part,
            negative
        }
    }
    
    pub fn get_int_part(&self) -> NumberString{
        self.clone().int_part
    }
    
    pub fn get_frac_part(&self) -> NumberString{
        self.clone().frac_part
    }
    
    pub fn match_frac(&mut self, other: &FloatNumberString){
        if self.frac_part.len() > other.frac_part.len(){
            return
        }
        if self.frac_part.len() == other.frac_part.len(){
            return
        }
        let diff_frac_len = other.frac_part.len() - self.frac_part.len();
        self.frac_part.set_zero_count(diff_frac_len as u16)
    }
    
    pub fn get_no_dot(&self) -> NumberString{
        let sign = if self.negative { "-" } else { "" };
        format!("{}{}{}",sign, self.int_part, self.frac_part).into()
    }
    
    pub fn to_friendly(&self) -> String{
        self.value().to_string()
    }

}


impl CalculateStringTrait<FloatNumberString> for FloatNumberString{
    fn len(&self) -> usize {
        self.value().len()
    }
    
    fn value(&self) -> String {
        let sign: String = match &self.negative {
            false => String::from(""),
            true => String::from("-")
        };
        let fl_string = self.clone();
        let temp = format!("{}{}.{}",sign ,fl_string.int_part, fl_string.frac_part);
        temp
    }

    fn to_char(&self) -> Vec<char> {
        self.value().chars().collect()
    }

    fn increment(&mut self) {
        self.int_part.increment();
    }

    fn decrement(&mut self) {
        self.int_part.decrement();
    }

    fn equals(&self, other: &FloatNumberString) -> bool {
        self.int_part == other.int_part && self.frac_part == other.frac_part && self.negative == other.negative 
    }

    fn is_negative(&self) -> bool {
        self.negative
    }

    fn is_positive(&self) -> bool {
        !self.int_part.is_positive()
    }

    fn is_greater_than(&self, other: &FloatNumberString) -> bool {
        if !self.negative {
            if other.negative{
                return true;
            }
        }
        
        if self.int_part > other.int_part {
            return true;
        }
        if self.int_part == other.int_part {
            if self.frac_part > other.frac_part {
                return true;
            }
        }
        
        false
    }

    fn is_smaller_than(&self, other: &FloatNumberString) -> bool {
        !self.is_greater_than(other)
    }

    fn is_greater_or_eq(&self, other: &FloatNumberString) -> bool {
        self.is_greater_than(other) || self.equals(other)
    }

    fn is_smaller_or_eq(&self, other: &FloatNumberString) -> bool {
        self.is_greater_than(other) || self.equals(other)
    }

    fn to_unsigned(&self) -> FloatNumberString {
        let temp = self.clone();
        temp.int_part.to_unsigned();
        temp
        
    }

    fn to_negative(&mut self) {
        self.negative = true;
    }

    fn to_positive(&mut self) {
        self.negative = false;
    }
    
}



