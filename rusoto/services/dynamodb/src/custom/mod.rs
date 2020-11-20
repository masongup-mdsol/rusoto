#[cfg(test)]
mod custom_tests;

use crate::generated::AttributeValue;
use bytes::Bytes;
use std::collections::HashMap;
use std::convert::TryInto;
use std::num::{ParseFloatError, ParseIntError};

/// Errors that can be produced when trying to convert an AttributeValue into a Rust primitive type
pub enum AttributeValueConversionError {
    /// The AttributeValue was not of the type it was attempted to be converted into
    WrongType,
    /// For numeric conversions, the AttributeValue Number failed to parse into the specified
    /// integer type
    ParseFailureInt(ParseIntError),
    /// For numeric conversions, the AttributeValue Number failed to parse into the specified
    /// float type
    ParseFailureFloat(ParseFloatError),
}

impl From<ParseIntError> for AttributeValueConversionError {
    fn from(err: ParseIntError) -> Self {
        AttributeValueConversionError::ParseFailureInt(err)
    }
}

impl From<ParseFloatError> for AttributeValueConversionError {
    fn from(err: ParseFloatError) -> Self {
        AttributeValueConversionError::ParseFailureFloat(err)
    }
}

macro_rules! impl_single_conversion {
    ($type:ty, $single_var:ident) => {
        impl From<$type> for AttributeValue {
            fn from(var: $type) -> Self {
                AttributeValue {
                    $single_var: Some(var),
                    ..Default::default()
                }
            }
        }
        impl TryInto<$type> for AttributeValue {
            type Error = AttributeValueConversionError;
            fn try_into(self) -> Result<$type, Self::Error> {
                match self.$single_var {
                    Some(item) => Ok(item),
                    None => Err(AttributeValueConversionError::WrongType),
                }
            }
        }
    };
}

macro_rules! impl_vec_conversion {
    ($type:ty, $vec_var:ident) => {
        impl From<Vec<$type>> for AttributeValue {
            fn from(vars: Vec<$type>) -> Self {
                AttributeValue {
                    $vec_var: Some(vars),
                    ..Default::default()
                }
            }
        }
        impl TryInto<Vec<$type>> for AttributeValue {
            type Error = AttributeValueConversionError;
            fn try_into(self) -> Result<Vec<$type>, Self::Error> {
                match self.$vec_var {
                    Some(items) => Ok(items),
                    None => Err(AttributeValueConversionError::WrongType),
                }
            }
        }
    };
}

macro_rules! impl_type_conversion {
    ($type:ty, $single_var:ident, $vec_var:ident) => {
        impl_single_conversion!($type, $single_var);
        impl_vec_conversion!($type, $vec_var);
    };
}

macro_rules! impl_num_conversions {
    ( $($t:ty),* ) => {
        $(
            impl From<$t> for AttributeValue {
                fn from(num: $t) -> AttributeValue {
                    AttributeValue {
                        n: Some(format!("{}", num)),
                        ..Default::default()
                    }
                }
            }

            impl From<Vec<$t>> for AttributeValue {
                fn from(num_vec: Vec<$t>) -> Self {
                    AttributeValue {
                        ns: Some(num_vec.into_iter().map(|num| format!("{}", num)).collect()),
                        ..Default::default()
                    }
                }
            }

            impl TryInto<$t> for AttributeValue {
                type Error = AttributeValueConversionError;
                fn try_into(self) -> Result<$t, Self::Error> {
                    match self.n {
                        Some(num_str) => num_str.parse().map_err(Self::Error::from),
                        None => Err(AttributeValueConversionError::WrongType),
                    }
                }
            }

            impl TryInto<Vec<$t>> for AttributeValue {
                type Error = AttributeValueConversionError;
                fn try_into(self) -> Result<Vec<$t>, Self::Error> {
                    match self.ns {
                        Some(num_str_vec) => {
                            let mut num_vec = Vec::with_capacity(num_str_vec.len());
                            for num_str in num_str_vec.into_iter() {
                                num_vec.push(num_str.parse()?);
                            }
                            Ok(num_vec)
                        }
                        None => Err(AttributeValueConversionError::WrongType),
                    }
                }
            }
        )*
    };
}

impl_type_conversion!(String, s, ss);
impl_type_conversion!(Bytes, b, bs);
impl_single_conversion!(bool, bool);
impl_vec_conversion!(AttributeValue, l);
impl_single_conversion!(HashMap<String, AttributeValue>, m);
impl_num_conversions!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

impl AttributeValue {
    pub fn null() -> AttributeValue {
        AttributeValue {
            null: Some(true),
            ..Default::default()
        }
    }

    pub fn is_null(&self) -> bool {
        self.null.unwrap_or(false)
    }
}
