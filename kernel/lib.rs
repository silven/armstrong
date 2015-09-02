/*!

	Armstrong Kernel WIP

*/

#![feature(core, core_intrinsics, no_std, lang_items, const_fn)]

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]
#![warn(dead_code)]

#![deny(box_pointers)]
#![deny(unused_results)]

#![deny(overflowing_literals)]

#![crate_name = "armstrong"]
#![crate_type = "rlib"]
#![no_std]

macro_rules! regs {
    ($($name:ident => [$($field:ident: $v:ty as $mask:expr,)+],)*) => (
        $(
        pub struct $name {
            base_addr: u32,
        }

        pub mod $name {
            pub struct Value {
                $(
                    $field: Option<$v>,
                )*
            }

            impl Value {
                pub const fn new() -> Self {
                    Value {
                        $(
                            $field: None,
                        )*
                    }
                }

                #[inline(always)]
                pub fn compute(&self) -> u32 {
                    use ::core::intrinsics::cttz32;
                    let mut to_write: u32 = 0x00000000;
                    $(
                        if let Some(ref v) = self.$field {
                            let my_bits: u32 = unsafe { (v.clone() << cttz32($mask)) & $mask };
                            to_write |= my_bits;
                        }
                    )*
                    return to_write;
                }

                $(
                pub fn $field<V: Into<$v>>(mut self, value: V) -> Self {
                    self.$field = Some(value.into());
                    return self;
                }
                )*
            }
        }

        impl $name {
            pub const fn new(base: u32) -> Self {
                $name {
                    base_addr: base,
                }
            }

            pub const fn value() -> $name::Value {
                $name::Value::new()
            }

            #[inline(always)]
            pub fn write(&mut self, value: &$name::Value) {
                let to_write = value.compute();

                unsafe {
                    core::intrinsics::volatile_store(self.base_addr as *mut u32, to_write);
                }
            }

            pub unsafe fn unsafe_write(&mut self, value: u32) {
                core::intrinsics::volatile_store(self.base_addr as *mut u32, value);
            }
        }
        )*
    )
}

macro_rules! reg_enum {
    ($name:ident: $($variant:ident: $value:expr,)+) => (
        #[allow(dead_code)]
        #[derive(Copy)]
        pub enum $name {
        $(
            $variant = $value,
        )*
        }

        impl Clone for $name {
            fn clone(&self) -> $name {
                match self {
                    $(
                        &$name::$variant => $name::$variant,
                    )*
                }
            }
        }

        impl core::ops::Shl<u32> for $name {
            type Output = u32;

            fn shl(self, rhs: u32) -> Self::Output {
                (self as u32) << rhs
            }
        }

        impl core::ops::BitOr<u32> for $name {
            type Output = u32;

            fn bitor(self, rhs: u32) -> Self::Output {
                (self as u32) | rhs
            }
        }

        impl core::ops::BitAnd<u32> for $name {
            type Output = u32;

            fn bitand(self, rhs: u32) -> Self::Output {
                (self as u32) & rhs
            }
        }
    )
}

reg_enum! {
    CCLK:
        Div4: 0b00,
        Div1: 0b01,
        Div2: 0b10,
        Div8: 0b11,
}

reg_enum! {
    UART_WLen:
        five: 0b00,
        six: 0b01,
        seven: 0b10,
        eight: 0b11,
}

reg_enum! {
    UART_Parity:
        odd: 0b00,
        even: 0b01,
        force_1_sticky: 0b10,
        force_0_sticky: 0b11,
}

reg_enum! {
    UART_StopBits:
        one: 0b0,
        two: 0b1,
}

reg_enum! {
    Bool:
        on:  0b1,
        off: 0b0,
}

reg_enum! {
    IODir:
        output:  0b1,
        input: 0b0,
}


reg_enum! {
    PinMode:
        Primary:  0b00,
        First:    0b01,
        Second:   0b10,
        Third:    0b11,
}

impl core::convert::From<bool> for Bool {
    fn from(v: bool) -> Self {
        if v {
            Bool::on
        } else {
            Bool::off
        }
    }
}

pub struct Byte(u8);

impl Clone for Byte {
    fn clone(&self) -> Byte {
        Byte(self.0)
    }
}

impl core::ops::Shl<u32> for Byte {
    type Output = u32;

    fn shl(self, rhs: u32) -> Self::Output {
        let v: u32 = self.0 as u32;
        v << rhs
    }
}

impl core::ops::Shr<u32> for Byte {
    type Output = u32;

    fn shr(self, rhs: u32) -> Self::Output {
        let v: u32 = self.0 as u32;
        v >> rhs
    }
}

impl core::ops::BitOr<u32> for Byte {
    type Output = u32;

    fn bitor(self, rhs: u32) -> Self::Output {
        let v: u32 = self.0 as u32;
        v | rhs
    }
}

impl core::ops::BitAnd<u32> for Byte {
    type Output = u32;

    fn bitand(self, rhs: u32) -> Self::Output {
        let v: u32 = self.0 as u32;
        v & rhs
    }
}


regs! {
    UART => [
        out: ::Byte as 0xFF,
    ],

    UARTnDLL => [
        dllsb: ::Byte as 0xFF,
    ],

    UARTnDLM => [
        dlmsb: ::Byte as 0xFF,
    ],

    UARTnLCR => [
        word_length:             ::UART_WLen as 0b11,
        stop_bit_select:    ::UART_StopBits as 0b100,
        parity_enable:              ::Bool as 0b1000,
        parity_select:    ::UART_Parity as 0b11_0000,
        break_control:          ::Bool as 0b100_0000,
        dlab:                  ::Bool as 0b1000_0000,
    ],

    UnFCR => [
        fifo:            ::Bool as 0b1,
    ],

    UnFDR => [
        divadd:         ::Byte as 0x00FF,
        mul:            ::Byte as 0xFF00,
    ],

    FIOnDIR => [
        p0:             ::IODir as 1 << 0,
        p1:             ::IODir as 1 << 1,
        p2:             ::IODir as 1 << 2,
        p3:             ::IODir as 1 << 3,
        p4:             ::IODir as 1 << 4,
        p5:             ::IODir as 1 << 5,
        p6:             ::IODir as 1 << 6,
        p7:             ::IODir as 1 << 7,
        p8:             ::IODir as 1 << 8,
        p9:             ::IODir as 1 << 9,
        p10:            ::IODir as 1 << 10,
        p11:            ::IODir as 1 << 11,
        p12:            ::IODir as 1 << 12,
        p13:            ::IODir as 1 << 13,
        p14:            ::IODir as 1 << 14,
        p15:            ::IODir as 1 << 15,
        p16:            ::IODir as 1 << 16,
        p17:            ::IODir as 1 << 17,
        p18:            ::IODir as 1 << 18,
        p19:            ::IODir as 1 << 19,
        p20:            ::IODir as 1 << 20,
        p21:            ::IODir as 1 << 21,
        p22:            ::IODir as 1 << 22,
        p23:            ::IODir as 1 << 23,
        p24:            ::IODir as 1 << 24,
        p25:            ::IODir as 1 << 25,
        p26:            ::IODir as 1 << 26,
        p27:            ::IODir as 1 << 27,
        p28:            ::IODir as 1 << 28,
        p29:            ::IODir as 1 << 29,
        p30:            ::IODir as 1 << 30,
        p31:            ::IODir as 1 << 31,
    ],

    FIOnSET => [
        p0:             ::Bool as 1 << 0,
        p1:             ::Bool as 1 << 1,
        p2:             ::Bool as 1 << 2,
        p3:             ::Bool as 1 << 3,
        p4:             ::Bool as 1 << 4,
        p5:             ::Bool as 1 << 5,
        p6:             ::Bool as 1 << 6,
        p7:             ::Bool as 1 << 7,
        p8:             ::Bool as 1 << 8,
        p9:             ::Bool as 1 << 9,
        p10:            ::Bool as 1 << 10,
        p11:            ::Bool as 1 << 11,
        p12:            ::Bool as 1 << 12,
        p13:            ::Bool as 1 << 13,
        p14:            ::Bool as 1 << 14,
        p15:            ::Bool as 1 << 15,
        p16:            ::Bool as 1 << 16,
        p17:            ::Bool as 1 << 17,
        p18:            ::Bool as 1 << 18,
        p19:            ::Bool as 1 << 19,
        p20:            ::Bool as 1 << 20,
        p21:            ::Bool as 1 << 21,
        p22:            ::Bool as 1 << 22,
        p23:            ::Bool as 1 << 23,
        p24:            ::Bool as 1 << 24,
        p25:            ::Bool as 1 << 25,
        p26:            ::Bool as 1 << 26,
        p27:            ::Bool as 1 << 27,
        p28:            ::Bool as 1 << 28,
        p29:            ::Bool as 1 << 29,
        p30:            ::Bool as 1 << 30,
        p31:            ::Bool as 1 << 31,
    ],

    UnTER => [
        enable:         ::Bool as 0b0100_0000,
    ],

    UARTnIER => [
        rda:             ::Bool as 0b01,
        thre:            ::Bool as 0b10,
        rxls:            ::Bool as 0b100,
        // 5 bits reserved
        ABEOIntEn:       ::Bool as 0b1 << 8,
        ABTOIntEn:       ::Bool as 0b1 << 9,
    ],

    PCLKSEL0 => [
        wdt:     ::CCLK as 0b11 << 0,
        timer0:  ::CCLK as 0b11 << 2,
        timer1:  ::CCLK as 0b11 << 4,
        uart0:   ::CCLK as 0b11 << 6,
        uart1:   ::CCLK as 0b11 << 8,
        // 2 bits reserved
        pwm1:    ::CCLK as 0b11 << 12,
        i2c0:    ::CCLK as 0b11 << 14,
        spi:     ::CCLK as 0b11 << 16,
        // 2 bits reserved
        ssp1:    ::CCLK as 0b11 << 20,
        dac:     ::CCLK as 0b11 << 22,
        adc:     ::CCLK as 0b11 << 24,
        can1:    ::CCLK as 0b11 << 26,
        can2:    ::CCLK as 0b11 << 28,
        acf:     ::CCLK as 0b11 << 30,
    ],

    PCLKSEL1 => [
        qei:        ::CCLK as 0b11 << 0,
        gpio_int:   ::CCLK as 0b11 << 2,
        pcb:        ::CCLK as 0b11 << 4,
        i2c1:       ::CCLK as 0b11 << 6,
        // 2 bits reserved
        ssp0:       ::CCLK as 0b11 << 10,
        timer2:     ::CCLK as 0b11 << 12,
        timer3:     ::CCLK as 0b11 << 14,
        uart2:      ::CCLK as 0b11 << 16,
        uart3:      ::CCLK as 0b11 << 18,
        i2c2:       ::CCLK as 0b11 << 20,
        i2s:        ::CCLK as 0b11 << 22,
        // 2 bits reserved
        rit:        ::CCLK as 0b11 << 26,
        syscon:     ::CCLK as 0b11 << 28,
        mc:         ::CCLK as 0b11 << 30,
    ],

    PINSEL0 => [
        P0:         ::PinMode as 0b11 << 0,
        P1:         ::PinMode as 0b11 << 2,
        P2:         ::PinMode as 0b11 << 4,
        P3:         ::PinMode as 0b11 << 6,
        P4:         ::PinMode as 0b11 << 8,
        P5:         ::PinMode as 0b11 << 10,
        P6:         ::PinMode as 0b11 << 12,
    ],
}

pub static mut UART0: UART = UART::new(0x4020C000);
pub static mut U0FCR: UnFCR = UnFCR::new(0x4000C008);
pub static mut U0FDR: UnFDR = UnFDR::new(0x4000C028);
pub static mut U0TER: UnTER = UnTER::new(0x4000C030);

pub static mut UART0DLL: UARTnDLL = UARTnDLL::new(0x4000C000);
pub static mut UART0DLM: UARTnDLM = UARTnDLM::new(0x4000C004);

pub static mut UART0LCR: UARTnLCR = UARTnLCR::new(0x4000C00C);
pub static mut UART0IER: UARTnIER = UARTnIER::new(0x4000C004);

pub static mut FIO1DIR: FIOnDIR = FIOnDIR::new(0x2009C020);
pub static mut FIO1SET: FIOnSET = FIOnSET::new(0x2009C034);

pub static mut PCLKSEL0: PCLKSEL0 = PCLKSEL0::new(0x400FC1A8);
pub static mut PCLKSEL1: PCLKSEL1 = PCLKSEL1::new(0x400FC1AC);
pub static mut PINSEL0: PINSEL0 = PINSEL0::new(0x4002C000);

mod lang_items;
