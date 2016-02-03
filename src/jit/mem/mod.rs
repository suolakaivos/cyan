extern crate libc;

use self::libc::{PROT_EXEC, PROT_READ, PROT_WRITE};
use std::ptr;
use std::mem;

const EXEC_READ_WRITE: i32 = PROT_EXEC | PROT_READ | PROT_WRITE;
const PAGE_SIZE: usize = 4096;

extern {
	fn mmap(
		addr: *const libc::c_char,
		length: libc::size_t,
		prot: libc::c_int,
		flags: libc::c_int,
		fd: libc::c_int,
		offset: libc::size_t
	) -> *mut u8;

	fn munmap(
		addr: *mut u8,
		length: libc::size_t
	);

	fn memset(
		ptr: *mut libc::c_void,
		value: libc::uint32_t,
		size: libc::size_t
	) -> *mut libc::c_void;
}

#[derive(Debug)]
pub struct MappedRegion {
	pub region: *mut u8,
	offset: usize,
	pages: usize,
}

impl MappedRegion {
	pub fn new(pages: usize) -> MappedRegion {
		let region: *mut u8;

		unsafe {
			let size = pages * PAGE_SIZE;
			let mut contents = mem::uninitialized();

			libc::posix_memalign(&mut contents, PAGE_SIZE, pages);
			libc::mprotect(contents, pages, EXEC_READ_WRITE);

			memset(contents, 0x00, pages);
			region = mem::transmute(contents);
		}

		MappedRegion {
			region: region,
			pages: pages,
			offset: 0
		}
	}

	pub fn push(&mut self, op: u8) {
		let offset = self.offset;
		self[offset] = op;
		self.offset += 1;
	}
}

impl Drop for MappedRegion {
	fn drop(&mut self) {
		unsafe { libc::free(mem::transmute(self.region)); }
	}
}

impl ::std::ops::Index<usize> for MappedRegion {
	type Output = u8;

	fn index(&self, idx: usize) -> &Self::Output {
		unsafe { &*self.region.offset(idx as isize) }
	}
}

impl ::std::ops::IndexMut<usize> for MappedRegion {
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
		unsafe { &mut *self.region.offset(idx as isize) }
	}
}

impl<'a> From<&'a [u8]> for MappedRegion {
	fn from(ops: &'a [u8]) -> MappedRegion {
		let pages = ops.len() / PAGE_SIZE + 1;
		let mut region = MappedRegion::new(pages);

		for op in ops {
			region.push(*op);
		}

		region
	}
}