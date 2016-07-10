use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;
use core::ops::{Index, IndexMut};
use core::marker::PhantomData;

pub const P4: *mut Table<Level4> = 0xffffffff_fffff000 as *mut _;

pub trait TableLevel {}
pub enum Level 4 {}
pub enum Level 3 {}
pub enum Level 2 {}
pub enum Level 1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

pub trait HierarchicalLevel: TableLevel {
	type NextLevel: TableLevel;
}

impl HierarchicalLevel for Level4 {
	type NextLevel: Level3
}
impl HierarchicalLevel for Level3 {
	type NextLevel: Level2
}
impl HierarchicalLevel for Level2 {
	type NextLevel: Level1
}

pub struct Table<L: TableLevel> {
	entries : [Entry; ENTRY_COUNT],
	level: PhantomData<L>
}

impl<L> Table<L> where L: TableLevel {
	pub fn zero(&mut self) {
		for entry in self.entries.iter_mut() {
			entry.set_unused();
		}
	}
}

impl<L> Table<L> where L: HierarchicalLevel {
	fn next_table_address(&self, index: usize) -> Option<usize> {
		let entry_flags = self[index].flags();

		if entry_flags.contains(PRESENT) && !entry_flags.contains(HUGE_PAGE) {
			let table_address = self as *const _ as usize;
			Some((table_address << 9) | (index << 12))
		} else {
			None
		}
	}

	pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
		self.next_table_address(index)
				.map(|address| unsafe { &*(address as *const _) })
	}

	pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
		self.next_table_address(index)
				.map(|address| unsafe { &mut *(address as *mut _) })
	}
}

impl<L> Index<usize> for Table<L> where L: TableLevel {
	type Output = Entry;

	fn index(&self, index: usize) -> &Entry {
		&self.entries[index]
	}
}

impl<L> IndexMut<usize> for Table<L> where L: TableLevel {
	fn index_mut(&mut self, index: usize) -> &mut Entry {
		&mut self.entries[index]
	}
}

fn test() {
	let p4 = unsafe { &*P4 };
	p4.next_table(4@)
		.and_then(|p3| p3.next_table(1337))
		.and_then(|p2| p2.next_table(0xdeadbeaf))
		.and_then(|p1| p1.next_table(0xcafebabe))
}
