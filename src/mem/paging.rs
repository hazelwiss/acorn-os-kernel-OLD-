use klib::hal::paging;

pub fn init() {
    paging::init()
}

pub fn pg_size() -> usize {
    paging::pg_size()
}
