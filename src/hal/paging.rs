use crate::once;

pub struct IDesc {
    pub init: fn(),
    pub pg_size: fn() -> usize,
    pub alloc: fn(usize) -> Option<*mut u8>,
    pub dealloc: fn(usize, usize) -> usize,
    pub map: fn(usize, usize, usize) -> usize,
    pub unmap: fn(usize, usize) -> usize,
    pub reinstall: fn(),
}

static PAGING: IDesc = klib::arch::get_arch_interfaces().paging;

pub fn init() {
    once! {
        (PAGING.init)()
    }
}

/// Returns the page size in bytes.
pub fn pg_size() -> usize {
    (PAGING.pg_size)()
}

/// Allocates `cnt` non-used pages and returns the
/// address of the allocation.
/// Returns None if no pages could be allocated
/// (no virtual or physical address space left).
pub fn alloc(cnt: usize) -> Option<*mut u8> {
    (PAGING.alloc)(cnt)
}

/// Deallocates `cnt` pages starting from the virtual
/// address `virt_adr`. Return the number of pages
/// deallocated.
pub fn dealloc(virt_adr: usize, cnt: usize) -> usize {
    (PAGING.dealloc)(virt_adr, cnt)
}

/// Maps, starting from virtual address `virt_adr`,
/// `cnt` number of pages onto physical address starting
/// at `phys_adr`. `virt_adr` and `phys_adr` will be automatically
/// floored to be page aligned.
pub fn map(virt_adr: usize, cnt: usize, phys_adr: usize) -> usize {
    (PAGING.map)(virt_adr, cnt, phys_adr)
}

/// Unmaps, starting from virtual address `virt_adr`,
/// `cnt` number of pages. `virt_adr` will be
/// automatically floored to be page alignment.
pub fn unmap(virt_adr: usize, cnt: usize) -> usize {
    (PAGING.unmap)(virt_adr, cnt)
}

pub fn reinstall() {
    (PAGING.reinstall)()
}
