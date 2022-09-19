use crate::once;

pub mod alloc_err;
pub mod heap;
pub mod paging;
pub mod wma;

pub fn init() {
    once! {
        wma::init();
        paging::init();
        heap::init();
    }
}
