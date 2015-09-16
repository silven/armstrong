/*!
    Module for most basic register handling

    Contains implementation for both "PC" version, used for testing,
    and the embedded version used for cross compiled apps, writing
    to addresses in memory.
*/

extern crate core;


#[allow(dead_code)]
pub mod raw_implementation  {
    /**
        Embedded Implementation, points at an address
        in memory which it writes to and reads from.
    */
    pub struct BasicRegister {
        address: *mut u32
    }

    #[allow(missing_docs)]
    impl BasicRegister {
        #[inline(always)]
        pub fn new(memory_address: u32) -> Self {
            BasicRegister{ address: memory_address as *mut u32}
        }

        #[inline(always)]
        pub fn write(&mut self, new_value: u32) {
            unsafe {
                ::core::intrinsics::volatile_store(self.address, new_value);
            }
        }

        #[inline(always)]
        pub fn read(&self) -> u32 {
            unsafe {
                return ::core::intrinsics::volatile_load(self.address);
            }
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
