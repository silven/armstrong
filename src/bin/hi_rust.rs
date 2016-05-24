#![feature(core_slice_ext, core_str_ext, core_intrinsics)]
#![feature(no_std)]
#![feature(const_fn)]

#![no_std]
#![no_main]

extern crate armstrong;

macro_rules! wait_for {
    ($cond:expr) => {
        while ! $cond {};
    };
}

#[derive(Copy, Clone)]
struct Volatile<T> {
    value: T,
}

impl<T> Volatile<T> {
    pub fn new(initial: T) -> Volatile<T> {
        Volatile { value: initial }
    }

    #[inline]
    pub fn get(&self) -> T {
        unsafe { core::intrinsics::volatile_load(&self.value) }
    }

    #[inline]
    pub fn set(&mut self, new: T) {
        unsafe {
            core::intrinsics::volatile_store(&mut self.value, new);
        }
    }
}

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


fn uart0_init() {
    use armstrong::m3::*;

    unsafe {
        PINSEL0_REGISTER.and(!0xF0);
        PINSEL0_REGISTER.or(0b01 << 4 | 0b01 << 6);

        U0LCR_REGISTER.write(0x83);

        U0FDR_REGISTER.write(0x21);

        U0DLL_REGISTER.write(0x6C); //(div / 256) as u8);
        U0DLM_REGISTER.write(0); //(div % 256) as u8);

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

fn uart0_write(msg: &[u8]) {
    for &b in msg {
        uart0_putc(b);
    }
}

fn uart0_putc(byte: u8) {
    use armstrong::m3::*;
    unsafe {
        wait_for!(U0LSR_REGISTER.read() & 0x20 != 0);
        UART0.write(byte);
    }
}

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

fn uart0_getc() -> u8 {
    use armstrong::m3::*;
    unsafe {
        wait_for!(U0LSR_REGISTER.read() & 0x01 != 0);
        return UART0.read();
    }
}

#[no_mangle]
pub extern "C" fn start() -> ! {
    setup();
    uart0_init();

    let mut fiodir = armstrong::BasicRegister::new(0x2009C020);
    let mut fioset = armstrong::BasicRegister::new(0x2009C034);
    fiodir.write(1 << 18);

    uart0_write(b"Hello world!\n\r");
    let mut input_buffer = [0; 32];
    loop {
        uart0_write(b"Write something?\n\r");
        let bytes = uart0_read(&mut input_buffer);
        uart0_write(b"You wrote: ");
        uart0_write(&input_buffer[..bytes]);
        uart0_write(b"\n\r");
    }

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
