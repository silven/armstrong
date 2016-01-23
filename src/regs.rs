/*!
    Module for most basic register handling

    Contains implementation for both "PC" version, used for testing,
    and the embedded version used for cross compiled apps, writing
    to addresses in memory.
*/

extern crate core;



#[allow(dead_code)]
pub mod raw_implementation  {
    use ::core::ops::{BitAnd, BitOr};

    /**
        Embedded Implementation, points at an address
        in memory which it writes to and reads from.
    */
    pub struct BasicRegister<T> {
        address: *mut T
    }

    #[allow(missing_docs)]
    impl<T: BitAnd<Output=T> + BitOr<Output=T>> BasicRegister<T> {
        #[inline(always)]
        pub const fn new(memory_address: usize) -> Self {
            BasicRegister{ address: memory_address as *mut T}
        }

        #[inline(always)]
        pub fn write(&mut self, new_value: T) {
            unsafe {
                ::core::intrinsics::volatile_store(self.address as *mut T, new_value);
            }
        }

        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe {
                return ::core::intrinsics::volatile_load(self.address as *const T);
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

#[allow(dead_code)]
pub mod pc_implementation  {
    /**
        PC Implementation, defaults to zero value.
        Reads and writes to/from local variable.
    */
    pub struct BasicRegister {
        value: u32
    }

    #[allow(missing_docs)]
    impl BasicRegister {
        #[inline(always)]
        pub fn new(_: u32) -> Self {
            BasicRegister{ value: 0 }
        }

        #[inline(always)]
        pub fn write(&mut self, new_value: u32) {
            self.value = new_value;
        }

        #[inline(always)]
        pub fn read(&self) -> u32 {
            return self.value;
        }
    }
}


#[cfg(feature = "kernel_mode")]
pub use self::raw_implementation::BasicRegister;

#[cfg(not(feature = "kernel_mode"))]
pub use self::pc_implementation::BasicRegister;
