
use std::fmt::format;
use std::io::{Error, ErrorKind};
use std::ops::Rem;
use regex::Regex;
use calculate_struct_trait::CalculateStringTrait;
use number_string::{NumberString};
#[derive(Clone, Debug)]
pub struct FloatNumberString{
    value: String,
}

impl FloatNumberString{
    pub fn new() -> FloatNumberString{
        FloatNumberString{
            value: String::from("0.0")
        }
    }

    pub fn from_number<T: Rem + ToString>(number: T) -> FloatNumberString{
        FloatNumberString::new_with_string(&number.to_string())
    }

    fn remove_useless_float_zero(value: &mut String) -> String {
        let chars = value.chars().collect::<Vec<char>>();
        for index in (0..chars.len()).rev(){
            if chars[index] == '0' {
                value.remove(index);
            }else{
                break;
            }
        }
        if value.len() == 0{
            value.push('0');
        }
        value.to_string()
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
        let mut frac_part:String = Self::remove_useless_float_zero(&mut frac_part.to_string());
        // println!("before remove useless float zero : 0.{}", frac_part);
        // println!("after remove useless float zero : 0.{}", frac_part);
        // println!("full floating number : {}.{}", int_part, frac_part);
        let value =  format!("{}.{}", int_part, frac_part);
        FloatNumberString{
            value
        }
    }
    
    pub fn get_no_dot(&self) -> NumberString{
        let mut temp = self.value.clone();
        temp = temp.replace(".","");
        NumberString::from(temp)
    }
    pub fn is_positive(&self) -> bool{
        if self.value.starts_with('-'){
            return false;
        }
        true
    }
    pub fn is_negative(&self) -> bool{
        !self.is_positive()
    }
    
    pub fn split_int_frac(&self) -> (NumberString, String){
        let (int_part, frac_part) = match self.value.clone().split_once(".") { 
            Some((int_part, frac_part)) => (int_part.to_string(), frac_part.to_string()),
            None => ("0".to_string(), "0".to_string())
        };
        (NumberString::from(int_part), frac_part)
    }
    
    pub fn get_frac_len(&self) -> usize{
        let temp = self.value.clone();
        let mut counter = 0;
        for char in temp.chars().rev(){
            if char == '.'{
                break
            }else{
                counter += 1;
            }
        }
        println!("value: {} | frac_len : {}", temp, counter);
        counter
    }
    
    pub fn match_frac(&mut self, other: &mut Self){
        let (self_int_part, mut self_frac_part) = self.split_int_frac();
        let (other_int_part, mut other_frac_part) = other.split_int_frac();
        let diff_len = if self_frac_part.len() > other_frac_part.len() {
                                self_frac_part.len() - other_frac_part.len()
                            }else{
                                other_frac_part.len() - self_frac_part.len()
                            };
        let other_frac_part_len = other_frac_part.len();
        let self_frac_part_len = self_frac_part.len();
        for _ in 0..diff_len  {
            if other_frac_part_len > self_frac_part_len{
                self_frac_part.push_str("0");
            }else{
                other_frac_part.push_str("0");
            }
        }
        self.value = format!("{}.{}", self_int_part, self_frac_part);
        other.value = format!("{}.{}", other_int_part, other_frac_part);
    }
}


impl CalculateStringTrait<FloatNumberString> for FloatNumberString{
    fn len(&self) -> usize {
        self.value().len()
    }
    
    fn value(&self) -> String {
        self.value.clone()
    }

    fn to_char(&self) -> Vec<char> {
        self.value().chars().collect()
    }

    fn increment(&mut self) {
        // self.int_part.increment();
    }

    fn decrement(&mut self) {
        // self.int_part.decrement();
    }

    fn equals(&self, other: &FloatNumberString) -> bool {
        let (self_int_part, self_frac_part) = self.split_int_frac();
        let (other_int_part, other_frac_part) = other.split_int_frac();
        if self_int_part > other_int_part || self_int_part < other_int_part{
            return false;
        }
        if self_frac_part != other_frac_part{
            return false;
        }
        true
    }

    fn is_negative(&self) -> bool {
        self.value.starts_with('-')
    }

    fn is_positive(&self) -> bool {
        !self.is_negative()
    }

    fn is_greater_than(&self, other: &FloatNumberString) -> bool {
        let (self_int_part, self_frac_part) = self.split_int_frac();
        let (other_int_part, other_frac_part) = other.split_int_frac();
        if self_int_part > other_int_part{
            return true
        }
        
        let self_chars = self_frac_part.chars().collect::<Vec<char>>();
        let other_chars = other_frac_part.chars().collect::<Vec<char>>();
        
        
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
        let mut temp =self.clone();
        if !self.is_positive(){
            temp.value = temp.value.trim_start_matches("-").to_string();
        }
        temp
        
    }

    fn to_negative(&mut self) {
        if self.is_positive(){
            self.value.insert(0,'-');
        }
    }

    fn to_positive(&mut self) {
        if self.is_negative(){
            self.value = self.value.replace('-', "");
        }
    }
    
}



