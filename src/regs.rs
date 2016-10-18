//! Module for most basic register handling
//!
//! Contains implementation for both "PC" version, used for testing,
//! and the embedded version used for cross compiled apps, writing
//! to addresses in memory.
//!

use ::core::ops::{BitAnd, BitOr};

/**
    Embedded Implementation, points at an address
    in memory which it writes to and reads from.
*/
pub struct BasicRegister<T> {
    address: *mut T,
}

#[allow(missing_docs)]
impl<T> BasicRegister<T>
    where T: BitAnd<Output = T> + BitOr<Output = T>
    {

    #[inline(always)]
    pub const fn new(memory_address: usize) -> Self {
        BasicRegister { address: memory_address as *mut T }
    }

    #[inline(always)]
    pub fn write(&mut self, new_value: T) {
        unsafe {
            ::core::ptr::write_volatile(self.address as *mut T, new_value);
        }
    }

    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe {
            return ::core::ptr::read_volatile(self.address as *const T);
        }
    }

    pub fn and(&mut self, value: T) {
        self.update(|x| x & value);
    }

    pub fn or(&mut self, value: T) {
        self.update(|x| x | value);
    }

    pub fn update<F: FnOnce(T)->T>(&mut self, operation: F) {
        let current_value = self.read();
        let new_value = operation(current_value);
        self.write(new_value);
    }
}