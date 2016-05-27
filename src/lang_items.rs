//! Lang items, required by the compiler
//!
extern crate core;

#[lang="eh_personality"]
pub fn eh_personality() {}


#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}
