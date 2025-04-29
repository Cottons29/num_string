use std::ops::{AddAssign, DivAssign, MulAssign, Neg, RemAssign, SubAssign};
use calculate_struct_trait::CalculateStringTrait;
use crate::{NumberString};

impl AddAssign for NumberString{
    fn add_assign(&mut self, other: Self) {
        let temp =  self.clone();
        *self = temp + other;
    }
}

impl SubAssign for NumberString{
    fn sub_assign(&mut self, other: Self) {
        let temp = self.clone();
        *self = temp - other;
    }
}

impl MulAssign for NumberString{
    fn mul_assign(&mut self, other: Self) {
        let temp = self.clone();
        *self = temp * other;
    }
}
impl DivAssign for NumberString{
    fn div_assign(&mut self, other: Self){
        let temp = self.clone();
        *self = temp / other;
    }
}

impl RemAssign for NumberString{
    fn rem_assign(&mut self, other: Self){
        let result =  self.clone() % other;
        *self = result;
    }
}

impl Neg for NumberString{
    type Output = NumberString;

    fn neg(self) -> Self::Output {
        let mut result = self.clone();
        if self.is_negative(){
            result.to_positive()
        }
        if self.is_positive(){
            result.to_negative()
        }
        if self.is_zero(){
            return self;
        }
        result
    }
}