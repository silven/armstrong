//! Module for most basic register handling
//!
//! Contains implementation for both "PC" version, used for testing,
//! and the embedded version used for cross compiled apps, writing
//! to addresses in memory.
//!

#[cfg(target_os="none")]
pub mod raw_implementation {
    use ::core::ops::{BitAnd, BitOr};

    /**
        Embedded Implementation, points at an address
        in memory which it writes to and reads from.
    */
    pub struct BasicRegister<T> {
        address: *mut T,
    }

    #[allow(missing_docs)]
    impl<T: BitAnd<Output = T> + BitOr<Output = T>> BasicRegister<T> {
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
            let current = self.read();
            self.write(current & value);
        }

        pub fn or(&mut self, value: T) {
            let current = self.read();
            self.write(current | value);
        }
    }
}

#[cfg(not(target_os="none"))]
pub mod pc_implementation {
    use ::std::ops::{BitAnd, BitOr};

    /**
        PC Implementation, defaults to zero value.
        Reads and writes to/from local variable.
    */
    pub struct BasicRegister<T> {
        addr: u32,
        values: Option<::std::vec::Vec<T>>,
    }

    #[allow(missing_docs)]
    impl<T: Copy + BitAnd<Output = T> + BitOr<Output = T>> BasicRegister<T> {
        #[inline(always)]
        pub const fn new(addr: u32) -> Self {
            BasicRegister {
                addr: addr,
                values: None,
            }
        }

        #[inline(always)]
        pub fn write(&mut self, new_value: T) {
            match self.values {
                None => self.values = Some(vec![new_value]),
                Some(ref mut vs) => vs.push(new_value),
            }
        }

        #[inline(always)]
        pub fn read(&self) -> T {
            match self.values {
                Some(ref vs) => return vs.last().unwrap().clone(),
                None => panic!("I havn't solved how to handle read-only registers yet!"),
            };
        }

        pub fn and(&mut self, value: T) {
            let current = self.read();
            self.write(current & value);
        }

        pub fn or(&mut self, value: T) {
            let current = self.read();
            self.write(current | value);
        }
    }
}


#[cfg(target_os="none")]
pub use self::raw_implementation::BasicRegister;

#[cfg(not(target_os="none"))]
pub use self::pc_implementation::BasicRegister;
