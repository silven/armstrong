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


unsafe fn pll0_feed() {
    use armstrong::m3::*;

    PLL0FEED_REGISTER.write(0xAA);
    PLL0FEED_REGISTER.write(0x55);
}

#[inline(never)]
fn setup() {
    use armstrong::m3::*;
    unsafe {
        // Aktiverar huvudoscillatorn, som håller F_OSC = 12 MHz
        SYS_CTRL_REGISTER.write(0x20);

        loop {
            let osc_status = SYS_CTRL_REGISTER.read() & (1 << 6);
            if osc_status > 0 {
                break;
            }
        }

        CPU_CLKCFG_REGISTER.write(0x03);

        PCLKSEL0_REGISTER.write(0x00);
        PCLKSEL1_REGISTER.write(0x00);

        CLOCK_SRCSEL_REGISTER.write(0x01);

        let M = 100;
        let N = 6;

        //LPC_SC->PLL0CFG = ((3-1)<<16)|((50-1)<<0);
        // M = 50, N = 3
        // PLLclk = (12 MHz * M * 2) / N

        let pll_clk = (12 * M * 2) / N; // 2400
        let final_clock = pll_clk / 4;

        let register_value = ((N - 1) << 16) | ((M - 1) << 0);

        // TODO: calculate
        PLL0CFG_REGISTER.write(register_value);

        pll0_feed();
        PLL0CON_REGISTER.write(0x01);
        pll0_feed();

        loop {
            let pll0_status = PLL0STAT_REGISTER.read() & (1 << 26);
            if pll0_status > 0 {
                break;
            }
        }

        PLL0CON_REGISTER.write(0b11);
        pll0_feed();

        loop {
            let pll0_status = PLL0STAT_REGISTER.read() & (0b11 << 24);
            if pll0_status == (0b11 << 24) {
                break;
            }
        }
    }
}

#[no_mangle]
pub extern fn start() -> ! {
    setup();

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
