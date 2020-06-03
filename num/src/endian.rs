use crate::u256;
use std::mem;

#[cfg(target_endian = "little")]
const LO: usize = 0;
#[cfg(target_endian = "big")]
const LO: usize = 1;
#[cfg(target_endian = "little")]
const HI: usize = 1;
#[cfg(target_endian = "big")]
const HI: usize = 0;

impl u256 {
    /// Creates a new 256-bit integer value from high and low words.
    #[inline]
    pub const fn from_words(hi: u128, lo: u128) -> Self {
        #[cfg(target_endian = "little")]
        {
            u256([lo, hi])
        }
        #[cfg(target_endian = "big")]
        {
            u256([hi, lo])
        }
    }

    /// Get the low 128-bit word for this unsigned integer.
    #[inline]
    pub fn low(&self) -> &u128 {
        &self.0[LO]
    }

    /// Get the low 128-bit word for this unsigned integer as a mutable
    /// reference.
    #[inline]
    pub fn low_mut(&mut self) -> &mut u128 {
        &mut self.0[LO]
    }

    /// Get the high 128-bit word for this unsigned integer.
    #[inline]
    pub fn high(&self) -> &u128 {
        &self.0[HI]
    }

    /// Get the high 128-bit word for this unsigned integer as a mutable
    /// reference.
    #[inline]
    pub fn high_mut(&mut self) -> &mut u128 {
        &mut self.0[HI]
    }

    /// Reverses the byte order of the integer.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let n = u256::from_words(
    ///     0x00010203_04050607_08090a0b_0c0d0e0f,
    ///     0x10111213_14151617_18191a1b_1c1d1e1f,
    /// );
    /// assert_eq!(
    ///     n.swap_bytes(),
    ///     u256::from_words(
    ///         0x1f1e1d1c_1b1a1918_17161514_13121110,
    ///         0x0f0e0d0c_0b0a0908_07060504_03020100,
    ///     ),
    /// );
    /// ```
    #[inline]
    pub const fn swap_bytes(self) -> Self {
        let u256([a, b]) = self;
        u256([b.swap_bytes(), a.swap_bytes()])
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let n = u256::new(0x1A);
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(u256::from_be(n), n);
    /// } else {
    ///     assert_eq!(u256::from_be(n), n.swap_bytes());
    /// }
    /// ```
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(not(target_endian = "big"))]
        {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let n = u256::new(0x1A);
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(u256::from_le(n), n)
    /// } else {
    ///     assert_eq!(u256::from_le(n), n.swap_bytes())
    /// }
    /// ```
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(not(target_endian = "little"))]
        {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let n = u256::new(0x1A);
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(n.to_be(), n)
    /// } else {
    ///     assert_eq!(n.to_be(), n.swap_bytes())
    /// }
    /// ```
    #[inline]
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(not(target_endian = "big"))]
        {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let n = u256::new(0x1A);
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(n.to_le(), n)
    /// } else {
    ///     assert_eq!(n.to_le(), n.swap_bytes())
    /// }
    /// ```
    #[inline]
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
        {
            self.swap_bytes()
        }
    }

    /// Return the memory representation of this integer as a byte array in
    /// big endian (network) byte order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let bytes = u256::from_words(
    ///     0x00010203_04050607_08090a0b_0c0d0e0f,
    ///     0x10111213_14151617_18191a1b_1c1d1e1f,
    /// );
    /// assert_eq!(
    ///     bytes.to_be_bytes(),
    ///     [
    ///         0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    ///         0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    ///     ],
    /// );
    /// ```
    #[inline]
    pub fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
        self.to_be().to_ne_bytes()
    }

    /// Return the memory representation of this integer as a byte array in
    /// little-endian byte order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let bytes = u256::from_words(
    ///     0x00010203_04050607_08090a0b_0c0d0e0f,
    ///     0x10111213_14151617_18191a1b_1c1d1e1f,
    /// );
    /// assert_eq!(
    ///     bytes.to_le_bytes(),
    ///     [
    ///         0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10,
    ///         0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
    ///     ],
    /// );
    /// ```
    #[inline]
    pub fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
        self.to_le().to_ne_bytes()
    }

    /// Return the memory representation of this integer as a byte array in
    /// native byte order.
    ///
    /// As the target platform's native endianness is used, portable code should
    /// use [`to_be_bytes`] or [`to_le_bytes`], as appropriate, instead.
    ///
    /// [`to_be_bytes`]: #method.to_be_bytes
    /// [`to_le_bytes`]: #method.to_le_bytes
    ///
    /// # Examples
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let bytes = u256::from_words(
    ///     0x00010203_04050607_08090a0b_0c0d0e0f,
    ///     0x10111213_14151617_18191a1b_1c1d1e1f,
    /// );
    /// assert_eq!(
    ///     bytes.to_ne_bytes(),
    ///     if cfg!(target_endian = "big") {
    ///         [
    ///             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    ///             0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    ///         ]
    ///     } else {
    ///         [
    ///             0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10,
    ///             0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
    ///         ]
    ///     }
    /// );
    /// ```
    #[inline]
    pub fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
        unsafe { mem::transmute(self) }
    }

    /// Create an integer value from its representation as a byte array in big
    /// endian.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let value = u256::from_be_bytes([
    ///     0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    ///     0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    /// ]);
    /// assert_eq!(
    ///     value,
    ///     u256::from_words(
    ///         0x00010203_04050607_08090a0b_0c0d0e0f,
    ///         0x10111213_14151617_18191a1b_1c1d1e1f,
    ///     ),
    /// );
    /// ```
    ///
    /// When starting from a slice rather than an array, fallible conversion
    /// APIs can be used:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// use std::convert::TryInto;
    ///
    /// fn read_be_u256(input: &mut &[u8]) -> u256 {
    ///     let (int_bytes, rest) = input.split_at(std::mem::size_of::<u256>());
    ///     *input = rest;
    ///     u256::from_be_bytes(int_bytes.try_into().unwrap())
    /// }
    /// ```
    #[inline]
    pub fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
        Self::from_be(Self::from_ne_bytes(bytes))
    }

    /// Create an integer value from its representation as a byte array in
    /// little endian.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let value = u256::from_le_bytes([
    ///     0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10,
    ///     0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
    /// ]);
    /// assert_eq!(
    ///     value,
    ///     u256::from_words(
    ///         0x00010203_04050607_08090a0b_0c0d0e0f,
    ///         0x10111213_14151617_18191a1b_1c1d1e1f,
    ///     ),
    /// );
    /// ```
    ///
    /// When starting from a slice rather than an array, fallible conversion
    /// APIs can be used:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// use std::convert::TryInto;
    ///
    /// fn read_be_u256(input: &mut &[u8]) -> u256 {
    ///     let (int_bytes, rest) = input.split_at(std::mem::size_of::<u256>());
    ///     *input = rest;
    ///     u256::from_le_bytes(int_bytes.try_into().unwrap())
    /// }
    /// ```
    #[inline]
    pub fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
        Self::from_le(Self::from_ne_bytes(bytes))
    }

    /// Create an integer value from its memory representation as a byte array
    /// in native endianness.
    ///
    /// As the target platform's native endianness is used, portable code likely
    /// wants to use [`from_be_bytes`] or [`from_le_bytes`], as appropriate
    /// instead.
    ///
    /// [`from_be_bytes`]: #method.from_be_bytes
    /// [`from_le_bytes`]: #method.from_le_bytes
    ///
    /// # Examples
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let value = u256::from_ne_bytes(if cfg!(target_endian = "big") {
    ///     [
    ///         0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    ///         0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    ///     ]
    /// } else {
    ///     [
    ///         0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10,
    ///         0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
    ///     ]
    /// });
    /// assert_eq!(
    ///     value,
    ///     u256::from_words(
    ///         0x00010203_04050607_08090a0b_0c0d0e0f,
    ///         0x10111213_14151617_18191a1b_1c1d1e1f,
    ///     ),
    /// );
    /// ```
    ///
    /// When starting from a slice rather than an array, fallible conversion
    /// APIs can be used:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// use std::convert::TryInto;
    ///
    /// fn read_be_u256(input: &mut &[u8]) -> u256 {
    ///     let (int_bytes, rest) = input.split_at(std::mem::size_of::<u256>());
    ///     *input = rest;
    ///     u256::from_ne_bytes(int_bytes.try_into().unwrap())
    /// }
    /// ```
    #[inline]
    pub fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
        unsafe { mem::transmute(bytes) }
    }

    /// Reverses the bit pattern of the integer.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// let n = u256::from_words(
    ///     0x00010203_04050607_08090a0b_0c0d0e0f,
    ///     0x10111213_14151617_18191a1b_1c1d1e1f,
    /// );
    /// assert_eq!(
    ///     n.reverse_bits(),
    ///     u256::from_words(
    ///         0xf878b838_d8589818_e868a828_c8488808,
    ///         0xf070b030_d0509010_e060a020_c0408000,
    ///     ),
    /// );
    /// ```
    #[inline]
    pub const fn reverse_bits(self) -> Self {
        let u256([a, b]) = self;
        u256([b.reverse_bits(), a.reverse_bits()])
    }
}
