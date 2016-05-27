//! LibC-like functions
//!

#[no_mangle]
#[allow(missing_docs)]
pub unsafe extern "C" fn __aeabi_memset(addr: *mut u8, val: u8, n: usize) {
    for i in 0..n {
        *addr.offset(i as isize) = val;
    }
}

#[no_mangle]
#[allow(missing_docs)]
pub unsafe extern "C" fn __aeabi_memcpy(dest: *mut u8, src: *const u8, n: usize) {
    for i in 0..n {
        *dest.offset(i as isize) = *src.offset(i as isize);
    }
}

#[no_mangle]
#[allow(missing_docs)]
pub unsafe extern "C" fn __aeabi_memclr(addr: *mut u8, n: usize) {
    __aeabi_memset(addr, 0u8, n);
}

#[no_mangle]
#[allow(missing_docs)]
pub unsafe extern "C" fn __aeabi_memcmp(a: *const u8, b: *const u8, n: usize) -> u32 {
    for i in 0..n {
        let at_a = *a.offset(i as isize) as u32;
        let at_b = *b.offset(i as isize) as u32;
        let delta = at_a - at_b;
        if delta != 0 {
            return delta;
        }
    }
    return 0;
}
