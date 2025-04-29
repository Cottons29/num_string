#[allow(dead_code)]
pub trait IntoNumberString {
    fn into(self) -> NumberString;
}

use std::fmt::{Display, Formatter};
use calculate_struct_trait::CalculateStringTrait;
use crate::{impl_from_for_number_string, impl_into_number_string_for, NumberString};


impl Default for NumberString {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for NumberString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl From<&str> for NumberString {
    fn from(s: &str) -> Self {
        NumberString::new_with_string(s).unwrap()
    }
}

impl From<String> for NumberString {
    fn from(s: String) -> Self {
        NumberString::new_with_string(&s).unwrap()
    }
}


impl_from_for_number_string!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);


impl_into_number_string_for!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);