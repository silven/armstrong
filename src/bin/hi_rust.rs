#![feature(const_fn, naked_functions)]

#![no_std]
#![no_main]

#[macro_use]
extern crate armstrong;
use armstrong::Volatile;

fn wait(duration: u32) {
    let mut i = Volatile::<u32>::new(duration);
    loop {
        let x = i.get() - 1;
        if x == 0 {
            break;
        }
        i.set(x);
    }
}

const SHORT: u32 = 0x200000;
const LONG: u32 = 0x600000;

// .... . .-.. .-.. ---  .-- --- .-. .-.. -..
static MESSAGE: [&'static [u32]; 11] = [&[SHORT, SHORT, SHORT, SHORT],
                                        &[SHORT],
                                        &[SHORT, LONG, SHORT, SHORT],
                                        &[SHORT, LONG, SHORT, SHORT],
                                        &[LONG, LONG, LONG],
                                        &[],
                                        &[SHORT, LONG, LONG],
                                        &[LONG, LONG, LONG],
                                        &[SHORT, LONG, SHORT],
                                        &[SHORT, LONG, SHORT, SHORT],
                                        &[LONG, SHORT, SHORT]];

#[inline(never)]
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

        wait_for!(SYS_CTRL_REGISTER.read() & (1 << 6) > 0);

        CPU_CLKCFG_REGISTER.write(0x03);

        PCLKSEL0_REGISTER.write(0x00);
        PCLKSEL1_REGISTER.write(0x00);

        CLOCK_SRCSEL_REGISTER.write(0x01);

        let m = 100;
        let n = 6;

        // M = 100, N = 6
        // PLLclk = (12 MHz * M * 2) / N
        // pll_clk = (12 * M * 2) / N; // 24000 / 6 == 400

        let register_value = ((n - 1) << 16) | ((m - 1) << 0);
        PLL0CFG_REGISTER.write(register_value);

        pll0_feed();
        PLL0CON_REGISTER.write(0x01);
        pll0_feed();

        wait_for!(PLL0STAT_REGISTER.read() & (1 << 26) > 0);

        PLL0CON_REGISTER.write(0b11);
        pll0_feed();

        wait_for!(PLL0STAT_REGISTER.read() & (0b11 << 24) == (0b11 << 24));
    }
}

#[inline(never)]
fn uart0_init() {
    use armstrong::m3::*;

    unsafe {
        PINSEL0_REGISTER &= (!0xF0);
        PINSEL0_REGISTER |= (0b01 << 4 | 0b01 << 6);

        U0LCR_REGISTER.write(0x83);

        U0FDR_REGISTER.write(0x21);

        U0DLM_REGISTER.write(0); //(div / 256) as u8);
        U0DLL_REGISTER.write(0x6C); //(div % 256) as u8);

        U0LCR_REGISTER.write(0x03);
        U0FCR_REGISTER.write(0x07);

        // Set PINSEL0 so that P0.2 = TXD0, P0.3 = RXD0
        //LPC_PINCON->PINSEL0 &= ~0xf0;
        //LPC_PINCON->PINSEL0 |= ((1 << 4) | (1 << 6));

        //LPC_UART0->LCR = 0x83;      // 8 bits, no Parity, 1 Stop bit, DLAB=1
        //Fdiv = ( pclk / 16 ) / baudrate ;   // Set baud rate
        //LPC_UART0->DLM = Fdiv / 256;
        //LPC_UART0->DLL = Fdiv % 256;
        //LPC_UART0->LCR = 0x03;      // 8 bits, no Parity, 1 Stop bit DLAB = 0
        //LPC_UART0->FCR = 0x07;      // Enable and reset TX and RX FIFO
    }
}

#[inline(never)]
fn uart0_write(msg: &[u8]) {
    for &b in msg {
        uart0_putc(b);
    }
}

#[inline(never)]
fn uart0_putc(byte: u8) {
    use armstrong::m3::*;
    unsafe {
        wait_for!(U0LSR_REGISTER.read() & 0x20 != 0);
        UART0.write(byte);
    }
}

#[inline(never)]
fn uart0_read(buffer: &mut [u8]) -> usize {
    for i in 0..buffer.len() {
        let c = uart0_getc();
        uart0_putc(c);
        buffer[i] = c;
        if c as char == '\r' {
            return i;
        }
    }
    return buffer.len();
}

#[inline(never)]
fn uart0_getc() -> u8 {
    use armstrong::m3::*;
    unsafe {
        wait_for!(U0LSR_REGISTER.read() & 0x01 != 0);
        return UART0.read();
    }
}

struct UART {
    addr: *mut u8,
}

impl ::core::fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> Result<(), ::core::fmt::Error> {
        for b in s.bytes() {
            uart0_putc(b);
        }
        return Ok(());
    }
}

#[naked]
#[no_mangle]
pub extern "C" fn start() -> ! {
    setup();
    uart0_init();

    let mut fiodir = armstrong::BasicRegister::new(0x2009C020);
    let mut fioset = armstrong::BasicRegister::new(0x2009C034);
    fiodir.write(1 << 18);

    use core::fmt::Write;
    let mut u0 = UART{addr: 0x00 as *mut u8};
    //loop {
        write!(&mut u0, "Hello {}\n\r\r", 1337);
    //}

    uart0_write(b"Hello world!\n\r");
    let mut input_buffer = [0; 32];
    loop {
        uart0_write(b"Write something?\n\r");
        let bytes = uart0_read(&mut input_buffer);
        //uart0_write(b"You wrote: ");

        if let Ok(as_utf8) = ::core::str::from_utf8(&input_buffer[..bytes]) {
            write!(&mut u0, "You wrote {}\n\r", as_utf8);

            match as_utf8 {
                "on" => fioset |= (1 << 18),
                "off" => fioset ^= (1 << 18),
                _ => {},
            }

        } else {
            write!(&mut u0, "..I didn't understand that");
        }
        //uart0_write(&input_buffer[..bytes]);
        uart0_write(b"\n\r");
    }

    loop {
        for &morse_char in MESSAGE.iter() {
            for &delay in morse_char.iter() {
                fioset |= (1 << 18);
                wait(delay);
                fioset &= (0);
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
