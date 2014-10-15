extern crate core;

#[lang="stack_exhausted"]
pub extern "C" fn rust_stack_exhausted() {
	unsafe { core::intrinsics::abort() }
}

#[lang="eh_personality"]
pub fn eh_personality() {}


#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
	unsafe { core::intrinsics::abort() }
}
