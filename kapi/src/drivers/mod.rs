use kutil::once;

pub mod console;
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
    );
}
