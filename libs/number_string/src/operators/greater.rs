use std::cmp::Ordering;
use calculate_struct_trait::CalculateStringTrait;
use crate::{NumberString};

impl PartialOrd for NumberString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.equals(other) {
            return Some(Ordering::Equal)
        }
        return if self.is_greater_than(other) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }

    fn lt(&self, other: &Self) -> bool {
        // println!("less than self");
        self.is_smaller_than(other)
    }

    fn le(&self, other: &Self) -> bool {
        // println!("greater than self");
        self.is_smaller_or_eq(other)
    }

    fn gt(&self, other: &Self) -> bool {
        // println!("greater than self");
        self.is_greater_than(other)
    }

    fn ge(&self, other: &Self) -> bool {
        // println!("greater eq than self");
        self.is_greater_or_eq(other)
    }
}