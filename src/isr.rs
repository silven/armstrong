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


#[naked]
#[no_mangle]
/**
  The reset handler function, sets
  up .data section in ROM, and calls
  the user-defined start() function.
*/
pub unsafe extern "C" fn reset_handler() {
    init_memory();
    start();
    hang();
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

    unsafe {

        let data_section_size = distance_between(&__data_start, &__data_end);
        ::core::ptr::copy_nonoverlapping(&__data_load as *const u32,
                                         &mut __data_start as *mut u32,
                                         data_section_size);

        let bss_section_size = distance_between(&__bss_start, &__bss_end);
        ::core::ptr::write_bytes(&mut __bss_start as *mut u32, 0x00, bss_section_size);
    }
}

fn distance_between<T>(a: *const T, b: *const T) -> usize {
    let ptr_diff = if a > b {
        a as usize - b as usize
    } else {
        b as usize - a as usize
    };
    return ptr_diff / ::core::mem::size_of::<T>();
}

mod test {
    #[test]
    fn distance_between_adjusts_for_number_of_bytes() {
        let expected_diff = 0x100; // 0x400 / 4
        let distance = super::distance_between(0x400 as *const u32, 0x800 as *const u32);
        assert_eq!(distance, expected_diff);
    }

    #[test]
    fn distance_between_calculates_positive_distance() {
        let expected_diff = 0x400;
        let distance = super::distance_between(0x800 as *const u8, 0x400 as *const u8);
        assert_eq!(distance, expected_diff);
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
