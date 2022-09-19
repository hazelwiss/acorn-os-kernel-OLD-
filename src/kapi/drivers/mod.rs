use crate::once;

pub mod console;
pub mod irq;
pub mod ps2;
pub mod serial_out;

pub trait Driver {
    fn init(&self);
    fn name(&self) -> &'static str;
    fn arches(&self) -> Option<()> {
        None
    }
}

pub fn init() {
    once!(
        console::init();
        serial_out::init();
        ps2::init();
    );
}
