use crate::symbols;
use core::{
    mem::size_of,
    sync::atomic::{AtomicUsize, Ordering},
};

static TOP: AtomicUsize = AtomicUsize::new(0);

/// # Safety
/// Returns possibly uninitialized data.
pub unsafe fn alloc_one<T>() -> &'static mut T {
    let mut adr: usize = 0;
    TOP.fetch_update(Ordering::Release, Ordering::Acquire, |top| {
        let wma_bounds = symbols::wma_bounds();
        adr = top + wma_bounds.start;
        let alloc_size = size_of::<T>();
        if adr + alloc_size > wma_bounds.end {
            None
        } else {
            Some(top + alloc_size)
        }
    })
    .expect("allocating outside of wma bounds.");
    &mut *(adr as *mut T)
}

/// # Safety
/// Returns possibly uninitialized data within the slice.
pub unsafe fn alloc_many<T>(count: usize) -> &'static mut [T] {
    let mut adr: usize = 0;
    TOP.fetch_update(Ordering::Release, Ordering::Acquire, |top| {
        let wma_bounds = symbols::wma_bounds();
        adr = top + wma_bounds.start;
        let alloc_size = size_of::<T>() * count;
        if adr + alloc_size > wma_bounds.end {
            None
        } else {
            Some(top + alloc_size)
        }
    })
    .expect("allocating outside of wma bounds.");
    core::slice::from_raw_parts_mut(adr as *mut T, count)
}

pub fn init() {}
