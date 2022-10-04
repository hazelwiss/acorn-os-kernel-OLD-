mod alloc;
mod wma;

pub fn init() {
    wma::init();
    logi!("initialized work memory allocator.");
    alloc::init();
    logi!("initialized heap allocator.");
}
