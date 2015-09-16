/*!

	Armstrong Kernel WIP

*/

#![feature(core, core_intrinsics, no_std, lang_items, const_fn)]

#![warn(missing_docs)]
#![deny(unused_extern_crates)]
#![warn(unused_qualifications)]

#![deny(box_pointers)]
#![deny(unused_results)]

#![deny(overflowing_literals)]

#![no_std]

#[allow(unused_extern_crates)]
#[cfg(not(feature = "kernel_mode"))]
extern crate std;


#[cfg(feature = "kernel_mode")]
mod lang_items;

#[cfg(feature = "kernel_mode")]
pub mod isr;

mod regs;

pub use regs::BasicRegister;

#[cfg(test)]
mod test {
    #[test]
    fn registers_work() {
        let mut reg = ::BasicRegister::new(0x00);

        let current_value = reg.read();
        assert_eq!(0x00, current_value);

        let new_value = 0xFF;
        reg.write(new_value);

        let current_value = reg.read();
        assert_eq!(new_value, current_value)
    }
}


