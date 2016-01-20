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

#[cfg(feature = "kernel_mode")]
#[allow(dead_code)]
#[allow(missing_docs)]
pub mod m3 {
    pub static mut SYS_CTRL_REGISTER: ::BasicRegister = ::BasicRegister::new(0x400FC1A0);

    pub static mut CPU_CLKCFG_REGISTER: ::BasicRegister = ::BasicRegister::new(0x400FC104);

    pub static mut CLOCK_SRCSEL_REGISTER: ::BasicRegister = ::BasicRegister::new(0x400FC10C);

    pub static mut PCLKSEL0_REGISTER: ::BasicRegister = ::BasicRegister::new(0x400FC1A8);
    pub static mut PCLKSEL1_REGISTER: ::BasicRegister = ::BasicRegister::new(0x400FC1AC);

    pub static mut PLL0FEED_REGISTER: ::BasicRegister = ::BasicRegister::new(0x400FC08C);

    pub static mut PLL0CFG_REGISTER: ::BasicRegister = ::BasicRegister::new(0x400FC084);

    pub static mut PLL0CON_REGISTER: ::BasicRegister = ::BasicRegister::new(0x400FC080);

    pub static mut PLL0STAT_REGISTER: ::BasicRegister = ::BasicRegister::new(0x400FC088);


}

