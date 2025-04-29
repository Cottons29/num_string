use calculate_struct_trait::CalculateStringTrait;
use crate::{NumberString};

impl PartialEq<NumberString> for NumberString {
    fn eq(&self, other: &NumberString) -> bool {
        self.equals(other)
    }

    fn ne(&self, other: &NumberString) -> bool {
        !self.equals(other)
    }
}