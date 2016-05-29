//! Lang items, required by the compiler
//!

#[lang="eh_personality"]
pub fn eh_personality() {}


#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}
