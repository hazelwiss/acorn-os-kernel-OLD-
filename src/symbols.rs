extern "C" {
    static __wma_start: u8;
    static __wma_end: u8;
    static __kernel_load_adr: u8;
    static __kernel_virt_adr: u8;
    static __kernel_virt_end: u8;
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

pub fn kernel_load_adr() -> usize {
    unsafe { &__kernel_load_adr as *const _ as usize }
}

pub fn kernel_virt_start() -> usize {
    unsafe { &__kernel_virt_adr as *const _ as usize }
}

pub fn kernel_virt_end() -> usize {
    unsafe { &__kernel_virt_end as *const _ as usize }
}
