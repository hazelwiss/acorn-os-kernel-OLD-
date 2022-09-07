extern "C" {
    static __wma_start: u8;
    static __wma_end: u8;
}

pub struct Bounds {
    pub start: usize,
    pub end: usize,
}

#[inline]
pub fn wma_bounds() -> Bounds {
    unsafe {
        Bounds {
            start: &__wma_start as *const _ as usize,
            end: &__wma_end as *const _ as usize,
        }
    }
}

#[inline]
pub fn wma_size() -> usize {
    wma_bounds().end - wma_bounds().start
}
