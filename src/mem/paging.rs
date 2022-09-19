use core::ops::RangeBounds;

use klib::hal::paging;
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
            map_range(virt_start..=virt_end, phys);
        }
    }

    fn pg_size(&self) -> usize {
        paging::pg_size()
    }

    fn map_range(&self, virt_range: impl RangeBounds<usize>, phys_start: usize) {}

    fn reinstall(&self) {}
}

static STATE: Mutex<State> = Mutex::new(State {});

pub fn init() {
    STATE.lock().init()
}

pub fn pg_size() -> usize {
    STATE.lock().pg_size()
}

pub fn map_range(virt_range: impl RangeBounds<usize>, phys_start: usize) {
    STATE.lock().map_range(virt_range, phys_start)
}

pub fn reinstall() {
    STATE.lock().reinstall()
}
