use super::wma;
use core::{
    alloc::{GlobalAlloc, Layout},
    mem::size_of,
    ptr,
    slice::from_raw_parts_mut,
};
use kernel::syms;
use spin::Once;

const PAGE_SIZE: usize = platform::utils::kernel_pg_size();
const BLOCK_SIZE: usize = PAGE_SIZE;
const BLOCK_COUNT: usize = PAGE_SIZE;

#[repr(C)]
#[derive(Clone, Copy)]
struct BlocKDesc(u8);

impl BlocKDesc {
    fn new_free() -> Self {
        Self(0)
    }

    fn new_allocated() -> Self {
        Self(1)
    }

    fn is_free(&self) -> bool {
        self.0 == 0
    }

    fn is_allocated(&self) -> bool {
        !self.is_free()
    }
}

struct Heap {
    block_offset: usize,
    block_desc_offset: usize,
}

impl Heap {
    fn init(&mut self) {
        assert!(syms::wma_bounds().start % PAGE_SIZE == 0);
        assert!(syms::wma_size() % PAGE_SIZE == 0);
        assert!(size_of::<BlocKDesc>() == 1);
        let block_desc_alloc = BLOCK_COUNT;
        let block_alloc = BLOCK_COUNT * BLOCK_SIZE;
        assert!(block_alloc + block_desc_alloc < syms::wma_size());
        self.block_desc_offset =
            unsafe { wma::alloc_many::<BlocKDesc>(BLOCK_COUNT).as_ptr() as usize };
        self.block_offset =
            unsafe { wma::alloc_many::<u8>(BLOCK_COUNT * BLOCK_SIZE).as_ptr() as usize };
        for block in self.get_block_desc() {
            *block = BlocKDesc::new_free();
        }
    }

    fn get_block_desc(&self) -> &'static mut [BlocKDesc] {
        unsafe { from_raw_parts_mut(self.block_desc_offset as *mut BlocKDesc, BLOCK_COUNT) }
    }

    fn ptr_to_index<T>(&self, ptr: *const T) -> usize {
        let ptr = ptr as usize;
        assert!(ptr >= self.block_offset && ptr < self.block_offset + BLOCK_COUNT * BLOCK_SIZE);
        let ptr = ptr - self.block_offset;
        ptr / BLOCK_SIZE
    }

    fn index_to_ptr<T>(&self, index: usize) -> *mut T {
        assert!(index < BLOCK_COUNT);
        let ptr = self.block_offset + index * BLOCK_SIZE;
        ptr as *mut T
    }

    fn find_empty_consecutive(&self, count: usize) -> Option<usize> {
        let mut i = 0;
        let empty_blocks = self.get_block_desc();
        while i < BLOCK_COUNT {
            let mut found = 0;
            let start_index = i;
            while i < BLOCK_COUNT && found < count && empty_blocks[i].is_free() {
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

    fn get_block_count_from_size(&self, size: usize) -> usize {
        (size / BLOCK_SIZE) + (size % BLOCK_SIZE != 0) as usize
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        assert!(INIT.is_completed());
        let allocated_blocks = self.get_block_count_from_size(layout.size());
        let index = self.find_empty_consecutive(allocated_blocks);
        if let Some(index) = index {
            let blocks = self.get_block_desc();
            for i in 0..allocated_blocks {
                blocks[i + index] = BlocKDesc::new_allocated();
            }
            self.index_to_ptr(index)
        } else {
            loge!("Unable to allocate enough empty consecutive blocks.");
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        assert!(INIT.is_completed());
        let index = self.ptr_to_index(ptr);
        let count = self.get_block_count_from_size(layout.size());
        let blocks = self.get_block_desc();
        for i in 0..count {
            let index = index + i;
            blocks[index] = BlocKDesc::new_free();
        }
    }
}

#[global_allocator]
static mut ALLOCATOR: Heap = Heap {
    block_offset: 0,
    block_desc_offset: 0,
};

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| unsafe {
        ALLOCATOR.init();
    });
}

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("allocation error: layout {layout:?}");
}
