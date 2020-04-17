//! Module providing formatting utilities for wrapping types.

use std::any;
use std::fmt::{Debug, Formatter, Result};
use std::ops::{Deref, DerefMut};

/// A struct wrapper that always implements `Debug`. This makes deriving `Debug`
/// possible for structs with non-`Debug` fields.
pub struct Dbg<T>(pub T);

impl<T> Debug for Dbg<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str(any::type_name::<T>())
    }
}

impl<T> Deref for Dbg<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Dbg<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Dbg<T> {
    fn from(inner: T) -> Self {
        Dbg(inner)
    }
}
