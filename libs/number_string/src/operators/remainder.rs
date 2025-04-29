use std::ops::Rem;
use calculate_struct_trait::CalculateStringTrait;
use crate::{NumberString};
use crate::operators::division::div_helper;

impl Rem for NumberString {
    type Output = NumberString;
    fn rem(self, other: Self) -> Self::Output {
        if self < other {
            return self;
        }
        let unsign_self = self.to_unsigned();
        let unsign_other = other.to_unsigned();
        let  (_ , remainder) = div_helper(unsign_self, unsign_other );
        remainder
    }
}