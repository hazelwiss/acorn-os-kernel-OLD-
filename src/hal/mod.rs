use crate::once;

pub mod console;
pub mod except;
pub mod fb;
pub mod irq;
pub mod keyboard;
pub mod paging;
pub mod serial;

pub struct ArchInterfaces {
    pub console: console::IDesc,
    pub fb: fb::IDesc,
    pub serial: serial::IDesc,
    pub irq: irq::IDesc,
    pub except: except::IDesc,
    pub keyboard: keyboard::IDesc,
    pub paging: paging::IDesc,
}

pub fn init() {
    once!(
        console::init();
        except::init();
        fb::init();
        irq::init();
        serial::init();
        keyboard::init();
    )
}
