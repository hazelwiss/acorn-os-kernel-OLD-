use core::mem::size_of;
use klib::kutil::math::align_floor;

#[derive(Clone, Copy)]
struct PDPEntry(u64);

#[derive(Clone, Copy)]
struct PDEntry(u64);

#[derive(Clone, Copy)]
struct PTEntry(u64);

struct PDP {
    entries: [PDPEntry; 512],
}

struct PD {
    entries: [PDEntry; 512],
}

struct PT {
    entries: [PTEntry; 512],
}

static ASSERT_PDPENTRY_SIZE: () = assert!(size_of::<PDPEntry>() == 8);
static ASSERT_PDENTRY_SIZE: () = assert!(size_of::<PDEntry>() == 8);
static ASSERT_PTENTRY_SIZE: () = assert!(size_of::<PTEntry>() == 8);
static _ASSERT_TABLE_SIZES: () = assert!(
    size_of::<PT>() == size_of::<PD>()
        && size_of::<PT>() == size_of::<PDP>()
        && size_of::<PT>() == 4096
);

struct Paging {
    page_size: usize,
    top_directory: PDP,
}

impl Paging {
    fn init(&self) {}

    fn alloc(&self, cnt: usize) -> Option<*mut u8> {
        todo!()
    }

    fn dealloc(&self, virt_adr: usize, cnt: usize) -> usize {
        todo!()
    }

    fn map(&self, virt_adr: usize, cnt: usize, phys_start: usize) -> usize {
        todo!()
    }

    fn unmap(&self, virt_adr: usize, cnt: usize) -> usize {
        todo!()
    }

    fn pg_size(&self) -> usize {
        self.page_size
    }

    fn reinstall(&self) {
        todo!()
    }
}

static PAGING: Paging = Paging {
    page_size: 4096,
    top_directory: PDP {
        entries: [PDPEntry(0); 512],
    },
};

pub fn init() {
    PAGING.init()
}

pub fn alloc(cnt: usize) -> Option<*mut u8> {
    PAGING.alloc(cnt)
}

pub fn dealloc(virt_start: usize, cnt: usize) -> usize {
    PAGING.dealloc(virt_start, cnt)
}

pub fn map(virt_start: usize, cnt: usize, phys_start: usize) -> usize {
    PAGING.map(virt_start, cnt, phys_start)
}

pub fn unmap(virt_start: usize, cnt: usize) -> usize {
    PAGING.unmap(virt_start, cnt)
}

pub fn pg_size() -> usize {
    PAGING.pg_size()
}

pub fn reinstall() {
    PAGING.reinstall()
}
