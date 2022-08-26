pub mod alloc_err;
pub mod heap;
pub mod paging;
pub mod wma;

pub fn init() {
    wma::init();
    heap::init();
}
