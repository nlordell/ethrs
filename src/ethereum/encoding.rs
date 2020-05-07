//! This module provides Ethereum encoding for Rust primitives used for JSON RPC
//! calls.

use hex::{FromHex, ToHex};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt::{self, LowerHex};
use std::marker::PhantomData;
use std::num::ParseIntError;

/// A trait for decoding a JSON RPC result into an API result. This is used for
/// using proxy types for doing deserialization (such as
/// [`ethrs::encoding::Data`]).
///
/// Note this trait is similar to [`std::convert::From`] while allowing
/// implementations on foreign types.
pub trait Decode<T> {
    fn decode(encoded: T) -> Self;
}

impl<T> Decode<T> for T {
    fn decode(encoded: T) -> Self {
        encoded
    }
}

/// A type wrapper around byte data that gets serialized as a hex string.
pub struct Data<T>(pub T);

impl<T> From<T> for Data<T> {
    fn from(inner: T) -> Self {
        Data(inner)
    }
}

impl<T> Decode<Data<T>> for T {
    fn decode(encoded: Data<T>) -> Self {
        encoded.0
    }
}

impl<T: ToHex> Serialize for Data<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let buffer = format!("0x{}", self.0.encode_hex::<String>());
        serializer.serialize_str(&buffer)
    }
}

impl<'de, T> Deserialize<'de> for Data<T>
where
    T: FromHex,
    T::Error: fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for DataVisitor<T>
        where
            T: FromHex,
            T::Error: fmt::Display,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a '0x' prefixed hex string with two digits per byte")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if !s.starts_with("0x") {
                    return Err(de::Error::custom("missing '0x' prefix"));
                }

                T::from_hex(&s[2..]).map_err(de::Error::custom)
            }
        }

        let inner = deserializer.deserialize_str(DataVisitor(PhantomData))?;
        Ok(Data(inner))
    }
}

/// Module for `#[serde(with = ...)]` to perform serialization with the
/// [`ethrs::encoding::Data`] type wrapper.
pub mod data {}

/// A type wrapper around primitive integer quantities that get serialized as
/// hex strings.
pub struct Quantity<T>(pub T);

impl<T> From<T> for Quantity<T> {
    fn from(inner: T) -> Self {
        Quantity(inner)
    }
}

impl<T> Decode<Quantity<T>> for T {
    fn decode(encoded: Quantity<T>) -> Self {
        encoded.0
    }
}

impl<T: LowerHex> Serialize for Quantity<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let buffer = format!("{:#x}", self.0);
        serializer.serialize_str(&buffer)
    }
}

impl<'de, T> Deserialize<'de> for Quantity<T>
where
    T: FromStrRadix,
    T::Error: fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QuantityVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for QuantityVisitor<T>
        where
            T: FromStrRadix,
            T::Error: fmt::Display,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a '0x' prefixed compact hex number")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let s = if s.starts_with("0x") {
                    &s[2..]
                } else {
                    return Err(de::Error::custom("missing '0x' prefix"));
                };
                if s != "0" && s.starts_with('0') {
                    return Err(de::Error::custom("hex number contains leading 0s"));
                }

                T::from_str_radix(s, 16).map_err(de::Error::custom)
            }
        }

        let value = deserializer.deserialize_str(QuantityVisitor(PhantomData))?;
        Ok(Quantity(value))
    }
}

/// Module for `#[serde(with = ...)]` to perform serialization with the
/// [`ethrs::encoding::Quantity`] type wrapper.
pub mod quantity {
    use super::*;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: LowerHex,
        S: Serializer,
    {
        Quantity(value).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStrRadix,
        T::Error: fmt::Display,
        D: Deserializer<'de>,
    {
        Ok(Quantity::<T>::deserialize(deserializer)?.0)
    }
}

/// A trait for reading integers from a string in the specified base.
pub trait FromStrRadix: Sized {
    type Error;

    /// Converts a string slice in a given base to an integer.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by
    /// digits. Leading and trailing whitespace represent an error. Digits are a
    /// subset of these characters, depending on `radix`:
    ///
    ///  * `0-9`
    ///  * `a-z`
    ///  * `A-Z`
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 36.
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::Error>;
}

macro_rules! impl_from_str_radix {
    ($($int:ty),* $(,)?) => {$(
        impl FromStrRadix for $int {
            type Error = ParseIntError;

            fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::Error> {
                <$int>::from_str_radix(s, radix)
            }
        }
    )*};
}

impl_from_str_radix! {
    isize, i8, i16, i32, i64, i128,
    usize, u8, u16, u32, u64, u128,
}
