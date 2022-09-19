use crate::once;

pub struct IDesc {
    pub init: fn(),
    pub pg_size: fn() -> usize,
    pub alloc: fn(usize) -> usize,
    pub dealloc: fn(usize, usize),
    pub map: fn(usize, usize, usize),
    pub unmap: fn(usize, usize),
}

static PAGING: IDesc = klib::arch::get_arch_interfaces().paging;

pub fn init() {
    (PAGING.init)()
}

pub fn pg_size() -> usize {
    (PAGING.pg_size)()
}

pub fn alloc(cnt: usize) -> usize {
    (PAGING.alloc)(cnt)
}

pub fn dealloc(virt_adr: usize, cnt: usize) {
    (PAGING.dealloc)(virt_adr, cnt)
}

pub fn map(virt_adr: usize, cnt: usize, phys_adr: usize) {
    (PAGING.map)(virt_adr, cnt, phys_adr)
}

pub fn unmap(virt_adr: usize, cnt: usize) {
    (PAGING.unmap)(virt_adr, cnt)
}
