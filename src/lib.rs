#![feature(lang_items)]
#![no_std]
#![feature(unique)]
#![feature(const_fn)]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;
use vga_buffer::*;

#[no_mangle]
pub extern fn rust_main() {
	// ATTENTION: we have a very small stack and no guard page
	use core::fmt::Write;

	vga_buffer::clear_screen();
	println!("Hello, this is Mahdi OS{}", "!");

	loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
	loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
