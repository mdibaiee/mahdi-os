extern crate multiboot2;
use multiboot2::{ MemoryMapTag };

pub fn info(information_address: usize, log: bool) -> (u64, u64, usize, usize, &'static MemoryMapTag) {
	let boot_info = unsafe { multiboot2::load(information_address) };
	let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
	
	let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");

	let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
	let kernel_end = elf_sections_tag.sections().map(|s| s.addr).max().unwrap();

	let multiboot_start = information_address;
	let multiboot_end = information_address + (boot_info.total_size as usize);

	if log {
		println!("Memory:");
		for area in memory_map_tag.memory_areas() {
			println!("start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
		}

		println!("Kernel sections:");
		for section in elf_sections_tag.sections() {
			println!("start: 0x{:x}, size: 0x{:x}, flags: 0x{:x}", section.addr, section.size, section.flags);
		}

		println!("Kernel: start 0x{:x} emd 0x{:x}", kernel_start, kernel_end);
		println!("Multiboot: start 0x{:x} emd 0x{:x}", multiboot_start, multiboot_end);
	}

	(kernel_start, kernel_end, multiboot_start, multiboot_end, memory_map_tag)
}
