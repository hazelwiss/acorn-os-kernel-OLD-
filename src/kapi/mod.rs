use crate::once;

pub mod arch;
pub mod drivers;
pub mod hal;

pub fn init() {
    once!(
        drivers::init();
        hal::init();
    )
}
