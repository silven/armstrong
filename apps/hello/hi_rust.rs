#![feature(core, core_slice_ext, core_intrinsics)]
#![feature(no_std)]
#![feature(const_fn)]


#![no_std]
#![no_main]

extern crate armstrong;
use armstrong::*;

fn wait(duration: u32) {
    let mut i: u32 = duration;

    let volatile_decrement = | addr: &mut u32 | -> u32 {
        unsafe {
            let value = core::intrinsics::volatile_load(addr as *const u32);
            core::intrinsics::volatile_store(addr as *mut u32, value - 1);
            return value - 1;
        }
    };

    loop {
        let x = volatile_decrement(&mut i);
        if x == 0 {
           break;
        }
    }
}

const s: u32 = 0x20000;
const m: u32 = 0x40000;
const l: u32 = 0x60000;

//.... . .-.. .-.. ---  .-- --- .-. .-.. -..
static MESSAGE: [&'static [u32]; 11] = [
                            &[s, s, s, s],
                            &[s],
                            &[s, l, s, s],
                            &[s, l, s, s],
                            &[l, l, l],
                            &[],
                            &[s, l, l],
                            &[l, l, l],
                            &[s, l, s],
                            &[s, l, s, s],
                            &[l, s, s]
                            ];


#[no_mangle]
pub extern "C" fn main() -> () {
    unsafe {
        FIO1DIR.write(&FIOnDIR::value().p18(IODir::output));

        let led_on = FIOnSET::value().p18(Bool::on);
        let led_off = FIOnSET::value().p18(Bool::off);

        loop {
            for &morse_char in MESSAGE.iter() {
                for &delay in morse_char.iter() {
                    FIO1SET.write(&led_on);
                    wait(delay);
                    FIO1SET.write(&led_off);
                    wait(s);
                }
                wait(2 * s);
            }
        }


    }
}
