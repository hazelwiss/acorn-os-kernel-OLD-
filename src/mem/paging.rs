use core::ops::RangeBounds;
use klib::{
    hal::paging,
    kutil::math::{align_floor, align_up},
};
use spin::Mutex;

use crate::{once, symbols};

struct State {}

impl State {
    fn init(&self) {
        once! {
            paging::init();
            let virt_start = symbols::kernel_virt_start();
            let virt_end = symbols::kernel_virt_end();
            let phys = symbols::kernel_load_adr();
            self.map_range(virt_start..=virt_end, phys);
        }
    }

    fn pg_size(&self) -> usize {
        paging::pg_size()
    }

    #[inline]
    fn map_range(&self, virt_range: impl RangeBounds<usize>, phys_start: usize) {
        let pgs = self.pg_size();
        let virt_beg = match virt_range.start_bound() {
            core::ops::Bound::Included(&v) => v,
            core::ops::Bound::Excluded(&v) => v - 1,
            core::ops::Bound::Unbounded => 0,
        };
        let virt_end = match virt_range.end_bound() {
            core::ops::Bound::Included(&v) => v,
            core::ops::Bound::Excluded(&v) => v - 1,
            core::ops::Bound::Unbounded => usize::MAX,
        };
        let virt_beg = align_floor(&pgs, &virt_beg);
        let virt_end = align_up(&pgs, &virt_end);
        let bytes = virt_end - virt_end;
        let cnt = bytes / pgs;
        paging::map(virt_beg, cnt, phys_start);
    }

    fn reinstall(&self) {
        paging::reinstall();
    }
}

static STATE: Mutex<State> = Mutex::new(State {});

pub fn init() {
    STATE.lock().init()
}

pub fn pg_size() -> usize {
    STATE.lock().pg_size()
}

/// Maps the page aligned address range of `virt_range` starting from
/// `phys_start` in physical memory.
pub fn map_range(virt_range: impl RangeBounds<usize>, phys_start: usize) {
    STATE.lock().map_range(virt_range, phys_start)
}

pub fn reinstall() {
    STATE.lock().reinstall()
}
