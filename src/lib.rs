#![feature(lang_items)]
#![no_std]
#![feature(unique)]
#![feature(const_fn)]

#[macro_use]
extern crate bitflags;
extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;
use vga_buffer::*;

mod debug;
use debug::info;

mod memory;
use memory::*;

#[no_mangle]
pub extern fn rust_main(information_address: usize) {
	// ATTENTION: we have a very small stack and no guard page
	use core::fmt::Write;

	vga_buffer::clear_screen();
	println!("Hello, this is Mahdi OS{}", "!");

	// show information about memory and kernel sections
	let (kernel_start, kernel_end, multiboot_start, multiboot_end, memory_map_tag) = info(information_address, false);
	let mut frame_allocator = memory::AreaFrameAllocator::new(
		kernel_start as usize, kernel_end as usize, multiboot_start, multiboot_end,
		memory_map_tag.memory_areas());

	for i in 0.. {
		if let None = frame_allocator.allocate_frame() {
			println!("allocated {} frames", i);
			break;
		}
	}

	loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
	loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
	vga_buffer::WRITER.lock().set_color(Color::LightRed, Color::Black);

	println!("");
	println!("DAMN, PANIC in {}:{}", file, line);
	println!("      {}", fmt);
	loop {}
}
