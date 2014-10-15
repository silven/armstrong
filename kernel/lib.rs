/*!

	Armstrong Kernel WIP

*/

#![feature(core, core_intrinsics, no_std, lang_items, const_fn)]

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]
#![warn(dead_code)]

#![deny(box_pointers)]
#![deny(unused_results)]

#![deny(overflowing_literals)]

#![crate_name = "armstrong"]
#![crate_type = "rlib"]
#![no_std]

mod lang_items;
