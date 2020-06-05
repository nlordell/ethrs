//! Module with comparison implementations for `u256`.
//!
//! `PartialEq` and `PartialOrd` implementations for `u128` are also provided
//! to allow notation such as:
//!
//! ```
//! # use ethrs_num::u256;
//! assert_eq!(u256::new(42), 42);
//! assert!(u256::ONE > 0 && u256::ZERO == 0);
//! ```

use crate::u256;
use std::cmp::Ordering;

impl PartialOrd for u256 {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(match self.high().cmp(rhs.high()) {
            Ordering::Equal => self.low().cmp(rhs.low()),
            ordering => ordering,
        })
    }
}

impl PartialEq<u128> for u256 {
    #[inline]
    fn eq(&self, other: &u128) -> bool {
        *self.high() == 0 && self.low() == other
    }
}

impl PartialOrd<u128> for u256 {
    #[inline]
    fn partial_cmp(&self, rhs: &u128) -> Option<Ordering> {
        Some(if *self.high() == 0 {
            self.low().cmp(rhs)
        } else {
            Ordering::Greater
        })
    }
}
