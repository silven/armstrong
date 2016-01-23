/*!
    LibC-like functions
*/

#[no_mangle]
pub unsafe extern "C" fn __aeabi_memset(addr: *mut u8, val: u8, n: usize) {
    for i in 0..n {
        *addr.offset(i as isize) = val;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __aeabi_memclr(addr: *mut u8, n: usize) -> *mut u8 {
    __aeabi_memset(addr, 0u8, n);
    return addr;
}

