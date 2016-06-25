#![feature(lang_items)]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub extern fn rust_main() {
	// ATTENTION: we have a very small stack and no guard page
	let hello = b"Hello World!";
	let color_byte = 0x1f; // white foreground, blue background
	
	let mut hello_colored = [color_byte; 24];
	for (i, char_byte) in hello.into_iter().enumerate() {
		hello_colored[i*2] = *char_byte;
	}

	let buffer_ptr = (0xb8000 + 1988) as *mut _;
	unsafe { *buffer_ptr = hello_colored };

	loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
	loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
