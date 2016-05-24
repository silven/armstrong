//! Module with ISR boot vectors
//!

extern "C" {
    fn start();
    fn _stack_start();
    fn _boot_checksum();
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
  The reset handler function, should in the future
  setup data sections in ROM, but currently only
  calls the user defined start() function.
*/
pub unsafe extern "C" fn reset_handler() {
    start()
}


/**
  ISR Handler function table.
*/
#[no_mangle]
#[allow(non_upper_case_globals)]
#[link_section=".isr_vector"]
pub static ISRVectors: [Option<unsafe extern fn()>; 16] = [
  Some(_stack_start),       // start of the stack
  Some(reset_handler),      // reset handler
  Some(hang),               // NMI handler
  Some(hang),               // hard fault handler
  Some(hang),               // MPU fault handler
  Some(hang),               // bus fault handler
  Some(hang),               // usage fault handler
  Some(_boot_checksum),     // reserved for boot checksum
  None,                     // reserved
  None,                     // reserved
  None,                     // reserved
  Some(hang),               // SV call handler
  Some(hang),               // debug monitor handler
  None,                     // reserved
  Some(hang),               // PendSV handler
  Some(hang),               // SysTick handler
];
