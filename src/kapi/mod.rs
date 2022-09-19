use crate::once;

pub mod arch;
pub mod drivers;
pub mod keyboard;

mod hal;

pub fn init() {
    once!(
        hal::init();
        drivers::init();
    )
}
