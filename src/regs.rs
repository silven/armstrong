//! Module for most basic register handling
//!
//! Contains implementation for both "PC" version, used for testing,
//! and the embedded version used for cross compiled apps, writing
//! to addresses in memory.
//!

use ::core::ops::{BitAnd, BitOr, BitXor};

/**
    Embedded Implementation, points at an address
    in memory which it writes to and reads from.
*/
pub struct BasicRegister<T> {
    address: *mut T,
}

#[allow(missing_docs)]
impl<T> BasicRegister<T> {

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

    pub fn update<F: FnOnce(T)->T>(&mut self, operation: F) {
        let current_value = self.read();
        let new_value = operation(current_value);
        self.write(new_value);
    }
}

use ::core::ops::BitAndAssign;
impl<T: BitAnd<Output=T>> BitAndAssign<T> for BasicRegister<T> {
    fn bitand_assign(&mut self, rhs: T) {
        self.update(|x| x & rhs);
    }
}

use ::core::ops::BitOrAssign;
impl<T: BitOr<Output=T>> BitOrAssign<T> for BasicRegister<T> {
    fn bitor_assign(&mut self, rhs: T) {
        self.update(|x| x | rhs);
    }
}

use ::core::ops::BitXorAssign;
impl<T: BitXor<Output=T>> BitXorAssign<T> for BasicRegister<T> {
    fn bitxor_assign(&mut self, rhs: T) {
        self.update(|x| x ^ rhs);
    }
}
/*
use ::core::ops::Deref;
impl<T> Deref for BasicRegister<T> {
    type Target = T;

    fn deref(&self) -> &T {
        return unsafe { &*self.address as &T }
    }
}

use ::core::ops::DerefMut;
impl<T> DerefMut for BasicRegister<T> {
    fn deref_mut(&mut self) -> &mut T {
        return unsafe { &mut *self.address as &mut T }
    }
}
*/

/*
    pub fn or(&mut self, value: T) {
        self.update(|x| x | value);
    }

    pub fn xor(&mut self, value: T) {
        self.update(|x| x ^value);
    }


}
*/