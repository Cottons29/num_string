use crate::NumberString;

#[macro_export]
macro_rules! impl_from_for_number_string {
    ($($t:ty),*) => {
        $(
            impl From<$t> for NumberString {
                fn from(value: $t) -> Self {
                    NumberString::new_with_string(value.to_string().as_str()).unwrap()
                }
            }
        
        )*
    };
}

#[macro_export]
macro_rules! impl_into_number_string_for {
    ($($t:ty),*) => {
        $(
            impl IntoNumberString for $t {
                fn into(self) -> NumberString {
                    NumberString::from_number(self)
                }
            }
        )*
    };
}



