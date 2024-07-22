pub use core::ptr::null_mut;
pub use std::{mem::transmute, sync::Once};
pub use winapi::{
    ctypes::{c_char, c_int, c_void},
    shared::minwindef::{DWORD, HINSTANCE__, HMODULE},
};

/// A macro to cast a raw pointer to a specific type.
///
/// This macro provides a convenient way to cast a raw pointer to either a mutable or immutable type.
/// It supports two forms:
///
/// 1. `(mut $address:expr, $type:ident)`: This form casts the mutable raw pointer `$address` to a mutable pointer of type `$type`.
/// 2. `($address:expr, $type:ident)`: This form casts the immutable raw pointer `$address` to a const pointer of type `$type`.
///
/// # Examples
///
/// ```rust
/// let mut int_ptr = 0x12345678 as *mut i32;
/// let float_ptr = cast!(int_ptr, f32);
///
/// let const_int_ptr = 0x87654321 as *const i32;
/// let const_float_ptr = cast!(const_int_ptr, f32);
/// ```
#[macro_export]
macro_rules! cast {
    // Value cast
    (mut $address:expr, $type:ident) => {
        $address as *mut $type
    };
    ($address:expr, $type:ident) => {
        $address as *const $type
    };
}
