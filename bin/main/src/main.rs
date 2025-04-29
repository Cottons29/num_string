mod test;

use colored::{ColoredString, Colorize};

use float_number_string::FloatNumberString;
use number_string::NumberString;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let a = FloatNumberString::from("12.12");
    let b = FloatNumberString::from("0.12111111111");
    
    let res = a + b;
    
    println!("res = {}", res);
    

    Ok(())
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
