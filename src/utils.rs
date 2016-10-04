//! Module for misc utils

pub struct Volatile<T> {
    value: T,
}

impl<T> Volatile<T> {
    pub fn new(initial: T) -> Volatile<T> {
        Volatile { value: initial }
    }

    #[inline]
    pub fn get(&self) -> T {
        unsafe { return ::core::ptr::read_volatile(&self.value); }
    }

    #[inline]
    pub fn set(&mut self, new: T) {
        unsafe { ::core::ptr::write_volatile(&mut self.value, new); }
    }
}