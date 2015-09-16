/*!
    Lang items, required by the compiler
*/
extern crate core;

#[lang="stack_exhausted"]
pub extern "C" fn rust_stack_exhausted() -> !{
	loop {}
}

#[lang="eh_personality"]
pub fn eh_personality() {}


#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
	loop {}
}
