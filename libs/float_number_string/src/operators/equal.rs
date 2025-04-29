use calculate_struct_trait::CalculateStringTrait;
use crate::FloatNumberString;

impl PartialEq<FloatNumberString> for FloatNumberString {
    fn eq(&self, other: &FloatNumberString) -> bool {
        self.equals(other)
    }
}