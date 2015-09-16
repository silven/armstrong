#![feature(core_slice_ext, core_intrinsics)]
#![feature(no_std)]
#![feature(const_fn)]

#![no_std]
#![no_main]

extern crate armstrong;

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

const SHORT: u32 = 0x20000;
const LONG: u32 = 0x60000;

//.... . .-.. .-.. ---  .-- --- .-. .-.. -..
static MESSAGE: [&'static [u32]; 11] = [
                            &[SHORT, SHORT, SHORT, SHORT],
                            &[SHORT],
                            &[SHORT, LONG, SHORT, SHORT],
                            &[SHORT, LONG, SHORT, SHORT],
                            &[LONG, LONG, LONG],
                            &[],
                            &[SHORT, LONG, LONG],
                            &[LONG, LONG, LONG],
                            &[SHORT, LONG, SHORT],
                            &[SHORT, LONG, SHORT, SHORT],
                            &[LONG, SHORT, SHORT]
                           ];


#[no_mangle]
pub extern fn start() -> ! {
    let mut fiodir = armstrong::BasicRegister::new(0x2009C020);
    let mut fioset = armstrong::BasicRegister::new(0x2009C034);
    fiodir.write(1 << 18);

    loop {
        for &morse_char in MESSAGE.iter() {
            for &delay in morse_char.iter() {
                fioset.write(1 << 18);
                wait(delay);
                fioset.write(0);
                wait(SHORT);
            }
            wait(2 * SHORT);
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> u32 {
    0
}
