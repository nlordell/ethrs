//! This module implements right and left rotation (**not** shifting) intrinsics
//! for 256-bit integers.

use crate::u256;
use std::mem::MaybeUninit;

#[inline]
pub fn rotate_left(r: &mut MaybeUninit<u256>, a: &u256, b: u32) {
    unsafe {
        r.as_mut_ptr()
            .write((a << (b & 0xff)) | (a >> ((256 - b) & 0xff)))
    };
}

#[inline]
pub fn rotate_right(r: &mut MaybeUninit<u256>, a: &u256, b: u32) {
    unsafe {
        r.as_mut_ptr()
            .write((a >> (b & 0xff)) | (a << ((256 - b) & 0xff)))
    };
}