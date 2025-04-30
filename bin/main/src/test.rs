

#[cfg(test)]
mod test_number_string {
    #[allow(unused_imports)]
    use number_string::NumberString;

    #[test]
    fn test_div_eq(){
        let first = NumberString::from("12");
        let second = NumberString::from("12");
        
        let res = first / second;
        
        assert_eq!(res, NumberString::from("1"));
    }
    
    #[test]
    fn test_div_greater(){
        let first = NumberString::from("12");
        let second = NumberString::from("13");

        let res = first / second;

        assert_eq!(res, NumberString::from("0"));
    }

    #[test]
    #[should_panic(expected = "division by zero is not allowed")]
    fn test_div_0(){
        let first = NumberString::from("12");
        let second = NumberString::from("0");

        let _res = first / second;
        
    }

    #[test]
    fn test_div_w_0(){
        let first = NumberString::from("0");
        let second = NumberString::from("13");

        let res = first / second;

        assert_eq!(res, NumberString::from("0"));
    }

    #[test]
    fn test_div_diff(){
        let first = NumberString::from("1700");
        let second = NumberString::from("-85");
        

        let res = first / second;

        assert_eq!(res, NumberString::from("-20"));
    }

    #[test]
    fn test_positive_div_positive() {
        let first = NumberString::from("1700");
        let second = NumberString::from("85");
        let res = first / second;
        assert_eq!(res, NumberString::from("20"));
    }

    #[test]
    fn test_positive_div_negative() {
        let first = NumberString::from("1700");
        let second = NumberString::from("-85");
        let res = first / second;
        assert_eq!(res, NumberString::from("-20"));
    }

    #[test]
    fn test_negative_div_positive() {
        let first = NumberString::from("-1700");
        let second = NumberString::from("85");
        let res = first / second;
        assert_eq!(res, NumberString::from("-20"));
    }

    #[test]
    fn test_negative_div_negative() {
        let first = NumberString::from("-1700");
        let second = NumberString::from("-85");
        let res = first / second;
        assert_eq!(res, NumberString::from("20"));
    }

    #[test]
    fn test_zero_div_positive() {
        let first = NumberString::from("0");
        let second = NumberString::from("12345");
        let res = first / second;
        assert_eq!(res, NumberString::from("0"));
    }

    #[test]
    fn test_zero_div_negative() {
        let first = NumberString::from("0");
        let second = NumberString::from("-12345");
        let res = first / second;
        assert_eq!(res, NumberString::from("0"));
    }

    #[test]
    #[should_panic(expected = "division by zero is not allowed")]
    fn test_positive_div_zero() {
        let first = NumberString::from("123");
        let second = NumberString::from("0");
        let _res = first / second;
    }

    #[test]
    #[should_panic(expected = "division by zero is not allowed")]
    fn test_negative_div_zero() {
        let first = NumberString::from("-123");
        let second = NumberString::from("0");
        let _res = first / second;
    }

    #[test]
    fn test_smaller_dividend() {
        let first = NumberString::from("5");
        let second = NumberString::from("10");
        let res = first / second;
        assert_eq!(res, NumberString::from("0"));
    }

    #[test]
    fn test_same_number_division_positive() {
        let first = NumberString::from("999");
        let second = NumberString::from("999");
        let res = first / second;
        assert_eq!(res, NumberString::from("1"));
    }

    #[test]
    fn test_same_number_division_negative() {
        let first = NumberString::from("-999");
        let second = NumberString::from("-999");
        let res = first / second;
        assert_eq!(res, NumberString::from("1"));
    }

    #[test]
    fn test_large_number_division() {
        let first = NumberString::from("2000000000000");
        let second = NumberString::from("2000000");
        let res = first / second;
        assert_eq!(res, NumberString::from("1000000"));
    }
    #[test]
    fn test_self_addition_assign(){
        let mut a = NumberString::from("1");
        let b = NumberString::from("2");
        a += b;
        
        assert_eq!(a, NumberString::from("3"));
    }
    #[test]
    fn test_self_subtraction_assign(){
        let mut a = NumberString::from("1");
        let b = NumberString::from("2");
        a -= b;
        
        assert_eq!(a, NumberString::from("-1"));
    }
    #[test]
    fn test_self_multiply_assign(){
        let mut a = NumberString::from("1");
        let b = NumberString::from("2");
        a *= b;
        
        assert_eq!(a, NumberString::from("2"));
    }
    #[test]
    fn test_self_div_assign(){
        let mut a = NumberString::from("2");
        let b = NumberString::from("2");
        a /= b;
        
        assert_eq!(a, NumberString::from("1"));
    }
    #[test]
    fn test_arithmetic_a_larger(){
        let a = NumberString::from("5");
        let b = NumberString::from("2");
        let res =  a % b;
        
        assert_eq!(res, NumberString::from("1"));
    }
    
    #[test]
    fn test_arithmetic_b_larger(){
        let a = NumberString::from("2");
        let b = NumberString::from("5");
        let res =  a % b;
        
        assert_eq!(res, NumberString::from("2"));
    }
    #[test]
    fn test_pos_to_neg(){
        let mut a = NumberString::from("2");
        a = -a;
        
        assert_eq!(a, NumberString::from("-2"));
    }
    #[test]
    fn test_neg_to_pos(){
        let mut a = NumberString::from("-2");
        a = -a;

        assert_eq!(a, NumberString::from("2"));
    }
    #[test]
    fn test_zero_to_neg(){
        let mut a = NumberString::from("0");
        a = -a;

        assert_eq!(a, NumberString::from("0"));
    }
    #[test]
    fn test_zero_to_pos(){
        let mut a = NumberString::from("0");
        a = -a;

        assert_eq!(a, NumberString::from("0"));
    }
    
}

#[cfg(test)]
mod test_float_number_string{
    use float_number_string::FloatNumberString;
    
    #[test]
    fn new_float_number(){
        let fl = FloatNumberString::new();
        
        assert_eq!(fl, FloatNumberString::new_with_string("0.0"))
    }
    
    #[test]
    fn add_same_sign(){
        let fl1 = FloatNumberString::new_with_string("2.22111");
        let fl2 = FloatNumberString::new_with_string("2.22");
        
        let res = fl1 + fl2;
        assert_eq!(res, FloatNumberString::from("4.44111"));
    }
    #[test]
    fn add_many_frac(){
        let fl1 = FloatNumberString::new_with_string("0.000000001");
        let fl2 = FloatNumberString::new_with_string("0.1");
        
        let res = fl1 + fl2;
        assert_eq!(res, FloatNumberString::from("0.100000001"));
    }
    #[test]
    fn add_same_sign_neg(){
        let fl1 = FloatNumberString::new_with_string("-2.22111");
        let fl2 = FloatNumberString::new_with_string("-2.22");
        
        let res = fl1 + fl2;
        assert_eq!(res, FloatNumberString::from("-4.44111"));
    }

    #[test]
    fn add_diff_sign_sec(){
        let fl1 = FloatNumberString::new_with_string("2.22");
        let fl2 = FloatNumberString::new_with_string("-2.22");

        let res = fl1 + fl2;
        assert_eq!(res, FloatNumberString::from("0.00"));
    }
    #[test]
    fn add_diff_sign_first(){
        let fl1 = FloatNumberString::new_with_string("-2.22");
        let fl2 = FloatNumberString::new_with_string("2.22");

        let res = fl1 + fl2;
        assert_eq!(res, FloatNumberString::from("0.00"));
    }
    #[test]
    fn sub_diff_sign_sec(){
        let fl1 = FloatNumberString::new_with_string("2.22");
        let fl2 = FloatNumberString::new_with_string("-2.22");

        let res = fl1 - fl2;
        assert_eq!(res, FloatNumberString::from("4.44"));
    }
    #[test]
    fn sub_same_sign_neg(){
        let fl1 = FloatNumberString::new_with_string("-2.22");
        let fl2 = FloatNumberString::new_with_string("-4.22");
        
        println!("fl1 = {}, fl2 = {}", fl1, fl2);

        let res = fl1 - fl2;
        assert_eq!(res, FloatNumberString::from("2.0"));
    }
    #[test]
    fn sub_same_sign_pos(){
        let fl1 = FloatNumberString::new_with_string("2.22");
        let fl2 = FloatNumberString::new_with_string("2.22");

        let res = fl1 - fl2;
        assert_eq!(res, FloatNumberString::from("0.0"));
    }
    #[test]
    fn sub_diff_sign_first(){
        let fl1 = FloatNumberString::new_with_string("-2.22");
        let fl2 = FloatNumberString::new_with_string("2.22");

        let res = fl1 - fl2;
        assert_eq!(res, FloatNumberString::from("-4.44"));
    }
    #[test]
    fn mul_same_sign_pos(){
        let fl1 = FloatNumberString::new_with_string("0.0001");
        let fl2 = FloatNumberString::new_with_string("0.0001");
    
        let res = fl1 * fl2;
        assert_eq!(res, FloatNumberString::from("0.00000001"));
    }
    #[test]
    fn mul_same_sign_neg(){
        let fl1 = FloatNumberString::new_with_string("-0.0001");
        let fl2 = FloatNumberString::new_with_string("-0.0001");
    
        let res = fl1 * fl2;
        assert_eq!(res, FloatNumberString::from("0.00000001"));
    }
    #[test]
    fn mul_large_frac(){
        let fl1 = FloatNumberString::new_with_string("0.000000000000000001");
        let fl2 = FloatNumberString::new_with_string("0.000000000000000001");
    
        let res = fl1 * fl2;
        assert_eq!(res, FloatNumberString::from("0.000000000000000000000000000000000001"));
    }
    #[test]
    fn mul_diff_sign_sec(){
        let fl1 = FloatNumberString::new_with_string("0.0001");
        let fl2 = FloatNumberString::new_with_string("-0.0001");
    
        let res = fl1 * fl2;
        assert_eq!(res, FloatNumberString::from("-0.00000001"));
    }
    #[test]
    fn mul_diff_sign_first(){
        let fl1 = FloatNumberString::new_with_string("-0.0001");
        let fl2 = FloatNumberString::new_with_string("0.0001");
    
        let res = fl1 * fl2;
        assert_eq!(res, FloatNumberString::from("-0.00000001"));
    }
    #[test]
    fn mul_large_num_large_frac(){
        let fl1 = FloatNumberString::new_with_string("12876811232737123.12876811232737123");
        let fl2 = FloatNumberString::new_with_string("989218712323412411.989218712323412411");
    
        let res = fl1 * fl2;
        assert_eq!(res, FloatNumberString::from("12737982626479869755534779969912118.89696650812976777915416971078633553"));
    }
    
    
    
    
}