//! A crate to help you set up explosive mines that go BOOM!
//!
//! All functions and methods in this crate will panic in debug build, and causes undefined
//! behavior in release build.

use std::mem;

enum Void {}

/// This function will panic on debug builds, and marks itself unreachable on release builds
/// (through unsafe compiler intrinsics).
#[inline]
pub unsafe fn boom(msg: &str) -> ! {
    debug_assert!(false, "BOOM! {}", msg);
    let v: &Void = mem::transmute(0usize);
    match *v {}
}

/// `boom` extensions to `Option`.
pub trait OptionExt<T> {
    /// `boom` assertion of `Some` variant.
    unsafe fn boom_some(self) -> T;

    /// `boom` assertion of `None` variant.
    unsafe fn boom_none(self);

    /// `take` with `boom` assertion.
    unsafe fn boom_take(&mut self) -> T;
}

/// `boom` extensions to `Result`.
pub trait ResultExt<T, E> {
    /// `boom` assertion of `Ok` variant.
    unsafe fn boom_ok(self) -> T;

    /// `boom` assertion of `Err` variant.
    unsafe fn boom_err(self) -> E;
}

/// `boom` extensions to `[T]`.
pub trait SliceExt<T> {
    /// `boom` variant of `get`.
    unsafe fn boom_get(&self, index: usize) -> &T;

    /// `boom` variant of `get_mut`.
    unsafe fn boom_get_mut(&mut self, index: usize) -> &mut T;
}

impl<T> OptionExt<T> for Option<T> {
    unsafe fn boom_some(self) -> T {
        match self {
            Some(x) => x,
            None => boom("Expected Some got None"),
        }
    }

    unsafe fn boom_none(self) {
        match self {
            Some(_) => boom("Expected None, got Some"),
            None => (),
        }
    }

    unsafe fn boom_take(&mut self) -> T {
        match self.take() {
            Some(x) => x,
            None => boom("Expected None, got Some"),
        }
    }
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    unsafe fn boom_ok(self) -> T {
        match self {
            Ok(x) => x,
            Err(_) => boom("Expected Ok, got Err"),
        }
    }

    unsafe fn boom_err(self) -> E {
        match self {
            Ok(_) => boom("Expected Err, got Ok"),
            Err(e) => e,
        }
    }
}

impl<T> SliceExt<T> for [T] {
    unsafe fn boom_get(&self, index: usize) -> &T {
        debug_assert!(index < self.len(), "BOOM! Index out of bounds!");
        self.get_unchecked(index)
    }

    unsafe fn boom_get_mut(&mut self, index: usize) -> &mut T {
        debug_assert!(index < self.len(), "BOOM! Index out of bounds!");
        self.get_unchecked_mut(index)
    }
}
