
use std::fmt::{Display, Formatter};
use calculate_struct_trait::CalculateStringTrait;
use crate::FloatNumberString;

impl Display for FloatNumberString{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl From<String> for FloatNumberString{
    fn from(value: String) -> Self {
        FloatNumberString::new_with_string(&value)
    }
}

impl From<&str> for FloatNumberString{
    fn from(value: &str) -> Self {
        FloatNumberString::new_with_string(value)
    }
}