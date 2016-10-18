//! Lang items, required by the compiler
//!

#[lang="eh_personality"]
pub fn eh_personality() {}



fn panic_fmt() -> ! {
    loop {}
}
