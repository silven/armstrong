#![feature(core, core_slice_ext, core_intrinsics)]
#![feature(no_std)]
#![feature(const_fn)]

#![no_std]
#![no_main]

extern crate armstrong;


#[inline]
fn volatile_decrement(addr: &mut u32) -> u32 {
    unsafe {
        let x = core::intrinsics::volatile_load(addr as *const u32);
        core::intrinsics::volatile_store(addr as *mut u32, x - 1);
        return x - 1;
    }
}


fn wait(duration: u32) {
    let mut i: u32 = duration;
    loop {
        let x = volatile_decrement(&mut i);
        if x == 0 {
           break;
        }
    }
}

const s: u32 = 0x10000;
const m: u32 = 0x20000;
const l: u32 = 0x30000;

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
        core::intrinsics::volatile_store(0x2009C020 as *mut u32, 1 << 18);
        loop {
            for &morse_char in MESSAGE.iter() {
                for &delay in morse_char.iter() {
                    core::intrinsics::volatile_store(0x2009C034 as *mut u32, 1 << 18);
                    wait(delay);
                    core::intrinsics::volatile_store(0x2009C034 as *mut u32, 0 << 18);
                    wait(s);
                }
                wait(2 * s);
            }
        }
    }

}
