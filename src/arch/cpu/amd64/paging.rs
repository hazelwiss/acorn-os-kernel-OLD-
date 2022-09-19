use klib::kutil::math::align_floor;

struct Paging {
    page_size: usize,
}

impl Paging {
    fn init(&self) {}

    fn alloc(&self, cnt: usize) -> usize {
        0
    }

    fn dealloc(&self, virt_adr: usize, cnt: usize) {}

    fn map(&self, virt_adr: usize, cnt: usize, phys_start: usize) {}

    fn unmap(&self, virt_adr: usize, cnt: usize) {}

    fn pg_size(&self) -> usize {
        self.page_size
    }
}

static PAGING: Paging = Paging { page_size: 4096 };

pub fn init() {
    PAGING.init()
}

pub fn alloc(cnt: usize) -> usize {
    PAGING.alloc(cnt)
}

pub fn dealloc(virt_start: usize, cnt: usize) {
    PAGING.dealloc(virt_start, cnt)
}

pub fn map(virt_start: usize, cnt: usize, phys_start: usize) {
    PAGING.map(virt_start, cnt, phys_start)
}

pub fn unmap(virt_start: usize, cnt: usize) {
    PAGING.unmap(virt_start, cnt)
}

pub fn pg_size() -> usize {
    PAGING.pg_size()
}
