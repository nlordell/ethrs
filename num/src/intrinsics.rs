//! This module contains intrinsics used by the `u256` implementation.

#[cfg(generate_intrinsics)]
mod llvm;
#[cfg(not(generate_intrinsics))]
mod native;

#[cfg(generate_intrinsics)]
pub use self::llvm::*;
#[cfg(not(generate_intrinsics))]
pub use self::native::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::u256;
    use std::mem::MaybeUninit;

    #[test]
    fn unchecked_addition() {
        let mut res = MaybeUninit::uninit();
        add3(&mut res, &u256([1, 2]), &u256([3, 0]));
        assert_eq!(unsafe { res.assume_init() }, u256([4, 2]));
    }
}
