mod intrinsics;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct u256(pub [u64; 4]);

pub struct ParseIntError;

impl u256 {
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = u256([0; 4]);

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = u256([!0; 4]);

    /// Converts a string slice in a given base to an integer.
    ///
    /// The string is expected to be an optional `+` sign followed by digits.
    /// Leading and trailing whitespace represent an error. Digits are a subset
    /// of these characters, depending on `radix`:
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(_src: &str, _radix: u32) -> Result<Self, ParseIntError> {
        todo!();
    }
}
