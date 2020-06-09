mod arith;
mod cmp;
mod convert;
mod endian;
pub mod intrinsics;
mod iter;
mod ops;

pub use self::convert::AsU256;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq)]
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
// - from_str_radix
//
// - count_ones (intrinsic)
// - count_zeros (intrinsic)
// - leading_ones (intrinsic)
// - leading_zeros (intrinsic)
// - trailing_ones (intrinsic)
// - trailing_zeros (intrinsic)
//
// - rotate_left (intrinsic)
// - rotate_right (intrinsic)
//
// - checked_next_power_of_two
// - is_power_of_two (intrinsic)
// - next_power_of_two
// - wrapping_next_power_of_two (unstable)
//
// # Trait Implementations
//
// - Display
// - FromStr
//
// - Binary
// - LowerExp
// - LowerHex
// - Octal
// - UpperExp
// - UpperHex
//
// - BitAndAssign<&'_ u128>
// - BitAndAssign<u128>
// - BitOrAssign<&'_ u128>
// - BitOrAssign<u128>
// - BitXorAssign<&'_ u128>
// - BitXorAssign<u128>
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
