pub trait CalculateStringTrait<Thing> {
    fn len(&self) -> usize;
    fn value(&self) -> String;
    fn to_char(&self) -> Vec<char>;
    fn increment(&mut self);
    fn decrement(&mut self);
    fn equals(&self, other: &Thing) -> bool;
    fn is_negative(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_greater_than(&self, other: &Thing) -> bool;
    fn is_smaller_than(&self, other: &Thing) -> bool;
    fn is_greater_or_eq(&self, other: &Thing) -> bool;
    fn is_smaller_or_eq(&self, other: &Thing) -> bool;
    fn to_unsigned(&self) -> Thing;
    fn to_negative(&mut self);
    fn to_positive(&mut self);
}
