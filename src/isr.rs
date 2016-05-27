//! Module with ISR boot vectors
//!

extern "C" {
    fn start();
    fn __stack_start();
    fn __boot_checksum();
}


#[no_mangle]
/**
  Hang function, loops for ever
*/
pub unsafe extern "C" fn hang() {
    loop {}
}


#[no_mangle]
/**
  The reset handler function, sets
  up .data section in ROM, and calls
  the user-defined start() function.
*/
pub unsafe extern "C" fn reset_handler() {
    init_memory();
    start();
}

/**
  Initializes memory by moving .data section
  from FLASH into RAM, and zeroes out the .bss
  section.
*/
fn init_memory() {
    extern "C" {
        static __data_load: u32;
        static mut __data_start: u32;
        static __data_end: u32;

        static mut __bss_start: u32;
        static __bss_end: u32;
    }

    use ::core::intrinsics::{volatile_copy_nonoverlapping_memory, volatile_set_memory};
    unsafe {
        let data_bytes_delta = (&__data_end as *const u32 as usize) -
                               (&__data_start as *const u32 as usize);
        let data_section_size = data_bytes_delta / ::core::mem::size_of::<u32>();
        volatile_copy_nonoverlapping_memory(&mut __data_start as *mut u32,
                                            &__data_load as *const u32,
                                            data_section_size);

        let bss_bytes_delta = (&__bss_end as *const u32 as usize) -
                              (&__bss_start as *const u32 as usize);
        let bss_section_size = bss_bytes_delta / ::core::mem::size_of::<u32>();
        volatile_set_memory(&mut __bss_start as *mut u32, 0x00, bss_section_size);
    }
}


/**
  ISR Handler function table.
*/
#[no_mangle]
#[allow(non_upper_case_globals)]
#[link_section=".isr_vector"]
pub static ISRVectors: [Option<unsafe extern fn()>; 16] = [
  Some(__stack_start),       // start of the stack
  Some(reset_handler),      // reset handler
  Some(hang),               // NMI handler
  Some(hang),               // hard fault handler
  Some(hang),               // MPU fault handler
  Some(hang),               // bus fault handler
  Some(hang),               // usage fault handler
  Some(__boot_checksum),     // reserved for boot checksum
  None,                     // reserved
  None,                     // reserved
  None,                     // reserved
  Some(hang),               // SV call handler
  Some(hang),               // debug monitor handler
  None,                     // reserved
  Some(hang),               // PendSV handler
  Some(hang),               // SysTick handler
];
