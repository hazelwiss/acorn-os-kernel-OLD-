use crate::once;

pub mod console;
pub mod ps2;
pub mod serial_out;

pub fn init() {
    once!(
        console::init();
        serial_out::init();
        ps2::init();
    );
}
