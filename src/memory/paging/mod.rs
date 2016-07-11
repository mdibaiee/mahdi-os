mod entry;
mod table;

use memory::PAGE_SIZE;
use memory::Frame;
pub use self::entry::*;
use memory::FrameAllocator;
use self::table::*;

const ENTRY_COUNT: usize = 512;

pub fn map_to<A>(page: Page, frame: Frame, flags: EntryFlags, allocator: &mut A)
								where A: FrameAllocator
{
	let p4 = unsafe { &mut *P4 };
	let mut p3 = p4.next_table_create(page.p4_index(), allocator);
	let mut p2 = p3.next_table_create(page.p3_index(), allocator);
	let mut p1 = p2.next_table_create(page.p2_index(), allocator);

	assert!(p1[page.p1_index()].is_unused());
	p1[page.p1_index()].set(frame, flags | PRESENT);
}

pub fn translate(virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
	let offset = virtual_address & PAGE_SIZE;
	translate_page(Page::containing_address(virtual_address))
								.map(|frame| frame.number * PAGE_SIZE + offset)
}

pub fn translate_page(page: Page) -> Option<Frame> {
	use self::entry::HUGE_PAGE;

	let p3 = unsafe { &*table::P4 }.next_table(page.p4_index());
	let huge_page = || {
		p3.and_then(|p3| {
			let p3_entry = &p3[page.p3_index()];
			
			// 1GiB page?
			if let Some(start_frame) = p3_entry.pointed_frame() {
				if p3_entry.flags().contains(HUGE_PAGE) {
					// address must be 1GiB aligned
					assert!(start_frame.number % (ENTRY_COUNT * ENTRY_COUNT) == 0);
					return Some(Frame {
						number: start_frame.number + page.p2_index() * ENTRY_COUNT + page.p1_index(),
					});
				}
			}

			if let Some(p2) = p3.next_table(page.p3_index()) {
				let p2_entry = &p2[page.p2_index()];
				// 2MiB page?
				if let Some(start_frame) = p2_entry.pointed_frame() {
					if p2_entry.flags().contains(HUGE_PAGE) {
						// address must be 2MiB aligned
						assert!(start_frame.number % ENTRY_COUNT == 0);
						return Some(Frame { number: start_frame.number + page.p1_index() });
					}
				}
			}

			None
		})
	};

	p3.and_then(|p3| p3.next_table(page.p3_index()))
	  .and_then(|p2| p2.next_table(page.p2_index()))
		.and_then(|p1| p1[page.p1_index()].pointed_frame())
		.or_else(huge_page)
}

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
	number: usize,
}

impl Page {
  pub fn containing_address(address: VirtualAddress) -> Page {
	  assert!(address < 0x0000_8000_0000_0000 ||
					  address >= 0xffff_8000_0000_0000,
					  "invalid address: 0x{:x}", address);

  	Page { number: address / PAGE_SIZE }
  }

	fn start_address(&self) -> usize {
		self.number * PAGE_SIZE
	}

	fn p4_index(&self) -> usize {
		(self.number >> 27) & 0o777
	}

	fn p3_index(&self) -> usize {
		(self.number >> 18) & 0o777
	}

	fn p2_index(&self) -> usize {
		(self.number >> 9) & 0o777
	}

	fn p1_index(&self) -> usize {
		(self.number >> 0) & 0o777
	}
}

/*
pub struct Entry(u64);

impl Entry {
	pub fn is_unused(&self) -> bool {
		self.0 == 0
	}

	pub fn set_unused(&mut self) {
		self.0 = 0;
	}

	pub fn flags(&self) -> EntryFlags {
		EntryFlags::from_bits_truncate(self.0)
	}

	pub fn pointed_frame(&self) -> Option<Frame> {
		if self.flags().contains(PRESENT) {
			Some(Frame::containing_address(
					self.0 as usize & 0x000fffff_fffff000
			))
		} else {
			None
		}
	}

	pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
		assert!(frame.start_address() & !0x000fffff_fffff000 == 0);
		self.0 = (frame.start_address() as u64) | flags.bits();
	}
}

bitflags! {
	pub flags EntryFlags: u64 {
		const PRESENT            = 1 << 0,
		const WRITABLE           = 1 << 1,
		const USER_ACCESSABLE    = 1 << 2,
		const WRITE_THROUGH      = 1 << 3,
		const NO_CACHE					 = 1 << 4,
		const ACCESSED           = 1 << 5,
		const DIRTY              = 1 << 6,
		const HUGE_PAGE          = 1 << 7,
		const GLOBAL						 = 1 << 8,
		const NO_EXECUTE				 = 1 << 63,
	}
}
*/
