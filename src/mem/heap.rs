use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
};

use lazy_static::lazy_static;
use spin::Mutex;

extern "C" {
    static __heap_size: usize;
    static __heap_start: usize;
    static __heap_end: usize;
}

const BLOCKS: usize = 4096;
const PAGE_SIZE: usize = 4096;

lazy_static! {
    static ref HEAP_START: usize = unsafe { (&__heap_start as *const _) as usize };
    static ref HEAP_END: usize = unsafe { (&__heap_end as *const _) as usize };
    static ref HEAP_SIZE: usize = unsafe { (&__heap_size as *const _) as usize };
}

struct Heap;

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let block_count = get_block_count(layout.size());
        let index = find_empty_consecutive_blocks(block_count);
        if let Some(index) = index {
            let ptr = *HEAP_START + index * PAGE_SIZE;
            let mut free_blocks = EMPTY_BLOCKS.lock();
            for i in 0..block_count {
                free_blocks[i + index] = false;
            }
            ptr as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let adr = ptr as usize;
        let size = layout.size();
        assert!(
            adr >= *HEAP_START && adr + size < *HEAP_END,
            "attempting to deallocate outside of heap range with the adr 0x{adr:X} and size {size}"
        );
        let index = (adr - *HEAP_START) / PAGE_SIZE;
        let count = get_block_count(size);
        let mut free_blocks = EMPTY_BLOCKS.lock();
        for i in 0..count {
            free_blocks[i + index] = true;
        }
    }
}

fn find_empty_consecutive_blocks(count: usize) -> Option<usize> {
    let mut i = 0;
    let empty_blocks = EMPTY_BLOCKS.lock();
    while i < BLOCKS {
        let mut found = 0;
        let start_index = i;
        while empty_blocks[i] && found < count && i < BLOCKS {
            found += 1;
            i += 1;
        }
        if found >= count {
            return Some(start_index);
        }
        i += (found == 0) as usize;
    }
    None
}

fn get_block_count(size: usize) -> usize {
    (size / PAGE_SIZE) + (size % PAGE_SIZE != 0) as usize
}

#[global_allocator]
static ALLOCATOR: Heap = Heap {};

static EMPTY_BLOCKS: Mutex<[bool; BLOCKS]> = Mutex::new([true; BLOCKS]);

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("allocation error with layout {layout:?}")
}
