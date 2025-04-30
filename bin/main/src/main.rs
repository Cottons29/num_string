mod test;

use std::collections::HashMap;
use std::iter::Map;
use std::time::{Duration, Instant, SystemTime};
use colored::{ColoredString, Colorize};
use rand::Rng;
use number_string::NumberString;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // test_random_multiply();
    let number1 = NumberString::from("814742273982937866670267012036612826553710913");
    let number2 = NumberString::from("756545270190436525319163108548750744679063400");
    println!("number1 : {}", number1);
    println!("number2 : {}", number2);
    let res = number1 * number2;
    println!("res: {}", res);
    Ok(())
}

#[allow(dead_code)]
fn test_random_multiply(){
    let mut data_collection: HashMap<usize, Duration> = HashMap::new();
    let temp_duration = Instant::now();
    for i in 5..100{
        print!("Running for : {:?}\r", temp_duration.elapsed());
        let num1 = NumberString::from(generate_random_numeric_string(i));
        let num2 = NumberString::from(generate_random_numeric_string(i));
        println!("num1 : {}", num1);
        println!("num2 : {}", num2);
        let start = Instant::now();
        let res_ = num1 * num2;
        let duration = start.elapsed();
        data_collection.insert(i, duration);

    }
    data_collection.iter().is_sorted();
    println!("{:?}", data_collection);
}
#[allow(dead_code)]
fn generate_random_numeric_string(length: usize) -> String {
    let mut rng = rand::rng();
    (0..length)
        .map(|_| {
            // Generate random digit from 0 to 9
            let digit: u8 = rng.random_range(0..10);
            // Convert to character
            (digit + b'0') as char
        })
        .collect()
}
#[allow(dead_code)]
fn debug_print(num1: NumberString, operator: char, num2: NumberString) -> ColoredString {
    let (a,b) = (num1.clone(), num2.clone());
    let res = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("Unknown operator {}", operator)
    };
    format!("{} {} {} = {}", a, operator, b, res).bright_green().bold()
}
