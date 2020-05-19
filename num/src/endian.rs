use crate::u256;

#[cfg(target_endian = "little")]
const LO: usize = 0;
#[cfg(target_endian = "big")]
const LO: usize = 1;
#[cfg(target_endian = "little")]
const HI: usize = 1;
#[cfg(target_endian = "big")]
const HI: usize = 0;

impl u256 {
    /// Creates a new 256-bit integer value from a primitive `u128` integer.
    #[inline(always)]
    pub const fn new(value: u128) -> Self {
        #[cfg(target_endian = "little")]
        {
            u256([value, 0])
        }
        #[cfg(target_endian = "big")]
        {
            u256([0, value])
        }
    }

    /// Get the low 128-bit word for this unsigned integer.
    #[inline(always)]
    pub fn low(&self) -> &u128 {
        &self.0[LO]
    }

    /// Get the low 128-bit word for this unsigned integer as a mutable
    /// reference.
    #[inline(always)]
    pub fn low_mut(&mut self) -> &mut u128 {
        &mut self.0[LO]
    }

    /// Get the high 128-bit word for this unsigned integer.
    #[inline(always)]
    pub fn high(&self) -> &u128 {
        &self.0[HI]
    }

    /// Get the high 128-bit word for this unsigned integer as a mutable
    /// reference.
    #[inline(always)]
    pub fn high_mut(&mut self) -> &mut u128 {
        &mut self.0[HI]
    }
}
