//! Module with comparison implementations for `U256`.
//!
//! `PartialEq` and `PartialOrd` implementations for `u128` are also provided
//! to allow notation such as:
//!
//! ```
//! # use ethnum::U256;
//! assert_eq!(U256::new(42), 42);
//! assert!(U256::ONE > 0 && U256::ZERO == 0);
//! ```

use crate::U256;
use std::cmp::Ordering;

impl PartialOrd for U256 {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(match self.high().cmp(rhs.high()) {
            Ordering::Equal => self.low().cmp(rhs.low()),
            ordering => ordering,
        })
    }
}

impl PartialEq<u128> for U256 {
    #[inline]
    fn eq(&self, other: &u128) -> bool {
        *self.high() == 0 && self.low() == other
    }
}

impl PartialOrd<u128> for U256 {
    #[inline]
    fn partial_cmp(&self, rhs: &u128) -> Option<Ordering> {
        Some(if *self.high() == 0 {
            self.low().cmp(rhs)
        } else {
            Ordering::Greater
        })
    }
}
