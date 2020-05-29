mod arith;
mod convert;
mod endian;
pub mod intrinsics;
mod iter;
mod ops;

pub use self::convert::AsU256;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct u256(pub [u128; 2]);

impl u256 {
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = u256([0; 2]);

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = u256([!0; 2]);

    /// The additive identity for this integer type, i.e. `0`.
    pub const ZERO: Self = u256([0; 2]);

    /// The multiplicative identity for this integer type, i.e. `1`.
    pub const ONE: Self = u256::new(1);

    /// Creates a new 256-bit integer value from a primitive `u128` integer.
    #[inline]
    pub const fn new(value: u128) -> Self {
        u256::from_words(0, value)
    }
}

// TODO(nlordell):
//
// # Methods
//
// - checked_next_power_of_two
//
// - count_ones
// - count_zeros
//
// - from_be
// - from_be_bytes
// - from_le
// - from_le_bytes
// - from_ne_bytes
//
// - from_str_radix
//
// - is_power_of_two
//
// - leading_ones
// - leading_zeros
//
// - next_power_of_two
//
// - reverse_bits
//
// - rotate_left
// - rotate_right
//
// - swap_bytes
//
// - to_be
// - to_be_bytes
// - to_le
// - to_le_bytes
// - to_ne_bytes
//
// - trailing_ones
// - trailing_zeros
//
// - wrapping_next_power_of_two
//
// # Trait Implementations
//
// - AddAssign<&'_ u128>
// - AddAssign<u128>
//
// - Binary
//
// - BitAndAssign<&'_ u128>
// - BitAndAssign<u128>
// - BitOrAssign<&'_ u128>
// - BitOrAssign<u128>
// - BitXorAssign<&'_ u128>
// - BitXorAssign<u128>
//
// - Display
//
// - DivAssign<&'_ u128>
// - DivAssign<u128>
//
// - FromStr
//
// - LowerExp
// - LowerHex
//
// - MulAssign<&'_ u128>
// - MulAssign<u128>
//
// - Octal
//
// - Ord
// - PartialOrd<u128>
//
// - RemAssign<&'_ u128>
// - RemAssign<u128>
//
// - ShlAssign<&'_ i128>
// - ShlAssign<&'_ i16>
// - ShlAssign<&'_ i32>
// - ShlAssign<&'_ i64>
// - ShlAssign<&'_ i8>
// - ShlAssign<&'_ isize>
// - ShlAssign<&'_ u128>
// - ShlAssign<&'_ u16>
// - ShlAssign<&'_ u32>
// - ShlAssign<&'_ u64>
// - ShlAssign<&'_ u8>
// - ShlAssign<&'_ usize>
// - ShlAssign<i128>
// - ShlAssign<i16>
// - ShlAssign<i32>
// - ShlAssign<i64>
// - ShlAssign<i8>
// - ShlAssign<isize>
// - ShlAssign<u128>
// - ShlAssign<u16>
// - ShlAssign<u32>
// - ShlAssign<u64>
// - ShlAssign<u8>
// - ShlAssign<usize>
// - ShrAssign<&'_ i128>
// - ShrAssign<&'_ i16>
// - ShrAssign<&'_ i32>
// - ShrAssign<&'_ i64>
// - ShrAssign<&'_ i8>
// - ShrAssign<&'_ isize>
// - ShrAssign<&'_ u128>
// - ShrAssign<&'_ u16>
// - ShrAssign<&'_ u32>
// - ShrAssign<&'_ u64>
// - ShrAssign<&'_ u8>
// - ShrAssign<&'_ usize>
// - ShrAssign<i128>
// - ShrAssign<i16>
// - ShrAssign<i32>
// - ShrAssign<i64>
// - ShrAssign<i8>
// - ShrAssign<isize>
// - ShrAssign<u128>
// - ShrAssign<u16>
// - ShrAssign<u32>
// - ShrAssign<u64>
// - ShrAssign<u8>
// - ShrAssign<usize>
//
// - Step
//
// - SubAssign<&'_ u128>
// - SubAssign<u128>
//
// - UpperExp
// - UpperHex
