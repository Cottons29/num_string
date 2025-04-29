use std::io::{Error, ErrorKind};
use std::ops::Rem;
use std::string::ToString;
use regex::Regex;
use calculate_struct_trait::CalculateStringTrait;

#[derive(Clone, Debug)]
pub struct NumberString {
    value: String,
}

impl  NumberString{
    pub fn new() -> NumberString {
        // println!("new with 0 value");
        NumberString::from("0")
    }

    pub fn from_number<T: Rem + ToString>(number: T) -> NumberString{
        NumberString::new_with_string(&number.to_string()).unwrap()
    }

    pub fn new_with_string(value: &str) -> Result<NumberString, Error> {
        let res = value.to_string();
        let format = Regex::new(r"^-?[0-9]*$").unwrap(); // allows empty string
        let is_negative = res.starts_with('-');
        let mut unsign_value =  res.trim_start_matches('-').trim_start_matches('0').to_string();
        if unsign_value.is_empty() {
            unsign_value.push('0');
        }
        // println!("unsigned value = {}", unsign_value);
        if is_negative {
            if unsign_value != "0"{
                unsign_value.insert(0, '-');
            }
        }
        if unsign_value.is_empty() {
            unsign_value.push('0');
        }
        if format.is_match(value) {
            Ok(NumberString { value: unsign_value.to_string() })
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Value is not numeric"))
        }
    }
    fn insert_zero_at_front(&mut self, count: usize) {
        for _ in 0..count {
            self.value.insert(0, '0');
        }
    }
    pub(crate) fn is_zero(&self) -> bool {
        self.value.eq("0")
    }

    pub fn get_zero_count(&self) -> u16 {
        if self.is_zero(){
            return 0
        }
        let temp_chars = self.to_char();
        let mut count: u16 = 0;
        for i in (0..temp_chars.len()).rev() {
            if temp_chars[i] == '0' {
                count += 1;
            }else{
                break;
            }
        }
        count
    }

    pub fn split_num_zero_len(&self) -> (NumberString, u16) {
        if self.is_zero(){
            return (NumberString::new(), 0);
        }
        let zero_count = self.get_zero_count();
        let mut temp_num = self.clone();
        temp_num.remove_all_zeros();
        (temp_num, zero_count)
    }
    pub fn insert_zero(&mut self, count: u16) {
        if count == 0 {
            return
        }
        for _ in 0..count {
            self.value.push('0');
        }
    }

    pub fn push_zero(&mut self) {
        self.insert_zero(1)
    }

    pub fn pop_zero(&mut self) {
        if self.is_zero(){
            return
        }
        let mut zero_count = 0;
        let temp_char = self.to_char();
        for index in (0..temp_char.len()).rev() {
            if temp_char[index] == '0'{
                zero_count += 1;
            }else{
                break;
            }
        }
        if zero_count == 0 {
            return
        }
        self.remove_all_zeros();
        self.set_zero_count(zero_count-1);
    }

    pub fn set_zero_count(&mut self, count: u16) {
        if count == 0{
            self.remove_all_zeros();
            return;
        }
        let mut counter = 0;
        let chars = self.to_char();
        for i in (0..self.len()).rev() {
            if chars[i] == '0' {
                counter += 1;
            }else{
                break;
            }
        }
        if counter > count{
            return
        }
        let to_add = count - counter;
        self.insert_zero(to_add);
    }

    pub fn remove_all_zeros(&mut self) {
        let chars = self.to_char();
        let mut counter = 0;
        for i in (0..self.len()).rev() {
            if chars[i] == '0' {
                counter += 1;
            }
        }
        let mut temp = self.value.clone();
        for _ in 0..counter {
            temp = temp.trim_end_matches('0').to_string();
        }
        *self = NumberString::from(temp);
    }
    pub fn pad_to_match(&self, target: &NumberString) -> NumberString {
        let mut cloned = self.clone();
        cloned.make_pair_digit(target);
        cloned
    }
    pub fn make_pair_digit(&mut self, target_pair: &NumberString) -> &mut NumberString {
        if target_pair.len() > self.len() {
            let diff_len = target_pair.len() - self.len();
            self.insert_zero_at_front(diff_len);
        }
        self
    }
}


impl CalculateStringTrait<NumberString> for NumberString {
    /// Create a new instance of `NewString` with numeric validation.
    
    
    /// Get the length of the numeric string.
    fn len(&self) -> usize {
        self.value.len()
    }

    /// Pad zeros at the front (private helper).
    

    /// Pad the current value with zeros to match the length of `target_pair`.
    /// Mutates in place and supports chaining.
    

    /// Immutable version that returns a new padded instance.
    

    /// Access the raw string value.
    fn value(&self) -> String {
        self.value.clone()
    }
    
    fn to_char(&self) -> Vec<char> {
        self.value.chars().collect()
    }
    
    fn increment(&mut self) {
        let temp = self.clone();
        let new_value = temp + NumberString::from("1");
        *self = new_value;
    }

    fn decrement(&mut self) {
        let temp = self.clone();
        let new_value = temp - NumberString::from("1");
        *self = new_value;
    }

    fn equals(&self, other: &NumberString) -> bool {
        if self.value.eq(&other.value) {
            return true
        }
        false
    }

    fn is_negative(&self) -> bool {
        self.value.starts_with('-')
    }

    fn is_positive(&self) -> bool {
        !self.is_negative()
    }

    fn is_greater_than(&self, other: &NumberString) -> bool {
        if self.equals(other) {
            return false
        }
        if self.is_negative() && other.is_positive(){
            return false
        }
        if self.is_positive() && other.is_negative(){
            return true
        }
        if self.equals(other) && self.len() == other.len() {
            return true
        }
        if self.len() > other.len() {
            return true
        }
        let mut result = false;
        if self.len() == other.len() {
            let char_1 = self.to_char();
            let char_2 = other.to_char();
            for i in (0..other.len()).rev() {
                let num1 = char_1[i];
                let num2 = char_2[i];
                if num1 == num2 {
                    continue
                }
                if num1 > num2 {
                    result = true;
                    break
                }
            }
        }
        if self.is_negative() && other.is_negative() {
            return !result
        }
        
        result
    }

    fn is_smaller_than(&self, other: &NumberString) -> bool {
        if self.equals(other) {
            return false
        }
       !self.is_greater_than(other)
    }

    fn is_greater_or_eq(&self, other: &NumberString) -> bool {
        self.is_greater_than(other) || self.equals(other)
    }

    fn is_smaller_or_eq(&self, other: &NumberString) -> bool {
        self.is_smaller_than(other) || self.equals(other)
    }
    
    
    
    fn to_unsigned(&self) -> NumberString {
        NumberString::from(self.value.trim_start_matches('-').trim_start_matches('0').to_string())
    }
    
    fn to_negative(&mut self) {
        if self.is_negative(){
            return;
        }else{
            self.value.insert(0, '-');
        }
    }
    
    fn to_positive(&mut self) {
        if self.is_positive(){
            return;
        }else{
            let temp = self.value.trim_start_matches('-');
            *self = NumberString::from(temp);
        }
    }
    
    
    
    
}








