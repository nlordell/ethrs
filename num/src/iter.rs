//! Module contains iterator specific trait implementations.

use crate::u256;
use std::iter::{Product, Sum};
use std::ops::{Add, Mul};

impl Sum for u256 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(u256::ZERO, Add::add)
    }
}

impl Product for u256 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(u256::ONE, Mul::mul)
    }
}

impl<'a> Sum<&'a u256> for u256 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(u256::ZERO, Add::add)
    }
}

impl<'a> Product<&'a u256> for u256 {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(u256::ONE, Mul::mul)
    }
}

// TODO(nlordell): Implement [`std::iter::Step`] once it stabilizes.
